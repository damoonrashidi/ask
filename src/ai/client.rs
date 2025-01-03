use serde_json::Value;
use std::sync::mpsc::Receiver;

use crate::config::Config;
use futures_util::StreamExt;

#[derive(Debug)]
pub struct AI;

impl AI {
    /// ask
    ///
    /// returns a list of Strings representing the shell commands for
    /// # Panics
    ///
    /// This function will panic if the remote resource could not be reached.
    #[must_use]
    pub fn ask(question: &str, shell: &String) -> Receiver<(u8, String)> {
        let config = Config::get_or_default();
        let url = config.ai.provider.get_url();
        let system_prompt = format!(
            "You are a helpful {shell} code snippet generator.
        You will be provided a description of the requested {shell}
        command and you should output the {shell} command and nothing else.
        Your response should be strictly a string with the command,
        do not add backticks, not json or any other format. Do not add any formatting."
        );
        let body = config
            .ai
            .provider
            .get_request_body(&config.ai.model, &system_prompt, question);

        let (tx, rx) = std::sync::mpsc::channel();

        for i in 0..config.command.choice_count {
            let tx = tx.clone();
            let body = body.clone();
            let url = url.clone();

            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let mut stream = client
                    .post(url)
                    .header(
                        config.ai.provider.get_api_key_header(),
                        config.ai.provider.get_api_key_value(),
                    )
                    .header("anthropic-version", "2023-06-01")
                    .json(&body)
                    .send()
                    .await
                    .expect("could not send")
                    .bytes_stream()
                    .filter_map(|result| futures_util::future::ready(result.ok()));

                while let Some(chunk) = stream.next().await {
                    let Ok(text) = String::from_utf8(chunk.to_vec()) else {
                        continue;
                    };

                    for line in text.lines() {
                        if let Some(content) = match config.ai.provider {
                            crate::config::AIProvider::Ollama => AI::parse_ollama_chunk(line),
                            crate::config::AIProvider::OpenAI => AI::parse_openai_chunk(line),
                            crate::config::AIProvider::Anthropic => AI::parse_anthropic_chunk(line),
                        } {
                            tx.send((i, content.to_string())).ok();
                        }
                    }
                }
            });
        }

        rx
    }

    fn parse_openai_chunk(chunk: &str) -> Option<String> {
        let stripped = chunk.trim().strip_prefix("data: ")?;
        let parsed = serde_json::from_str::<Value>(stripped).ok()?;

        parsed["choices"][0]["delta"]["content"]
            .as_str()
            .map(String::from)
    }

    fn parse_ollama_chunk(chunk: &str) -> Option<String> {
        serde_json::from_str::<Value>(chunk)
            .ok()
            .and_then(|json| json["message"]["content"].as_str().map(String::from))
    }

    fn parse_anthropic_chunk(chunk: &str) -> Option<String> {
        let stripped = chunk.trim().strip_prefix("data: ")?;
        let parsed = serde_json::from_str::<Value>(stripped).ok()?;

        parsed.get("delta")?.get("text")?.as_str().map(String::from)
    }
}

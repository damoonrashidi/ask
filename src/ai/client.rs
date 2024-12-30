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
    ///
    /// # Errors
    ///
    /// Errors if the response could not be parsed.
    #[must_use]
    pub fn ask(question: &String, shell: &String) -> Receiver<(u8, String)> {
        let config = Config::get_or_default();
        let url = config.ai.provider.get_url();
        let system_prompt = format!(
            "You are a helpful {shell} code snippet generator.
        You will be provided a description of the requested {shell}
        command and you should output the {shell} command and nothing else.
        Your response should be strictly a string with the command,
        do not add backticks, not json or any other format. Do not add any formatting."
        );
        let body = serde_json::json!({
            "model": config.ai.model,
            "temperature": 1.0,
            "stream": true,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": question}
            ],
        });

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
                        format!("Bearer {}", config.ai.provider.get_api_key_value()),
                    )
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

                    match config.ai.provider {
                        crate::config::AIProvider::Ollama => {
                            for line in text.lines() {
                                let Some(content) =
                                    serde_json::from_str::<Value>(line).ok().and_then(|json| {
                                        let message = json.get("message")?.get("content")?;
                                        message.as_str().map(String::from)
                                    })
                                else {
                                    continue;
                                };
                                tx.send((i, content.to_string())).unwrap();
                            }
                        }
                        crate::config::AIProvider::OpenAI => {
                            for line in text.lines() {
                                let Some(content) = AI::parse_openai_chunk(line) else {
                                    continue;
                                };
                                tx.send((i, content.to_string())).unwrap();
                            }
                        }
                        crate::config::AIProvider::Anthropic => todo!(),
                    };
                }
            });
        }

        rx
    }

    fn parse_openai_chunk(chunk: &str) -> Option<String> {
        let stripped = chunk.trim().strip_prefix("data: ")?;
        let parsed = serde_json::from_str::<Value>(stripped).ok()?;

        parsed
            .get("choices")?
            .get(0)?
            .get("delta")?
            .get("content")?
            .as_str()
            .map(String::from)
    }
}

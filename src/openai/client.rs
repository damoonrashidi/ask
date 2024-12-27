use anyhow::Error;

use crate::{
    config::Config,
    openai::response::{OpenAIResponse, Role},
};

#[derive(Debug)]
pub struct OpenAI;

impl OpenAI {
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
    pub fn ask(
        &self,
        question: &String,
        shell: &String,
    ) -> anyhow::Result<Vec<String>, anyhow::Error> {
        const URL: &str = "http://localhost:11434/v1/chat/completions";
        let config = Config::get_or_default();

        let body = serde_json::json!({
            "model": config.command.model,
            "n": config.command.choice_count,
            "messages": [
                {
                "role": Role::System,
                "content": format!(
                    r#"
You are a helpful {shell} code snippet generator.
You will be provided a description of the requested {shell}
command and you should output the {shell} command and nothing else.
Your response should be strictly a string with the command, no backticks, not json or anything else. Just the command."#,
                ),
            },
            {
                "role": Role::User,
                "content": question,
            }
            ],
        });

        let result = reqwest::blocking::Client::new()
            .post(URL)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .expect("remote resource could not be reached")
            .json::<OpenAIResponse>()
            .map_err(|_| Error::msg("Failed tpo parse AI response"))?
            .choices
            .into_iter()
            .map(|choice| choice.message.content)
            .collect();

        Ok(result)
    }
}

use anyhow::Error;

use crate::openai::response::{OpenAIResponse, Role};

#[derive(Debug)]
pub struct OpenAI {
    api_key: String,
}

impl OpenAI {
    #[must_use]
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

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
        const URL: &str = "https://api.openai.com/v1/chat/completions";

        let body = serde_json::json!({
            "model": "gpt-4-1106-preview",
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

        let Ok(response) = reqwest::blocking::Client::new()
            .post(URL)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .expect("remote resource could not be reached")
            .json::<OpenAIResponse>()
        else {
            return Err(Error::msg("Failed to parse OpenAI response"));
        };

        Ok(response
            .choices
            .into_iter()
            .map(|choice| choice.message.content)
            .collect())
    }
}

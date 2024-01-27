use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize, Clone)]
struct Message {
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.openai.com/v1/chat/completions";
    let shell_var = env::var("SHELL")?;
    let is_nushell = env::var("_")?.contains("bin/nu");
    let shell = if is_nushell {
        "nushell"
    } else {
        shell_var.split('/').last().unwrap_or("bash")
    };

    let openai_key = env::var("OPENAI_KEY").expect("No OPENAI_KEY found in environment variables");
    let question = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let body = json!({
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

    let response = reqwest::blocking::Client::new()
        .post(URL)
        .header("Authorization", format!("Bearer {openai_key}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .expect("coult not get a response from OpenAI");

    if response.status().is_success() {
        if let Some(message) = response
            .json::<OpenAIResponse>()
            .expect("could not decode json")
            .choices
            .first()
            .map(|choice| choice.message.clone())
        {
            println!("{}", message.content);
        }
    } else {
        dbg!(response.text()?);
    }

    Ok(())
}

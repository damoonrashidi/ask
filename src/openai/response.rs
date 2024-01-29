use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIResponse {
    pub(crate) choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    pub(crate) message: Message,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Message {
    pub(crate) content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

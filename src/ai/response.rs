use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(super) struct AIRequest {
    pub(super) model: String,
    pub(super) created_at: String,
    pub(super) response: AIResponse,
    pub(super) done: bool,
    pub(super) done_reason: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct AIResponse {
    pub(super) model: String,
    pub(super) created: usize,
    pub(super) choice: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(super) struct Choice {
    pub(super) index: u32,
    pub(super) message: Message,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Clone)]
pub(super) struct Message {
    pub(super) role: Role,
    pub(super) content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

/*
"{\"model\":\"mistral-nemo\",
\"created_at\":\"2024-12-28T12:53:27.33538Z\",
\"response\":\"\",
\"done\":true,\
"done_reason\":\"load\"}
*/
/*
{
    "id":"chatcmpl-266",
    "object":"chat.completion",
    "created":1735376290,
    "model":"mistral-nemo",
    "system_fingerprint":"fp_ollama",
    "choices":[
        {"index":0,"message":
        {"role":"assistant",
        "content":"find -size +4M"},
        "finish_reason":"stop"}
    ],
    "usage":{
        "prompt_tokens":89,
        "completion_tokens":7,
        "total_tokens":96
    }
}
*/

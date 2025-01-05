use super::Message;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatCompletionsResponse {
    pub choices: Vec<ChatCompletion>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub system_fingerprint: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatCompletion {
    pub finish_reason: String,
    pub index: u64,
    pub logprobs: Option<serde_json::Value>,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Usage {
    pub completion_tokens: u64,
    pub prompt_cache_hit_tokens: u64,
    pub prompt_cache_miss_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}

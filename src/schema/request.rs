use crate::errors::{Error, Result};
use crate::schema::tool::{Tool, ToolChoice};
use crate::schema::Message;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StreamOptions {
    #[serde(default)]
    pub include_usage: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ChatCompletionsRequest {
    pub messages: Vec<Message>,
    pub model: String,
    #[serde(default)]
    pub frequency_penalty: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u64>,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    pub response_format: Option<serde_json::Value>,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub stream: Option<bool>,
    #[serde(default)]
    pub stream_options: Option<StreamOptions>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub tools: Option<Vec<Tool>>,
    #[serde(default)]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default)]
    pub logprobs: Option<bool>,
    #[serde(default)]
    pub top_logprobs: Option<u8>,
}

impl ChatCompletionsRequest {
    pub fn builder() -> ChatCompletionsRequestBuilder {
        ChatCompletionsRequestBuilder::new()
    }
}

#[derive(Default, Debug)]
pub struct ChatCompletionsRequestBuilder {
    messages: Option<Vec<Message>>,
    model: Option<String>,
    frequency_penalty: Option<f32>,
    max_tokens: Option<u64>,
    presence_penalty: Option<f32>,
    response_format: Option<serde_json::Value>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    stream_options: Option<StreamOptions>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    logprobs: Option<bool>,
    top_logprobs: Option<u8>,
}

impl ChatCompletionsRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Required
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = Some(messages);
        self
    }

    /// Required
    pub fn model<S: Into<String>>(mut self, model: S) -> Self {
        self.model = Some(model.into().to_lowercase());
        self
    }

    /// Respond in JSON format
    pub fn json(mut self) -> Self {
        self.response_format = Some(serde_json::json!({"type": "json_object"}));
        self
    }

    pub fn frequency_penalty(mut self, freq: f32) -> Self {
        self.frequency_penalty = Some(freq);
        self
    }

    pub fn max_tokens(mut self, max: u64) -> Self {
        self.max_tokens = Some(max);
        self
    }

    /// Range: [-2.0, 2.0]
    pub fn presence_penalty(mut self, presence: f32) -> Self {
        self.presence_penalty = Some(presence);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    /// If set to true, partial message deltas are streamed as SSE.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn stream_options(mut self, options: StreamOptions) -> Self {
        self.stream_options = Some(options);
        self
    }

    /// Range: [0, 2].
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Range: [0, 1].
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    /// Up to 20
    pub fn top_logprobs(mut self, top_logprobs: u8) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn build(self) -> Result<ChatCompletionsRequest> {
        if self.messages.is_none() || self.model.is_none() {
            return Err(Error::BuildFailed(
                "Missing required fields: 'messages' and/or 'model'",
            ));
        }

        Ok(ChatCompletionsRequest {
            messages: self.messages.unwrap(),
            model: self.model.unwrap(),
            frequency_penalty: self.frequency_penalty,
            max_tokens: self.max_tokens,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
        })
    }
}

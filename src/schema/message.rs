use super::tool::ToolCall;
use crate::errors::{Error, Result};

use serde::{Deserialize, Serialize};
use std::fmt;

// TODO: Add "prefix" mode for assistant (requires base url to be set to beta)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Assistant,
    System,
    User,
    Tool,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

#[derive(Default, Debug)]
pub struct MessageBuilder {
    content: Option<String>,
    role: Option<Role>,
    name: Option<String>,
    tool_call_id: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> MessageBuilder {
        self.content = Some(content.into());
        self
    }

    pub fn role<S: Into<String>>(mut self, role: S) -> Result<MessageBuilder> {
        let role = serde_json::from_str::<Role>(&format!("\"{}\"", role.into().to_lowercase()))?;
        self.role = Some(role);
        Ok(self)
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> MessageBuilder {
        self.name = Some(name.into());
        self
    }

    pub fn tool_call_id<S: Into<String>>(mut self, id: S) -> MessageBuilder {
        self.tool_call_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<Message> {
        if self.content.is_none() || self.role.is_none() {
            return Err(Error::BuildFailed(
                "Building message failed because there is 1 or missing paramters",
            ));
        }

        let content = self.content.unwrap();
        let role = self.role.unwrap().to_string();

        let message = Message {
            content,
            role,
            name: self.name,
            tool_calls: None,
            tool_call_id: self.tool_call_id,
        };

        Ok(message)
    }
}

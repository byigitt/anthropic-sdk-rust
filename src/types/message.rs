//! Message types.

use serde::{Deserialize, Serialize};

use super::{ContentBlock, ContentBlockParam, Usage};

/// The role of a message participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// User message.
    User,
    /// Assistant message.
    Assistant,
}

/// The reason the model stopped generating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// The model reached a natural stopping point.
    EndTurn,
    /// The model reached the maximum number of tokens.
    MaxTokens,
    /// The model generated a stop sequence.
    StopSequence,
    /// The model made a tool use request.
    ToolUse,
    /// The model was paused (for extended thinking).
    PauseTurn,
    /// The model refused to generate content.
    Refusal,
}

/// A message response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique object identifier.
    pub id: String,

    /// Object type, always "message".
    #[serde(rename = "type")]
    pub object_type: String,

    /// The role of the message, always "assistant".
    pub role: Role,

    /// The content of the message.
    pub content: Vec<ContentBlock>,

    /// The model that generated the message.
    pub model: String,

    /// The reason the model stopped generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<StopReason>,

    /// The stop sequence that caused the model to stop, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,

    /// Token usage information.
    pub usage: Usage,
}

impl Message {
    /// Get the text content of the message.
    pub fn text(&self) -> String {
        self.content
            .iter()
            .filter_map(|block| block.as_text())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get all text blocks from the message.
    pub fn text_blocks(&self) -> Vec<&str> {
        self.content
            .iter()
            .filter_map(|block| block.as_text())
            .collect()
    }

    /// Get all tool use blocks from the message.
    pub fn tool_uses(&self) -> Vec<(&str, &str, &serde_json::Value)> {
        self.content
            .iter()
            .filter_map(|block| block.as_tool_use())
            .collect()
    }

    /// Check if the message contains any tool use requests.
    pub fn has_tool_use(&self) -> bool {
        self.content.iter().any(|block| block.is_tool_use())
    }

    /// Check if the model stopped due to tool use.
    pub fn stopped_for_tool_use(&self) -> bool {
        self.stop_reason == Some(StopReason::ToolUse)
    }
}

/// A message parameter for API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageParam {
    /// The role of the message.
    pub role: Role,

    /// The content of the message.
    pub content: MessageContent,
}

impl MessageParam {
    /// Create a user message with text content.
    pub fn user(content: impl Into<String>) -> Self {
        MessageParam {
            role: Role::User,
            content: MessageContent::Text(content.into()),
        }
    }

    /// Create a user message with multiple content blocks.
    pub fn user_with_blocks(blocks: Vec<ContentBlockParam>) -> Self {
        MessageParam {
            role: Role::User,
            content: MessageContent::Blocks(blocks),
        }
    }

    /// Create an assistant message with text content.
    pub fn assistant(content: impl Into<String>) -> Self {
        MessageParam {
            role: Role::Assistant,
            content: MessageContent::Text(content.into()),
        }
    }

    /// Create an assistant message with multiple content blocks.
    pub fn assistant_with_blocks(blocks: Vec<ContentBlockParam>) -> Self {
        MessageParam {
            role: Role::Assistant,
            content: MessageContent::Blocks(blocks),
        }
    }
}

/// Message content, either text or multiple blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content.
    Text(String),

    /// Multiple content blocks.
    Blocks(Vec<ContentBlockParam>),
}

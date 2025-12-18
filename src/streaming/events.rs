//! Streaming event types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{ContentBlock, Message, StopReason};

/// A streaming event from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageStreamEvent {
    /// Message started event.
    MessageStart { message: Message },

    /// Message delta event (updates to the message).
    MessageDelta {
        delta: MessageDelta,
        usage: MessageDeltaUsage,
    },

    /// Message stopped event.
    MessageStop,

    /// Content block started event.
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },

    /// Content block delta event (incremental content).
    ContentBlockDelta {
        index: usize,
        delta: ContentBlockDelta,
    },

    /// Content block stopped event.
    ContentBlockStop { index: usize },

    /// Ping event (keep-alive).
    Ping,

    /// Error event.
    Error { error: StreamError },
}

/// Message delta (updates to the message).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    /// The reason the model stopped generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<StopReason>,

    /// The stop sequence that caused the model to stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
}

/// Message delta usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDeltaUsage {
    /// The number of output tokens generated so far.
    pub output_tokens: u32,
}

/// Content block delta (incremental content update).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockDelta {
    /// Text delta.
    TextDelta { text: String },

    /// Input JSON delta (for tool use).
    InputJsonDelta { partial_json: String },

    /// Thinking delta (for extended thinking).
    ThinkingDelta { thinking: String },

    /// Signature delta (for extended thinking).
    SignatureDelta { signature: String },

    /// Citations delta.
    CitationsDelta { citation: Value },
}

impl ContentBlockDelta {
    /// Get the text content if this is a text delta.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlockDelta::TextDelta { text } => Some(text),
            _ => None,
        }
    }

    /// Get the thinking content if this is a thinking delta.
    pub fn as_thinking(&self) -> Option<&str> {
        match self {
            ContentBlockDelta::ThinkingDelta { thinking } => Some(thinking),
            _ => None,
        }
    }

    /// Get the partial JSON if this is an input JSON delta.
    pub fn as_input_json(&self) -> Option<&str> {
        match self {
            ContentBlockDelta::InputJsonDelta { partial_json } => Some(partial_json),
            _ => None,
        }
    }
}

/// Stream error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamError {
    /// Error type.
    #[serde(rename = "type")]
    pub error_type: String,

    /// Error message.
    pub message: String,
}

/// Raw streaming event from the API (before parsing).
#[derive(Debug, Clone)]
pub struct RawStreamEvent {
    /// The event type.
    pub event: String,

    /// The event data (JSON).
    pub data: String,
}

/// Accumulated message state from streaming.
#[derive(Debug, Clone, Default)]
pub struct StreamState {
    /// The accumulated message.
    pub message: Option<Message>,

    /// Accumulated text content.
    pub text: String,

    /// Accumulated thinking content.
    pub thinking: String,

    /// Whether the stream has completed.
    pub is_complete: bool,

    /// The final stop reason.
    pub stop_reason: Option<StopReason>,

    /// Total output tokens.
    pub output_tokens: u32,
}

impl StreamState {
    /// Create a new stream state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the state with a new event.
    pub fn update(&mut self, event: &MessageStreamEvent) {
        match event {
            MessageStreamEvent::MessageStart { message } => {
                self.message = Some(message.clone());
            }
            MessageStreamEvent::MessageDelta { delta, usage } => {
                if let Some(stop_reason) = delta.stop_reason {
                    self.stop_reason = Some(stop_reason);
                }
                self.output_tokens = usage.output_tokens;
            }
            MessageStreamEvent::MessageStop => {
                self.is_complete = true;
            }
            MessageStreamEvent::ContentBlockDelta { delta, .. } => {
                match delta {
                    ContentBlockDelta::TextDelta { text } => {
                        self.text.push_str(text);
                    }
                    ContentBlockDelta::ThinkingDelta { thinking } => {
                        self.thinking.push_str(thinking);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Get the final message with accumulated content.
    pub fn into_message(self) -> Option<Message> {
        self.message.map(|mut msg| {
            // Update usage with final output tokens
            msg.usage.output_tokens = self.output_tokens;
            msg.stop_reason = self.stop_reason;
            msg
        })
    }
}

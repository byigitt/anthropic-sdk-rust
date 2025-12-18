//! Request parameter types.

use serde::{Deserialize, Serialize};

use super::{ContentBlockParam, MessageParam, Tool, ToolChoice};

/// Parameters for creating a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateParams {
    /// The model to use.
    pub model: String,

    /// The maximum number of tokens to generate.
    pub max_tokens: u32,

    /// The messages in the conversation.
    pub messages: Vec<MessageParam>,

    /// System prompt (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPrompt>,

    /// Custom metadata (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Stop sequences (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Sampling temperature (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Top-K sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Top-P (nucleus) sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Tools available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Tool choice strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Thinking configuration for extended thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,
}

impl MessageCreateParams {
    /// Create a new builder for message params.
    pub fn builder() -> MessageCreateParamsBuilder {
        MessageCreateParamsBuilder::default()
    }
}

/// Builder for MessageCreateParams.
#[derive(Debug, Default)]
pub struct MessageCreateParamsBuilder {
    model: Option<String>,
    max_tokens: Option<u32>,
    messages: Vec<MessageParam>,
    system: Option<SystemPrompt>,
    metadata: Option<Metadata>,
    stop_sequences: Option<Vec<String>>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_k: Option<u32>,
    top_p: Option<f32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    thinking: Option<ThinkingConfig>,
}

impl MessageCreateParamsBuilder {
    /// Set the model to use.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the maximum number of tokens to generate.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the messages in the conversation.
    pub fn messages(mut self, messages: Vec<MessageParam>) -> Self {
        self.messages = messages;
        self
    }

    /// Add a single message to the conversation.
    pub fn message(mut self, message: MessageParam) -> Self {
        self.messages.push(message);
        self
    }

    /// Set the system prompt as text.
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(SystemPrompt::Text(system.into()));
        self
    }

    /// Set the system prompt with content blocks.
    pub fn system_blocks(mut self, blocks: Vec<ContentBlockParam>) -> Self {
        self.system = Some(SystemPrompt::Blocks(blocks));
        self
    }

    /// Set custom metadata.
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set stop sequences.
    pub fn stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(sequences);
        self
    }

    /// Enable streaming.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set the sampling temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the top-K sampling parameter.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set the top-P sampling parameter.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set available tools.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set the tool choice strategy.
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Enable extended thinking with a token budget.
    pub fn thinking(mut self, budget_tokens: u32) -> Self {
        self.thinking = Some(ThinkingConfig::Enabled { budget_tokens });
        self
    }

    /// Disable extended thinking.
    pub fn no_thinking(mut self) -> Self {
        self.thinking = Some(ThinkingConfig::Disabled);
        self
    }

    /// Build the MessageCreateParams.
    pub fn build(self) -> MessageCreateParams {
        MessageCreateParams {
            model: self
                .model
                .unwrap_or_else(|| "claude-sonnet-4-5-20250929".into()),
            max_tokens: self.max_tokens.unwrap_or(1024),
            messages: self.messages,
            system: self.system,
            metadata: self.metadata,
            stop_sequences: self.stop_sequences,
            stream: self.stream,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            thinking: self.thinking,
        }
    }
}

/// System prompt, either text or content blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemPrompt {
    /// Simple text system prompt.
    Text(String),

    /// System prompt with content blocks.
    Blocks(Vec<ContentBlockParam>),
}

/// Thinking configuration for extended thinking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThinkingConfig {
    /// Enable extended thinking with a token budget.
    Enabled { budget_tokens: u32 },

    /// Disable extended thinking.
    Disabled,
}

/// Custom metadata for the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// User ID for tracking purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Metadata {
    /// Create metadata with a user ID.
    pub fn with_user_id(user_id: impl Into<String>) -> Self {
        Metadata {
            user_id: Some(user_id.into()),
        }
    }
}

/// Parameters for counting tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountTokensParams {
    /// The model to use for counting.
    pub model: String,

    /// The messages to count tokens for.
    pub messages: Vec<MessageParam>,

    /// System prompt (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPrompt>,

    /// Tools (optional, affects token count).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Thinking configuration (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,
}

/// Token count response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCount {
    /// The number of input tokens.
    pub input_tokens: u32,
}

/// Parameters for listing models.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListModelsParams {
    /// Number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Cursor for pagination (before this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,

    /// Cursor for pagination (after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
}

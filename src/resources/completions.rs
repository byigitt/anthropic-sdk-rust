//! Completions API resource (legacy).

use serde::{Deserialize, Serialize};

use crate::client::{Anthropic, AsyncAnthropic};
use crate::error::Result;

/// Legacy completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    /// Unique object identifier.
    pub id: String,

    /// Object type, always "completion".
    #[serde(rename = "type")]
    pub object_type: String,

    /// The generated completion text.
    pub completion: String,

    /// The reason the model stopped generating.
    pub stop_reason: String,

    /// The model that generated the completion.
    pub model: String,
}

/// Parameters for creating a completion (legacy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionCreateParams {
    /// The model to use.
    pub model: String,

    /// The prompt to complete.
    pub prompt: String,

    /// The maximum number of tokens to generate.
    pub max_tokens_to_sample: u32,

    /// Stop sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Top-K sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Top-P sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl CompletionCreateParams {
    /// Create new completion params.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>, max_tokens: u32) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            max_tokens_to_sample: max_tokens,
            stop_sequences: None,
            temperature: None,
            top_k: None,
            top_p: None,
            stream: None,
        }
    }

    /// Set stop sequences.
    pub fn stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(sequences);
        self
    }

    /// Set the temperature.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

/// Completions API resource (async, legacy).
pub struct Completions<'a> {
    client: &'a AsyncAnthropic,
}

impl<'a> Completions<'a> {
    /// Create a new Completions resource.
    pub(crate) fn new(client: &'a AsyncAnthropic) -> Self {
        Self { client }
    }

    /// Create a completion (legacy API).
    ///
    /// Note: This is the legacy completions API. For new projects, use the Messages API instead.
    pub async fn create(&self, params: CompletionCreateParams) -> Result<Completion> {
        self.client.post("/complete", &params).await
    }
}

/// Completions API resource (blocking, legacy).
pub struct BlockingCompletions<'a> {
    client: &'a Anthropic,
}

impl<'a> BlockingCompletions<'a> {
    /// Create a new blocking Completions resource.
    pub(crate) fn new(client: &'a Anthropic) -> Self {
        Self { client }
    }

    /// Create a completion (legacy API).
    ///
    /// Note: This is the legacy completions API. For new projects, use the Messages API instead.
    pub fn create(&self, params: CompletionCreateParams) -> Result<Completion> {
        self.client
            .block_on(self.client.inner().completions().create(params))
    }
}

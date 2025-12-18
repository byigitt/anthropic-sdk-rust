//! Token usage types.

use serde::{Deserialize, Serialize};

/// Token usage information for a request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// The number of input tokens used.
    pub input_tokens: u32,

    /// The number of output tokens generated.
    pub output_tokens: u32,

    /// The number of tokens used to create the cache entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u32>,

    /// The number of tokens read from the cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u32>,
}

/// Cache creation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCreation {
    /// Type identifier.
    #[serde(rename = "type")]
    pub cache_type: String,
}

/// Server tool usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerToolUsage {
    /// Number of web search requests made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_requests: Option<u32>,
}

/// Message delta usage (for streaming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDeltaUsage {
    /// The number of output tokens generated so far.
    pub output_tokens: u32,
}

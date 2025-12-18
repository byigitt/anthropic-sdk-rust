//! Model types.

use serde::{Deserialize, Serialize};

/// Information about an available model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Unique model identifier.
    pub id: String,

    /// Object type, always "model".
    #[serde(rename = "type")]
    pub object_type: String,

    /// Human-readable name of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// When the model was created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// List of models response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelList {
    /// The list of models.
    pub data: Vec<Model>,

    /// Whether there are more models.
    pub has_more: bool,

    /// Cursor for the first item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Cursor for the last item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
}

// Model ID constants for convenience
pub mod models {
    /// Claude Opus 4.5 (latest)
    pub const CLAUDE_OPUS_4_5: &str = "claude-opus-4-5-20251101";

    /// Claude Sonnet 4.5 (latest)
    pub const CLAUDE_SONNET_4_5: &str = "claude-sonnet-4-5-20250929";

    /// Claude Haiku 4.5 (latest)
    pub const CLAUDE_HAIKU_4_5: &str = "claude-haiku-4-5-20251001";

    /// Claude 3.5 Sonnet (latest)
    pub const CLAUDE_3_5_SONNET: &str = "claude-3-5-sonnet-20241022";

    /// Claude 3.5 Haiku (latest)
    pub const CLAUDE_3_5_HAIKU: &str = "claude-3-5-haiku-20241022";

    /// Claude 3 Opus
    pub const CLAUDE_3_OPUS: &str = "claude-3-opus-20240229";

    /// Claude 3 Sonnet
    pub const CLAUDE_3_SONNET: &str = "claude-3-sonnet-20240229";

    /// Claude 3 Haiku
    pub const CLAUDE_3_HAIKU: &str = "claude-3-haiku-20240307";
}

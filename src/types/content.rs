//! Content block types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A content block in a message response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Text content block.
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        citations: Option<Vec<Citation>>,
    },

    /// Extended thinking content block.
    Thinking { thinking: String, signature: String },

    /// Redacted thinking content block.
    RedactedThinking { data: String },

    /// Tool use content block.
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },

    /// Server-side tool use content block.
    ServerToolUse {
        id: String,
        name: String,
        input: Value,
    },

    /// Web search tool result content block.
    WebSearchToolResult {
        tool_use_id: String,
        content: Vec<WebSearchResult>,
    },
}

impl ContentBlock {
    /// Get text content if this is a text block.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text, .. } => Some(text),
            _ => None,
        }
    }

    /// Get tool use details if this is a tool use block.
    pub fn as_tool_use(&self) -> Option<(&str, &str, &Value)> {
        match self {
            ContentBlock::ToolUse { id, name, input } => Some((id, name, input)),
            _ => None,
        }
    }

    /// Check if this is a text block.
    pub fn is_text(&self) -> bool {
        matches!(self, ContentBlock::Text { .. })
    }

    /// Check if this is a tool use block.
    pub fn is_tool_use(&self) -> bool {
        matches!(self, ContentBlock::ToolUse { .. })
    }
}

/// A content block parameter for request messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockParam {
    /// Text content block.
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },

    /// Image content block.
    Image {
        source: ImageSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },

    /// Document content block (PDF).
    Document {
        source: DocumentSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },

    /// Tool use block (for assistant messages in multi-turn).
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },

    /// Tool result block.
    ToolResult {
        tool_use_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<ToolResultContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

impl ContentBlockParam {
    /// Create a text content block.
    pub fn text(text: impl Into<String>) -> Self {
        ContentBlockParam::Text {
            text: text.into(),
            cache_control: None,
        }
    }

    /// Create a text content block with cache control.
    pub fn text_with_cache(text: impl Into<String>) -> Self {
        ContentBlockParam::Text {
            text: text.into(),
            cache_control: Some(CacheControl::ephemeral()),
        }
    }

    /// Create an image content block from base64 data.
    pub fn image_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        ContentBlockParam::Image {
            source: ImageSource::Base64 {
                media_type: media_type.into(),
                data: data.into(),
            },
            cache_control: None,
        }
    }

    /// Create an image content block from a URL.
    pub fn image_url(url: impl Into<String>) -> Self {
        ContentBlockParam::Image {
            source: ImageSource::Url { url: url.into() },
            cache_control: None,
        }
    }

    /// Create a tool result content block.
    pub fn tool_result(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        ContentBlockParam::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: Some(ToolResultContent::Text(content.into())),
            is_error: None,
            cache_control: None,
        }
    }

    /// Create a tool error result content block.
    pub fn tool_error(tool_use_id: impl Into<String>, error: impl Into<String>) -> Self {
        ContentBlockParam::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: Some(ToolResultContent::Text(error.into())),
            is_error: Some(true),
            cache_control: None,
        }
    }
}

/// Image source for image content blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageSource {
    /// Base64-encoded image data.
    Base64 { media_type: String, data: String },

    /// URL to an image.
    Url { url: String },
}

/// Document source for document content blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DocumentSource {
    /// Base64-encoded document data.
    Base64 { media_type: String, data: String },

    /// URL to a document.
    Url { url: String },
}

/// Tool result content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    /// Simple text result.
    Text(String),

    /// Multiple content blocks as result.
    Blocks(Vec<ContentBlockParam>),
}

/// Cache control settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CacheControl {
    /// Ephemeral cache control.
    Ephemeral,
}

impl CacheControl {
    /// Create an ephemeral cache control.
    pub fn ephemeral() -> Self {
        CacheControl::Ephemeral
    }
}

/// Citation information for text content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Citation {
    /// Character location citation.
    CharLocation {
        cited_text: String,
        document_index: u32,
        document_title: Option<String>,
        start_char_index: u32,
        end_char_index: u32,
    },

    /// Page location citation (for PDFs).
    PageLocation {
        cited_text: String,
        document_index: u32,
        document_title: Option<String>,
        page_number: u32,
    },

    /// Content block location citation.
    ContentBlockLocation {
        cited_text: String,
        document_index: u32,
        document_title: Option<String>,
        start_block_index: u32,
        end_block_index: u32,
    },

    /// Web search result location citation.
    WebSearchResultLocation {
        cited_text: String,
        url: String,
        title: Option<String>,
    },
}

/// Web search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    /// The URL of the search result.
    pub url: String,

    /// The title of the search result.
    pub title: String,

    /// Snippet of the search result content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

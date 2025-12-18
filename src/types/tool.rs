//! Tool use types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool definition for the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// The name of the tool.
    pub name: String,

    /// A description of what the tool does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The JSON schema for the tool's input parameters.
    pub input_schema: ToolInputSchema,

    /// Cache control settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<super::CacheControl>,
}

impl Tool {
    /// Create a new tool with the given name and input schema.
    pub fn new(name: impl Into<String>, input_schema: ToolInputSchema) -> Self {
        Tool {
            name: name.into(),
            description: None,
            input_schema,
            cache_control: None,
        }
    }

    /// Create a new tool with a description.
    pub fn with_description(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: ToolInputSchema,
    ) -> Self {
        Tool {
            name: name.into(),
            description: Some(description.into()),
            input_schema,
            cache_control: None,
        }
    }

    /// Set the description for this tool.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Enable cache control for this tool.
    pub fn with_cache_control(mut self) -> Self {
        self.cache_control = Some(super::CacheControl::Ephemeral);
        self
    }
}

/// JSON schema for tool input parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    /// The schema type, always "object".
    #[serde(rename = "type")]
    pub schema_type: String,

    /// The properties of the input object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// Required property names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    /// Additional properties allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,
}

impl ToolInputSchema {
    /// Create a new empty object schema.
    pub fn empty() -> Self {
        ToolInputSchema {
            schema_type: "object".into(),
            properties: None,
            required: None,
            additional_properties: None,
        }
    }

    /// Create a schema from a JSON value.
    pub fn from_value(value: Value) -> Self {
        ToolInputSchema {
            schema_type: "object".into(),
            properties: value.get("properties").cloned(),
            required: value
                .get("required")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            additional_properties: value
                .get("additionalProperties")
                .and_then(|v| v.as_bool()),
        }
    }

    /// Create a schema with properties.
    pub fn with_properties(properties: Value, required: Vec<String>) -> Self {
        ToolInputSchema {
            schema_type: "object".into(),
            properties: Some(properties),
            required: Some(required),
            additional_properties: None,
        }
    }
}

/// Tool choice parameter for controlling tool usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    /// Let the model decide whether to use tools.
    Auto {
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// Force the model to use any available tool.
    Any {
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// Force the model to use a specific tool.
    Tool {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// Disable tool use entirely.
    None,
}

impl ToolChoice {
    /// Create an auto tool choice.
    pub fn auto() -> Self {
        ToolChoice::Auto {
            disable_parallel_tool_use: None,
        }
    }

    /// Create an any tool choice.
    pub fn any() -> Self {
        ToolChoice::Any {
            disable_parallel_tool_use: None,
        }
    }

    /// Create a specific tool choice.
    pub fn tool(name: impl Into<String>) -> Self {
        ToolChoice::Tool {
            name: name.into(),
            disable_parallel_tool_use: None,
        }
    }

    /// Create a none tool choice.
    pub fn none() -> Self {
        ToolChoice::None
    }

    /// Disable parallel tool use for this choice.
    pub fn disable_parallel(mut self) -> Self {
        match &mut self {
            ToolChoice::Auto {
                disable_parallel_tool_use,
            }
            | ToolChoice::Any {
                disable_parallel_tool_use,
            }
            | ToolChoice::Tool {
                disable_parallel_tool_use,
                ..
            } => {
                *disable_parallel_tool_use = Some(true);
            }
            ToolChoice::None => {}
        }
        self
    }
}

/// Tool use block from a message response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseBlock {
    /// Unique identifier for this tool use.
    pub id: String,

    /// The name of the tool being used.
    pub name: String,

    /// The input parameters for the tool.
    pub input: Value,
}

/// Tool result block for sending tool execution results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultBlockParam {
    /// The ID of the tool use this is a result for.
    pub tool_use_id: String,

    /// The content of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether this result represents an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl ToolResultBlockParam {
    /// Create a successful tool result.
    pub fn success(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        ToolResultBlockParam {
            tool_use_id: tool_use_id.into(),
            content: Some(content.into()),
            is_error: None,
        }
    }

    /// Create an error tool result.
    pub fn error(tool_use_id: impl Into<String>, error: impl Into<String>) -> Self {
        ToolResultBlockParam {
            tool_use_id: tool_use_id.into(),
            content: Some(error.into()),
            is_error: Some(true),
        }
    }
}

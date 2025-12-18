//! Error types for the Anthropic SDK.

use std::time::Duration;

/// The main error type for the Anthropic SDK.
#[derive(Debug, thiserror::Error)]
pub enum AnthropicError {
    /// Bad request error (HTTP 400)
    #[error("Bad request: {message}")]
    BadRequest {
        message: String,
        request_id: Option<String>,
    },

    /// Authentication error (HTTP 401)
    #[error("Authentication failed: {message}")]
    Authentication {
        message: String,
        request_id: Option<String>,
    },

    /// Permission denied error (HTTP 403)
    #[error("Permission denied: {message}")]
    PermissionDenied {
        message: String,
        request_id: Option<String>,
    },

    /// Resource not found error (HTTP 404)
    #[error("Not found: {message}")]
    NotFound {
        message: String,
        request_id: Option<String>,
    },

    /// Conflict error (HTTP 409)
    #[error("Conflict: {message}")]
    Conflict {
        message: String,
        request_id: Option<String>,
    },

    /// Unprocessable entity error (HTTP 422)
    #[error("Unprocessable entity: {message}")]
    UnprocessableEntity {
        message: String,
        request_id: Option<String>,
    },

    /// Rate limit error (HTTP 429)
    #[error("Rate limited: {message}")]
    RateLimited {
        message: String,
        request_id: Option<String>,
        retry_after: Option<Duration>,
    },

    /// Internal server error (HTTP 5xx)
    #[error("Internal server error: {message}")]
    InternalServer {
        message: String,
        status: u16,
        request_id: Option<String>,
    },

    /// Server overloaded error (HTTP 529)
    #[error("Server overloaded: {message}")]
    Overloaded {
        message: String,
        request_id: Option<String>,
    },

    /// Request too large error (HTTP 413)
    #[error("Request too large: {message}")]
    RequestTooLarge {
        message: String,
        request_id: Option<String>,
    },

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(#[from] reqwest::Error),

    /// Request timeout
    #[error("Request timed out")]
    Timeout,

    /// Invalid response from API
    #[error("Invalid response: {message}")]
    InvalidResponse { message: String },

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Missing API key
    #[error("Missing API key: set ANTHROPIC_API_KEY environment variable or provide api_key")]
    MissingApiKey,

    /// Stream error
    #[error("Stream error: {message}")]
    Stream { message: String },
}

impl AnthropicError {
    /// Create an error from an HTTP status code and response body.
    pub fn from_status(
        status: u16,
        message: String,
        request_id: Option<String>,
        retry_after: Option<Duration>,
    ) -> Self {
        match status {
            400 => Self::BadRequest {
                message,
                request_id,
            },
            401 => Self::Authentication {
                message,
                request_id,
            },
            403 => Self::PermissionDenied {
                message,
                request_id,
            },
            404 => Self::NotFound {
                message,
                request_id,
            },
            409 => Self::Conflict {
                message,
                request_id,
            },
            413 => Self::RequestTooLarge {
                message,
                request_id,
            },
            422 => Self::UnprocessableEntity {
                message,
                request_id,
            },
            429 => Self::RateLimited {
                message,
                request_id,
                retry_after,
            },
            529 => Self::Overloaded {
                message,
                request_id,
            },
            500..=599 => Self::InternalServer {
                message,
                status,
                request_id,
            },
            _ => Self::InvalidResponse {
                message: format!("Unexpected status {}: {}", status, message),
            },
        }
    }

    /// Get the request ID if available.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::BadRequest { request_id, .. }
            | Self::Authentication { request_id, .. }
            | Self::PermissionDenied { request_id, .. }
            | Self::NotFound { request_id, .. }
            | Self::Conflict { request_id, .. }
            | Self::UnprocessableEntity { request_id, .. }
            | Self::RateLimited { request_id, .. }
            | Self::InternalServer { request_id, .. }
            | Self::Overloaded { request_id, .. }
            | Self::RequestTooLarge { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited { .. }
                | Self::InternalServer { .. }
                | Self::Overloaded { .. }
                | Self::Timeout
                | Self::Connection(_)
        )
    }

    /// Get the retry-after duration if available.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

/// API error response structure from Anthropic API.
#[derive(Debug, serde::Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: ErrorObject,
}

/// Error object within the API response.
#[derive(Debug, serde::Deserialize)]
pub struct ErrorObject {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

pub type Result<T> = std::result::Result<T, AnthropicError>;

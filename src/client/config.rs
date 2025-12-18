//! Client configuration.

use reqwest::header::HeaderMap;
use std::time::Duration;

use crate::{DEFAULT_BASE_URL, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_SECS};

/// Configuration for the Anthropic client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authentication (X-Api-Key header).
    pub api_key: Option<String>,

    /// Bearer token for authentication (Authorization header).
    pub auth_token: Option<String>,

    /// Base URL for the API.
    pub base_url: String,

    /// Request timeout.
    pub timeout: Duration,

    /// Maximum number of retries for failed requests.
    pub max_retries: u32,

    /// Default headers to include in all requests.
    pub default_headers: HeaderMap,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            auth_token: std::env::var("ANTHROPIC_AUTH_TOKEN").ok(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
            default_headers: HeaderMap::new(),
        }
    }
}

impl ClientConfig {
    /// Create a new configuration with an API key.
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    /// Create a new configuration with a bearer token.
    pub fn with_auth_token(auth_token: impl Into<String>) -> Self {
        Self {
            auth_token: Some(auth_token.into()),
            ..Default::default()
        }
    }

    /// Set the base URL.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Add a default header.
    pub fn default_header(
        mut self,
        name: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        if let (Ok(name), Ok(value)) = (name.try_into(), value.try_into()) {
            self.default_headers.insert(name, value);
        }
        self
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), crate::AnthropicError> {
        if self.api_key.is_none() && self.auth_token.is_none() {
            return Err(crate::AnthropicError::MissingApiKey);
        }
        Ok(())
    }

    /// Get the API key.
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Get the auth token.
    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }
}

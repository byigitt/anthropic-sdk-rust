//! Blocking (synchronous) client for the Anthropic API.

use tokio::runtime::Runtime;

use crate::error::Result;
use crate::resources::{BlockingCompletions, BlockingMessages, BlockingModels};

use super::{AsyncAnthropic, ClientConfig};

/// Blocking (synchronous) client for the Anthropic API.
///
/// This is a wrapper around [`AsyncAnthropic`] that blocks on async operations.
pub struct Anthropic {
    inner: AsyncAnthropic,
    runtime: Runtime,
}

impl Anthropic {
    /// Create a new blocking client with default configuration.
    ///
    /// This will use the `ANTHROPIC_API_KEY` environment variable for authentication.
    pub fn new() -> Result<Self> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new blocking client with an API key.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        Self::with_config(ClientConfig::with_api_key(api_key))
    }

    /// Create a new blocking client with the given configuration.
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let inner = AsyncAnthropic::with_config(config)?;

        let runtime = Runtime::new().map_err(|e| {
            crate::AnthropicError::Config {
                message: format!("Failed to create tokio runtime: {}", e),
            }
        })?;

        Ok(Self { inner, runtime })
    }

    /// Get a reference to the client configuration.
    pub fn config(&self) -> &ClientConfig {
        self.inner.config()
    }

    /// Get a reference to the inner async client.
    pub fn inner(&self) -> &AsyncAnthropic {
        &self.inner
    }

    /// Get a reference to the tokio runtime.
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Access the Messages API.
    pub fn messages(&self) -> BlockingMessages<'_> {
        BlockingMessages::new(self)
    }

    /// Access the Completions API (legacy).
    pub fn completions(&self) -> BlockingCompletions<'_> {
        BlockingCompletions::new(self)
    }

    /// Access the Models API.
    pub fn models(&self) -> BlockingModels<'_> {
        BlockingModels::new(self)
    }

    /// Block on an async operation.
    pub(crate) fn block_on<F, T>(&self, future: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        self.runtime.block_on(future)
    }
}

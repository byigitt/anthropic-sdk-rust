//! Messages API resource.

use crate::client::{Anthropic, AsyncAnthropic};
use crate::error::Result;
use crate::streaming::{BlockingMessageStream, MessageStream};
use crate::types::{CountTokensParams, Message, MessageCreateParams, TokenCount};

/// Messages API resource (async).
pub struct Messages<'a> {
    client: &'a AsyncAnthropic,
}

impl<'a> Messages<'a> {
    /// Create a new Messages resource.
    pub(crate) fn new(client: &'a AsyncAnthropic) -> Self {
        Self { client }
    }

    /// Create a message.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    ///     let client = AsyncAnthropic::new()?;
    ///
    ///     let message = client.messages().create(
    ///         MessageCreateParams::builder()
    ///             .model("claude-sonnet-4-5-20250929")
    ///             .max_tokens(1024)
    ///             .messages(vec![MessageParam::user("Hello, Claude!")])
    ///             .build()
    ///     ).await?;
    ///
    ///     println!("{}", message.text());
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, params: MessageCreateParams) -> Result<Message> {
        self.client.post("/messages", &params).await
    }

    /// Create a message with streaming.
    ///
    /// Returns a stream of events that can be iterated over.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam, MessageStreamEvent};
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    ///     let client = AsyncAnthropic::new()?;
    ///
    ///     let mut stream = client.messages().create_stream(
    ///         MessageCreateParams::builder()
    ///             .model("claude-sonnet-4-5-20250929")
    ///             .max_tokens(1024)
    ///             .messages(vec![MessageParam::user("Hello, Claude!")])
    ///             .build()
    ///     ).await?;
    ///
    ///     while let Some(event) = stream.next().await {
    ///         match event? {
    ///             MessageStreamEvent::ContentBlockDelta { delta, .. } => {
    ///                 if let Some(text) = delta.as_text() {
    ///                     print!("{}", text);
    ///                 }
    ///             }
    ///             _ => {}
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_stream(&self, params: MessageCreateParams) -> Result<MessageStream> {
        self.client.post_stream("/messages", &params).await
    }

    /// Count the tokens in a message.
    ///
    /// This can be used to estimate costs before sending a request.
    pub async fn count_tokens(&self, params: CountTokensParams) -> Result<TokenCount> {
        self.client.post("/messages/count_tokens", &params).await
    }
}

/// Messages API resource (blocking).
pub struct BlockingMessages<'a> {
    client: &'a Anthropic,
}

impl<'a> BlockingMessages<'a> {
    /// Create a new blocking Messages resource.
    pub(crate) fn new(client: &'a Anthropic) -> Self {
        Self { client }
    }

    /// Create a message.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use anthropic_sdk::{Anthropic, MessageCreateParams, MessageParam};
    ///
    /// fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    ///     let client = Anthropic::new()?;
    ///
    ///     let message = client.messages().create(
    ///         MessageCreateParams::builder()
    ///             .model("claude-sonnet-4-5-20250929")
    ///             .max_tokens(1024)
    ///             .messages(vec![MessageParam::user("Hello, Claude!")])
    ///             .build()
    ///     )?;
    ///
    ///     println!("{}", message.text());
    ///     Ok(())
    /// }
    /// ```
    pub fn create(&self, params: MessageCreateParams) -> Result<Message> {
        self.client
            .block_on(self.client.inner().messages().create(params))
    }

    /// Create a message with streaming.
    ///
    /// Returns a blocking iterator over stream events.
    pub fn create_stream(&self, params: MessageCreateParams) -> Result<BlockingMessageStream> {
        let stream = self
            .client
            .block_on(self.client.inner().messages().create_stream(params))?;

        // Create a new runtime handle for the blocking stream
        let runtime = std::sync::Arc::new(
            tokio::runtime::Runtime::new().map_err(|e| crate::AnthropicError::Config {
                message: format!("Failed to create runtime for stream: {}", e),
            })?,
        );

        Ok(BlockingMessageStream::new(stream, runtime))
    }

    /// Count the tokens in a message.
    pub fn count_tokens(&self, params: CountTokensParams) -> Result<TokenCount> {
        self.client
            .block_on(self.client.inner().messages().count_tokens(params))
    }
}

//! # Anthropic Rust SDK
//!
//! Official Rust SDK for the Anthropic API.
//!
//! ## Quick Start (Async)
//!
//! ```rust,no_run
//! use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
//!     let client = AsyncAnthropic::new()?;
//!
//!     let message = client.messages().create(
//!         MessageCreateParams::builder()
//!             .model("claude-sonnet-4-5-20250929")
//!             .max_tokens(1024)
//!             .messages(vec![MessageParam::user("Hello, Claude!")])
//!             .build()
//!     ).await?;
//!
//!     println!("{}", message.text());
//!     Ok(())
//! }
//! ```
//!
//! ## Quick Start (Blocking)
//!
//! ```rust,no_run
//! use anthropic_sdk::{Anthropic, MessageCreateParams, MessageParam};
//!
//! fn main() -> Result<(), anthropic_sdk::AnthropicError> {
//!     let client = Anthropic::new()?;
//!
//!     let message = client.messages().create(
//!         MessageCreateParams::builder()
//!             .model("claude-sonnet-4-5-20250929")
//!             .max_tokens(1024)
//!             .messages(vec![MessageParam::user("Hello, Claude!")])
//!             .build()
//!     )?;
//!
//!     println!("{}", message.text());
//!     Ok(())
//! }
//! ```
//!
//! ## Streaming
//!
//! ```rust,no_run
//! use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam, MessageStreamEvent};
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
//!     let client = AsyncAnthropic::new()?;
//!
//!     let mut stream = client.messages().create_stream(
//!         MessageCreateParams::builder()
//!             .model("claude-sonnet-4-5-20250929")
//!             .max_tokens(1024)
//!             .messages(vec![MessageParam::user("Hello, Claude!")])
//!             .build()
//!     ).await?;
//!
//!     while let Some(event) = stream.next().await {
//!         if let MessageStreamEvent::ContentBlockDelta { delta, .. } = event? {
//!             if let Some(text) = delta.as_text() {
//!                 print!("{}", text);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod resources;
pub mod streaming;
pub mod types;

// Re-export main types for convenience
pub use client::{Anthropic, AsyncAnthropic, ClientConfig};
pub use error::{AnthropicError, Result};
pub use types::{
    ContentBlock, ContentBlockParam, Message, MessageContent, MessageCreateParams,
    MessageCreateParamsBuilder, MessageParam, Model, ModelList, Role, StopReason, Tool,
    ToolChoice, ToolInputSchema, ToolResultBlockParam, ToolUseBlock, Usage,
};

// Re-export streaming types
pub use streaming::{
    BlockingMessageStream, ContentBlockDelta, MessageDelta, MessageDeltaUsage, MessageStream,
    MessageStreamEvent, StreamState,
};

// Re-export resource types
pub use resources::{BlockingCompletions, BlockingMessages, BlockingModels};
pub use resources::{Completions, Messages, Models};

/// Default API version header value
pub const API_VERSION: &str = "2023-06-01";

/// Default base URL for the Anthropic API
pub const DEFAULT_BASE_URL: &str = "https://api.anthropic.com";

/// Default timeout in seconds (10 minutes)
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;

/// Default maximum retries
pub const DEFAULT_MAX_RETRIES: u32 = 2;

/// Human prompt prefix (legacy)
pub const HUMAN_PROMPT: &str = "\n\nHuman:";

/// AI prompt prefix (legacy)
pub const AI_PROMPT: &str = "\n\nAssistant:";

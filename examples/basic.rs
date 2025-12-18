//! Basic usage example for the Anthropic SDK.
//!
//! Run with: cargo run --example basic

use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam};

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    // Create a client (uses ANTHROPIC_API_KEY environment variable)
    let client = AsyncAnthropic::new()?;

    // Create a simple message
    let message = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user(
                    "What is the capital of France? Answer in one word.",
                )])
                .build(),
        )
        .await?;

    // Print the response
    println!("Response: {}", message.text());
    println!("Stop reason: {:?}", message.stop_reason);
    println!(
        "Usage: {} input tokens, {} output tokens",
        message.usage.input_tokens, message.usage.output_tokens
    );

    Ok(())
}

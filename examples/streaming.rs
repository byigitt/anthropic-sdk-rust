//! Streaming example for the Anthropic SDK.
//!
//! Run with: cargo run --example streaming

use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam, MessageStreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    // Create a client
    let client = AsyncAnthropic::new()?;

    // Create a streaming message
    let mut stream = client
        .messages()
        .create_stream(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user(
                    "Write a short poem about Rust programming language.",
                )])
                .build(),
        )
        .await?;

    println!("Streaming response:");
    println!("---");

    // Process events as they arrive
    while let Some(event) = stream.next().await {
        match event? {
            MessageStreamEvent::MessageStart { message } => {
                println!("[Started message: {}]", message.id);
            }
            MessageStreamEvent::ContentBlockStart { index, .. } => {
                println!("[Content block {} started]", index);
            }
            MessageStreamEvent::ContentBlockDelta { delta, .. } => {
                // Print text as it streams in
                if let Some(text) = delta.as_text() {
                    print!("{}", text);
                }
                // Handle thinking deltas if extended thinking is enabled
                if let Some(thinking) = delta.as_thinking() {
                    print!("[Thinking: {}]", thinking);
                }
            }
            MessageStreamEvent::ContentBlockStop { index } => {
                println!("\n[Content block {} stopped]", index);
            }
            MessageStreamEvent::MessageDelta { delta, usage } => {
                if let Some(stop_reason) = delta.stop_reason {
                    println!("[Stop reason: {:?}]", stop_reason);
                }
                println!("[Output tokens: {}]", usage.output_tokens);
            }
            MessageStreamEvent::MessageStop => {
                println!("[Message complete]");
            }
            MessageStreamEvent::Ping => {
                // Keep-alive ping, ignore
            }
            MessageStreamEvent::Error { error } => {
                eprintln!("Stream error: {} - {}", error.error_type, error.message);
            }
        }
    }

    println!("---");
    println!("Final accumulated text: {}", stream.text());

    Ok(())
}

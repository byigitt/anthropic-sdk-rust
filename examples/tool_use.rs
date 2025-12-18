//! Tool use example for the Anthropic SDK.
//!
//! Run with: cargo run --example tool_use

use anthropic_sdk::{
    AsyncAnthropic, ContentBlockParam, MessageCreateParams, MessageParam, Tool, ToolChoice,
    ToolInputSchema,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    // Create a client
    let client = AsyncAnthropic::new()?;

    // Define a weather tool
    let weather_tool = Tool::with_description(
        "get_weather",
        "Get the current weather in a given location",
        ToolInputSchema::with_properties(
            json!({
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The unit for temperature"
                }
            }),
            vec!["location".to_string()],
        ),
    );

    // Create a message with tool use
    let message = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .tools(vec![weather_tool])
                .tool_choice(ToolChoice::auto())
                .messages(vec![MessageParam::user(
                    "What's the weather like in San Francisco?",
                )])
                .build(),
        )
        .await?;

    println!("Initial response:");
    println!("Stop reason: {:?}", message.stop_reason);

    // Check if the model wants to use a tool
    if message.has_tool_use() {
        println!("\nModel wants to use tools:");

        for (id, name, input) in message.tool_uses() {
            println!("  Tool: {} (id: {})", name, id);
            println!("  Input: {}", serde_json::to_string_pretty(input)?);

            // Simulate executing the tool
            let result = if name == "get_weather" {
                // In a real application, you would call an actual weather API here
                json!({
                    "temperature": 72,
                    "unit": "fahrenheit",
                    "conditions": "Partly cloudy",
                    "humidity": 65
                })
                .to_string()
            } else {
                "Unknown tool".to_string()
            };

            println!("  Result: {}", result);

            // Continue the conversation with the tool result
            let follow_up = client
                .messages()
                .create(
                    MessageCreateParams::builder()
                        .model("claude-sonnet-4-5-20250929")
                        .max_tokens(1024)
                        .messages(vec![
                            MessageParam::user("What's the weather like in San Francisco?"),
                            // Include the assistant's response with tool use
                            MessageParam::assistant_with_blocks(
                                message
                                    .content
                                    .iter()
                                    .map(|block| match block {
                                        anthropic_sdk::ContentBlock::Text { text, .. } => {
                                            ContentBlockParam::text(text.clone())
                                        }
                                        anthropic_sdk::ContentBlock::ToolUse { id, name, input } => {
                                            ContentBlockParam::ToolUse {
                                                id: id.clone(),
                                                name: name.clone(),
                                                input: input.clone(),
                                            }
                                        }
                                        _ => ContentBlockParam::text(""),
                                    })
                                    .collect(),
                            ),
                            // Include the tool result
                            MessageParam::user_with_blocks(vec![ContentBlockParam::tool_result(
                                id, result,
                            )]),
                        ])
                        .build(),
                )
                .await?;

            println!("\nFinal response: {}", follow_up.text());
        }
    } else {
        println!("Response: {}", message.text());
    }

    Ok(())
}

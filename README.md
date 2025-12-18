# Anthropic Rust API Library

[![Crates.io](https://img.shields.io/crates/v/anthropic-sdk.svg)](https://crates.io/crates/anthropic-sdk)
[![Documentation](https://docs.rs/anthropic-sdk/badge.svg)](https://docs.rs/anthropic-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The Anthropic Rust library provides convenient access to the Anthropic REST API from any Rust application. It includes type definitions for all request params and response fields, and offers both synchronous and asynchronous clients powered by [reqwest](https://github.com/seanmonstar/reqwest).

## Documentation

The REST API documentation can be found on [docs.anthropic.com](https://docs.anthropic.com/claude/reference/). The full API of this library can be found in [api.md](api.md).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anthropic-sdk = "0.1"
```

## Usage

The full API of this library can be found in [api.md](api.md).

```rust
use anthropic_sdk::{Anthropic, MessageCreateParams, MessageParam};

fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    // Create a client (uses ANTHROPIC_API_KEY environment variable)
    let client = Anthropic::new()?;

    // Create a simple message
    let message = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user("Hello, Claude")])
                .build(),
        )?;

    println!("{}", message.text());
    Ok(())
}
```

While you can provide an API key programmatically, we recommend using environment variables:

```bash
export ANTHROPIC_API_KEY="my-anthropic-api-key"
```

## Async Usage

Simply import `AsyncAnthropic` instead of `Anthropic` and use `.await` with each API call:

```rust
use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam};

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    let client = AsyncAnthropic::new()?;

    let message = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user("Hello, Claude")])
                .build(),
        )
        .await?;

    println!("{}", message.text());
    Ok(())
}
```

Functionality between the synchronous and asynchronous clients is otherwise identical.

## Streaming Responses

We provide support for streaming responses using Server-Sent Events (SSE).

```rust
use anthropic_sdk::{AsyncAnthropic, MessageCreateParams, MessageParam, MessageStreamEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    let client = AsyncAnthropic::new()?;

    let mut stream = client
        .messages()
        .create_stream(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user("Hello, Claude")])
                .build(),
        )
        .await?;

    while let Some(event) = stream.next().await {
        match event? {
            MessageStreamEvent::ContentBlockDelta { delta, .. } => {
                if let Some(text) = delta.as_text() {
                    print!("{}", text);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

The blocking client also supports streaming:

```rust
use anthropic_sdk::{Anthropic, MessageCreateParams, MessageParam, MessageStreamEvent};

fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    let client = Anthropic::new()?;

    let stream = client
        .messages()
        .create_stream(
            MessageCreateParams::builder()
                .model("claude-sonnet-4-5-20250929")
                .max_tokens(1024)
                .messages(vec![MessageParam::user("Hello, Claude")])
                .build(),
        )?;

    for event in stream {
        match event? {
            MessageStreamEvent::ContentBlockDelta { delta, .. } => {
                if let Some(text) = delta.as_text() {
                    print!("{}", text);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Tool Use

This SDK provides support for tool use (function calling):

```rust
use anthropic_sdk::{
    AsyncAnthropic, ContentBlockParam, MessageCreateParams, MessageParam,
    Tool, ToolChoice, ToolInputSchema,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
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
                }
            }),
            vec!["location".to_string()],
        ),
    );

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

    // Check if the model wants to use a tool
    if message.has_tool_use() {
        for (id, name, input) in message.tool_uses() {
            println!("Tool: {} (id: {})", name, id);
            println!("Input: {}", serde_json::to_string_pretty(input)?);
        }
    }

    Ok(())
}
```

More details can be found in [the documentation](https://docs.anthropic.com/claude/docs/tool-use).

## Handling Errors

When the library is unable to connect to the API (for example, due to network connection problems or a timeout), a subclass of `AnthropicError` is returned.

When the API returns a non-success status code, the appropriate error variant is returned:

```rust
use anthropic_sdk::{Anthropic, AnthropicError, MessageCreateParams, MessageParam};

fn main() {
    let client = Anthropic::new().unwrap();

    let result = client.messages().create(
        MessageCreateParams::builder()
            .model("claude-sonnet-4-5-20250929")
            .max_tokens(1024)
            .messages(vec![MessageParam::user("Hello, Claude")])
            .build(),
    );

    match result {
        Ok(message) => println!("{}", message.text()),
        Err(AnthropicError::RateLimited { retry_after, .. }) => {
            println!("Rate limited, retry after {:?}", retry_after);
        }
        Err(AnthropicError::Authentication { .. }) => {
            println!("Invalid API key");
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

Error codes are as follows:

| Status Code | Error Type                 |
| ----------- | -------------------------- |
| 400         | `BadRequest`               |
| 401         | `Authentication`           |
| 403         | `PermissionDenied`         |
| 404         | `NotFound`                 |
| 422         | `UnprocessableEntity`      |
| 429         | `RateLimited`              |
| >=500       | `InternalServer`           |
| 529         | `Overloaded`               |
| N/A         | `Connection`               |

### Retries

Certain errors are automatically retried 2 times by default, with a short exponential backoff. Connection errors, 408 Request Timeout, 409 Conflict, 429 Rate Limit, and >=500 Internal errors are all retried by default.

You can configure retry settings:

```rust
use anthropic_sdk::ClientConfig;

let config = ClientConfig::new()
    .with_max_retries(5);  // default is 2
```

### Timeouts

By default, requests time out after 10 minutes. You can configure this:

```rust
use anthropic_sdk::ClientConfig;
use std::time::Duration;

let config = ClientConfig::new()
    .with_timeout(Duration::from_secs(30));
```

## Client Configuration

You can customize the client with various options:

```rust
use anthropic_sdk::{AsyncAnthropic, ClientConfig};
use std::time::Duration;

let config = ClientConfig::new()
    .with_api_key("your-api-key")
    .with_base_url("https://custom-api-endpoint.com")
    .with_timeout(Duration::from_secs(120))
    .with_max_retries(3);

let client = AsyncAnthropic::with_config(config)?;
```

## Requirements

- Rust 1.82 or higher
- tokio runtime (for async client)

## Examples

See the [examples](./examples) directory for more complete examples:

- [basic.rs](./examples/basic.rs) - Simple message creation
- [streaming.rs](./examples/streaming.rs) - Streaming responses
- [tool_use.rs](./examples/tool_use.rs) - Tool use / function calling

Run examples with:

```bash
cargo run --example basic
cargo run --example streaming
cargo run --example tool_use
```

## Contributing

See [the contributing documentation](./CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

# API Reference

## Clients

### AsyncAnthropic

The async client for the Anthropic API.

```rust
use anthropic_sdk::AsyncAnthropic;

// Create with default configuration (uses ANTHROPIC_API_KEY env var)
let client = AsyncAnthropic::new()?;

// Create with API key
let client = AsyncAnthropic::with_api_key("your-api-key")?;

// Create with custom configuration
let client = AsyncAnthropic::with_config(config)?;
```

### Anthropic

The blocking (synchronous) client for the Anthropic API.

```rust
use anthropic_sdk::Anthropic;

let client = Anthropic::new()?;
```

## Messages API

### Types

```rust
use anthropic_sdk::{
    // Core message types
    Message,
    MessageParam,
    MessageCreateParams,

    // Content types
    ContentBlock,
    ContentBlockParam,

    // Tool types
    Tool,
    ToolChoice,
    ToolInputSchema,

    // Other types
    Role,
    StopReason,
    Usage,
};
```

### Methods

#### `messages().create()`

Create a new message.

```rust
let message = client
    .messages()
    .create(
        MessageCreateParams::builder()
            .model("claude-sonnet-4-5-20250929")
            .max_tokens(1024)
            .messages(vec![MessageParam::user("Hello!")])
            .build(),
    )
    .await?;
```

#### `messages().create_stream()`

Create a streaming message.

```rust
let stream = client
    .messages()
    .create_stream(
        MessageCreateParams::builder()
            .model("claude-sonnet-4-5-20250929")
            .max_tokens(1024)
            .messages(vec![MessageParam::user("Hello!")])
            .build(),
    )
    .await?;
```

#### `messages().count_tokens()`

Count tokens for a message without creating it.

```rust
let count = client
    .messages()
    .count_tokens(
        MessageCreateParams::builder()
            .model("claude-sonnet-4-5-20250929")
            .max_tokens(1024)
            .messages(vec![MessageParam::user("Hello!")])
            .build(),
    )
    .await?;
```

## Models API

### Types

```rust
use anthropic_sdk::ModelInfo;
```

### Methods

#### `models().retrieve()`

Get information about a specific model.

```rust
let model = client.models().retrieve("claude-sonnet-4-5-20250929").await?;
```

#### `models().list()`

List available models.

```rust
let models = client.models().list().await?;
```

## Completions API (Legacy)

### Types

```rust
use anthropic_sdk::{
    Completion,
    CompletionCreateParams,
};
```

### Methods

#### `completions().create()`

Create a text completion.

```rust
let completion = client
    .completions()
    .create(
        CompletionCreateParams::builder()
            .model("claude-2.1")
            .prompt("\n\nHuman: Hello\n\nAssistant:")
            .max_tokens_to_sample(256)
            .build(),
    )
    .await?;
```

## Streaming Types

```rust
use anthropic_sdk::{
    MessageStreamEvent,
    MessageStream,
    BlockingMessageStream,
    ContentBlockDelta,
    StreamState,
};
```

### MessageStreamEvent

```rust
pub enum MessageStreamEvent {
    MessageStart { message: Message },
    MessageDelta { delta: MessageDelta, usage: MessageDeltaUsage },
    MessageStop,
    ContentBlockStart { index: usize, content_block: ContentBlock },
    ContentBlockDelta { index: usize, delta: ContentBlockDelta },
    ContentBlockStop { index: usize },
    Ping,
    Error { error: StreamError },
}
```

### ContentBlockDelta

```rust
pub enum ContentBlockDelta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
    ThinkingDelta { thinking: String },
    SignatureDelta { signature: String },
}
```

## Error Types

```rust
use anthropic_sdk::AnthropicError;

pub enum AnthropicError {
    BadRequest { message: String, request_id: Option<String> },
    Authentication { message: String, request_id: Option<String> },
    PermissionDenied { message: String, request_id: Option<String> },
    NotFound { message: String, request_id: Option<String> },
    Conflict { message: String, request_id: Option<String> },
    UnprocessableEntity { message: String, request_id: Option<String> },
    RateLimited { message: String, request_id: Option<String>, retry_after: Option<Duration> },
    InternalServer { message: String, request_id: Option<String> },
    Overloaded { message: String, request_id: Option<String> },
    Connection(reqwest::Error),
    Timeout { message: String },
    InvalidResponse { message: String },
    MissingApiKey,
    Json(serde_json::Error),
}
```

## Configuration

```rust
use anthropic_sdk::ClientConfig;

let config = ClientConfig::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.anthropic.com")
    .with_timeout(Duration::from_secs(600))
    .with_max_retries(2);
```

## Helper Types

### MessageParam

```rust
// User message with text
MessageParam::user("Hello!")

// User message with content blocks
MessageParam::user_with_blocks(vec![
    ContentBlockParam::text("Hello!"),
    ContentBlockParam::image_url("https://example.com/image.jpg", "image/jpeg"),
])

// Assistant message
MessageParam::assistant("Hi there!")
```

### ContentBlockParam

```rust
// Text content
ContentBlockParam::text("Hello!")

// Image from URL
ContentBlockParam::image_url(url, media_type)

// Image from base64
ContentBlockParam::image_base64(data, media_type)

// Tool use
ContentBlockParam::ToolUse { id, name, input }

// Tool result
ContentBlockParam::tool_result(tool_use_id, content)
```

### Tool

```rust
// Basic tool
Tool::new("tool_name", input_schema)

// Tool with description
Tool::with_description("tool_name", "Tool description", input_schema)

// Tool input schema
ToolInputSchema::with_properties(
    json!({
        "param1": { "type": "string", "description": "..." }
    }),
    vec!["param1".to_string()],  // required fields
)
```

### ToolChoice

```rust
ToolChoice::auto()     // Let model decide
ToolChoice::any()      // Must use some tool
ToolChoice::none()     // Don't use tools
ToolChoice::tool("specific_tool")  // Use specific tool
```

## Message Helper Methods

```rust
// Get concatenated text content
message.text()

// Check if message has tool use
message.has_tool_use()

// Iterate over tool uses
for (id, name, input) in message.tool_uses() {
    // ...
}
```

## Stream Helper Methods

```rust
// Get accumulated text
stream.text()

// Get accumulated thinking
stream.thinking()

// Check if complete
stream.is_complete()

// Get current state
stream.state()

// Collect all text (consumes stream)
let text = stream.collect_text().await?;
```

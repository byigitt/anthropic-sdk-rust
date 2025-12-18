//! Message stream implementation.

use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::Stream;
use pin_project_lite::pin_project;
use reqwest::Response;

use crate::error::{AnthropicError, Result};

use super::events::{MessageStreamEvent, RawStreamEvent, StreamState};
use super::sse::SseDecoder;

pin_project! {
    /// A stream of message events from the API.
    pub struct MessageStream {
        #[pin]
        inner: futures::stream::BoxStream<'static, std::result::Result<Bytes, reqwest::Error>>,
        decoder: SseDecoder,
        state: StreamState,
        finished: bool,
    }
}

impl MessageStream {
    /// Create a new message stream from a response.
    pub(crate) fn new(response: Response) -> Self {
        use futures::StreamExt;

        let inner = response.bytes_stream().boxed();

        Self {
            inner,
            decoder: SseDecoder::new(),
            state: StreamState::new(),
            finished: false,
        }
    }

    /// Get the current accumulated state.
    pub fn state(&self) -> &StreamState {
        &self.state
    }

    /// Get the accumulated text so far.
    pub fn text(&self) -> &str {
        &self.state.text
    }

    /// Get the accumulated thinking so far.
    pub fn thinking(&self) -> &str {
        &self.state.thinking
    }

    /// Check if the stream has completed.
    pub fn is_complete(&self) -> bool {
        self.state.is_complete
    }

    /// Consume the stream and collect all text.
    pub async fn collect_text(mut self) -> Result<String> {
        use futures::StreamExt;

        while let Some(result) = self.next().await {
            result?;
        }

        Ok(self.state.text)
    }
}

/// Parse a raw event into a typed event.
fn parse_event(event: &RawStreamEvent) -> Result<MessageStreamEvent> {
    // Handle ping events
    if event.event == "ping" {
        return Ok(MessageStreamEvent::Ping);
    }

    // Handle error events
    if event.event == "error" {
        let error: super::events::StreamError =
            serde_json::from_str(&event.data).map_err(AnthropicError::Json)?;
        return Ok(MessageStreamEvent::Error { error });
    }

    // Parse the data JSON, injecting the event type if needed
    let mut data: serde_json::Value =
        serde_json::from_str(&event.data).map_err(AnthropicError::Json)?;

    // Add type field if missing
    if data.get("type").is_none() {
        data["type"] = serde_json::Value::String(event.event.clone());
    }

    // Parse as the appropriate event type
    serde_json::from_value(data).map_err(AnthropicError::Json)
}

impl Stream for MessageStream {
    type Item = Result<MessageStreamEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if *this.finished {
            return Poll::Ready(None);
        }

        loop {
            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    // Decode SSE events from bytes
                    let raw_events = this.decoder.decode(bytes);

                    for raw_event in raw_events {
                        match parse_event(&raw_event) {
                            Ok(event) => {
                                // Update state
                                this.state.update(&event);

                                // Check if this is the final event
                                if matches!(event, MessageStreamEvent::MessageStop) {
                                    *this.finished = true;
                                }

                                return Poll::Ready(Some(Ok(event)));
                            }
                            Err(e) => {
                                return Poll::Ready(Some(Err(e)));
                            }
                        }
                    }

                    // Continue reading if no events were produced
                    continue;
                }
                Poll::Ready(Some(Err(e))) => {
                    *this.finished = true;
                    return Poll::Ready(Some(Err(AnthropicError::Connection(e))));
                }
                Poll::Ready(None) => {
                    // Stream ended, flush any remaining data
                    if let Some(raw_event) = this.decoder.flush() {
                        match parse_event(&raw_event) {
                            Ok(event) => {
                                this.state.update(&event);
                                *this.finished = true;
                                return Poll::Ready(Some(Ok(event)));
                            }
                            Err(e) => {
                                *this.finished = true;
                                return Poll::Ready(Some(Err(e)));
                            }
                        }
                    }

                    *this.finished = true;
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

/// A blocking iterator over stream events.
pub struct BlockingMessageStream {
    inner: MessageStream,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl BlockingMessageStream {
    /// Create a new blocking stream.
    pub(crate) fn new(inner: MessageStream, runtime: std::sync::Arc<tokio::runtime::Runtime>) -> Self {
        Self { inner, runtime }
    }

    /// Get the current accumulated state.
    pub fn state(&self) -> &StreamState {
        self.inner.state()
    }

    /// Get the accumulated text so far.
    pub fn text(&self) -> &str {
        self.inner.text()
    }

    /// Consume the stream and collect all text.
    pub fn collect_text(self) -> Result<String> {
        self.runtime.block_on(self.inner.collect_text())
    }
}

impl Iterator for BlockingMessageStream {
    type Item = Result<MessageStreamEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        use futures::StreamExt;
        self.runtime.block_on(self.inner.next())
    }
}

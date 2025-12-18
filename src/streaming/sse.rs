//! Server-Sent Events (SSE) decoder.

use bytes::Bytes;

use super::events::RawStreamEvent;

/// SSE decoder state.
#[derive(Debug, Default)]
pub struct SseDecoder {
    /// Buffer for incomplete lines.
    buffer: String,

    /// Current event type.
    event_type: Option<String>,

    /// Current event data lines.
    data_lines: Vec<String>,
}

impl SseDecoder {
    /// Create a new SSE decoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode bytes into SSE events.
    pub fn decode(&mut self, bytes: Bytes) -> Vec<RawStreamEvent> {
        let mut events = Vec::new();

        // Convert bytes to string and append to buffer
        let text = String::from_utf8_lossy(&bytes);
        self.buffer.push_str(&text);

        // Process complete lines
        while let Some(newline_pos) = self.buffer.find('\n') {
            let line = self.buffer[..newline_pos].trim_end_matches('\r').to_string();
            self.buffer = self.buffer[newline_pos + 1..].to_string();

            if let Some(event) = self.process_line(&line) {
                events.push(event);
            }
        }

        events
    }

    /// Process a single line.
    fn process_line(&mut self, line: &str) -> Option<RawStreamEvent> {
        // Empty line signals end of event
        if line.is_empty() {
            return self.emit_event();
        }

        // Ignore comments
        if line.starts_with(':') {
            return None;
        }

        // Parse field
        let (field, value) = if let Some(colon_pos) = line.find(':') {
            let field = &line[..colon_pos];
            let value = line[colon_pos + 1..].trim_start_matches(' ');
            (field, value)
        } else {
            (line, "")
        };

        match field {
            "event" => {
                self.event_type = Some(value.to_string());
            }
            "data" => {
                self.data_lines.push(value.to_string());
            }
            "id" => {
                // We don't use event IDs currently
            }
            "retry" => {
                // We don't use retry timing from SSE
            }
            _ => {
                // Ignore unknown fields
            }
        }

        None
    }

    /// Emit a complete event.
    fn emit_event(&mut self) -> Option<RawStreamEvent> {
        if self.data_lines.is_empty() {
            self.event_type = None;
            return None;
        }

        let event = self.event_type.take().unwrap_or_else(|| "message".to_string());
        let data = self.data_lines.join("\n");
        self.data_lines.clear();

        Some(RawStreamEvent { event, data })
    }

    /// Check if there's buffered data.
    pub fn has_buffered_data(&self) -> bool {
        !self.buffer.is_empty() || !self.data_lines.is_empty()
    }

    /// Flush any remaining data.
    pub fn flush(&mut self) -> Option<RawStreamEvent> {
        if self.has_buffered_data() {
            // Process any remaining buffer content
            if !self.buffer.is_empty() {
                let remaining = std::mem::take(&mut self.buffer);
                self.process_line(&remaining);
            }
            self.emit_event()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple_event() {
        let mut decoder = SseDecoder::new();
        let bytes = Bytes::from("event: message_start\ndata: {\"type\":\"message_start\"}\n\n");

        let events = decoder.decode(bytes);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "message_start");
        assert_eq!(events[0].data, "{\"type\":\"message_start\"}");
    }

    #[test]
    fn test_decode_multiple_events() {
        let mut decoder = SseDecoder::new();
        let bytes = Bytes::from(
            "event: content_block_start\ndata: {}\n\nevent: content_block_delta\ndata: {\"text\":\"Hello\"}\n\n",
        );

        let events = decoder.decode(bytes);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event, "content_block_start");
        assert_eq!(events[1].event, "content_block_delta");
    }

    #[test]
    fn test_decode_split_data() {
        let mut decoder = SseDecoder::new();

        // First chunk (incomplete)
        let events1 = decoder.decode(Bytes::from("event: test\ndata: {\"par"));
        assert!(events1.is_empty());

        // Second chunk (completes the event)
        let events2 = decoder.decode(Bytes::from("tial\":true}\n\n"));
        assert_eq!(events2.len(), 1);
        assert_eq!(events2[0].data, "{\"partial\":true}");
    }

    #[test]
    fn test_ignore_comments() {
        let mut decoder = SseDecoder::new();
        let bytes = Bytes::from(": this is a comment\nevent: test\ndata: {}\n\n");

        let events = decoder.decode(bytes);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "test");
    }
}

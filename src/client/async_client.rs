//! Async HTTP client for the Anthropic API.

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response, StatusCode};

use crate::error::{AnthropicError, ErrorResponse, Result};
use crate::resources::{Completions, Messages, Models};
use crate::streaming::MessageStream;
use crate::types::MessageCreateParams;
use crate::API_VERSION;

use super::ClientConfig;

/// Async client for the Anthropic API.
#[derive(Debug, Clone)]
pub struct AsyncAnthropic {
    config: ClientConfig,
    http_client: Client,
}

impl AsyncAnthropic {
    /// Create a new client with default configuration.
    ///
    /// This will use the `ANTHROPIC_API_KEY` environment variable for authentication.
    pub fn new() -> Result<Self> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new client with an API key.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        Self::with_config(ClientConfig::with_api_key(api_key))
    }

    /// Create a new client with the given configuration.
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let http_client = Client::builder()
            .timeout(config.timeout)
            .default_headers(config.default_headers.clone())
            .build()
            .map_err(AnthropicError::Connection)?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Get a reference to the client configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Access the Messages API.
    pub fn messages(&self) -> Messages<'_> {
        Messages::new(self)
    }

    /// Access the Completions API (legacy).
    pub fn completions(&self) -> Completions<'_> {
        Completions::new(self)
    }

    /// Access the Models API.
    pub fn models(&self) -> Models<'_> {
        Models::new(self)
    }

    /// Build the authentication headers.
    fn build_auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = &self.config.api_key {
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(api_key).unwrap_or_else(|_| HeaderValue::from_static("")),
            );
        }

        if let Some(auth_token) = &self.config.auth_token {
            let value = format!("Bearer {}", auth_token);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&value).unwrap_or_else(|_| HeaderValue::from_static("")),
            );
        }

        headers
    }

    /// Build the common request headers.
    fn build_headers(&self) -> HeaderMap {
        let mut headers = self.build_auth_headers();

        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(API_VERSION),
        );
        headers.insert(
            "x-stainless-lang",
            HeaderValue::from_static("rust"),
        );

        headers
    }

    /// Make a GET request.
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/v1{}", self.config.base_url, path);
        let headers = self.build_headers();

        let response = self
            .request_with_retry(|| {
                self.http_client
                    .get(&url)
                    .headers(headers.clone())
                    .send()
            })
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request.
    pub(crate) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}/v1{}", self.config.base_url, path);
        let headers = self.build_headers();

        let response = self
            .request_with_retry(|| {
                self.http_client
                    .post(&url)
                    .headers(headers.clone())
                    .json(body)
                    .send()
            })
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request and return a stream.
    pub(crate) async fn post_stream(
        &self,
        path: &str,
        body: &MessageCreateParams,
    ) -> Result<MessageStream> {
        let url = format!("{}/v1{}", self.config.base_url, path);
        let headers = self.build_headers();

        // Create a modified body with stream: true
        let mut body = body.clone();
        body.stream = Some(true);

        let response = self
            .request_with_retry(|| {
                self.http_client
                    .post(&url)
                    .headers(headers.clone())
                    .json(&body)
                    .send()
            })
            .await?;

        // Check for errors before creating stream
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let request_id = response
                .headers()
                .get("request-id")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            let body_text = response.text().await.unwrap_or_default();
            let message = if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&body_text) {
                error_response.error.message
            } else {
                body_text
            };

            return Err(AnthropicError::from_status(status, message, request_id, None));
        }

        Ok(MessageStream::new(response))
    }

    /// Execute a request with retry logic.
    async fn request_with_retry<F, Fut>(&self, request_fn: F) -> Result<Response>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<Response, reqwest::Error>>,
    {
        let mut last_error = None;
        let mut attempts = 0;

        while attempts <= self.config.max_retries {
            match request_fn().await {
                Ok(response) => {
                    let status = response.status();

                    // Check if we should retry based on status
                    if self.should_retry(status) && attempts < self.config.max_retries {
                        let retry_after = self.parse_retry_after(response.headers());
                        let delay = self.calculate_delay(attempts, retry_after);
                        tokio::time::sleep(delay).await;
                        attempts += 1;
                        continue;
                    }

                    return Ok(response);
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = Some(AnthropicError::Timeout);
                    } else if e.is_connect() {
                        last_error = Some(AnthropicError::Connection(e));
                    } else {
                        last_error = Some(AnthropicError::Connection(e));
                    }

                    if attempts < self.config.max_retries {
                        let delay = self.calculate_delay(attempts, None);
                        tokio::time::sleep(delay).await;
                        attempts += 1;
                        continue;
                    }

                    break;
                }
            }
        }

        Err(last_error.unwrap_or(AnthropicError::Timeout))
    }

    /// Check if a status code should trigger a retry.
    fn should_retry(&self, status: StatusCode) -> bool {
        matches!(
            status.as_u16(),
            408 | 409 | 429 | 500 | 502 | 503 | 504 | 529
        )
    }

    /// Parse the Retry-After header.
    fn parse_retry_after(&self, headers: &HeaderMap) -> Option<Duration> {
        // Try retry-after-ms first (non-standard but more precise)
        if let Some(value) = headers.get("retry-after-ms") {
            if let Ok(ms) = value.to_str().unwrap_or("").parse::<u64>() {
                return Some(Duration::from_millis(ms));
            }
        }

        // Try standard retry-after (seconds)
        if let Some(value) = headers.get("retry-after") {
            if let Ok(secs) = value.to_str().unwrap_or("").parse::<u64>() {
                return Some(Duration::from_secs(secs));
            }
        }

        None
    }

    /// Calculate the delay for a retry attempt.
    fn calculate_delay(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        const INITIAL_DELAY: f64 = 0.5;
        const MAX_DELAY: f64 = 8.0;

        // Use retry-after if provided and reasonable (within 60 seconds)
        if let Some(retry_after) = retry_after {
            if retry_after <= Duration::from_secs(60) {
                return retry_after;
            }
        }

        // Exponential backoff with jitter
        let base_delay = INITIAL_DELAY * 2.0_f64.powi(attempt as i32);
        let delay = base_delay.min(MAX_DELAY);

        // Add some jitter (±25%)
        let jitter = 1.0 - 0.25 * rand_f64();
        let final_delay = delay * jitter;

        Duration::from_secs_f64(final_delay)
    }

    /// Handle the response, parsing errors if needed.
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let request_id = response
            .headers()
            .get("request-id")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        if status.is_success() {
            let body = response.text().await.map_err(AnthropicError::Connection)?;
            serde_json::from_str(&body).map_err(AnthropicError::Json)
        } else {
            let retry_after = self.parse_retry_after(response.headers());
            let body_text = response.text().await.unwrap_or_default();

            let message = if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&body_text) {
                error_response.error.message
            } else {
                body_text
            };

            Err(AnthropicError::from_status(
                status.as_u16(),
                message,
                request_id,
                retry_after,
            ))
        }
    }
}

/// Simple random number generator for jitter (0.0 to 1.0).
fn rand_f64() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    hasher.write_u64(std::time::Instant::now().elapsed().as_nanos() as u64);
    let hash = hasher.finish();

    (hash as f64) / (u64::MAX as f64)
}

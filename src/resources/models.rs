//! Models API resource.

use crate::client::{Anthropic, AsyncAnthropic};
use crate::error::Result;
use crate::types::{ListModelsParams, Model, ModelList};

/// Models API resource (async).
pub struct Models<'a> {
    client: &'a AsyncAnthropic,
}

impl<'a> Models<'a> {
    /// Create a new Models resource.
    pub(crate) fn new(client: &'a AsyncAnthropic) -> Self {
        Self { client }
    }

    /// List available models.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use anthropic_sdk::AsyncAnthropic;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    ///     let client = AsyncAnthropic::new()?;
    ///
    ///     let models = client.models().list(Default::default()).await?;
    ///     for model in models.data {
    ///         println!("{}: {:?}", model.id, model.display_name);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&self, params: ListModelsParams) -> Result<ModelList> {
        let mut path = "/models".to_string();
        let mut query_parts = Vec::new();

        if let Some(limit) = params.limit {
            query_parts.push(format!("limit={}", limit));
        }
        if let Some(before_id) = params.before_id {
            query_parts.push(format!("before_id={}", before_id));
        }
        if let Some(after_id) = params.after_id {
            query_parts.push(format!("after_id={}", after_id));
        }

        if !query_parts.is_empty() {
            path.push('?');
            path.push_str(&query_parts.join("&"));
        }

        self.client.get(&path).await
    }

    /// Retrieve a specific model.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use anthropic_sdk::AsyncAnthropic;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anthropic_sdk::AnthropicError> {
    ///     let client = AsyncAnthropic::new()?;
    ///
    ///     let model = client.models().retrieve("claude-sonnet-4-5-20250929").await?;
    ///     println!("{}: {:?}", model.id, model.display_name);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn retrieve(&self, model_id: &str) -> Result<Model> {
        self.client.get(&format!("/models/{}", model_id)).await
    }
}

/// Models API resource (blocking).
pub struct BlockingModels<'a> {
    client: &'a Anthropic,
}

impl<'a> BlockingModels<'a> {
    /// Create a new blocking Models resource.
    pub(crate) fn new(client: &'a Anthropic) -> Self {
        Self { client }
    }

    /// List available models.
    pub fn list(&self, params: ListModelsParams) -> Result<ModelList> {
        self.client
            .block_on(self.client.inner().models().list(params))
    }

    /// Retrieve a specific model.
    pub fn retrieve(&self, model_id: &str) -> Result<Model> {
        self.client
            .block_on(self.client.inner().models().retrieve(model_id))
    }
}

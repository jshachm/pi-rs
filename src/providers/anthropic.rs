//! Anthropic provider implementation

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

/// Anthropic API version
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

/// Anthropic provider
pub struct AnthropicProvider {
    api_key: String,
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl AnthropicProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.anthropic.com")
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "claude-sonnet-4-20250514".to_string(),
                    name: "Claude Sonnet 4".to_string(),
                    provider: "anthropic".to_string(),
                    context_window: 200000,
                    max_tokens: 8192,
                    supports_thinking: true,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 3.0,
                        output: 15.0,
                        cache_read: 0.3,
                        cache_write: 1.5,
                    },
                },
                Model {
                    id: "claude-opus-4-20250514".to_string(),
                    name: "Claude Opus 4".to_string(),
                    provider: "anthropic".to_string(),
                    context_window: 200000,
                    max_tokens: 8192,
                    supports_thinking: true,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 15.0,
                        output: 75.0,
                        cache_read: 1.5,
                        cache_write: 7.5,
                    },
                },
                Model {
                    id: "claude-haiku-3-5-20250520".to_string(),
                    name: "Claude Haiku 3.5".to_string(),
                    provider: "anthropic".to_string(),
                    context_window: 200000,
                    max_tokens: 8192,
                    supports_thinking: false,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.8,
                        output: 4.0,
                        cache_read: 0.08,
                        cache_write: 0.4,
                    },
                },
            ],
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn models(&self) -> Vec<Model> {
        self.models.clone()
    }

    async fn chat(
        &self,
        model: &str,
        messages: Vec<crate::core::Message>,
        _tools: Option<Vec<serde_json::Value>>,
        thinking: Option<bool>,
    ) -> Result<ProviderResponse, ProviderError> {
        let thinking_enabled = thinking.unwrap_or(false);

        let mut anthropic_messages: Vec<serde_json::Value> = Vec::new();
        let mut system_prompt = String::new();

        for msg in &messages {
            match msg.role {
                Role::System => {
                    system_prompt = msg.content.as_text().to_string();
                }
                Role::User => {
                    anthropic_messages.push(serde_json::json!({
                        "role": "user",
                        "content": msg.content.as_text()
                    }));
                }
                Role::Assistant => {
                    anthropic_messages.push(serde_json::json!({
                        "role": "assistant",
                        "content": msg.content.as_text()
                    }));
                }
                Role::Tool => {
                    anthropic_messages.push(serde_json::json!({
                        "role": "user",
                        "content": msg.content.as_text()
                    }));
                }
                Role::Custom => {}
            }
        }

        let mut body = serde_json::json!({
            "model": model,
            "messages": anthropic_messages,
            "max_tokens": 4096,
        });

        if !system_prompt.is_empty() {
            body["system"] = serde_json::json!(system_prompt);
        }

        if thinking_enabled {
            body["thinking"] = serde_json::json!({
                "type": "enabled",
                "budget_tokens": 1024
            });
        }

        let response = self.client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::new(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::new(format!("API error ({}): {}", status, error_text)));
        }

        let response: AnthropicResponse = response.json()
            .await
            .map_err(|e| ProviderError::new(format!("Failed to parse response: {}", e)))?;

        let content = response.content.first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        let thinking = response.content.iter()
            .find(|c| c.r#type == "thinking")
            .map(|c| c.thinking.clone());

        let choices = vec![ProviderChoice {
            index: 0,
            message: crate::core::Message::assistant(content, Some("anthropic"), Some(model)),
            finish_reason: response.stop_reason,
        }];

        Ok(ProviderResponse {
            id: response.id,
            model: model.to_string(),
            choices,
            usage: Usage::with_cache(
                response.usage.cache_read_tokens,
                response.usage.cache_creation_tokens,
                response.usage.input_tokens,
                response.usage.output_tokens,
            ),
            thinking: thinking.flatten(),
        })
    }

    async fn chat_stream(
        &self,
        _model: &str,
        _messages: Vec<crate::core::Message>,
        _tools: Option<Vec<serde_json::Value>>,
        _thinking: Option<bool>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamingChunk, ProviderError>> + Send>>, ProviderError> {
        Err(ProviderError::new("Streaming not implemented for Anthropic"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        // Rough estimate: ~4 characters per token
        (text.len() as u64 / 4) + 1
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct AnthropicResponse {
    id: String,
    #[serde(rename = "type")]
    response_type: String,
    role: String,
    content: Vec<AnthropicContent>,
    model: String,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnthropicContent {
    #[serde(rename = "type")]
    r#type: String,
    text: String,
    thinking: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnthropicUsage {
    input_tokens: u64,
    output_tokens: u64,
    cache_read_tokens: u64,
    cache_creation_tokens: u64,
}

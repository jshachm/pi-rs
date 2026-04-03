//! Mistral provider

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

pub struct MistralProvider {
    api_key: String,
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl MistralProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.mistral.ai")
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "mistral-large-latest".to_string(),
                    name: "Mistral Large".to_string(),
                    provider: "mistral".to_string(),
                    context_window: 128000,
                    max_tokens: 32000,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 2.0,
                        output: 6.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "mistral-medium-latest".to_string(),
                    name: "Mistral Medium".to_string(),
                    provider: "mistral".to_string(),
                    context_window: 128000,
                    max_tokens: 32000,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.85,
                        output: 2.5,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "mistral-small-latest".to_string(),
                    name: "Mistral Small".to_string(),
                    provider: "mistral".to_string(),
                    context_window: 128000,
                    max_tokens: 32000,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.2,
                        output: 0.6,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "codestral-latest".to_string(),
                    name: "Codestral".to_string(),
                    provider: "mistral".to_string(),
                    context_window: 256000,
                    max_tokens: 32000,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.2,
                        output: 0.6,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
            ],
        }
    }
}

#[async_trait]
impl Provider for MistralProvider {
    fn name(&self) -> &str {
        "mistral"
    }

    fn models(&self) -> Vec<Model> {
        self.models.clone()
    }

    async fn chat(
        &self,
        model: &str,
        messages: Vec<crate::core::Message>,
        tools: Option<Vec<serde_json::Value>>,
        _thinking: Option<bool>,
    ) -> Result<ProviderResponse, ProviderError> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let mut request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
        });

        if let Some(t) = tools {
            request_body["tools"] = serde_json::json!(t);
        }

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ProviderError::new(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::new(format!("Mistral API error: {} - {}", status, text)));
        }

        let body: MistralResponse = response.json().await
            .map_err(|e| ProviderError::new(e.to_string()))?;

        Ok(ProviderResponse {
            id: body.id,
            model: body.model,
            choices: body.choices.into_iter().map(|c| {
                let content = c.message.content.unwrap_or_default();
                ProviderChoice {
                    index: c.index,
                    message: crate::core::Message {
                        role: Role::Assistant,
                        content: crate::core::MessageContent::Text(content),
                        tool_call_id: None,
                        provider: Some("mistral".to_string()),
                        model: Some(model.to_string()),
                        thinking: None,
                        timestamp: Some(chrono::Utc::now().timestamp_millis()),
                        tool_calls: None,
                    },
                    finish_reason: c.finish_reason,
                }
            }).collect(),
            usage: Usage {
                input_tokens: body.usage.prompt_tokens as u64,
                output_tokens: body.usage.completion_tokens as u64,
                cache_read_tokens: 0,
                cache_write_tokens: 0,
                total_tokens: body.usage.total_tokens as u64,
            },
            thinking: None,
        })
    }

    async fn chat_stream(
        &self,
        _model: &str,
        _messages: Vec<crate::core::Message>,
        _tools: Option<Vec<serde_json::Value>>,
        _thinking: Option<bool>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamingChunk, ProviderError>> + Send>>, ProviderError> {
        Err(ProviderError::new("Streaming not yet implemented for Mistral"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        text.chars().count() as u64 / 4
    }
}

#[derive(Deserialize)]
struct MistralResponse {
    id: String,
    model: String,
    choices: Vec<MistralChoice>,
    usage: MistralUsage,
}

#[derive(Deserialize)]
struct MistralChoice {
    index: u32,
    message: MistralMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct MistralMessage {
    content: Option<String>,
    role: String,
}

#[derive(Deserialize)]
struct MistralUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

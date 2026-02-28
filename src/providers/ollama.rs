//! Ollama provider (local models)

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

/// Ollama provider
pub struct OllamaProvider {
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl OllamaProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "llama2".to_string(),
                    name: "Llama 2".to_string(),
                    provider: "ollama".to_string(),
                    context_window: 4096,
                    max_tokens: 2048,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.0,
                        output: 0.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "codellama".to_string(),
                    name: "Code Llama".to_string(),
                    provider: "ollama".to_string(),
                    context_window: 16384,
                    max_tokens: 4096,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.0,
                        output: 0.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "mistral".to_string(),
                    name: "Mistral".to_string(),
                    provider: "ollama".to_string(),
                    context_window: 8192,
                    max_tokens: 4096,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.0,
                        output: 0.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
            ],
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    fn models(&self) -> Vec<Model> {
        self.models.clone()
    }

    async fn chat(
        &self,
        model: &str,
        messages: Vec<crate::core::Message>,
        _tools: Option<Vec<serde_json::Value>>,
        _thinking: Option<bool>,
    ) -> Result<ProviderResponse, ProviderError> {
        let ollama_messages: Vec<serde_json::Value> = messages.iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::System => "system",
                    Role::Tool => "tool",
                    Role::Custom => "user",
                };
                serde_json::json!({
                    "role": role,
                    "content": msg.content.as_text()
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": model,
            "messages": ollama_messages,
            "stream": false,
        });

        let response = self.client
            .post(format!("{}/api/chat", self.base_url))
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

        let response: OllamaResponse = response.json()
            .await
            .map_err(|e| ProviderError::new(format!("Failed to parse response: {}", e)))?;

        let content = response.message.content;
        
        let choices = vec![ProviderChoice {
            index: 0,
            message: crate::core::Message::assistant(content, Some("ollama"), Some(model)),
            finish_reason: Some("stop".to_string()),
        }];

        Ok(ProviderResponse {
            id: response.id.unwrap_or_else(|| "unknown".to_string()),
            model: model.to_string(),
            choices,
            usage: Usage::default(),
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
        Err(ProviderError::new("Streaming not implemented for Ollama"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        (text.len() as u64 / 4) + 1
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OllamaResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    message: OllamaMessage,
    done: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

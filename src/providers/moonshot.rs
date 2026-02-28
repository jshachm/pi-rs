//! Moonshot (Kimi) provider implementation

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

/// Moonshot provider
pub struct MoonshotProvider {
    api_key: String,
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl MoonshotProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.moonshot.cn/v1".to_string(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "moonshot-v1-8k".to_string(),
                    name: "Moonshot v1 8K".to_string(),
                    provider: "moonshot".to_string(),
                    context_window: 8000,
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
                    id: "moonshot-v1-32k".to_string(),
                    name: "Moonshot v1 32K".to_string(),
                    provider: "moonshot".to_string(),
                    context_window: 32000,
                    max_tokens: 8192,
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
                    id: "moonshot-v1-128k".to_string(),
                    name: "Moonshot v1 128K".to_string(),
                    provider: "moonshot".to_string(),
                    context_window: 128000,
                    max_tokens: 16384,
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
impl Provider for MoonshotProvider {
    fn name(&self) -> &str {
        "moonshot"
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
        let openai_messages: Vec<serde_json::Value> = messages.iter()
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

        let mut body = serde_json::json!({
            "model": model,
            "messages": openai_messages,
        });

        if let Some(t) = tools {
            body["tools"] = serde_json::json!(t);
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::new(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::new(format!("API error ({}): {} | Body: {}", status, error_text, body)));
        }

        let response_text = response.text().await.unwrap_or_default();
        let response: MoonshotResponse = serde_json::from_str(&response_text)
            .map_err(|e| ProviderError::new(format!("Failed to parse response: {} | Response: {}", e, response_text)))?;

        let choice = response.choices.first()
            .ok_or_else(|| ProviderError::new("No choices in response"))?;

        let content = choice.message.content.clone();
        
        let choices = vec![ProviderChoice {
            index: choice.index,
            message: crate::core::Message::assistant(content, Some("moonshot"), Some(model)),
            finish_reason: choice.finish_reason.clone(),
        }];

        let usage = response.usage.map(|u| Usage::new(u.prompt_tokens, u.completion_tokens))
            .unwrap_or_default();

        Ok(ProviderResponse {
            id: response.id,
            model: model.to_string(),
            choices,
            usage,
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
        Err(ProviderError::new("Streaming not implemented for Moonshot"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        (text.len() as u64 / 4) + 1
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoonshotResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<MoonshotChoice>,
    usage: Option<MoonshotUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoonshotChoice {
    index: u32,
    message: MoonshotMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoonshotMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MoonshotUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

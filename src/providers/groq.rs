//! Groq provider (fast inference)

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

pub struct GroqProvider {
    api_key: String,
    client: Client,
    models: Vec<Model>,
}

impl GroqProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "llama-3.3-70b-versatile".to_string(),
                    name: "Llama 3.3 70B".to_string(),
                    provider: "groq".to_string(),
                    context_window: 128000,
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
                    id: "llama-3.1-70b-speculative".to_string(),
                    name: "Llama 3.1 70B Speculative".to_string(),
                    provider: "groq".to_string(),
                    context_window: 128000,
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
                    id: "llama-3.1-8b-instant".to_string(),
                    name: "Llama 3.1 8B Instant".to_string(),
                    provider: "groq".to_string(),
                    context_window: 128000,
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
                    id: "mixtral-8x7b-32768".to_string(),
                    name: "Mixtral 8x7B".to_string(),
                    provider: "groq".to_string(),
                    context_window: 32768,
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
                Model {
                    id: "gemma2-9b-it".to_string(),
                    name: "Gemma 2 9B".to_string(),
                    provider: "groq".to_string(),
                    context_window: 8192,
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
            ],
        }
    }
}

#[async_trait]
impl Provider for GroqProvider {
    fn name(&self) -> &str {
        "groq"
    }

    fn models(&self) -> Vec<Model> {
        self.models.clone()
    }

    async fn chat(
        &self,
        model: &str,
        messages: Vec<crate::core::Message>,
        tools: Option<Vec<serde_json::Value>>,
        thinking: Option<bool>,
    ) -> Result<ProviderResponse, ProviderError> {
        let url = "https://api.groq.com/openai/v1/chat/completions";

        let mut request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
        });

        if let Some(t) = tools {
            request_body["tools"] = serde_json::json!(t);
        }

        if let Some(th) = thinking {
            if th {
                request_body["reasoning_effort"] = serde_json::json!("high");
            }
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
            return Err(ProviderError::new(format!("Groq API error: {} - {}", status, text)));
        }

        let body: GroqResponse = response.json().await
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
                        provider: Some("groq".to_string()),
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
        Err(ProviderError::new("Streaming not yet implemented for Groq"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        text.chars().count() as u64 / 4
    }
}

#[derive(Deserialize)]
struct GroqResponse {
    id: String,
    model: String,
    choices: Vec<GroqChoice>,
    usage: GroqUsage,
}

#[derive(Deserialize)]
struct GroqChoice {
    index: u32,
    message: GroqMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct GroqMessage {
    content: Option<String>,
    role: String,
}

#[derive(Deserialize)]
struct GroqUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

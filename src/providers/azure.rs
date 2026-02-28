//! Azure OpenAI provider

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

pub struct AzureProvider {
    api_key: String,
    endpoint: String,
    api_version: String,
    client: Client,
    models: Vec<Model>,
}

impl AzureProvider {
    pub fn new(api_key: impl Into<String>, endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        
        Self {
            api_key: api_key.into(),
            endpoint: endpoint.clone(),
            api_version: "2024-02-15-preview".to_string(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "gpt-4".to_string(),
                    name: "GPT-4".to_string(),
                    provider: "azure".to_string(),
                    context_window: 8192,
                    max_tokens: 4096,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.03,
                        output: 0.06,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "gpt-4-turbo".to_string(),
                    name: "GPT-4 Turbo".to_string(),
                    provider: "azure".to_string(),
                    context_window: 128000,
                    max_tokens: 4096,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.01,
                        output: 0.03,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "gpt-35-turbo".to_string(),
                    name: "GPT-3.5 Turbo".to_string(),
                    provider: "azure".to_string(),
                    context_window: 16385,
                    max_tokens: 4096,
                    supports_thinking: false,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.001,
                        output: 0.002,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
            ],
        }
    }

    fn get_deployment_name(&self, model: &str) -> String {
        model.replace("gpt-", "").replace(".", "-")
    }
}

#[async_trait]
impl Provider for AzureProvider {
    fn name(&self) -> &str {
        "azure"
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
        let deployment = self.get_deployment_name(model);
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, deployment, self.api_version
        );

        let mut request_body = serde_json::json!({
            "messages": messages,
            "max_tokens": 4096,
        });

        if let Some(t) = tools {
            request_body["tools"] = serde_json::json!(t);
        }

        if let Some(th) = thinking {
            if th {
                request_body["extra_body"] = serde_json::json!({"thinking": {"type": "enabled"}});
            }
        }

        let response = self.client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ProviderError::new(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::new(format!("Azure API error: {} - {}", status, text)));
        }

        let body: AzureResponse = response.json().await
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
                        provider: Some("azure".to_string()),
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
        Err(ProviderError::new("Streaming not yet implemented for Azure"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        text.chars().count() as u64 / 4
    }
}

#[derive(Deserialize)]
struct AzureResponse {
    id: String,
    model: String,
    choices: Vec<AzureChoice>,
    usage: AzureUsage,
}

#[derive(Deserialize)]
struct AzureChoice {
    index: u32,
    message: AzureMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct AzureMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct AzureUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

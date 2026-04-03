//! OpenAI provider implementation

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::{Model, Role, Usage};
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

/// OpenAI provider
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl OpenAIProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.openai.com")
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "gpt-4o".to_string(),
                    name: "GPT-4o".to_string(),
                    provider: "openai".to_string(),
                    context_window: 128000,
                    max_tokens: 16384,
                    supports_thinking: false,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 2.5,
                        output: 10.0,
                        cache_read: 1.25,
                        cache_write: 10.0,
                    },
                },
                Model {
                    id: "gpt-4o-mini".to_string(),
                    name: "GPT-4o Mini".to_string(),
                    provider: "openai".to_string(),
                    context_window: 128000,
                    max_tokens: 16384,
                    supports_thinking: false,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.15,
                        output: 0.6,
                        cache_read: 0.075,
                        cache_write: 0.3,
                    },
                },
                Model {
                    id: "o1".to_string(),
                    name: "OpenAI o1".to_string(),
                    provider: "openai".to_string(),
                    context_window: 128000,
                    max_tokens: 32768,
                    supports_thinking: true,
                    input_types: vec!["text".to_string()],
                    cost: crate::core::ModelCost {
                        input: 15.0,
                        output: 60.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
            ],
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
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
                let mut json = serde_json::json!({
                    "role": role,
                    "content": msg.content.as_text()
                });
                if msg.role == Role::Tool {
                    if let Some(ref tool_call_id) = msg.tool_call_id {
                        json["tool_call_id"] = serde_json::json!(tool_call_id);
                    }
                }
                if msg.role == Role::Assistant {
                    if let Some(ref tool_calls) = msg.tool_calls {
                        let tc_json: Vec<serde_json::Value> = tool_calls.iter().map(|tc| {
                            serde_json::json!({
                                "id": tc.id,
                                "type": "function",
                                "function": {
                                    "name": tc.name,
                                    "arguments": tc.input.to_string()
                                }
                            })
                        }).collect();
                        json["tool_calls"] = serde_json::json!(tc_json);
                    }
                }
                json
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
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
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

        let response: OpenAIResponse = response.json()
            .await
            .map_err(|e| ProviderError::new(format!("Failed to parse response: {}", e)))?;

        let choice = response.choices.first()
            .ok_or_else(|| ProviderError::new("No choices in response"))?;

        let content = choice.message.content.clone();
        
        let tool_calls = choice.message.tool_calls.as_ref().map(|calls| {
            calls.iter().map(|c| {
                let input: serde_json::Value = serde_json::from_str(&c.function.arguments).unwrap_or_default();
                crate::core::types::ToolUse {
                    id: c.id.clone(),
                    name: c.function.name.clone(),
                    input,
                }
            }).collect()
        });

        let mut message = crate::core::Message::assistant(content, Some("openai"), Some(model));
        message.tool_calls = tool_calls;
        
        let choices = vec![ProviderChoice {
            index: choice.index,
            message,
            finish_reason: choice.finish_reason.clone(),
        }];

        Ok(ProviderResponse {
            id: response.id,
            model: model.to_string(),
            choices,
            usage: Usage::new(response.usage.prompt_tokens, response.usage.completion_tokens),
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
        Err(ProviderError::new("Streaming not implemented for OpenAI"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        // Rough estimate: ~4 characters per token
        (text.len() as u64 / 4) + 1
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenAIMessage {
    role: String,
    content: String,
    #[serde(default)]
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenAIToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: OpenAIFunctionCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenAIUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

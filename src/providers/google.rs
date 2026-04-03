//! Google provider implementation

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;
use serde::Deserialize;

use crate::core::Model;
use crate::providers::provider::{Provider, ProviderChoice, ProviderError, ProviderResponse, StreamingChunk};

/// Google provider
pub struct GoogleProvider {
    api_key: String,
    base_url: String,
    client: Client,
    models: Vec<Model>,
}

impl GoogleProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://generativelanguage.googleapis.com")
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            client: Client::new(),
            models: vec![
                Model {
                    id: "gemini-2.0-flash".to_string(),
                    name: "Gemini 2.0 Flash".to_string(),
                    provider: "google".to_string(),
                    context_window: 1000000,
                    max_tokens: 8192,
                    supports_thinking: false,
                    input_types: vec!["text".to_string(), "image".to_string()],
                    cost: crate::core::ModelCost {
                        input: 0.0,
                        output: 0.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                },
                Model {
                    id: "gemini-2.0-flash-exp".to_string(),
                    name: "Gemini 2.0 Flash Experimental".to_string(),
                    provider: "google".to_string(),
                    context_window: 1000000,
                    max_tokens: 8192,
                    supports_thinking: true,
                    input_types: vec!["text".to_string(), "image".to_string()],
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
impl Provider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
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
        let contents: Vec<serde_json::Value> = messages.iter()
            .filter(|m| m.role != crate::core::Role::System)
            .map(|msg| {
                let role = match msg.role {
                    crate::core::Role::User => "user",
                    crate::core::Role::Assistant => "model",
                    _ => "user",
                };
                serde_json::json!({
                    "role": role,
                    "parts": [{"text": msg.content.as_text()}]
                })
            })
            .collect();

        let system_instruction = messages.iter()
            .find(|m| m.role == crate::core::Role::System)
            .map(|m| serde_json::json!([{"text": m.content.as_text()}]));

        let mut body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "maxOutputTokens": 8192,
                "temperature": 1.0,
            }
        });

        if let Some(system) = system_instruction {
            body["systemInstruction"] = system;
        }

        let url = format!("{}/v1beta/models/{}:generateContent?key={}", 
            self.base_url, model, self.api_key);

        let response = self.client
            .post(&url)
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

        let response: GoogleResponse = response.json()
            .await
            .map_err(|e| ProviderError::new(format!("Failed to parse response: {}", e)))?;

        let content = response.candidates.first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_default();

        let choices = vec![ProviderChoice {
            index: 0,
            message: crate::core::Message::assistant(content, Some("google"), Some(model)),
            finish_reason: Some("stop".to_string()),
        }];

        Ok(ProviderResponse {
            id: response.model_version.clone(),
            model: model.to_string(),
            choices,
            usage: crate::core::Usage::default(),
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
        Err(ProviderError::new("Streaming not implemented for Google"))
    }

    fn estimate_tokens(&self, text: &str) -> u64 {
        // Rough estimate: ~4 characters per token
        (text.len() as u64 / 4) + 1
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleResponse {
    candidates: Vec<GoogleCandidate>,
    model_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleCandidate {
    content: GoogleContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleContent {
    parts: Vec<GooglePart>,
    role: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GooglePart {
    text: String,
}

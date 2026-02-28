//! Provider trait for LLM interactions

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::core::{Message, Model, Usage};

/// Provider error
#[derive(Debug, Serialize)]
pub struct ProviderError {
    pub message: String,
    pub code: Option<String>,
}

impl ProviderError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }
    
    pub fn with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some(code.into()),
        }
    }
}

/// Response from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ProviderChoice>,
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
}

/// Choice in response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderChoice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

/// Streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingChunk {
    pub id: String,
    pub choices: Vec<StreamingChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingChoice {
    pub index: u32,
    pub delta: StreamingDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Provider trait
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;
    
    /// Get available models
    fn models(&self) -> Vec<Model>;
    
    /// Chat completion
    async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
        thinking: Option<bool>,
    ) -> Result<ProviderResponse, ProviderError>;
    
    /// Streaming chat
    async fn chat_stream(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
        thinking: Option<bool>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamingChunk, ProviderError>> + Send>>, ProviderError>;
    
    /// Estimate token count
    fn estimate_tokens(&self, text: &str) -> u64;
}

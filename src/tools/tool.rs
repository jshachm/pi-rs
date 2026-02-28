//! Tool definitions

use crate::core::errors::Result;
use serde::{Deserialize, Serialize};

/// Tool parameter schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub r#type: String,
    pub properties: serde_json::Value,
    pub required: Vec<String>,
}

impl ToolSchema {
    pub fn from_json(schema: serde_json::Value) -> Self {
        Self {
            r#type: "object".to_string(),
            properties: schema,
            required: vec![],
        }
    }
}

/// Tool call from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub success: bool,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

impl ToolResult {
    pub fn success(content: impl Into<String>) -> Self {
        Self {
            success: true,
            content: content.into(),
            error: None,
            truncated: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            content: String::new(),
            error: Some(message.into()),
            truncated: None,
        }
    }

    pub fn truncated(content: impl Into<String>) -> Self {
        Self {
            success: true,
            content: content.into(),
            error: None,
            truncated: Some(true),
        }
    }
}

/// Tool definition - synchronous version
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub schema: ToolSchema,
    pub execute: fn(args: serde_json::Value, cwd: &str) -> Result<ToolResult>,
}

impl Tool {
    pub fn new(
        name: &str,
        description: &str,
        schema: ToolSchema,
        execute: fn(serde_json::Value, &str) -> Result<ToolResult>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            schema,
            execute,
        }
    }
}

/// Tool trait for async execution
pub trait ToolTrait: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> &ToolSchema;
    fn execute(&self, args: serde_json::Value, cwd: &str) -> Result<ToolResult>;
}

/// Wrapper to convert sync Tool to trait object
pub struct ToolWrapper {
    name: String,
    description: String,
    schema: ToolSchema,
    execute: fn(serde_json::Value, &str) -> Result<ToolResult>,
}

impl ToolWrapper {
    pub fn from_tool(tool: Tool) -> Self {
        Self {
            name: tool.name,
            description: tool.description,
            schema: tool.schema,
            execute: tool.execute,
        }
    }
}

impl ToolTrait for ToolWrapper {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn schema(&self) -> &ToolSchema {
        &self.schema
    }

    fn execute(&self, args: serde_json::Value, cwd: &str) -> Result<ToolResult> {
        (self.execute)(args, cwd)
    }
}

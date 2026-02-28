//! Extension types

use serde::{Deserialize, Serialize};

/// Extension info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

/// Extension API (placeholder)
pub struct ExtensionAPI {
    // Placeholder for extension API
}

impl Default for ExtensionAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensionAPI {
    pub fn new() -> Self {
        Self {}
    }
}

/// Extension result
#[derive(Debug, Clone)]
pub struct ExtensionResult {
    pub success: bool,
    pub message: String,
}

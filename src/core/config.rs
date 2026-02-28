//! Configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Get the default agent config directory
pub fn get_agent_dir() -> PathBuf {
    dirs::home_dir()
        .map(|p| p.join(".pi").join("agent"))
        .unwrap_or_else(|| PathBuf::from(".pi/agent"))
}

/// Get the sessions directory
pub fn get_sessions_dir() -> PathBuf {
    get_agent_dir().join("sessions")
}

/// Get the config file path
pub fn get_config_path() -> PathBuf {
    get_agent_dir().join("settings.json")
}

/// Get the auth storage path
pub fn get_auth_path() -> PathBuf {
    get_agent_dir().join("auth.json")
}

/// Get the models config path
pub fn get_models_path() -> PathBuf {
    get_agent_dir().join("models.json")
}

/// Get extensions directory
pub fn get_extensions_dir() -> PathBuf {
    get_agent_dir().join("extensions")
}

/// Get skills directory
pub fn get_skills_dir() -> PathBuf {
    get_agent_dir().join("skills")
}

/// Get prompts directory
pub fn get_prompts_dir() -> PathBuf {
    get_agent_dir().join("prompts")
}

/// Get themes directory
pub fn get_themes_dir() -> PathBuf {
    get_agent_dir().join("themes")
}

/// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub thinking_level: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub transport: String,
    #[serde(default)]
    pub steering_mode: String,
    #[serde(default)]
    pub follow_up_mode: String,
    #[serde(default = "default_true")]
    pub show_images: bool,
    #[serde(default = "default_true")]
    pub auto_compact: bool,
    #[serde(default)]
    pub compact_threshold: f64,
    #[serde(default = "default_true")]
    pub compact_proactive: bool,
    #[serde(default)]
    pub max_tokens: u64,
}

fn default_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            thinking_level: "medium".to_string(),
            theme: "dark".to_string(),
            transport: "auto".to_string(),
            steering_mode: "one-at-a-time".to_string(),
            follow_up_mode: "one-at-a-time".to_string(),
            show_images: true,
            auto_compact: true,
            compact_threshold: 0.9,
            compact_proactive: true,
            max_tokens: 8192,
        }
    }
}

/// Session settings (project-level)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionSettings {
    #[serde(default)]
    pub enabled_tools: Vec<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub append_system_prompt: Option<String>,
}

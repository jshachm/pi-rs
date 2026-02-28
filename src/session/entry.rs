//! Session entry types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::{Message, MessageContent, ThinkingLevel};

/// Current session version
pub const CURRENT_SESSION_VERSION: u32 = 3;

/// Session header (first line in JSONL file)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionHeader {
    pub r#type: String,
    pub version: Option<u32>,
    pub id: String,
    pub timestamp: String,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_session: Option<String>,
}

impl SessionHeader {
    pub fn new(id: String, cwd: &str, parent_session: Option<&str>) -> Self {
        Self {
            r#type: "session".to_string(),
            version: Some(CURRENT_SESSION_VERSION),
            id,
            timestamp: Utc::now().to_rfc3339(),
            cwd: cwd.to_string(),
            parent_session: parent_session.map(String::from),
        }
    }
}

/// Base fields for all session entries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntryBase {
    pub r#type: String,
    pub id: String,
    pub parent_id: Option<String>,
    pub timestamp: String,
}

/// User or assistant message entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMessageEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub message: Message,
}

impl SessionMessageEntry {
    pub fn new(id: String, parent_id: Option<String>, message: Message) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "message".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            message,
        }
    }
}

/// Thinking level change entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingLevelChangeEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub thinking_level: String,
}

impl ThinkingLevelChangeEntry {
    pub fn new(id: String, parent_id: Option<String>, level: ThinkingLevel) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "thinking_level_change".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            thinking_level: level.as_str().to_string(),
        }
    }
}

/// Model change entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelChangeEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub provider: String,
    pub model_id: String,
}

impl ModelChangeEntry {
    pub fn new(id: String, parent_id: Option<String>, provider: &str, model_id: &str) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "model_change".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            provider: provider.to_string(),
            model_id: model_id.to_string(),
        }
    }
}

/// Compaction summary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactionEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub summary: String,
    pub first_kept_entry_id: String,
    pub tokens_before: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_hook: Option<bool>,
}

impl CompactionEntry {
    pub fn new(
        id: String,
        parent_id: Option<String>,
        summary: &str,
        first_kept_entry_id: &str,
        tokens_before: u64,
    ) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "compaction".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            summary: summary.to_string(),
            first_kept_entry_id: first_kept_entry_id.to_string(),
            tokens_before,
            details: None,
            from_hook: None,
        }
    }
}

/// Branch summary entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchSummaryEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub from_id: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_hook: Option<bool>,
}

impl BranchSummaryEntry {
    pub fn new(id: String, parent_id: Option<String>, from_id: &str, summary: &str) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "branch_summary".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            from_id: from_id.to_string(),
            summary: summary.to_string(),
            details: None,
            from_hook: None,
        }
    }
}

/// Custom entry for extensions (not sent to LLM)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub custom_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl CustomEntry {
    pub fn new(id: String, parent_id: Option<String>, custom_type: &str) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "custom".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            custom_type: custom_type.to_string(),
            data: None,
        }
    }
}

/// Custom message entry for extensions (sent to LLM)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomMessageEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub custom_type: String,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub display: bool,
}

impl CustomMessageEntry {
    pub fn new(
        id: String,
        parent_id: Option<String>,
        custom_type: &str,
        content: MessageContent,
        display: bool,
    ) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "custom_message".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            custom_type: custom_type.to_string(),
            content,
            details: None,
            display,
        }
    }
}

/// Label entry for bookmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub target_id: String,
    pub label: Option<String>,
}

impl LabelEntry {
    pub fn new(
        id: String,
        parent_id: Option<String>,
        target_id: &str,
        label: Option<&str>,
    ) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "label".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            target_id: target_id.to_string(),
            label: label.map(String::from),
        }
    }
}

/// Session info entry (display name)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfoEntry {
    #[serde(flatten)]
    pub base: SessionEntryBase,
    pub name: Option<String>,
}

impl SessionInfoEntry {
    pub fn new(id: String, parent_id: Option<String>, name: &str) -> Self {
        Self {
            base: SessionEntryBase {
                r#type: "session_info".to_string(),
                id,
                parent_id,
                timestamp: Utc::now().to_rfc3339(),
            },
            name: Some(name.to_string()),
        }
    }
}

/// Any session entry type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionEntry {
    Message(SessionMessageEntry),
    ThinkingLevelChange(ThinkingLevelChangeEntry),
    ModelChange(ModelChangeEntry),
    Compaction(CompactionEntry),
    BranchSummary(BranchSummaryEntry),
    Custom(CustomEntry),
    CustomMessage(CustomMessageEntry),
    Label(LabelEntry),
    SessionInfo(SessionInfoEntry),
}

impl SessionEntry {
    pub fn id(&self) -> &str {
        match self {
            Self::Message(e) => &e.base.id,
            Self::ThinkingLevelChange(e) => &e.base.id,
            Self::ModelChange(e) => &e.base.id,
            Self::Compaction(e) => &e.base.id,
            Self::BranchSummary(e) => &e.base.id,
            Self::Custom(e) => &e.base.id,
            Self::CustomMessage(e) => &e.base.id,
            Self::Label(e) => &e.base.id,
            Self::SessionInfo(e) => &e.base.id,
        }
    }

    pub fn parent_id(&self) -> Option<&str> {
        match self {
            Self::Message(e) => e.base.parent_id.as_deref(),
            Self::ThinkingLevelChange(e) => e.base.parent_id.as_deref(),
            Self::ModelChange(e) => e.base.parent_id.as_deref(),
            Self::Compaction(e) => e.base.parent_id.as_deref(),
            Self::BranchSummary(e) => e.base.parent_id.as_deref(),
            Self::Custom(e) => e.base.parent_id.as_deref(),
            Self::CustomMessage(e) => e.base.parent_id.as_deref(),
            Self::Label(e) => e.base.parent_id.as_deref(),
            Self::SessionInfo(e) => e.base.parent_id.as_deref(),
        }
    }

    pub fn timestamp(&self) -> &str {
        match self {
            Self::Message(e) => &e.base.timestamp,
            Self::ThinkingLevelChange(e) => &e.base.timestamp,
            Self::ModelChange(e) => &e.base.timestamp,
            Self::Compaction(e) => &e.base.timestamp,
            Self::BranchSummary(e) => &e.base.timestamp,
            Self::Custom(e) => &e.base.timestamp,
            Self::CustomMessage(e) => &e.base.timestamp,
            Self::Label(e) => &e.base.timestamp,
            Self::SessionInfo(e) => &e.base.timestamp,
        }
    }
}

/// Raw file entry (header or session entry)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileEntry {
    Header(SessionHeader),
    Entry(SessionEntry),
}

/// Session context (what gets sent to LLM)
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub messages: Vec<Message>,
    pub thinking_level: ThinkingLevel,
    pub model: Option<(String, String)>, // (provider, model_id)
}

/// Session tree node
#[derive(Debug, Clone)]
pub struct SessionTreeNode {
    pub entry: SessionEntry,
    pub children: Vec<SessionTreeNode>,
    pub label: Option<String>,
}

/// Session info for listing
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub path: String,
    pub id: String,
    pub cwd: String,
    pub name: Option<String>,
    pub parent_session_path: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub message_count: u32,
    pub first_message: String,
}

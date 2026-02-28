//! Pi - Terminal AI Coding Agent
//!
//! A minimal terminal coding harness with session management,
//! tool system, and multi-provider LLM support.

pub mod core;
pub mod session;
pub mod tools;
pub mod providers;
pub mod cli;
pub mod utils;
pub mod extensions;
pub mod agent;
pub mod settings;
pub mod auth;
pub mod tui;
pub mod input;
pub mod theme;
pub mod skills;
pub mod prompts;
pub mod compaction;

pub use session::{SessionManager, SessionEntry};
pub use tools::{Tool, ToolResult, ToolSchema, ToolTrait, coding_tools, coding_tools_arc, read_tool, write_tool, edit_tool, bash_tool, grep_tool, find_tool, ls_tool, get_tool_by_name};
pub use core::{Message, TextContent, ImageContent, Role, Model, ThinkingLevel};
pub use agent::AgentSession;
pub use agent::session::AgentConfig;
pub use agent::session::AgentState;
pub use agent::events::EventType;
pub use settings::{Settings, SettingsManager};
pub use auth::{AuthStorage, Credential};
pub use input::{InputHandler, CompletionEngine};
pub use theme::Theme;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

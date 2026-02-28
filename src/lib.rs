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

pub use session::{SessionManager, SessionEntry};
pub use tools::{Tool, ToolResult, ToolCall, coding_tools, read_tool, write_tool, edit_tool, bash_tool, grep_tool, find_tool, ls_tool};
pub use core::{Message, TextContent, ImageContent, Role, Model, ThinkingLevel};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

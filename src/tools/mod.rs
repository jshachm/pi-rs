//! Tool system for the coding agent
//!
//! Provides built-in tools: read, write, edit, bash, grep, find, ls

pub mod tool;
pub mod read;
pub mod write;
pub mod edit;
pub mod bash;
pub mod grep;
pub mod find;
pub mod ls;

pub use tool::{Tool, ToolCall, ToolResult, ToolSchema};

pub use read::read_tool;
pub use write::write_tool;
pub use edit::edit_tool;
pub use bash::bash_tool;
pub use grep::grep_tool;
pub use find::find_tool;
pub use ls::ls_tool;

/// All coding tools (read, bash, edit, write)
pub fn coding_tools() -> Vec<Tool> {
    vec![
        read_tool(),
        bash_tool(),
        edit_tool(),
        write_tool(),
    ]
}

/// Read-only tools (read, grep, find, ls)
pub fn read_only_tools() -> Vec<Tool> {
    vec![
        read_tool(),
        grep_tool(),
        find_tool(),
        ls_tool(),
    ]
}

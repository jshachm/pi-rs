//! Tool system for the coding agent
//!
//! Provides built-in tools: read, write, edit, bash, grep, find, ls

use std::sync::Arc;

pub mod bash;
pub mod edit;
pub mod find;
pub mod grep;
pub mod ls;
pub mod read;
pub mod tool;
pub mod write;

pub use tool::{Tool, ToolResult, ToolSchema, ToolTrait, ToolWrapper};

pub use bash::bash_tool;
pub use edit::edit_tool;
pub use find::find_tool;
pub use grep::grep_tool;
pub use ls::ls_tool;
pub use read::read_tool;
pub use write::write_tool;

/// All coding tools (read, bash, edit, write)
pub fn coding_tools() -> Vec<Tool> {
    vec![read_tool(), bash_tool(), edit_tool(), write_tool()]
}

/// All coding tools as Arc<dyn ToolTrait>
pub fn coding_tools_arc() -> Vec<Arc<dyn ToolTrait>> {
    vec![
        Arc::new(ToolWrapper::from_tool(read_tool())),
        Arc::new(ToolWrapper::from_tool(bash_tool())),
        Arc::new(ToolWrapper::from_tool(edit_tool())),
        Arc::new(ToolWrapper::from_tool(write_tool())),
    ]
}

/// Read-only tools (read, grep, find, ls)
pub fn read_only_tools() -> Vec<Tool> {
    vec![read_tool(), grep_tool(), find_tool(), ls_tool()]
}

/// Read-only tools as Arc<dyn ToolTrait>
pub fn read_only_tools_arc() -> Vec<Arc<dyn ToolTrait>> {
    vec![
        Arc::new(ToolWrapper::from_tool(read_tool())),
        Arc::new(ToolWrapper::from_tool(grep_tool())),
        Arc::new(ToolWrapper::from_tool(find_tool())),
        Arc::new(ToolWrapper::from_tool(ls_tool())),
    ]
}

/// Get tool by name
pub fn get_tool_by_name(name: &str) -> Option<Arc<dyn ToolTrait>> {
    match name {
        "read" => Some(Arc::new(ToolWrapper::from_tool(read_tool()))),
        "write" => Some(Arc::new(ToolWrapper::from_tool(write_tool()))),
        "edit" => Some(Arc::new(ToolWrapper::from_tool(edit_tool()))),
        "bash" => Some(Arc::new(ToolWrapper::from_tool(bash_tool()))),
        "grep" => Some(Arc::new(ToolWrapper::from_tool(grep_tool()))),
        "find" => Some(Arc::new(ToolWrapper::from_tool(find_tool()))),
        "ls" => Some(Arc::new(ToolWrapper::from_tool(ls_tool()))),
        _ => None,
    }
}

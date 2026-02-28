//! App state for TUI

use crate::agent::AgentSession;

/// Application state
pub struct AppState {
    pub agent: Option<AgentSession>,
    pub is_thinking: bool,
    pub current_input: String,
    pub scroll_offset: usize,
    pub show_help: bool,
    pub show_tools: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            agent: None,
            is_thinking: false,
            current_input: String::new(),
            scroll_offset: 0,
            show_help: false,
            show_tools: false,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

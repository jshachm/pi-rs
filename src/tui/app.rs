//! TUI Application - Simplified version

use std::io::{self, Write};
use std::sync::Arc;

use crate::providers::ModelRegistry;
use crate::session::SessionManager;
use crate::tools::Tool;
use crate::tui::state::AppState;

pub struct TuiApp {
    state: AppState,
    session: SessionManager,
    model_registry: Arc<ModelRegistry>,
    tools: Vec<Tool>,
}

impl TuiApp {
    pub fn new(
        session: SessionManager,
        model_registry: Arc<ModelRegistry>,
        tools: Vec<Tool>,
    ) -> Self {
        Self {
            state: AppState::new(),
            session,
            model_registry,
            tools,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        println!("\x1b[2J\x1b[H"); // Clear screen
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║           Pi - Terminal AI Coding Agent                     ║");
        println!("╚══════════════════════════════════════════════════════════════════╝");
        println!();
        println!(
            "Session: {} | Provider: moonshot | Model: moonshot-v1-8k",
            &self.session.get_session_id()[..8]
        );
        println!("Type '/help' for commands, '/quit' to exit");
        println!();

        loop {
            print!("\n> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input == "/quit" || input == "/q" {
                println!("Goodbye!");
                break;
            }

            if input == "/help" {
                println!("Commands:");
                println!("  /new     - Start new session");
                println!("  /tree    - Show session tree");
                println!("  /compact - Compact context");
                println!("  /model   - Show current model");
                println!("  /help    - Show this help");
                println!("  /quit    - Exit");
                continue;
            }

            if input.starts_with('/') {
                println!("Unknown command: {}", input);
                continue;
            }

            // Display user message
            println!("\n[User] {}", input);
            println!("[Assistant] Processing...");
        }

        Ok(())
    }
}

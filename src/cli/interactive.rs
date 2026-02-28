//! Interactive TUI mode

use std::io::{self, Write};
use std::sync::Arc;

use crate::core::{Message, ThinkingLevel};
use crate::session::SessionManager;
use crate::tools::Tool;
use crate::providers::ModelRegistry;

/// Interactive mode
pub struct InteractiveMode {
    session: SessionManager,
    model_registry: Arc<ModelRegistry>,
    tools: Vec<Tool>,
}

impl InteractiveMode {
    pub fn new(
        session: SessionManager,
        model_registry: Arc<ModelRegistry>,
        tools: Vec<Tool>,
    ) -> Self {
        Self {
            session,
            model_registry,
            tools,
        }
    }

    /// Run the interactive loop
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Print welcome message
        self.print_welcome();

        // Main loop
        loop {
            // Get user input
            print!("\n> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // Handle commands
            if input.starts_with('/') {
                if let Err(e) = self.handle_command(input).await {
                    eprintln!("Error: {}", e);
                }
                continue;
            }

            // Exit commands
            if input == "quit" || input == "exit" || input == "q" {
                println!("Goodbye!");
                break;
            }

            // Send message to agent
            match self.send_message(input).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(())
    }

    fn print_welcome(&self) {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                     Pi - Coding Agent                          ║");
        println!("║                                                                  ║");
        println!("║  Type /help for commands, /quit to exit                        ║");
        println!("╚══════════════════════════════════════════════════════════════════╝");
        println!();
    }

    async fn handle_command(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd = parts.first().unwrap_or(&"");

        match *cmd {
            "/help" => {
                println!("Commands:");
                println!("  /new          - Start new session");
                println!("  /continue     - Continue session");
                println!("  /tree         - Show session tree");
                println!("  /compact      - Compact context");
                println!("  /model        - Select model");
                println!("  /thinking     - Set thinking level");
                println!("  /session      - Show session info");
                println!("  /help         - Show this help");
                println!("  /quit         - Exit");
            }
            "/new" => {
                self.session.new_session(None);
                println!("Started new session");
            }
            "/model" => {
                let models = self.model_registry.get_all_models();
                println!("Available models:");
                for model in models {
                    println!("  {} ({})", model.id, model.name);
                }
            }
            "/thinking" => {
                if let Some(level) = parts.get(1) {
                    let level = match *level {
                        "off" => ThinkingLevel::Off,
                        "minimal" => ThinkingLevel::Minimal,
                        "low" => ThinkingLevel::Low,
                        "medium" => ThinkingLevel::Medium,
                        "high" => ThinkingLevel::High,
                        "xhigh" => ThinkingLevel::XHigh,
                        _ => {
                            println!("Invalid level. Use: off, minimal, low, medium, high, xhigh");
                            return Ok(());
                        }
                    };
                    self.session.append_thinking_level_change(level);
                    println!("Thinking level set to {}", level.as_str());
                } else {
                    println!("Usage: /thinking <level>");
                    println!("Levels: off, minimal, low, medium, high, xhigh");
                }
            }
            "/session" => {
                println!("Session ID: {}", self.session.get_session_id());
                println!("Working directory: {}", self.session.get_cwd());
                if let Some(name) = self.session.get_session_name() {
                    println!("Name: {}", name);
                }
            }
            "/tree" => {
                let tree = self.session.get_tree();
                self.print_tree(&tree, 0);
            }
            "/quit" | "/exit" => {
                std::process::exit(0);
            }
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }

        Ok(())
    }

    fn print_tree(&self, nodes: &[crate::session::SessionTreeNode], depth: usize) {
        for node in nodes {
            let prefix = "  ".repeat(depth);
            let marker = if node.children.is_empty() { "└─ " } else { "├─ " };
            
            let label = match &node.entry {
                crate::session::SessionEntry::Message(m) => {
                    match m.message.role {
                        crate::core::Role::User => "User".to_string(),
                        crate::core::Role::Assistant => "Assistant".to_string(),
                        _ => "Message".to_string(),
                    }
                }
                crate::session::SessionEntry::Compaction(_) => "Compaction".to_string(),
                crate::session::SessionEntry::ModelChange(m) => format!("Model: {}", m.model_id),
                crate::session::SessionEntry::ThinkingLevelChange(t) => format!("Thinking: {}", t.thinking_level),
                _ => node.entry.id().to_string(),
            };

            println!("{}{}{}", prefix, marker, label);
            
            if !node.children.is_empty() {
                self.print_tree(&node.children, depth + 1);
            }
        }
    }

    async fn send_message(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Add user message to session
        let message = Message::user(content);
        self.session.append_message(message);

        // Build context
        let context = self.session.build_session_context();
        
        println!("\n[Context: {} messages, thinking: {}]", 
            context.messages.len(),
            context.thinking_level.as_str()
        );

        // For now, just print what would be sent
        // In a full implementation, this would call the LLM
        println!("\n[This is where the LLM would respond]");
        println!("[Tool execution would happen here]");

        Ok(())
    }
}

//! Pi - Terminal AI Coding Agent
//!
//! Main entry point

use std::sync::Arc;
use std::path::PathBuf;

use clap::Parser;

mod cli;
mod session;
mod tools;
mod providers;
mod core;
mod extensions;
mod utils;

use cli::Args;
use cli::interactive::InteractiveMode;
use session::SessionManager;
use tools::coding_tools;
use tools::Tool;
use providers::ModelRegistry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Args::parse();

    // Get current directory
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    // Initialize session
    let session = if args.no_session {
        SessionManager::in_memory(&cwd)
    } else if let Some(ref session_path) = args.session {
        SessionManager::open(&PathBuf::from(session_path), None)
    } else if args.continue_session {
        SessionManager::continue_recent(&cwd, None)
    } else if args.resume {
        SessionManager::continue_recent(&cwd, None)
    } else {
        SessionManager::create(&cwd, None)
    };

    // Initialize model registry
    let model_registry = Arc::new(ModelRegistry::new());

    // List models if requested
    if args.list_models {
        println!("Available models:");
        for model in model_registry.get_all_models() {
            println!("  {} ({}): {}", model.id, model.provider, model.name);
            println!("    Context: {} tokens", model.context_window);
            println!("    Max output: {} tokens", model.max_tokens);
            println!();
        }
        return Ok(());
    }

    // Get tools
    let tools: Vec<Tool> = if args.no_tools {
        vec![]
    } else if let Some(ref tools_str) = args.tools {
        // Parse specific tools
        let tool_names: Vec<&str> = tools_str.split(',').map(|s| s.trim()).collect();
        let mut selected: Vec<Tool> = vec![];
        
        for name in tool_names {
            match name {
                "read" => selected.push(tools::read_tool()),
                "write" => selected.push(tools::write_tool()),
                "edit" => selected.push(tools::edit_tool()),
                "bash" => selected.push(tools::bash_tool()),
                "grep" => selected.push(tools::grep_tool()),
                "find" => selected.push(tools::find_tool()),
                "ls" => selected.push(tools::ls_tool()),
                _ => eprintln!("Unknown tool: {}", name),
            }
        }
        selected
    } else {
        coding_tools()
    };

    // Print mode
    if args.print {
        if !args.message.is_empty() {
            println!("Print mode: {}", args.message);
            // In a full implementation, would send to LLM and print response
        }
        return Ok(());
    }

    // Interactive mode
    let mut interactive = InteractiveMode::new(
        session,
        model_registry,
        tools,
    );

    // If there's an initial message, run it first
    if !args.message.is_empty() {
        // In a full implementation, would handle initial message
        println!("Initial message: {}", args.message);
    }

    // Run interactive loop
    interactive.run().await?;

    Ok(())
}

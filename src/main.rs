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
mod agent;
mod skills;

use cli::Args;
use cli::interactive::InteractiveMode;
use session::SessionManager;
use tools::{coding_tools, Tool};
use tools::ToolWrapper;
use providers::ModelRegistry;
use agent::AgentSession;
use agent::session::AgentConfig;
use skills::SkillLoader;

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

    // Get provider and model from args
    let provider_name = args.provider.unwrap_or_else(|| {
        if let Some(ref model) = args.model {
            model_registry.get_provider_for_model(model)
                .unwrap_or_else(|| model_registry.list_providers().first().cloned().unwrap_or_else(|| "moonshot".to_string()))
        } else {
            model_registry.list_providers().first().cloned().unwrap_or_else(|| "moonshot".to_string())
        }
    });
    
    let model_id = args.model.unwrap_or_else(|| {
        model_registry.get_models_for_provider(&provider_name)
            .and_then(|models| models.first().map(|m| m.id.clone()))
            .unwrap_or_else(|| "moonshot-v1-8k".to_string())
    });

    // If there's a message, do a simple chat
    if !args.message.is_empty() {
        // Get provider
        let provider = model_registry.get_provider(&provider_name)
            .ok_or_else(|| format!("Provider not found: {}", provider_name))?;

        // Get tools for the agent
        let tools_arc: Vec<Arc<dyn tools::ToolTrait>> = coding_tools()
            .into_iter()
            .map(|t| Arc::new(ToolWrapper::from_tool(t)) as Arc<dyn tools::ToolTrait>)
            .collect();

        // Load skills if enabled
        let mut skill_loader = SkillLoader::new(PathBuf::from("."));
        if !args.no_skills {
            for skill_path in &args.skills {
                skill_loader.add_search_path(PathBuf::from(skill_path));
            }
            skill_loader.load_skills();
        }

        // Create agent config
        let config = AgentConfig {
            provider: provider_name.clone(),
            model: model_id.clone(),
            thinking_level: core::ThinkingLevel::Off,
            cwd: cwd.clone(),
            tools: vec!["read".to_string(), "write".to_string(), "bash".to_string()],
        };

        // Create agent session
        let mut agent = AgentSession::new(
            config,
            session,
            provider,
            tools_arc,
        );

        // Send message and get response
        println!("Sending to {}: {}", provider_name, args.message);
        
        // Prepend skill system prompt if skill is triggered
        let mut message = args.message.clone();
        if !args.no_skills {
            for skill in skill_loader.get_skills() {
                if message.contains(&skill.trigger) {
                    message = format!("{}\n\n{}", skill.content, message);
                    break;
                }
            }
        }
        
        match agent.prompt(&message).await {
            Ok(response) => {
                println!("\n=== Response ===\n{}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        return Ok(());
    }

    // Get tools
    let tools: Vec<Tool> = if args.no_tools {
        vec![]
    } else if let Some(ref tools_str) = args.tools {
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

    // Interactive mode
    let mut interactive = InteractiveMode::new(
        session,
        model_registry,
        tools,
    );

    // Run interactive loop
    interactive.run().await?;

    Ok(())
}

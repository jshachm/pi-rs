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
mod sandbox;

use cli::Args;
use cli::interactive::InteractiveMode;
use session::SessionManager;
use tools::{coding_tools, Tool};
use tools::ToolWrapper;
use providers::ModelRegistry;
use std::collections::HashMap;
use providers::ProviderOverride;
use agent::AgentSession;
use agent::session::AgentConfig;
use skills::SkillLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Args::parse();

    // Handle sandbox mode
    if let Some(project_path) = args.sandbox {
        
        use sandbox::{Sandbox, SandboxConfig};

        // Load config from project directory if exists
        let config = SandboxConfig::load_from_cwd(&std::path::Path::new(&project_path))?;

        // Check if sandbox should be enabled
        let enabled = args.no_sandbox == false;

        if !enabled && !config.enabled {
            // No sandbox requested, continue with normal execution
        } else if enabled {
            // Validate -v without --sandbox is handled by clap (requires = "sandbox")

            // Build sandbox
            let mut sandbox = Sandbox::new(std::path::PathBuf::from(&project_path));

            // Add mounts from CLI
            for mount in &args.sandbox_mounts {
                sandbox.mounts.push(std::path::PathBuf::from(mount));
            }

            // Add mounts from config
            for mount in &config.mounts {
                sandbox.mounts.push(std::path::PathBuf::from(mount));
            }

            // Add env vars from CLI (format: KEY=VALUE)
            for env in &args.sandbox_env {
                if let Some((key, value)) = env.split_once('=') {
                    sandbox.env_vars.insert(key.to_string(), value.to_string());
                }
            }

            // Add env vars from config
            for (key, value) in &config.env {
                sandbox.env_vars.insert(key.clone(), value.clone());
            }

            // Set sandbox type
            let sandbox_type = args.sandbox_type.unwrap_or(config.r#type);
            sandbox.sandbox_type = sandbox_type;

            // Add auto-propagated env vars
            sandbox.add_auto_propagated_env_vars();

            // Launch sandbox
            match sandbox.launch() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error launching sandbox: {}", e);
                    std::process::exit(1);
                }
            }

            // If we get here, sandbox exited
            return Ok(());
        }
    }

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

    // Initialize model registry with CLI overrides
    let mut overrides: HashMap<String, ProviderOverride> = HashMap::new();

    // If --provider is specified, apply CLI overrides only to that provider
    let target_provider = args.provider.as_deref();
    let apply_override = |name: &str| -> bool {
        target_provider.map_or(true, |t| t == name)
    };

    if apply_override("openai") && args.api_key.is_some() {
        overrides.entry("openai".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("openai") && args.base_url.is_some() {
        overrides.entry("openai".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("anthropic") && args.api_key.is_some() {
        overrides.entry("anthropic".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("anthropic") && args.base_url.is_some() {
        overrides.entry("anthropic".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("google") && args.api_key.is_some() {
        overrides.entry("google".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("google") && args.base_url.is_some() {
        overrides.entry("google".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("moonshot") && args.api_key.is_some() {
        overrides.entry("moonshot".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("moonshot") && args.base_url.is_some() {
        overrides.entry("moonshot".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("mistral") && args.api_key.is_some() {
        overrides.entry("mistral".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("mistral") && args.base_url.is_some() {
        overrides.entry("mistral".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("groq") && args.api_key.is_some() {
        overrides.entry("groq".to_string()).or_default().api_key = args.api_key.clone();
    }
    if apply_override("groq") && args.base_url.is_some() {
        overrides.entry("groq".to_string()).or_default().base_url = args.base_url.clone();
    }
    if apply_override("ollama") && args.base_url.is_some() {
        overrides.entry("ollama".to_string()).or_default().base_url = args.base_url.clone();
    }

    let model_registry = Arc::new(ModelRegistry::new_with_overrides(overrides));

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
        let tools_arc: Vec<Arc<dyn tools::ToolTrait>> = if let Some(ref tools_str) = args.tools {
            let tool_names: Vec<&str> = tools_str.split(',').map(|s| s.trim()).collect();
            let mut selected: Vec<tools::Tool> = vec![];
            for name in tool_names {
                match name {
                    "read" => selected.push(tools::read_tool()),
                    "write" => selected.push(tools::write_tool()),
                    "edit" => selected.push(tools::edit_tool()),
                    "bash" => selected.push(tools::bash_tool()),
                    "grep" => selected.push(tools::grep_tool()),
                    "find" => selected.push(tools::find_tool()),
                    "ls" => selected.push(tools::ls_tool()),
                    "epkg" => selected.push(tools::epkg_tool()),
                    _ => eprintln!("Unknown tool: {}", name),
                }
            }
            selected
        } else {
            coding_tools()
        }
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
                "epkg" => selected.push(tools::epkg_tool()),
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

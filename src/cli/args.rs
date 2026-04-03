//! Command-line arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pi")]
#[command(version = "0.1.0")]
#[command(about = "Terminal AI coding agent", long_about = None)]
pub struct Args {
    /// Initial message to send
    #[arg(default_value = "")]
    pub message: String,

    /// Continue most recent session
    #[arg(short = 'c', long = "continue")]
    pub continue_session: bool,

    /// Resume/select a session
    #[arg(short = 'r', long = "resume")]
    pub resume: bool,

    /// Use specific session file
    #[arg(long = "session")]
    pub session: Option<String>,

    /// No session (ephemeral mode)
    #[arg(long = "no-session")]
    pub no_session: bool,

    /// Session directory
    #[arg(long = "session-dir")]
    pub session_dir: Option<String>,

    /// Provider name
    #[arg(long = "provider")]
    pub provider: Option<String>,

    /// Model name or pattern
    #[arg(long = "model")]
    pub model: Option<String>,

    /// Thinking level (off, minimal, low, medium, high, xhigh)
    #[arg(long = "thinking")]
    pub thinking: Option<String>,

    /// API key
    #[arg(long = "api-key")]
    pub api_key: Option<String>,

    /// Base URL for provider API (overrides default)
    #[arg(long = "base-url", value_name = "URL")]
    pub base_url: Option<String>,

    /// List available models
    #[arg(long = "list-models")]
    pub list_models: bool,

    /// Enable specific tools (comma-separated)
    #[arg(long = "tools")]
    pub tools: Option<String>,

    /// Disable all built-in tools
    #[arg(long = "no-tools")]
    pub no_tools: bool,

    /// Load extension from path
    #[arg(short = 'e', long = "extension")]
    pub extensions: Vec<String>,

    /// Disable extensions
    #[arg(long = "no-extensions")]
    pub no_extensions: bool,

    /// Load skill from path
    #[arg(long = "skill")]
    pub skills: Vec<String>,

    /// Disable skills
    #[arg(long = "no-skills")]
    pub no_skills: bool,

    /// Load theme
    #[arg(long = "theme")]
    pub theme: Option<String>,

    /// Disable themes
    #[arg(long = "no-themes")]
    pub no_themes: bool,

    /// Print mode (non-interactive)
    #[arg(short = 'p', long = "print")]
    pub print: bool,

    /// JSON mode
    #[arg(long = "mode")]
    pub mode: Option<String>,

    /// System prompt
    #[arg(long = "system-prompt")]
    pub system_prompt: Option<String>,

    /// Append to system prompt
    #[arg(long = "append-system-prompt")]
    pub append_system_prompt: Option<String>,

    /// Verbose output
    #[arg(long = "verbose")]
    pub verbose: bool,

    /// Enable sandbox mode with specified project path
    #[arg(long = "sandbox", value_name = "PROJECT_PATH")]
    pub sandbox: Option<String>,

    /// Additional mount paths in sandbox (requires --sandbox, short: -v)
    #[arg(
        short = 'v',
        long = "sandbox-mount",
        value_name = "PATH",
        requires = "sandbox"
    )]
    pub sandbox_mounts: Vec<String>,

    /// Environment variables in sandbox (requires --sandbox, format: KEY=VALUE, short: -E)
    #[arg(
        short = 'E',
        long = "sandbox-env",
        value_name = "KEY=VALUE",
        requires = "sandbox"
    )]
    pub sandbox_env: Vec<String>,

    /// Sandbox type (default: epkg)
    #[arg(long = "sandbox-type", value_name = "TYPE", requires = "sandbox")]
    pub sandbox_type: Option<String>,

    /// Disable sandbox (override config file)
    #[arg(long = "no-sandbox", requires = "sandbox")]
    pub no_sandbox: bool,

    /// Files to include (prefix with @)
    #[arg()]
    pub files: Vec<String>,
}

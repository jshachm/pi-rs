//! Utility functions

pub mod shell;

/// Get shell configuration
pub fn get_shell_config() -> ShellConfig {
    ShellConfig {
        shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
        home: dirs::home_dir().map(|p| p.to_string_lossy().to_string()),
    }
}

/// Shell configuration
#[derive(Debug)]
pub struct ShellConfig {
    pub shell: String,
    pub home: Option<String>,
}

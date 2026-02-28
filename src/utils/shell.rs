//! Shell utilities

/// Shell configuration
#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub shell: String,
    pub home: Option<String>,
}

/// Get shell configuration
pub fn get_shell_config() -> ShellConfig {
    ShellConfig {
        shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
        home: dirs::home_dir().map(|p| p.to_string_lossy().to_string()),
    }
}

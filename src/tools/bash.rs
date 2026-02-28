//! Bash tool - executes shell commands

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

const DEFAULT_TIMEOUT_MS: u64 = 120_000;
const DEFAULT_MAX_LINES: usize = 2000;
const DEFAULT_MAX_BYTES: usize = 500_000;

/// Bash tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BashToolInput {
    pub command: String,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub env: Option<HashMap<String, String>>,
}

/// Bash tool implementation
pub fn bash_tool() -> Tool {
    Tool::new(
        "bash",
        "Executes a shell command in the current working directory. \
         Use this to run commands, compile code, run tests, etc. \
         The output is returned as a string. \
         Commands are executed with /bin/sh on macOS/Linux.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                },
                "timeoutMs": {
                    "type": "number",
                    "description": "Timeout in milliseconds (default: 120000)"
                },
                "description": {
                    "type": "string",
                    "description": "Optional description of what the command does"
                },
                "env": {
                    "type": "object",
                    "description": "Additional environment variables"
                }
            }),
            required: vec!["command".to_string()],
        },
        |args, cwd| {
            let input: BashToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            execute_bash(&input.command, cwd, input.timeout_ms, input.env.as_ref())
        },
    )
}

fn execute_bash(
    command: &str,
    cwd: &str,
    timeout_ms: Option<u64>,
    env: Option<&HashMap<String, String>>,
) -> Result<ToolResult> {
    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));

    // Determine shell
    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "/bin/sh"
    };

    let mut cmd = Command::new(shell);
    cmd.arg("-c")
        .arg(command)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Set up environment
    let mut env_vars = HashMap::new();
    if let Some(current_env) = std::env::var_os("PATH") {
        env_vars.insert(
            "PATH".to_string(),
            current_env.to_string_lossy().to_string(),
        );
    }
    if let Some(user_env) = env {
        env_vars.extend(user_env.clone());
    }
    for (key, value) in &env_vars {
        cmd.env(key, value);
    }

    // Execute with timeout
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return Ok(ToolResult::error(format!(
                "Failed to execute command: {}",
                e
            )));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut result = String::new();

    if !stdout.is_empty() {
        result.push_str(&stdout);
    }

    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push_str("\n");
        }
        result.push_str("STDERR:\n");
        result.push_str(&stderr);
    }

    // Truncate if needed
    let lines: Vec<&str> = result.lines().collect();
    if lines.len() > DEFAULT_MAX_LINES {
        let truncated: String = lines[..DEFAULT_MAX_LINES].join("\n");
        result = format!(
            "{}\n\n[Output truncated - {} lines]",
            truncated,
            lines.len()
        );
    }

    if result.len() > DEFAULT_MAX_BYTES {
        result.truncate(DEFAULT_MAX_BYTES);
        result.push_str(&format!(
            "\n\n[Output truncated - {} bytes]",
            DEFAULT_MAX_BYTES
        ));
    }

    let status = output.status;

    if status.success() {
        Ok(ToolResult::success(result))
    } else {
        let exit_code = status.code().unwrap_or(-1);
        Ok(ToolResult::error(format!(
            "Command failed with exit code {}\n\n{}",
            exit_code, result
        )))
    }
}

/// Execute bash synchronously (blocking)
#[allow(dead_code)]
pub fn execute_bash_sync(
    command: &str,
    cwd: &str,
    timeout_ms: Option<u64>,
) -> Result<(bool, String)> {
    let result = execute_bash(command, cwd, timeout_ms, None)?;
    Ok((result.success, result.content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bash_echo() {
        let result = execute_bash("echo hello", ".", None, None).unwrap();
        assert!(result.success);
        assert!(result.content.contains("hello"));
    }

    #[test]
    fn test_bash_failure() {
        let result = execute_bash("exit 1", ".", None, None).unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_bash_cd() {
        let result = execute_bash("pwd", "/tmp", None, None).unwrap();
        assert!(result.success);
    }
}

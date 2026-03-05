//! Epkg tool - package manager integration

use std::process::{Command, Stdio};

use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

const DEFAULT_TIMEOUT_MS: u64 = 300_000;
const DEFAULT_MAX_LINES: usize = 2000;
const DEFAULT_MAX_BYTES: usize = 500_000;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpkgToolInput {
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default = "default_env")]
    pub env: Option<String>,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub assume_yes: bool,
    #[serde(default)]
    pub quiet: bool,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub download_only: bool,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
}

fn default_env() -> Option<String> {
    None
}

pub fn epkg_tool() -> Tool {
    Tool::new(
        "epkg",
        "A lightweight multi-source package manager for Linux. \
         Install, search, and manage packages from multiple distributions \
         (RPM, DEB, Alpine, Arch, Conda) without root. \
         Supports environment isolation and atomic upgrades with rollback.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "command": {
                    "type": "string",
                    "description": "The epkg subcommand to execute: install, remove, update, upgrade, search, info, list, env, run, history, restore, gc, repo, self, build, hash, unpack, convert, service, busybox",
                    "enum": ["install", "remove", "update", "upgrade", "search", "info", "list", "env", "run", "history", "restore", "gc", "repo", "self", "build", "hash", "unpack", "convert", "service", "busybox"]
                },
                "args": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Additional arguments for the epkg command (e.g., package names, options)"
                },
                "env": {
                    "type": "string",
                    "description": "Environment name to use (-e flag)"
                },
                "root": {
                    "type": "string",
                    "description": "Environment root directory (-r flag)"
                },
                "assumeYes": {
                    "type": "boolean",
                    "description": "Automatically answer yes to all prompts (-y flag)",
                    "default": false
                },
                "quiet": {
                    "type": "boolean",
                    "description": "Suppress output (-q flag)",
                    "default": false
                },
                "verbose": {
                    "type": "boolean",
                    "description": "Verbose operation, show debug messages (-v flag)",
                    "default": false
                },
                "dryRun": {
                    "type": "boolean",
                    "description": "Simulated run without changing the system (--dry-run flag)",
                    "default": false
                },
                "downloadOnly": {
                    "type": "boolean",
                    "description": "Download packages without installing (--download-only flag)",
                    "default": false
                },
                "timeoutMs": {
                    "type": "number",
                    "description": "Timeout in milliseconds (default: 300000)"
                },
                "description": {
                    "type": "string",
                    "description": "Optional description of what the command does"
                }
            }),
            required: vec!["command".to_string()],
        },
        |args, _cwd| {
            let input: EpkgToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            execute_epkg(&input)
        },
    )
}

fn execute_epkg(input: &EpkgToolInput) -> Result<ToolResult> {
    let epkg_path = std::env::var("EPKG_PATH").unwrap_or_else(|_| "epkg".to_string());

    let mut cmd_args = vec![input.command.clone()];

    if let Some(env) = &input.env {
        cmd_args.extend(["-e".to_string(), env.clone()]);
    }

    if let Some(root) = &input.root {
        cmd_args.extend(["-r".to_string(), root.clone()]);
    }

    if input.assume_yes {
        cmd_args.push("-y".to_string());
    }

    if input.quiet {
        cmd_args.push("-q".to_string());
    }

    if input.verbose {
        cmd_args.push("-v".to_string());
    }

    if input.dry_run {
        cmd_args.push("--dry-run".to_string());
    }

    if input.download_only {
        cmd_args.push("--download-only".to_string());
    }

    if let Some(args) = input.args.clone() {
        cmd_args.extend(args);
    }

    let mut cmd = Command::new(&epkg_path);
    cmd.args(&cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let timeout = std::time::Duration::from_millis(input.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));

    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return Ok(ToolResult::error(format!("Failed to execute epkg: {}", e)));
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
            result.push('\n');
        }
        result.push_str("STDERR:\n");
        result.push_str(&stderr);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epkg_tool_name() {
        let tool = epkg_tool();
        assert_eq!(tool.name, "epkg");
    }

    #[test]
    fn test_epkg_tool_description() {
        let tool = epkg_tool();
        assert!(!tool.description.is_empty());
    }

    #[test]
    fn test_epkg_tool_schema() {
        let tool = epkg_tool();
        assert_eq!(tool.schema.r#type, "object");
        assert!(tool.schema.required.contains(&"command".to_string()));
    }
}

//! Ls tool - lists directory contents

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

/// Ls tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LsToolInput {
    pub path: String,
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub long: bool,
}

/// Ls tool implementation
pub fn ls_tool() -> Tool {
    Tool::new(
        "ls",
        "Lists files and directories in a path. \
         Shows file names, sizes, and modification times in long format.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "path": {
                    "type": "string",
                    "description": "The path to list (default: current directory)"
                },
                "all": {
                    "type": "boolean",
                    "description": "Show hidden files (default: false)"
                },
                "long": {
                    "type": "boolean",
                    "description": "Show detailed information (default: false)"
                }
            }),
            required: vec!["path".to_string()],
        },
        |args, cwd| {
            let input: LsToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            list_directory(&input.path, cwd, input.all, input.long)
        },
    )
}

fn list_directory(path: &str, cwd: &str, show_all: bool, long: bool) -> Result<ToolResult> {
    let target = Path::new(cwd).join(path);

    if !target.exists() {
        return Err(PiError::Tool(format!("Path not found: {}", path)));
    }

    if target.is_file() {
        return Ok(ToolResult::success(format_file(&target, long)));
    }

    let entries: Vec<_> = fs::read_dir(&target)
        .map_err(|e| PiError::Tool(format!("Failed to read directory: {}", e)))?
        .filter_map(|e| e.ok())
        .collect();

    let mut items: Vec<String> = Vec::new();

    for entry in entries {
        let entry_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files unless show_all
        if !show_all && file_name.starts_with('.') {
            continue;
        }

        if long {
            items.push(format_dir_entry(&entry_path, &file_name));
        } else {
            if entry_path.is_dir() {
                items.push(format!("{}/", file_name));
            } else {
                items.push(file_name);
            }
        }
    }

    // Sort: directories first, then files, alphabetically
    items.sort_by(|a, b| {
        let a_is_dir = a.ends_with('/');
        let b_is_dir = b.ends_with('/');
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.to_lowercase().cmp(&b.to_lowercase()),
        }
    });

    Ok(ToolResult::success(items.join("\n")))
}

fn format_file(path: &Path, long: bool) -> String {
    if long {
        format_dir_entry(path, &path.file_name().unwrap().to_string_lossy())
    } else {
        path.file_name().unwrap().to_string_lossy().to_string()
    }
}

fn format_dir_entry(path: &Path, name: &str) -> String {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return name.to_string(),
    };

    let file_type = if path.is_dir() { "d" } else { "-" };

    let size = metadata.len();
    let modified = metadata
        .modified()
        .map(|t| {
            let datetime: chrono::DateTime<chrono::Local> = t.into();
            datetime.format("%Y-%m-%d %H:%M").to_string()
        })
        .unwrap_or_else(|_| "unknown".to_string());

    format!("{}{:>10} {} {}", file_type, size, modified, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_list_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "").unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();

        let result = list_directory(".", temp_dir.path().to_str().unwrap(), false, false).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_list_directory_long() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

        let result = list_directory(".", temp_dir.path().to_str().unwrap(), false, true).unwrap();
        assert!(result.success);
        assert!(result.content.contains("file.txt"));
    }
}

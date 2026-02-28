//! Find tool - finds files by name pattern

use std::path::Path;

use glob::Pattern;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

const DEFAULT_MAX_RESULTS: usize = 100;

/// Find tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindToolInput {
    pub path: String,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub max_results: Option<usize>,
    #[serde(default)]
    pub file_type: Option<String>,
}

/// Find tool implementation
pub fn find_tool() -> Tool {
    Tool::new(
        "find",
        "Finds files in a directory tree matching a pattern. \
         Use this to locate files by name or pattern.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "path": {
                    "type": "string",
                    "description": "The directory path to search in"
                },
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern to match file names"
                },
                "name": {
                    "type": "string",
                    "description": "Simple name pattern (same as pattern)"
                },
                "maxResults": {
                    "type": "number",
                    "description": "Maximum number of results (default: 100)"
                },
                "fileType": {
                    "type": "string",
                    "description": "Filter by type: 'f' for files, 'd' for directories"
                }
            }),
            required: vec!["path".to_string()],
        },
        |args, cwd| {
            let input: FindToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            find_files(
                &input.path,
                input.name.or(input.pattern).as_deref(),
                cwd,
                input.file_type.as_deref(),
                input.max_results,
            )
        },
    )
}

fn find_files(
    path: &str,
    pattern: Option<&str>,
    cwd: &str,
    file_type: Option<&str>,
    max_results: Option<usize>,
) -> Result<ToolResult> {
    let target = Path::new(cwd).join(path);
    let max = max_results.unwrap_or(DEFAULT_MAX_RESULTS);

    if !target.exists() {
        return Err(PiError::Tool(format!("Path not found: {}", path)));
    }

    let glob_pattern = pattern.map(|p| {
        if p.contains('*') || p.contains('?') {
            p.to_string()
        } else {
            format!("*{}*", p)
        }
    });

    let compiled_pattern = glob_pattern
        .as_ref()
        .map(|p| Pattern::new(p).map_err(|e| PiError::Tool(format!("Invalid pattern: {}", e))));

    let mut results: Vec<String> = Vec::new();

    for entry in WalkDir::new(&target)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if results.len() >= max {
            break;
        }

        let entry_path = entry.path();

        // Skip hidden directories
        if entry_path
            .components()
            .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        {
            continue;
        }

        // Filter by type
        if let Some(ft) = file_type {
            match ft {
                "f" if !entry_path.is_file() => continue,
                "d" if !entry_path.is_dir() => continue,
                _ => {}
            }
        }

        // Filter by pattern
        if let Some(ref pattern) = compiled_pattern {
            if let Ok(ref pat) = pattern {
                let file_name = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if !pat.matches(file_name) {
                    continue;
                }
            }
        }

        // Make path relative to cwd
        if let Ok(relative) = entry_path.strip_prefix(cwd) {
            let path_str = relative.to_string_lossy().to_string();
            if path_str.starts_with('/') || path_str.starts_with('\\') {
                results.push(path_str[1..].to_string());
            } else {
                results.push(path_str);
            }
        }
    }

    if results.is_empty() {
        return Ok(ToolResult::success("No files found".to_string()));
    }

    results.sort();
    let output = results.join("\n");
    Ok(ToolResult::success(output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "").unwrap();
        fs::write(temp_dir.path().join("other.rs"), "").unwrap();

        let result = find_files(
            ".",
            Some("*.txt"),
            temp_dir.path().to_str().unwrap(),
            Some("f"),
            Some(10),
        )
        .unwrap();

        assert!(result.success);
    }
}

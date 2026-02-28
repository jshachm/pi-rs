//! Grep tool - searches file contents with regex

use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use regex::Regex;
use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

const DEFAULT_MAX_LINES: usize = 100;

/// Grep tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrepToolInput {
    pub pattern: String,
    pub path: String,
    #[serde(default = "default_true")]
    pub case_sensitive: bool,
    #[serde(default)]
    pub include: Option<String>,
    #[serde(default)]
    pub max_results: Option<usize>,
}

fn default_true() -> bool {
    true
}

/// Grep tool implementation
pub fn grep_tool() -> Tool {
    Tool::new(
        "grep",
        "Searches for a pattern in files matching the given path. \
         Uses regex pattern matching. Returns matching lines with file:line:content format.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "pattern": {
                    "type": "string",
                    "description": "The regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "The path to search in (file or directory)"
                },
                "caseSensitive": {
                    "type": "boolean",
                    "description": "Whether the search is case sensitive (default: true)"
                },
                "include": {
                    "type": "string",
                    "description": "Glob pattern for files to include"
                },
                "maxResults": {
                    "type": "number",
                    "description": "Maximum number of results (default: 100)"
                }
            }),
            required: vec!["pattern".to_string(), "path".to_string()],
        },
        |args, cwd| {
            let input: GrepToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            grep(
                &input.pattern,
                &input.path,
                cwd,
                input.case_sensitive,
                input.max_results,
            )
        },
    )
}

fn grep(
    pattern: &str,
    path: &str,
    cwd: &str,
    case_sensitive: bool,
    max_results: Option<usize>,
) -> Result<ToolResult> {
    let max = max_results.unwrap_or(DEFAULT_MAX_LINES);
    let target = Path::new(cwd).join(path);

    // Build regex
    let pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){}", pattern)
    };

    let regex =
        Regex::new(&pattern).map_err(|e| PiError::Tool(format!("Invalid regex pattern: {}", e)))?;

    let mut results: Vec<String> = Vec::new();

    if target.is_file() {
        results = grep_file(&target, &regex)?;
    } else if target.is_dir() {
        results = grep_directory(&target, &regex, max)?;
    } else {
        return Err(PiError::Tool(format!("Path not found: {}", path)));
    }

    if results.is_empty() {
        return Ok(ToolResult::success("No matches found".to_string()));
    }

    let output = if results.len() > max {
        format!(
            "{}\n\n[{} more results not shown]",
            results[..max].join("\n"),
            results.len() - max
        )
    } else {
        results.join("\n")
    };

    Ok(ToolResult::success(output))
}

fn grep_file(path: &Path, regex: &Regex) -> Result<Vec<String>> {
    let file =
        fs::File::open(path).map_err(|e| PiError::Tool(format!("Failed to open file: {}", e)))?;

    let reader = BufReader::new(file);
    let mut results = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| PiError::Tool(format!("Read error: {}", e)))?;
        if regex.is_match(&line) {
            results.push(format!("{}:{}:{}", path.display(), line_num + 1, line));
        }
    }

    Ok(results)
}

fn grep_directory(dir: &Path, regex: &Regex, max: usize) -> Result<Vec<String>> {
    let mut results = Vec::new();

    fn walk_dir(dir: &Path, regex: &Regex, results: &mut Vec<String>, max: usize) -> Result<()> {
        if results.len() >= max {
            return Ok(());
        }

        let entries = fs::read_dir(dir)
            .map_err(|e| PiError::Tool(format!("Failed to read directory: {}", e)))?;

        for entry in entries.flatten() {
            if results.len() >= max {
                break;
            }

            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with('.'))
                    .unwrap_or(false)
                {
                    continue;
                }
                walk_dir(&path, regex, results, max)?;
            } else {
                // Skip binary files
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "exe" | "dll" | "so" | "dylib" | "bin") {
                        continue;
                    }
                }

                if let Ok(file_results) = grep_file(&path, regex) {
                    results.extend(file_results);
                }
            }
        }

        Ok(())
    }

    walk_dir(dir, regex, &mut results, max)?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_grep_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.txt");
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "hello world").unwrap();
        writeln!(file, "foo bar").unwrap();

        let result = grep(
            "world",
            "test.txt",
            temp_dir.path().to_str().unwrap(),
            true,
            Some(10),
        )
        .unwrap();
        assert!(result.success);
        assert!(result.content.contains("world"));
    }
}

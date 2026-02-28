//! Read tool - reads file contents

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

const DEFAULT_MAX_LINES: usize = 2000;
const DEFAULT_MAX_BYTES: usize = 500_000;

/// Read tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadToolInput {
    pub path: String,
    #[serde(default = "default_max_lines")]
    pub max_lines: Option<usize>,
    #[serde(default = "default_max_bytes")]
    pub max_bytes: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
}

fn default_max_lines() -> Option<usize> {
    Some(DEFAULT_MAX_LINES)
}

fn default_max_bytes() -> Option<usize> {
    Some(DEFAULT_MAX_BYTES)
}

/// Read tool implementation
pub fn read_tool() -> Tool {
    Tool::new(
        "read",
        "Reads a file from the file system. Use this tool to view file contents. \
         If the file is larger than max_lines or max_bytes, it will be truncated. \
         Returns the file content or an error if the file cannot be read.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "path": {
                    "type": "string",
                    "description": "The path to the file to read"
                },
                "maxLines": {
                    "type": "number",
                    "description": "Maximum number of lines to read (default: 2000)"
                },
                "maxBytes": {
                    "type": "number",
                    "description": "Maximum number of bytes to read (default: 500000)"
                },
                "offset": {
                    "type": "number",
                    "description": "Line offset to start reading from (0-indexed)"
                }
            }),
            required: vec!["path".to_string()],
        },
        |args, cwd| {
            let input: ReadToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            let path = Path::new(cwd).join(&input.path);
            let max_lines = input.max_lines.unwrap_or(DEFAULT_MAX_LINES);
            let max_bytes = input.max_bytes.unwrap_or(DEFAULT_MAX_BYTES);
            let offset = input.offset.unwrap_or(0);

            read_file(&path, max_lines, max_bytes, offset)
        },
    )
}

fn read_file(path: &Path, max_lines: usize, max_bytes: usize, offset: usize) -> Result<ToolResult> {
    let file = File::open(path)
        .map_err(|e| PiError::Tool(format!("Failed to read file '{}': {}", path.display(), e)))?;

    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut bytes_count = 0;

    for (i, line) in reader.lines().enumerate() {
        if i < offset {
            continue;
        }
        if lines.len() >= max_lines {
            return Ok(ToolResult::truncated(format!(
                "{}\n\n[Output truncated - {} lines, {} bytes]",
                lines.join("\n"),
                lines.len(),
                bytes_count
            )));
        }

        let line = line.map_err(|e| PiError::Tool(format!("Failed to read line: {}", e)))?;

        bytes_count += line.len() + 1;
        if bytes_count > max_bytes {
            return Ok(ToolResult::truncated(format!(
                "{}\n\n[Output truncated - {} lines, {} bytes]",
                lines.join("\n"),
                lines.len(),
                bytes_count
            )));
        }

        lines.push(line);
    }

    Ok(ToolResult::success(lines.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();

        let result = read_file(file.path(), 10, 1000, 0).unwrap();
        assert!(result.success);
        assert!(result.content.contains("line 1"));
    }

    #[test]
    fn test_read_file_with_offset() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();

        let result = read_file(file.path(), 10, 1000, 1).unwrap();
        assert!(result.success);
        assert!(result.content.contains("line 2"));
        assert!(!result.content.contains("line 1"));
    }
}

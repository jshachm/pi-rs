//! Write tool - writes content to files

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

/// Write tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteToolInput {
    pub path: String,
    pub content: String,
}

/// Write tool implementation
pub fn write_tool() -> Tool {
    Tool::new(
        "write",
        "Writes a file to the file system with the given content. \
         Creates the file if it doesn't exist, overwrites if it does. \
         Use this to create new files or completely replace existing file content.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "path": {
                    "type": "string",
                    "description": "The path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            }),
            required: vec!["path".to_string(), "content".to_string()],
        },
        |args, cwd| {
            let input: WriteToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            write_file(&input.path, &input.content, cwd)
        },
    )
}

fn write_file(path: &str, content: &str, cwd: &str) -> Result<ToolResult> {
    let file_path = Path::new(cwd).join(path);

    // Create parent directories if they don't exist
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                PiError::Tool(format!(
                    "Failed to create directory '{}': {}",
                    parent.display(),
                    e
                ))
            })?;
        }
    }

    // Write the file
    let mut file = File::create(&file_path).map_err(|e| {
        PiError::Tool(format!(
            "Failed to create file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    file.write_all(content.as_bytes()).map_err(|e| {
        PiError::Tool(format!(
            "Failed to write to file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    Ok(ToolResult::success(format!(
        "Successfully wrote {} bytes to '{}'",
        content.len(),
        path
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.txt");

        let result = write_file(path.to_str().unwrap(), "hello world", "").unwrap();
        assert!(result.success);

        let contents = fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "hello world");
    }

    #[test]
    fn test_write_file_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("subdir").join("test.txt");

        let result = write_file(path.to_str().unwrap(), "content", "").unwrap();
        assert!(result.success);
        assert!(path.exists());
    }
}

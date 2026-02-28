//! Edit tool - edits files using diff algorithm

use std::fs;
use std::path::Path;

use serde::Deserialize;
use similar::TextDiff;

use crate::core::errors::{PiError, Result};
use crate::tools::{Tool, ToolResult, ToolSchema};

/// Edit tool input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditToolInput {
    pub path: String,
    #[serde(default)]
    pub find: Option<String>,
    #[serde(default)]
    pub replace: Option<String>,
}

/// Edit tool implementation
pub fn edit_tool() -> Tool {
    Tool::new(
        "edit",
        "Edits a file using a find and replace approach. \
         The 'find' parameter specifies the text to search for, \
         and 'replace' specifies the replacement text. \
         Use this to modify specific parts of existing files.",
        ToolSchema {
            r#type: "object".to_string(),
            properties: serde_json::json!({
                "path": {
                    "type": "string",
                    "description": "The path to the file to edit"
                },
                "find": {
                    "type": "string",
                    "description": "The text to find in the file"
                },
                "replace": {
                    "type": "string",
                    "description": "The replacement text"
                }
            }),
            required: vec!["path".to_string(), "find".to_string()],
        },
        |args, cwd| {
            let input: EditToolInput = serde_json::from_value(args)
                .map_err(|e| PiError::Tool(format!("Invalid arguments: {}", e)))?;

            edit_file(
                &input.path,
                input.find.as_deref(),
                input.replace.as_deref(),
                cwd,
            )
        },
    )
}

fn edit_file(
    path: &str,
    find: Option<&str>,
    replace: Option<&str>,
    cwd: &str,
) -> Result<ToolResult> {
    let file_path = Path::new(cwd).join(path);

    // Read original content
    let original = fs::read_to_string(&file_path).map_err(|e| {
        PiError::Tool(format!(
            "Failed to read file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    let find_text = find.ok_or_else(|| PiError::Tool("Missing 'find' parameter".to_string()))?;

    let replace_text = replace.unwrap_or("");

    // Check if the find text exists
    if !original.contains(find_text) {
        return Err(PiError::Tool(format!(
            "Could not find '{}' in file '{}'",
            find_text, path
        )));
    }

    // Simple string replacement
    let edited = if replace_text.is_empty() {
        // Just delete the find text
        original.replace(find_text, "")
    } else {
        original.replace(find_text, replace_text)
    };

    // Write the edited content
    fs::write(&file_path, &edited).map_err(|e| {
        PiError::Tool(format!(
            "Failed to write file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    let lines_changed = edited.lines().count() as isize - original.lines().count() as isize;

    Ok(ToolResult::success(format!(
        "Successfully edited '{}' ({} lines changed)",
        path, lines_changed
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_edit_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.txt");
        fs::write(&path, "Hello World").unwrap();

        let result = edit_file(path.to_str().unwrap(), Some("World"), Some("Rust"), "").unwrap();

        assert!(result.success);

        let contents = fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "Hello Rust");
    }

    #[test]
    fn test_edit_file_delete() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.txt");
        fs::write(&path, "Hello World").unwrap();

        let result = edit_file(path.to_str().unwrap(), Some(" World"), None, "").unwrap();

        assert!(result.success);

        let contents = fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "Hello");
    }
}

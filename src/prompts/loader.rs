//! Prompt template loader

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub description: String,
    pub content: String,
    pub variables: Vec<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptManifest {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub variables: Option<Vec<String>>,
}

pub struct PromptLoader {
    prompts_dir: PathBuf,
    loaded_prompts: Vec<PromptTemplate>,
}

impl PromptLoader {
    pub fn new(prompts_dir: PathBuf) -> Self {
        Self {
            prompts_dir,
            loaded_prompts: Vec::new(),
        }
    }

    pub fn load_prompts(&mut self) -> Vec<PromptTemplate> {
        let mut prompts = Vec::new();

        if !self.prompts_dir.is_dir() {
            return prompts;
        }

        if let Ok(entries) = std::fs::read_dir(&self.prompts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(prompt) = self.load_prompt(&path) {
                        prompts.push(prompt);
                    }
                } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Some(prompt) = self.load_prompt_file(&path) {
                        prompts.push(prompt);
                    }
                }
            }
        }

        self.loaded_prompts = prompts.clone();
        prompts
    }

    fn load_prompt(&self, path: &Path) -> Option<PromptTemplate> {
        let manifest_path = path.join("prompt.json");

        if !manifest_path.exists() {
            return None;
        }

        let manifest_content = std::fs::read_to_string(&manifest_path).ok()?;
        let manifest: PromptManifest = serde_json::from_str(&manifest_content).ok()?;

        let content_path = path.join("content.md");
        let content = std::fs::read_to_string(&content_path).unwrap_or_default();

        let prompt = PromptTemplate {
            name: manifest.name,
            description: manifest.description.unwrap_or_default(),
            content,
            variables: manifest.variables.unwrap_or_default(),
            category: manifest.category,
        };

        Some(prompt)
    }

    fn load_prompt_file(&self, path: &Path) -> Option<PromptTemplate> {
        let name = path.file_stem()?.to_str()?.to_string();
        let content = std::fs::read_to_string(path).ok()?;

        let prompt = PromptTemplate {
            name: name.clone(),
            description: format!("Auto-loaded prompt: {}", name),
            content,
            variables: Vec::new(),
            category: Some("general".to_string()),
        };

        Some(prompt)
    }

    pub fn get_prompts(&self) -> &[PromptTemplate] {
        &self.loaded_prompts
    }

    pub fn get_prompt(&self, name: &str) -> Option<&PromptTemplate> {
        self.loaded_prompts.iter().find(|p| p.name == name)
    }

    pub fn get_prompts_by_category(&self, category: &str) -> Vec<&PromptTemplate> {
        self.loaded_prompts
            .iter()
            .filter(|p| p.category.as_deref() == Some(category))
            .collect()
    }

    pub fn interpolate(&self, name: &str, variables: &[(String, String)]) -> Option<String> {
        let prompt = self.get_prompt(name)?;
        let mut content = prompt.content.clone();

        for (key, value) in variables {
            content = content.replace(&format!("{{{}}}", key), value);
        }

        Some(content)
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self
            .loaded_prompts
            .iter()
            .filter_map(|p| p.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    pub fn reload(&mut self) -> Vec<PromptTemplate> {
        self.load_prompts()
    }
}

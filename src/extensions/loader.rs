//! Extension loader

use std::path::{Path, PathBuf};

use crate::extensions::types::ExtensionInfo;

#[derive(Debug, Clone)]
pub struct Extension {
    pub info: ExtensionInfo,
    pub path: PathBuf,
    pub enabled: bool,
}

pub struct ExtensionLoader {
    paths: Vec<PathBuf>,
    loaded_extensions: Vec<Extension>,
}

impl ExtensionLoader {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            loaded_extensions: Vec::new(),
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn load_extensions(&mut self) -> Vec<Extension> {
        let mut extensions = Vec::new();

        for path in &self.paths {
            if path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            if let Some(ext) = self.load_extension(&entry_path) {
                                extensions.push(ext);
                            }
                        }
                    }
                }
            }
        }

        self.loaded_extensions = extensions.clone();
        extensions
    }

    fn load_extension(&self, path: &Path) -> Option<Extension> {
        let manifest_path = path.join("extension.json");

        if !manifest_path.exists() {
            return None;
        }

        let content = std::fs::read_to_string(&manifest_path).ok()?;
        let info: ExtensionInfo = serde_json::from_str(&content).ok()?;

        Some(Extension {
            info,
            path: path.to_path_buf(),
            enabled: true,
        })
    }

    pub fn get_extensions(&self) -> &[Extension] {
        &self.loaded_extensions
    }

    pub fn get_extension(&self, name: &str) -> Option<&Extension> {
        self.loaded_extensions.iter().find(|e| e.info.name == name)
    }

    pub fn enable_extension(&mut self, name: &str) -> bool {
        if let Some(ext) = self
            .loaded_extensions
            .iter_mut()
            .find(|e| e.info.name == name)
        {
            ext.enabled = true;
            return true;
        }
        false
    }

    pub fn disable_extension(&mut self, name: &str) -> bool {
        if let Some(ext) = self
            .loaded_extensions
            .iter_mut()
            .find(|e| e.info.name == name)
        {
            ext.enabled = false;
            return true;
        }
        false
    }

    pub fn reload(&mut self) -> Vec<Extension> {
        self.load_extensions()
    }
}

impl Default for ExtensionLoader {
    fn default() -> Self {
        Self::new()
    }
}

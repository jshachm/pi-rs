//! Skill loader

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub trigger: String,
    pub content: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub triggers: Vec<String>,
    pub variables: Option<Vec<String>>,
}

pub struct SkillLoader {
    search_paths: Vec<PathBuf>,
    loaded_skills: Vec<Skill>,
}

impl SkillLoader {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self {
            search_paths: vec![skills_dir],
            loaded_skills: Vec::new(),
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    pub fn load_skills(&mut self) -> Vec<Skill> {
        let mut skills = Vec::new();

        for dir in &self.search_paths {
            // Check if the path itself is a skill directory (has skill.json)
            let manifest_path = dir.join("skill.json");
            if manifest_path.exists() {
                if let Some(skill) = self.load_skill(dir) {
                    skills.push(skill);
                }
                continue;
            }

            // Otherwise, look for subdirectories with skill.json
            if !dir.is_dir() {
                continue;
            }

            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(skill) = self.load_skill(&path) {
                            skills.push(skill);
                        }
                    }
                }
            }
        }

        self.loaded_skills = skills.clone();
        skills
    }

    fn load_skill(&self, path: &Path) -> Option<Skill> {
        let manifest_path = path.join("skill.json");

        if !manifest_path.exists() {
            return None;
        }

        let manifest_content = std::fs::read_to_string(&manifest_path).ok()?;
        let manifest: SkillManifest = serde_json::from_str(&manifest_content).ok()?;

        let content_path = path.join("content.md");
        let content = std::fs::read_to_string(&content_path).unwrap_or_default();

        let skill = Skill {
            name: manifest.name,
            description: manifest.description.unwrap_or_default(),
            trigger: manifest.triggers.first()?.clone(),
            content,
            variables: manifest.variables.unwrap_or_default(),
        };

        Some(skill)
    }

    pub fn get_skills(&self) -> &[Skill] {
        &self.loaded_skills
    }

    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.loaded_skills.iter().find(|s| s.name == name)
    }

    pub fn get_skill_by_trigger(&self, trigger: &str) -> Option<&Skill> {
        self.loaded_skills.iter().find(|s| s.trigger == trigger)
    }

    pub fn interpolate(&self, name: &str, variables: &[(String, String)]) -> Option<String> {
        let skill = self.get_skill(name)?;
        let mut content = skill.content.clone();

        for (key, value) in variables {
            content = content.replace(&format!("{{{}}}", key), value);
        }

        Some(content)
    }

    pub fn reload(&mut self) -> Vec<Skill> {
        self.load_skills()
    }
}

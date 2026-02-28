//! Completion engine for commands, skills, and prompts

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Completion {
    pub text: String,
    pub display: String,
    pub kind: CompletionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionKind {
    Command,
    Skill,
    Prompt,
    File,
    Symbol,
}

pub struct CompletionEngine {
    commands: HashMap<String, String>,
    skills: Vec<SkillDefinition>,
    prompts: Vec<PromptTemplate>,
}

#[derive(Debug, Clone)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub trigger: String,
}

#[derive(Debug, Clone)]
pub struct PromptTemplate {
    pub name: String,
    pub content: String,
    pub description: String,
}

impl CompletionEngine {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        commands.insert("/help".to_string(), "Show available commands".to_string());
        commands.insert("/clear".to_string(), "Clear chat messages".to_string());
        commands.insert("/quit".to_string(), "Exit the application".to_string());
        commands.insert("/new".to_string(), "Start a new session".to_string());
        commands.insert("/tree".to_string(), "Show session tree".to_string());
        commands.insert("/compact".to_string(), "Compact context".to_string());
        commands.insert("/model".to_string(), "Show current model".to_string());
        commands.insert("/provider".to_string(), "Show current provider".to_string());
        commands.insert("/think".to_string(), "Set thinking level".to_string());

        Self {
            commands,
            skills: Vec::new(),
            prompts: Vec::new(),
        }
    }

    pub fn register_skill(&mut self, skill: SkillDefinition) {
        self.skills.push(skill);
    }

    pub fn register_prompt(&mut self, prompt: PromptTemplate) {
        self.prompts.push(prompt);
    }

    pub fn get_completions(&self, input: &str) -> Vec<Completion> {
        let mut results = Vec::new();

        if input.starts_with('/') {
            for (cmd, desc) in &self.commands {
                if cmd.starts_with(input) {
                    results.push(Completion {
                        text: cmd.clone(),
                        display: format!("{} - {}", cmd, desc),
                        kind: CompletionKind::Command,
                    });
                }
            }
        } else if let Some(skill_query) = input.strip_prefix('@') {
            for skill in &self.skills {
                if skill.name.contains(skill_query) || skill.trigger.contains(skill_query) {
                    results.push(Completion {
                        text: format!("@{}", skill.name),
                        display: format!("@{} - {}", skill.name, skill.description),
                        kind: CompletionKind::Skill,
                    });
                }
            }
        } else {
            let input_lower = input.to_lowercase();

            for (cmd, desc) in &self.commands {
                let cmd_name = cmd.trim_start_matches('/');
                if cmd_name.starts_with(&input_lower) {
                    results.push(Completion {
                        text: format!("/{}", cmd_name),
                        display: format!("/{} - {}", cmd_name, desc),
                        kind: CompletionKind::Command,
                    });
                }
            }

            for prompt in &self.prompts {
                if prompt.name.to_lowercase().contains(&input_lower)
                    || prompt.description.to_lowercase().contains(&input_lower)
                {
                    results.push(Completion {
                        text: prompt.name.clone(),
                        display: format!("{} - {}", prompt.name, prompt.description),
                        kind: CompletionKind::Prompt,
                    });
                }
            }
        }

        results.truncate(10);
        results
    }

    pub fn get_command_suggestions(&self, partial: &str) -> Vec<(&str, &str)> {
        self.commands
            .iter()
            .filter(|(cmd, _)| cmd.starts_with(partial))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

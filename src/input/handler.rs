//! Input handler for keyboard events

use crate::input::completion::{Completion, CompletionEngine};

pub struct InputHandler {
    pub content: String,
    cursor_position: usize,
    history: Vec<String>,
    history_index: Option<usize>,
    completion_engine: CompletionEngine,
    completions: Vec<Completion>,
    selected_completion: usize,
    pub is_completing: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            completion_engine: CompletionEngine::new(),
            completions: Vec::new(),
            selected_completion: 0,
            is_completing: false,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.is_completing {
            self.cancel_completion();
        }

        let mut chars: Vec<char> = self.content.chars().collect();
        if self.cursor_position <= chars.len() {
            chars.insert(self.cursor_position, c);
            self.content = chars.into_iter().collect();
            self.cursor_position += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.is_completing {
            self.cancel_completion();
        }

        if self.cursor_position > 0 {
            let mut chars: Vec<char> = self.content.chars().collect();
            chars.remove(self.cursor_position - 1);
            self.content = chars.into_iter().collect();
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.content.len();
    }

    pub fn submit(&mut self) -> String {
        if self.is_completing {
            self.apply_completion();
        }

        let content = self.content.trim().to_string();

        if !content.is_empty() {
            self.history.push(content.clone());
            self.history_index = None;
        }

        self.content.clear();
        self.cursor_position = 0;
        self.cancel_completion();

        content
    }

    pub fn history_up(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }

        let index = match self.history_index {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => self.history.len() - 1,
        };

        self.history_index = Some(index);
        self.content = self.history[index].clone();
        self.cursor_position = self.content.len();
        self.cancel_completion();

        Some(self.content.clone())
    }

    pub fn history_down(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }

        let index = match self.history_index {
            Some(i) => {
                if i < self.history.len() - 1 {
                    i + 1
                } else {
                    return None;
                }
            }
            None => return None,
        };

        self.history_index = Some(index);
        self.content = self.history[index].clone();
        self.cursor_position = self.content.len();
        self.cancel_completion();

        Some(self.content.clone())
    }

    pub fn trigger_completion(&mut self) -> bool {
        if self.content.is_empty() {
            return false;
        }

        self.completions = self.completion_engine.get_completions(&self.content);

        if self.completions.is_empty() {
            return false;
        }

        self.selected_completion = 0;
        self.is_completing = true;
        true
    }

    pub fn complete_next(&mut self) -> bool {
        if !self.is_completing || self.completions.is_empty() {
            return false;
        }

        self.selected_completion = (self.selected_completion + 1) % self.completions.len();
        true
    }

    pub fn complete_prev(&mut self) -> bool {
        if !self.is_completing || self.completions.is_empty() {
            return false;
        }

        if self.selected_completion == 0 {
            self.selected_completion = self.completions.len() - 1;
        } else {
            self.selected_completion -= 1;
        }
        true
    }

    pub fn apply_completion(&mut self) {
        if self.selected_completion < self.completions.len() {
            let completion = &self.completions[self.selected_completion];
            self.content = completion.text.clone();
            self.cursor_position = self.content.len();
            self.cancel_completion();
        }
    }

    pub fn cancel_completion(&mut self) {
        self.is_completing = false;
        self.completions.clear();
        self.selected_completion = 0;
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn get_current_completions(&self) -> &[Completion] {
        &self.completions
    }

    pub fn get_selected_completion_index(&self) -> usize {
        self.selected_completion
    }

    pub fn register_skill(&mut self, name: &str, description: &str, trigger: &str) {
        self.completion_engine
            .register_skill(crate::input::completion::SkillDefinition {
                name: name.to_string(),
                description: description.to_string(),
                trigger: trigger.to_string(),
            });
    }

    pub fn register_prompt(&mut self, name: &str, content: &str, description: &str) {
        self.completion_engine
            .register_prompt(crate::input::completion::PromptTemplate {
                name: name.to_string(),
                content: content.to_string(),
                description: description.to_string(),
            });
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

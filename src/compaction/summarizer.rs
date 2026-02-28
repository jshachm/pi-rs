//! Context summarizer for compaction

use crate::core::Message;

#[derive(Debug, Clone)]
pub struct CompactionResult {
    pub summary: String,
    pub compacted_messages: Vec<Message>,
    pub removed_count: usize,
}

pub struct ContextCompactor {
    max_messages: usize,
    summary_threshold: usize,
}

impl ContextCompactor {
    pub fn new(max_messages: usize, summary_threshold: usize) -> Self {
        Self {
            max_messages,
            summary_threshold,
        }
    }

    pub fn should_compact(&self, messages: &[Message]) -> bool {
        messages.len() >= self.summary_threshold
    }

    pub fn compact(
        &self,
        messages: Vec<Message>,
        _provider: Option<&str>,
        _model: Option<&str>,
    ) -> CompactionResult {
        if messages.len() <= self.max_messages {
            return CompactionResult {
                summary: String::new(),
                compacted_messages: messages,
                removed_count: 0,
            };
        }

        let keep_count = self.max_messages / 2;
        let to_compact = messages.len() - keep_count;

        let summary_messages = &messages[..to_compact];
        let summary = self.summarize(summary_messages);

        let kept_messages: Vec<Message> = messages[to_compact..].to_vec();

        let summary_message = Message::system(crate::core::MessageContent::Text(format!(
            "[Previous conversation summarized: {}]",
            summary
        )));

        let mut compacted = vec![summary_message];
        compacted.extend(kept_messages);

        CompactionResult {
            summary,
            compacted_messages: compacted,
            removed_count: to_compact,
        }
    }

    pub fn summarize(&self, messages: &[Message]) -> String {
        let mut topics = Vec::new();
        let mut tool_usage = Vec::new();

        for msg in messages {
            match msg.role {
                crate::core::Role::User => {
                    let content = msg.content.as_text();
                    if content.len() > 50 {
                        topics.push(content.chars().take(50).collect::<String>() + "...");
                    } else {
                        topics.push(content.to_string());
                    }
                }
                crate::core::Role::Assistant => {
                    if msg.tool_calls.is_some() {
                        tool_usage.push("used tools".to_string());
                    }
                }
                _ => {}
            }
        }

        let mut summary = String::new();

        if !topics.is_empty() {
            summary.push_str("User requests: ");
            summary.push_str(&topics.join(", "));
        }

        if !tool_usage.is_empty() {
            if !summary.is_empty() {
                summary.push_str(". ");
            }
            summary.push_str("Tools used: ");
            summary.push_str(&tool_usage.join(", "));
        }

        if summary.is_empty() {
            summary = "Previous conversation completed".to_string();
        }

        summary
    }

    pub fn estimate_tokens(&self, text: &str) -> usize {
        text.chars().count() / 4
    }

    pub fn set_max_messages(&mut self, max: usize) {
        self.max_messages = max;
    }

    pub fn set_summary_threshold(&mut self, threshold: usize) {
        self.summary_threshold = threshold;
    }
}

impl Default for ContextCompactor {
    fn default() -> Self {
        Self::new(50, 40)
    }
}

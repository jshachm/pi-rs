//! TUI components

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

/// Message item for the list
pub struct MessageItem {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

impl MessageItem {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        }
    }
}

/// Message list widget
pub struct MessageList {
    messages: Vec<MessageItem>,
    scroll: usize,
}

impl MessageList {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll: 0,
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(MessageItem::new(role, content));
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll < self.messages.len().saturating_sub(1) {
            self.scroll += 1;
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll = 0;
    }
}

impl Default for MessageList {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &MessageList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Messages");

        let inner = block.inner(area);
        block.render(area, buf);

        if self.messages.is_empty() {
            let text = Text::from("No messages yet. Start a conversation!");
            let paragraph = Paragraph::new(text)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(ratatui::layout::Alignment::Center);
            paragraph.render(inner, buf);
            return;
        }

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .skip(self.scroll)
            .map(|msg| {
                let style = if msg.role == "user" {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::Green)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("[{}] ", msg.timestamp),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(format!("{}: ", msg.role), style),
                    Span::styled(&msg.content, Style::default()),
                ]))
            })
            .collect();

        let list = List::new(messages).block(Block::default().borders(Borders::NONE));

        list.render(inner, buf);
    }
}

/// Input widget
pub struct InputWidget {
    pub text: String,
    pub cursor_position: usize,
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_position <= self.text.len() {
            self.text.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 && self.cursor_position <= self.text.len() {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
    }

    pub fn submit(&mut self) -> String {
        let content = self.text.clone();
        self.clear();
        content
    }
}

impl Default for InputWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Input");
        let inner = block.inner(area);
        block.render(area, buf);

        let text = if self.text.is_empty() {
            "Type your message..."
        } else {
            &self.text
        };

        let style = if self.text.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        let paragraph = Paragraph::new(text)
            .style(style)
            .scroll((0, self.cursor_position as u16));

        paragraph.render(inner, buf);
    }
}

/// Status bar widget
pub struct StatusBar {
    pub provider: String,
    pub model: String,
    pub thinking: bool,
    pub session_id: String,
}

impl StatusBar {
    pub fn new(provider: &str, model: &str, session_id: &str) -> Self {
        Self {
            provider: provider.to_string(),
            model: model.to_string(),
            thinking: false,
            session_id: session_id.to_string(),
        }
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(area);

        let provider = Paragraph::new(format!("Provider: {}", self.provider))
            .style(Style::default().fg(Color::Blue));
        provider.render(chunks[0], buf);

        let model = Paragraph::new(format!("Model: {}", self.model))
            .style(Style::default().fg(Color::Magenta));
        model.render(chunks[1], buf);

        let status = if self.thinking {
            "Thinking..."
        } else {
            "Ready"
        };
        let status_style = if self.thinking {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        };
        let status_para = Paragraph::new(status).style(status_style);
        status_para.render(chunks[2], buf);

        let session = Paragraph::new(format!("Session: {}", &self.session_id[..8]))
            .style(Style::default().fg(Color::DarkGray));
        session.render(chunks[3], buf);
    }
}

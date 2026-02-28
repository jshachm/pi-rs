//! TUI Application - Full ratatui implementation

use std::io;
use std::sync::Arc;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::agent::AgentSession;
use crate::providers::ModelRegistry;
use crate::session::SessionManager;
use crate::tools::Tool;

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<io::Stderr>>,
    session: SessionManager,
    model_registry: Arc<ModelRegistry>,
    tools: Vec<Tool>,
    agent: Option<AgentSession>,
    messages: Vec<(String, String)>,
    input: String,
    scroll_offset: usize,
    should_quit: bool,
}

impl TuiApp {
    pub fn new(
        session: SessionManager,
        model_registry: Arc<ModelRegistry>,
        tools: Vec<Tool>,
    ) -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stderr();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            session,
            model_registry,
            tools,
            agent: None,
            messages: Vec::new(),
            input: String::new(),
            scroll_offset: 0,
            should_quit: false,
        })
    }

    pub fn set_agent(&mut self, agent: AgentSession) {
        self.agent = Some(agent);
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.draw(|f| {
            render_app(
                f,
                &self.messages,
                &self.input,
                self.session.get_session_id(),
                &self.model_registry.list_providers(),
            );
        })?;

        loop {
            if self.should_quit {
                break;
            }

            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            self.should_quit = true;
                        }
                        KeyCode::Char('q')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            self.should_quit = true;
                        }
                        KeyCode::Enter => {
                            let input = self.input.trim().to_string();
                            if !input.is_empty() {
                                self.messages.push(("user".to_string(), input.clone()));
                                self.input.clear();
                                self.handle_command(&input);
                            }
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Up => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            if self.scroll_offset + 1 < self.messages.len() {
                                self.scroll_offset += 1;
                            }
                        }
                        KeyCode::PageUp => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(10);
                        }
                        KeyCode::PageDown => {
                            self.scroll_offset = (self.scroll_offset + 10)
                                .min(self.messages.len().saturating_sub(1));
                        }
                        KeyCode::Home => {
                            self.scroll_offset = 0;
                        }
                        KeyCode::End => {
                            self.scroll_offset = self.messages.len().saturating_sub(1);
                        }
                        _ => {}
                    }
                }
            }

            if self.should_quit {
                break;
            }

            self.terminal.draw(|f| {
                render_app(
                    f,
                    &self.messages,
                    &self.input,
                    self.session.get_session_id(),
                    &self.model_registry.list_providers(),
                );
            })?;
        }

        self.cleanup()?;
        Ok(())
    }

    fn handle_command(&mut self, command: &str) {
        if !command.starts_with('/') {
            self.messages
                .push(("assistant".to_string(), "Thinking...".to_string()));
            self.scroll_offset = self.messages.len().saturating_sub(1);
            return;
        }

        match command {
            "/help" => {
                let help = vec![
                    ("system".to_string(), "Commands:".to_string()),
                    (
                        "system".to_string(),
                        "  /help  - Show this help".to_string(),
                    ),
                    (
                        "system".to_string(),
                        "  /clear - Clear messages".to_string(),
                    ),
                    ("system".to_string(), "  /quit  - Exit (Ctrl+C)".to_string()),
                ];
                self.messages.extend(help);
            }
            "/clear" => {
                self.messages.clear();
                self.scroll_offset = 0;
            }
            "/quit" | "/q" => {
                self.should_quit = true;
            }
            _ => {
                self.messages
                    .push(("error".to_string(), format!("Unknown command: {}", command)));
            }
        }
    }

    fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn render_app(
    f: &mut Frame,
    messages: &[(String, String)],
    input: &str,
    session_id: &str,
    providers: &[String],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    let session_short = &session_id[..8.min(session_id.len())];
    let provider = providers
        .first()
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let header_text = vec![Line::from(vec![
        Span::raw(" Pi - Terminal AI Coding Agent "),
        Span::styled("│", Style::default().fg(Color::DarkGray)),
        Span::raw(" Session: "),
        Span::styled(session_short, Style::default().fg(Color::Cyan)),
        Span::raw(" │ Provider: "),
        Span::styled(provider, Style::default().fg(Color::Green)),
    ])];

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title(" Pi "))
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));
    f.render_widget(header, chunks[0]);

    let visible_messages: Vec<_> = messages.iter().rev().take(20).rev().collect();

    let items: Vec<ListItem> = visible_messages
        .iter()
        .map(|(role, content)| {
            let style = match role.as_str() {
                "user" => Style::default().fg(Color::Cyan),
                "assistant" => Style::default().fg(Color::Green),
                "system" => Style::default().fg(Color::Yellow),
                "error" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::White),
            };

            let prefix = match role.as_str() {
                "user" => "[User] ",
                "assistant" => "[Assistant] ",
                "system" => "[System] ",
                "error" => "[Error] ",
                _ => "",
            };

            let lines: Vec<Line> = content
                .lines()
                .enumerate()
                .map(|(j, line)| {
                    if j == 0 {
                        Line::from(vec![Span::styled(format!("{}{}", prefix, line), style)])
                    } else {
                        Line::from(vec![Span::styled(format!("  {}", line), style)])
                    }
                })
                .collect();

            ListItem::new(lines)
        })
        .collect();

    let messages_widget = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Messages "))
        .style(Style::default().bg(Color::Rgb(20, 20, 30)));
    f.render_widget(messages_widget, chunks[1]);

    let input_text = vec![Line::from(vec![Span::raw("> "), Span::raw(input)])];

    let input_widget = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL).title(" Input "))
        .style(Style::default().bg(Color::Rgb(25, 25, 35)));
    f.render_widget(input_widget, chunks[2]);

    let input_width = (chunks[2].width as usize).saturating_sub(2);
    if input.len() < input_width {
        let cursor_x = (input.len() as u16) + 1;
        f.set_cursor(cursor_x, chunks[2].y + 1);
    }
}

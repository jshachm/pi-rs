//! Session manager - manages conversation sessions stored as JSONL with tree structure

use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::core::config::get_sessions_dir;
use crate::core::{Message, ThinkingLevel};

use super::entry::*;

/// Generate a unique short ID (8 hex chars)
fn generate_id(existing: &HashSet<String>) -> String {
    for _ in 0..100 {
        let id = Uuid::new_v4().to_string()[..8].to_string();
        if !existing.contains(&id) {
            return id;
        }
    }
    // Fallback to full UUID
    Uuid::new_v4().to_string()
}

/// Compute the default session directory for a cwd
fn get_default_session_dir(cwd: &str) -> PathBuf {
    let safe_path = format!("--{}--", cwd.replace('/', "-").replace('\\', "-"));
    get_sessions_dir().join(&safe_path)
}

/// Session manager for managing conversation sessions
pub struct SessionManager {
    session_id: String,
    session_file: Option<PathBuf>,
    session_dir: PathBuf,
    cwd: String,
    persist: bool,
    flushed: bool,
    file_entries: Vec<FileEntry>,
    by_id: HashMap<String, SessionEntry>,
    labels_by_id: HashMap<String, String>,
    leaf_id: Option<String>,
}

impl SessionManager {
    /// Create a new session manager
    fn new(cwd: &str, session_dir: PathBuf, session_file: Option<PathBuf>, persist: bool) -> Self {
        let mut manager = Self {
            session_id: String::new(),
            session_file: None,
            session_dir,
            cwd: cwd.to_string(),
            persist,
            flushed: false,
            file_entries: Vec::new(),
            by_id: HashMap::new(),
            labels_by_id: HashMap::new(),
            leaf_id: None,
        };

        if let Some(path) = session_file {
            manager.set_session_file(&path);
        } else {
            manager.new_session(None);
        }

        manager
    }

    /// Set the session file
    pub fn set_session_file(&mut self, path: &Path) {
        self.session_file = Some(path.to_path_buf());

        if path.exists() {
            self.file_entries = Self::load_entries_from_file(path);

            if self.file_entries.is_empty() {
                let explicit_path = self.session_file.clone();
                self.new_session(None);
                self.session_file = explicit_path;
                self.rewrite_file();
                self.flushed = true;
                return;
            }

            if let Some(FileEntry::Header(header)) = self.file_entries.first() {
                self.session_id = header.id.clone();
            } else {
                self.session_id = Uuid::new_v4().to_string();
            }

            self.build_index();
            self.flushed = true;
        } else {
            let explicit_path = self.session_file.clone();
            self.new_session(None);
            self.session_file = explicit_path;
        }
    }

    /// Create a new session
    pub fn new_session(&mut self, parent_session: Option<&str>) {
        self.session_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();
        let header = SessionHeader::new(self.session_id.clone(), &self.cwd, parent_session);

        self.file_entries = vec![FileEntry::Header(header)];
        self.by_id.clear();
        self.labels_by_id.clear();
        self.leaf_id = None;
        self.flushed = false;

        if self.persist {
            let file_timestamp = timestamp.replace(':', "-").replace('.', "-");
            self.session_file = Some(self.session_dir.join(format!(
                "{}_{}.jsonl",
                file_timestamp,
                &self.session_id[..8]
            )));
        }
    }

    /// Build the ID index
    fn build_index(&mut self) {
        self.by_id.clear();
        self.labels_by_id.clear();
        self.leaf_id = None;

        for entry in &self.file_entries {
            if let FileEntry::Entry(e) = entry {
                self.by_id.insert(e.id().to_string(), e.clone());
                self.leaf_id = Some(e.id().to_string());

                if let SessionEntry::Label(label) = e {
                    if let Some(l) = &label.label {
                        self.labels_by_id.insert(label.target_id.clone(), l.clone());
                    } else {
                        self.labels_by_id.remove(&label.target_id);
                    }
                }
            }
        }
    }

    /// Rewrite the entire session file
    fn rewrite_file(&self) {
        if !self.persist {
            return;
        }
        if let Some(ref path) = self.session_file {
            if let Ok(mut file) = File::create(path) {
                for entry in &self.file_entries {
                    let json = serde_json::to_string(entry).unwrap_or_default();
                    let _ = writeln!(file, "{}", json);
                }
            }
        }
    }

    /// Load entries from a file
    fn load_entries_from_file(path: &Path) -> Vec<FileEntry> {
        if !path.exists() {
            return Vec::new();
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(entry) = serde_json::from_str::<FileEntry>(&line) {
                    entries.push(entry);
                }
            }
        }

        entries
    }

    /// Check if session is persisted
    pub fn is_persisted(&self) -> bool {
        self.persist
    }

    /// Get current working directory
    pub fn get_cwd(&self) -> &str {
        &self.cwd
    }

    /// Get session directory
    pub fn get_session_dir(&self) -> &Path {
        &self.session_dir
    }

    /// Get session ID
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    /// Get session file path
    pub fn get_session_file(&self) -> Option<&Path> {
        self.session_file.as_deref()
    }

    /// Get leaf entry ID
    pub fn get_leaf_id(&self) -> Option<&str> {
        self.leaf_id.as_deref()
    }

    /// Get leaf entry
    pub fn get_leaf_entry(&self) -> Option<&SessionEntry> {
        self.leaf_id.as_ref().and_then(|id| self.by_id.get(id))
    }

    /// Get entry by ID
    pub fn get_entry(&self, id: &str) -> Option<&SessionEntry> {
        self.by_id.get(id)
    }

    /// Get label for an entry
    pub fn get_label(&self, id: &str) -> Option<&str> {
        self.labels_by_id.get(id).map(|s| s.as_str())
    }

    /// Get children of an entry
    pub fn get_children(&self, parent_id: &str) -> Vec<&SessionEntry> {
        self.by_id
            .values()
            .filter(|e| e.parent_id() == Some(parent_id))
            .collect()
    }

    /// Get all entries
    pub fn get_entries(&self) -> Vec<&SessionEntry> {
        self.file_entries
            .iter()
            .filter_map(|e| {
                if let FileEntry::Entry(entry) = e {
                    Some(entry)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get header
    pub fn get_header(&self) -> Option<&SessionHeader> {
        self.file_entries.first().and_then(|e| {
            if let FileEntry::Header(h) = e {
                Some(h)
            } else {
                None
            }
        })
    }

    /// Append a message entry
    pub fn append_message(&mut self, message: Message) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = SessionMessageEntry::new(id.clone(), self.leaf_id.clone(), message);

        self.append_entry(SessionEntry::Message(entry));
        id
    }

    /// Append a thinking level change
    pub fn append_thinking_level_change(&mut self, level: ThinkingLevel) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = ThinkingLevelChangeEntry::new(id.clone(), self.leaf_id.clone(), level);

        self.append_entry(SessionEntry::ThinkingLevelChange(entry));
        id
    }

    /// Append a model change
    pub fn append_model_change(&mut self, provider: &str, model_id: &str) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = ModelChangeEntry::new(id.clone(), self.leaf_id.clone(), provider, model_id);

        self.append_entry(SessionEntry::ModelChange(entry));
        id
    }

    /// Append a compaction entry
    pub fn append_compaction(
        &mut self,
        summary: &str,
        first_kept_entry_id: &str,
        tokens_before: u64,
    ) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = CompactionEntry::new(
            id.clone(),
            self.leaf_id.clone(),
            summary,
            first_kept_entry_id,
            tokens_before,
        );

        self.append_entry(SessionEntry::Compaction(entry));
        id
    }

    /// Append a custom entry
    pub fn append_custom_entry(&mut self, custom_type: &str) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = CustomEntry::new(id.clone(), self.leaf_id.clone(), custom_type);

        self.append_entry(SessionEntry::Custom(entry));
        id
    }

    /// Append a session info entry
    pub fn append_session_info(&mut self, name: &str) -> String {
        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = SessionInfoEntry::new(id.clone(), self.leaf_id.clone(), name);

        self.append_entry(SessionEntry::SessionInfo(entry));
        id
    }

    /// Get session name
    pub fn get_session_name(&self) -> Option<String> {
        for entry in self.get_entries().iter().rev() {
            if let SessionEntry::SessionInfo(info) = entry {
                if let Some(name) = &info.name {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    /// Append a label
    pub fn append_label(&mut self, target_id: &str, label: Option<&str>) -> String {
        if !self.by_id.contains_key(target_id) {
            panic!("Entry {} not found", target_id);
        }

        let id = generate_id(&self.by_id.keys().cloned().collect());
        let entry = LabelEntry::new(id.clone(), self.leaf_id.clone(), target_id, label);

        if let Some(l) = label {
            self.labels_by_id
                .insert(target_id.to_string(), l.to_string());
        } else {
            self.labels_by_id.remove(target_id);
        }

        self.append_entry(SessionEntry::Label(entry));
        id
    }

    /// Internal: append an entry
    fn append_entry(&mut self, entry: SessionEntry) {
        let id = entry.id().to_string();
        let entry_clone = entry.clone();
        self.file_entries.push(FileEntry::Entry(entry_clone));
        self.by_id.insert(id.clone(), entry.clone());
        self.leaf_id = Some(id);
        self.persist_entry(&entry);
    }

    /// Persist a single entry
    fn persist_entry(&self, entry: &SessionEntry) {
        if !self.persist {
            return;
        }

        let has_assistant = self.file_entries.iter().any(|e| {
            if let FileEntry::Entry(SessionEntry::Message(m)) = e {
                m.message.role == crate::core::Role::Assistant
            } else {
                false
            }
        });

        if !has_assistant {
            return;
        }

        if let Some(ref path) = self.session_file {
            if let Ok(mut file) = OpenOptions::new().append(true).open(path) {
                if let Ok(json) = serde_json::to_string(entry) {
                    let _ = writeln!(file, "{}", json);
                }
            }
        }
    }

    /// Get branch from leaf to root
    pub fn get_branch(&self, from_id: Option<&str>) -> Vec<&SessionEntry> {
        let start_id = from_id.or(self.leaf_id.as_deref());
        let mut path = Vec::new();
        let mut current = start_id;

        while let Some(id) = current {
            if let Some(entry) = self.by_id.get(id) {
                path.insert(0, entry);
                current = entry.parent_id();
            } else {
                break;
            }
        }

        path
    }

    /// Build session context (what gets sent to LLM)
    pub fn build_session_context(&self) -> SessionContext {
        let path = self.get_branch(None);

        let mut thinking_level = ThinkingLevel::Off;
        let mut model: Option<(String, String)> = None;
        let mut compaction: Option<&CompactionEntry> = None;
        let mut messages = Vec::new();

        // First pass: find settings
        for entry in &path {
            match entry {
                SessionEntry::ThinkingLevelChange(t) => {
                    thinking_level = match t.thinking_level.as_str() {
                        "off" => ThinkingLevel::Off,
                        "minimal" => ThinkingLevel::Minimal,
                        "low" => ThinkingLevel::Low,
                        "medium" => ThinkingLevel::Medium,
                        "high" => ThinkingLevel::High,
                        "xhigh" => ThinkingLevel::XHigh,
                        _ => ThinkingLevel::Medium,
                    };
                }
                SessionEntry::ModelChange(m) => {
                    model = Some((m.provider.clone(), m.model_id.clone()));
                }
                SessionEntry::Message(m) => {
                    if m.message.role == crate::core::Role::Assistant {
                        if let (Some(p), Some(mid)) = (&m.message.provider, &m.message.model) {
                            model = Some((p.clone(), mid.clone()));
                        }
                    }
                }
                SessionEntry::Compaction(c) => {
                    compaction = Some(c);
                }
                _ => {}
            }
        }

        // Second pass: build messages
        if let Some(comp) = compaction {
            // Add summary first
            messages.push(Message::assistant(comp.summary.clone(), None, None));

            // Find entries after compaction
            let compaction_idx = path.iter().position(|e| {
                if let SessionEntry::Compaction(c) = e {
                    c.base.id == comp.base.id
                } else {
                    false
                }
            });

            if let Some(idx) = compaction_idx {
                // Emit kept messages starting from first_kept_entry_id
                let mut found_first_kept = false;
                for i in 0..idx {
                    let entry = path[i];
                    if entry.id() == comp.first_kept_entry_id {
                        found_first_kept = true;
                    }
                    if found_first_kept {
                        Self::add_message_from_entry(entry, &mut messages);
                    }
                }

                // Emit messages after compaction
                for i in (idx + 1)..path.len() {
                    Self::add_message_from_entry(path[i], &mut messages);
                }
            }
        } else {
            // No compaction - emit all messages
            for entry in &path {
                Self::add_message_from_entry(entry, &mut messages);
            }
        }

        SessionContext {
            messages,
            thinking_level,
            model,
        }
    }

    /// Add message from entry to context
    fn add_message_from_entry(entry: &SessionEntry, messages: &mut Vec<Message>) {
        match entry {
            SessionEntry::Message(m) => {
                messages.push(m.message.clone());
            }
            SessionEntry::CustomMessage(c) => {
                messages.push(Message::user(c.content.clone()));
            }
            SessionEntry::BranchSummary(b) => {
                messages.push(Message::assistant(b.summary.clone(), None, None));
            }
            _ => {}
        }
    }

    /// Get session as tree
    pub fn get_tree(&self) -> Vec<SessionTreeNode> {
        let entries: Vec<&SessionEntry> = self.get_entries();
        let mut node_map: HashMap<String, SessionTreeNode> = HashMap::new();
        let mut roots: Vec<SessionTreeNode> = Vec::new();

        // Create nodes
        for entry in &entries {
            let label = self.labels_by_id.get(entry.id()).cloned();
            let entry_clone = (*entry).clone();
            node_map.insert(
                entry.id().to_string(),
                SessionTreeNode {
                    entry: entry_clone,
                    children: Vec::new(),
                    label,
                },
            );
        }

        // Build tree
        let nodes_to_process: Vec<(Option<String>, SessionTreeNode)> = entries
            .iter()
            .map(|entry| {
                let node = node_map.get(entry.id()).unwrap().clone();
                (entry.parent_id().map(String::from), node)
            })
            .collect();

        for (parent_id, node) in nodes_to_process {
            if let Some(pid) = parent_id {
                if let Some(parent) = node_map.get_mut(&pid) {
                    parent.children.push(node);
                } else {
                    roots.push(node);
                }
            } else {
                roots.push(node);
            }
        }

        // Sort children by timestamp
        fn sort_children(nodes: &mut [SessionTreeNode]) {
            nodes.sort_by(|a, b| a.entry.timestamp().cmp(b.entry.timestamp()));
            for node in nodes.iter_mut() {
                sort_children(&mut node.children);
            }
        }
        sort_children(&mut roots);

        roots
    }

    /// Branch from an earlier entry
    pub fn branch(&mut self, branch_from_id: &str) -> Result<(), String> {
        if !self.by_id.contains_key(branch_from_id) {
            return Err(format!("Entry {} not found", branch_from_id));
        }
        self.leaf_id = Some(branch_from_id.to_string());
        Ok(())
    }

    /// Reset leaf to null (before first entry)
    pub fn reset_leaf(&mut self) {
        self.leaf_id = None;
    }

    /// Branch with a summary
    pub fn branch_with_summary(
        &mut self,
        branch_from_id: Option<&str>,
        summary: &str,
    ) -> Result<String, String> {
        if let Some(id) = branch_from_id {
            if !self.by_id.contains_key(id) {
                return Err(format!("Entry {} not found", id));
            }
        }

        let id = generate_id(&self.by_id.keys().cloned().collect());
        let from_id = branch_from_id.unwrap_or("root");

        let entry = BranchSummaryEntry::new(id.clone(), self.leaf_id.clone(), from_id, summary);

        self.leaf_id = branch_from_id.map(String::from);
        self.append_entry(SessionEntry::BranchSummary(entry));

        Ok(id)
    }

    /// Create a new session (static constructor)
    pub fn create(cwd: &str, session_dir: Option<&Path>) -> Self {
        let dir = session_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| get_default_session_dir(cwd));

        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }

        Self::new(cwd, dir, None, true)
    }

    /// Open an existing session
    pub fn open(path: &Path, session_dir: Option<&Path>) -> Self {
        let entries = Self::load_entries_from_file(path);
        let cwd = entries
            .iter()
            .find_map(|e| {
                if let FileEntry::Header(h) = e {
                    Some(h.cwd.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            });

        let dir = session_dir.map(PathBuf::from).unwrap_or_else(|| {
            path.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| get_default_session_dir(&cwd))
        });

        Self::new(&cwd, dir, Some(path.to_path_buf()), true)
    }

    /// Continue the most recent session
    pub fn continue_recent(cwd: &str, session_dir: Option<&Path>) -> Self {
        let dir = session_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| get_default_session_dir(cwd));

        if !dir.exists() {
            return Self::new(cwd, dir, None, true);
        }

        // Find most recent .jsonl file
        if let Ok(entries) = fs::read_dir(&dir) {
            let mut files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|s| s == "jsonl").unwrap_or(false))
                .collect();

            files.sort_by_key(|e| std::cmp::Reverse(e.metadata().and_then(|m| m.modified()).ok()));

            if let Some(file) = files.first() {
                return Self::new(cwd, dir, Some(file.path()), true);
            }
        }

        Self::new(cwd, dir, None, true)
    }

    /// Create an in-memory session
    pub fn in_memory(cwd: &str) -> Self {
        Self::new(cwd, PathBuf::from(""), None, false)
    }
}

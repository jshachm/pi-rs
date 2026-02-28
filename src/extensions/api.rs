//! Extension runtime API

use std::collections::HashMap;
use std::sync::RwLock;

use crate::extensions::types::ExtensionInfo;

pub struct ExtensionRuntime {
    extensions: RwLock<HashMap<String, ExtensionState>>,
    registered_tools: RwLock<HashMap<String, RegisteredTool>>,
    registered_commands: RwLock<HashMap<String, RegisteredCommand>>,
    event_handlers: RwLock<HashMap<String, Vec<Box<dyn EventHandler>>>>,
}

struct ExtensionState {
    info: ExtensionInfo,
    enabled: bool,
}

#[derive(Debug, Clone)]
pub struct RegisteredTool {
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct RegisteredCommand {
    pub name: String,
    pub description: String,
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &str, data: serde_json::Value) -> Option<serde_json::Value>;
}

impl ExtensionRuntime {
    pub fn new() -> Self {
        Self {
            extensions: RwLock::new(HashMap::new()),
            registered_tools: RwLock::new(HashMap::new()),
            registered_commands: RwLock::new(HashMap::new()),
            event_handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_extension(&self, info: ExtensionInfo) {
        let mut exts = self.extensions.write().unwrap();
        exts.insert(
            info.name.clone(),
            ExtensionState {
                info,
                enabled: true,
            },
        );
    }

    pub fn unregister_extension(&self, name: &str) {
        let mut exts = self.extensions.write().unwrap();
        exts.remove(name);

        let mut tools = self.registered_tools.write().unwrap();
        tools.retain(|_, t| !t.name.starts_with(&format!("{}.", name)));

        let mut commands = self.registered_commands.write().unwrap();
        commands.retain(|_, c| !c.name.starts_with(&format!("{}.", name)));
    }

    pub fn register_tool(
        &self,
        extension: &str,
        name: &str,
        description: &str,
        schema: serde_json::Value,
    ) {
        let full_name = format!("{}.{}", extension, name);
        let mut tools = self.registered_tools.write().unwrap();
        tools.insert(
            full_name,
            RegisteredTool {
                name: name.to_string(),
                description: description.to_string(),
                schema,
            },
        );
    }

    pub fn register_command(&self, extension: &str, name: &str, description: &str) {
        let full_name = format!("{}.{}", extension, name);
        let mut commands = self.registered_commands.write().unwrap();
        commands.insert(
            full_name,
            RegisteredCommand {
                name: name.to_string(),
                description: description.to_string(),
            },
        );
    }

    pub fn on_event(&self, event: &str, handler: Box<dyn EventHandler>) {
        let mut handlers = self.event_handlers.write().unwrap();
        handlers.entry(event.to_string()).or_default().push(handler);
    }

    pub fn emit_event(&self, event: &str, data: serde_json::Value) -> Vec<serde_json::Value> {
        let handlers = self.event_handlers.read().unwrap();
        let handlers = handlers.get(event);

        match handlers {
            Some(hs) => hs
                .iter()
                .filter_map(|h| h.handle(event, data.clone()))
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn get_tools(&self) -> Vec<RegisteredTool> {
        let tools = self.registered_tools.read().unwrap();
        tools.values().cloned().collect()
    }

    pub fn get_commands(&self) -> Vec<RegisteredCommand> {
        let commands = self.registered_commands.read().unwrap();
        commands.values().cloned().collect()
    }

    pub fn list_extensions(&self) -> Vec<ExtensionInfo> {
        let exts = self.extensions.read().unwrap();
        exts.values().map(|e| e.info.clone()).collect()
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        let exts = self.extensions.read().unwrap();
        exts.get(name).map(|e| e.enabled).unwrap_or(false)
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) -> bool {
        let mut exts = self.extensions.write().unwrap();
        if let Some(ext) = exts.get_mut(name) {
            ext.enabled = enabled;
            return true;
        }
        false
    }
}

impl Default for ExtensionRuntime {
    fn default() -> Self {
        Self::new()
    }
}

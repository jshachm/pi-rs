//! Event system for the agent

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

/// Event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Session events
    SessionStart,
    SessionEnd,
    SessionSwitch,
    SessionFork,
    SessionCompact,

    // Agent events
    AgentStart,
    AgentEnd,
    TurnStart,
    TurnEnd,
    ContextUpdate,

    // Message events
    MessageStart,
    MessageUpdate,
    MessageEnd,

    // Tool events
    ToolExecutionStart,
    ToolExecutionEnd,
    ToolCall,
    ToolResult,

    // Model events
    ModelChange,

    // Input events
    UserInput,
    BashInput,
}

/// Event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventPayload {
    None,
    SessionStart {
        session_id: String,
    },
    SessionEnd {
        session_id: String,
    },
    Message {
        content: String,
        role: String,
    },
    ToolCall {
        tool_name: String,
        args: serde_json::Value,
    },
    ToolResult {
        tool_name: String,
        success: bool,
    },
    ModelChange {
        model_id: String,
        provider: String,
    },
    Context {
        tokens: u64,
        messages: u32,
    },
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub payload: EventPayload,
    pub timestamp: i64,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            payload: EventPayload::None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn with_payload(mut self, payload: EventPayload) -> Self {
        self.payload = payload;
        self
    }
}

/// Event listener
pub type EventListener = Arc<dyn Fn(&Event) + Send + Sync>;

/// Event bus for publish-subscribe
pub struct EventBus {
    listeners: RwLock<HashMap<EventType, Vec<EventListener>>>,
    history: RwLock<Vec<Event>>,
    max_history: usize,
}

impl EventBus {
    pub fn new(max_history: usize) -> Self {
        Self {
            listeners: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
            max_history,
        }
    }

    pub fn subscribe(&self, event_type: EventType, listener: EventListener) {
        let mut listeners = self.listeners.write().unwrap();
        listeners
            .entry(event_type)
            .or_default()
            .push(listener);
    }

    pub fn unsubscribe(&self, event_type: &EventType, listener: &EventListener) {
        let mut listeners = self.listeners.write().unwrap();
        if let Some(listeners_vec) = listeners.get_mut(event_type) {
            listeners_vec.retain(|l| Arc::as_ptr(l) != Arc::as_ptr(listener));
        }
    }

    pub fn publish(&self, event: Event) {
        // Store in history
        {
            let mut history = self.history.write().unwrap();
            history.push(event.clone());
            if history.len() > self.max_history {
                history.remove(0);
            }
        }

        // Notify listeners
        let listeners = self.listeners.read().unwrap();

        // Notify specific listeners
        if let Some(specific_listeners) = listeners.get(&event.event_type) {
            for listener in specific_listeners {
                listener(&event);
            }
        }

        // Notify wildcard listeners
        if let Some(_wildcard_listeners) = listeners.get(&EventType::SessionStart) {
            // For wildcard, we could use a special type
            // For now, just notify specific
        }
    }

    pub fn get_history(&self, event_type: Option<EventType>) -> Vec<Event> {
        let history = self.history.read().unwrap();
        match event_type {
            Some(et) => history
                .iter()
                .filter(|e| e.event_type == et)
                .cloned()
                .collect(),
            None => history.clone(),
        }
    }

    pub fn clear_history(&self) {
        let mut history = self.history.write().unwrap();
        history.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus() {
        let bus = EventBus::new(10);
        let event_received = Arc::new(RwLock::new(false));
        let event_received_clone = event_received.clone();

        bus.subscribe(
            EventType::SessionStart,
            Arc::new(move |_| {
                *event_received_clone.write().unwrap() = true;
            }),
        );

        bus.publish(Event::new(EventType::SessionStart));

        assert!(*event_received.read().unwrap());
    }
}

//! Agent Session - Core agent logic

use std::sync::Arc;

use crate::core::{Message, ThinkingLevel, Role};
use crate::session::SessionManager;
use crate::providers::{Provider, provider::ProviderResponse};
use crate::tools::{ToolTrait, ToolResult};
use crate::agent::events::{EventBus, Event, EventType, EventPayload};

/// Agent configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub thinking_level: ThinkingLevel,
    pub cwd: String,
    pub tools: Vec<String>,
}

impl AgentConfig {
    pub fn new(provider: &str, model: &str) -> Self {
        Self {
            provider: provider.to_string(),
            model: model.to_string(),
            thinking_level: ThinkingLevel::default(),
            cwd: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            tools: vec!["read".to_string(), "bash".to_string(), "write".to_string(), "edit".to_string()],
        }
    }
}

/// Agent state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Idle,
    Thinking,
    WaitingForTool,
    ExecutingTool,
    WaitingForInput,
}

/// Agent session
pub struct AgentSession {
    config: AgentConfig,
    session: SessionManager,
    provider: Arc<dyn Provider>,
    tools: Vec<Arc<dyn ToolTrait>>,
    state: AgentState,
    event_bus: Arc<EventBus>,
    current_response: Option<String>,
}

impl AgentSession {
    /// Create a new agent session
    pub fn new(
        config: AgentConfig,
        session: SessionManager,
        provider: Arc<dyn Provider>,
        tools: Vec<Arc<dyn ToolTrait>>,
    ) -> Self {
        let event_bus = Arc::new(EventBus::new(1000));
        
        Self {
            config,
            session,
            provider,
            tools,
            state: AgentState::Idle,
            event_bus,
            current_response: None,
        }
    }

    /// Get the event bus
    pub fn event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }

    /// Get current state
    pub fn state(&self) -> AgentState {
        self.state
    }

    /// Get current working directory
    pub fn cwd(&self) -> &str {
        &self.config.cwd
    }

    /// Get available tools
    pub fn tools(&self) -> &[Arc<dyn ToolTrait>] {
        &self.tools
    }

    /// Get the current model
    pub fn model(&self) -> Option<Model> {
        self.provider.models().into_iter()
            .find(|m| m.id == self.config.model)
    }

    /// Get thinking level
    pub fn thinking_level(&self) -> ThinkingLevel {
        self.config.thinking_level
    }

    /// Set thinking level
    pub fn set_thinking_level(&mut self, level: ThinkingLevel) {
        self.config.thinking_level = level;
        self.session.append_thinking_level_change(level);
    }

    /// Subscribe to events
    pub fn on<F>(&self, event_type: EventType, handler: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.event_bus.subscribe(event_type, Arc::new(handler));
    }

    /// Send a prompt to the agent
    pub async fn prompt(&mut self, text: &str) -> Result<String, String> {
        // Add user message to session
        let user_message = Message::user(text);
        self.session.append_message(user_message.clone());

        // Publish event
        self.event_bus.publish(Event::new(EventType::TurnStart)
            .with_payload(EventPayload::Message {
                content: text.to_string(),
                role: "user".to_string(),
            }));

        // Build context
        let context = self.session.build_session_context();

        // Publish context event
        let token_count = context.messages.iter()
            .map(|m| m.content.as_text().len() as u64 / 4)
            .sum();
        self.event_bus.publish(Event::new(EventType::ContextUpdate)
            .with_payload(EventPayload::Context {
                tokens: token_count,
                messages: context.messages.len() as u32,
            }));

        // Set state to thinking
        self.state = AgentState::Thinking;
        self.event_bus.publish(Event::new(EventType::AgentStart));

        // Build tool schemas
        let tool_schemas: Option<Vec<serde_json::Value>> = if self.tools.is_empty() {
            None
        } else {
            Some(
                self.tools.iter().map(|t| serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name(),
                        "description": t.description(),
                        "parameters": t.schema().clone(),
                    }
                })).collect()
            )
        };

        let thinking = self.config.thinking_level != ThinkingLevel::Off;
        
        // Call the LLM
        let response = self.provider.chat(
            &self.config.model,
            context.messages,
            tool_schemas.clone(),
            Some(thinking),
        ).await.map_err(|e| e.message)?;

        // Process the response - handle tool calls in a loop
        let final_content = self.process_response_loop(response, tool_schemas, thinking).await?;

        Ok(final_content)
    }

    /// Process LLM response with tool calling loop (iterative, not recursive)
    async fn process_response_loop(
        &mut self,
        mut response: ProviderResponse,
        tool_schemas: Option<Vec<serde_json::Value>>,
        thinking: bool,
    ) -> Result<String, String> {
        loop {
            let choice = response.choices.first()
                .ok_or("No response choices")?;

            let content = choice.message.content.as_text().clone();
            let tool_calls = choice.message.tool_calls.clone();

            // Check if there are tool calls to execute
            if let Some(tool_calls) = tool_calls {
                if !tool_calls.is_empty() {
                    self.state = AgentState::WaitingForTool;

                    // Add assistant message with tool calls to session
                    let assistant_message = Message::assistant_with_tools(
                        content.clone(),
                        tool_calls.clone(),
                        Some(&self.config.provider),
                        Some(&self.config.model),
                    );
                    self.session.append_message(assistant_message);

                    // Execute each tool call
                    for tool_call in &tool_calls {
                        self.state = AgentState::ExecutingTool;
                        self.event_bus.publish(Event::new(EventType::ToolExecutionStart)
                            .with_payload(EventPayload::ToolCall {
                                tool_name: tool_call.name.clone(),
                                args: tool_call.input.clone(),
                            }));

                        let tool = self.tools.iter()
                            .find(|t| t.name() == tool_call.name)
                            .ok_or_else(|| format!("Tool not found: {}", tool_call.name))?;

                        let result = tool.execute(tool_call.input.clone(), &self.config.cwd)
                            .map_err(|e| e.to_string())?;

                        self.event_bus.publish(Event::new(EventType::ToolExecutionEnd)
                            .with_payload(EventPayload::ToolResult {
                                tool_name: tool_call.name.clone(),
                                success: result.success,
                            }));

                        // Add tool result as message
                        let tool_message = Message::tool_result(&tool_call.id, &result.content);
                        self.session.append_message(tool_message);
                    }

                    // Continue conversation with tool results
                    self.state = AgentState::Thinking;
                    let context = self.session.build_session_context();
                    
                    response = self.provider.chat(
                        &self.config.model,
                        context.messages,
                        tool_schemas.clone(),
                        Some(thinking),
                    ).await.map_err(|e| e.message)?;

                    // Loop continues to check for more tool calls
                    continue;
                }
            }

            // No tool calls - return the content
            self.state = AgentState::Idle;

            // Add assistant message to session
            let assistant_message = Message::assistant(
                content.clone(),
                Some(&self.config.provider),
                Some(&self.config.model),
            );
            self.session.append_message(assistant_message);

            self.event_bus.publish(Event::new(EventType::TurnEnd));
            self.event_bus.publish(Event::new(EventType::AgentEnd));

            // Store current response
            self.current_response = Some(content.clone());

            return Ok(content);
        }
    }

    /// Execute a tool
    pub async fn execute_tool(&self, name: &str, args: serde_json::Value) -> Result<ToolResult, String> {
        self.event_bus.publish(Event::new(EventType::ToolExecutionStart)
            .with_payload(EventPayload::ToolCall {
                tool_name: name.to_string(),
                args: args.clone(),
            }));

        let tool = self.tools.iter()
            .find(|t| t.name() == name)
            .ok_or_else(|| format!("Tool not found: {}", name))?;

        let result = tool.execute(args, &self.config.cwd)
            .map_err(|e| e.to_string())?;

        self.event_bus.publish(Event::new(EventType::ToolExecutionEnd)
            .with_payload(EventPayload::ToolResult {
                tool_name: name.to_string(),
                success: result.success,
            }));

        Ok(result)
    }

    /// Process tool calls from LLM response
    pub async fn process_tool_calls(&mut self, tool_calls: Vec<serde_json::Value>) -> Result<String, String> {
        let mut results = Vec::new();

        for tool_call in tool_calls {
            let name = tool_call.get("function")
                .and_then(|f| f.get("name"))
                .and_then(|n| n.as_str())
                .ok_or("Invalid tool call")?;

            let args = tool_call.get("function")
                .and_then(|f| f.get("arguments"))
                .cloned()
                .unwrap_or(serde_json::Value::Object(Default::default()));

            let result = self.execute_tool(name, args).await?;

            results.push(serde_json::json!({
                "tool_use_id": tool_call.get("id").and_then(|i| i.as_str()).unwrap_or("unknown"),
                "output": result.content,
            }));
        }

        Ok(serde_json::to_string(&results).unwrap_or_default())
    }

    /// Continue the conversation after tool results
    pub async fn continue_prompt(&mut self, tool_results: &str) -> Result<String, String> {
        // Add tool results as user message
        let tool_message = Message {
            role: Role::Tool,
            content: crate::core::MessageContent::Text(tool_results.to_string()),
            tool_call_id: None,
            provider: None,
            model: None,
            thinking: None,
            timestamp: Some(chrono::Utc::now().timestamp_millis()),
            tool_calls: None,
        };
        self.session.append_message(tool_message);

        // Build context and call LLM again
        let context = self.session.build_session_context();

        self.state = AgentState::Thinking;

        let tool_schemas: Option<Vec<serde_json::Value>> = Some(
            self.tools.iter().map(|t| serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name(),
                    "description": t.description(),
                    "parameters": t.schema().clone(),
                }
            })).collect()
        );

        let thinking = self.config.thinking_level != ThinkingLevel::Off;
        
        let response = self.provider.chat(
            &self.config.model,
            context.messages,
            tool_schemas,
            Some(thinking),
        ).await.map_err(|e| e.message)?;

        self.state = AgentState::Idle;

        let content = response.choices.first()
            .map(|c| c.message.content.as_text().clone())
            .unwrap_or_default();

        let assistant_message = Message::assistant(
            content.clone(),
            Some(&self.config.provider),
            Some(&self.config.model),
        );
        self.session.append_message(assistant_message);

        self.current_response = Some(content.clone());
        Ok(content)
    }

    /// Get session manager
    pub fn session(&self) -> &SessionManager {
        &self.session
    }

    /// Get mutable session manager
    pub fn session_mut(&mut self) -> &mut SessionManager {
        &mut self.session
    }
}

/// Simple Model struct (re-exported from core)
use crate::core::Model;

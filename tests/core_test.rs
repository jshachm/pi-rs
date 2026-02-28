#[cfg(test)]
mod core_tests {
    use pi_rs::core::ModelCost;
    use pi_rs::{Message, Model, Role, ThinkingLevel};

    #[test]
    fn test_message_user() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content.as_text(), "Hello");
    }

    #[test]
    fn test_message_assistant() {
        let msg = Message::assistant("Hi there!", Some("openai"), Some("gpt-4"));
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.content.as_text(), "Hi there!");
        assert_eq!(msg.provider, Some("openai".to_string()));
        assert_eq!(msg.model, Some("gpt-4".to_string()));
    }

    #[test]
    fn test_message_system() {
        let msg = Message::system("You are helpful");
        assert_eq!(msg.role, Role::System);
    }

    #[test]
    fn test_message_tool_result() {
        let msg = Message::tool_result("tool-123", "result content");
        assert_eq!(msg.role, Role::Tool);
        assert_eq!(msg.tool_call_id, Some("tool-123".to_string()));
    }

    #[test]
    fn test_message_content_text() {
        let msg = Message::user("Hello");
        assert_eq!(msg.content.as_text(), "Hello");
    }

    #[test]
    fn test_thinking_level_default() {
        let level = ThinkingLevel::default();
        assert_eq!(level, ThinkingLevel::Medium);
    }

    #[test]
    fn test_thinking_level_variants() {
        assert!(matches!(ThinkingLevel::Off, ThinkingLevel::Off));
        assert!(matches!(ThinkingLevel::Minimal, ThinkingLevel::Minimal));
        assert!(matches!(ThinkingLevel::Low, ThinkingLevel::Low));
        assert!(matches!(ThinkingLevel::Medium, ThinkingLevel::Medium));
        assert!(matches!(ThinkingLevel::High, ThinkingLevel::High));
        assert!(matches!(ThinkingLevel::XHigh, ThinkingLevel::XHigh));
    }

    #[test]
    fn test_model_create() {
        let model = Model {
            id: "gpt-4".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            context_window: 8192,
            max_tokens: 4096,
            supports_thinking: false,
            input_types: vec!["text".to_string()],
            cost: ModelCost {
                input: 0.03,
                output: 0.06,
                cache_read: 0.0,
                cache_write: 0.0,
            },
        };

        assert_eq!(model.id, "gpt-4");
        assert_eq!(model.provider, "openai");
    }
}

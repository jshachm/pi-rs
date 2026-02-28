#[cfg(test)]
mod input_tests {
    use pi_rs::input::completion::SkillDefinition;
    use pi_rs::input::{CompletionEngine, InputHandler};

    #[test]
    fn test_input_handler_create() {
        let handler = InputHandler::new();
        assert_eq!(handler.get_content(), "");
    }

    #[test]
    fn test_input_handler_insert_char() {
        let mut handler = InputHandler::new();
        handler.insert_char('h');
        handler.insert_char('i');
        assert_eq!(handler.get_content(), "hi");
    }

    #[test]
    fn test_input_handler_delete_char() {
        let mut handler = InputHandler::new();
        handler.insert_char('h');
        handler.insert_char('i');
        handler.delete_char();
        assert_eq!(handler.get_content(), "h");
    }

    #[test]
    fn test_input_handler_submit() {
        let mut handler = InputHandler::new();
        handler.insert_char('h');
        handler.insert_char('e');
        handler.insert_char('l');
        handler.insert_char('l');
        handler.insert_char('o');

        let submitted = handler.submit();
        assert_eq!(submitted, "hello");
        assert_eq!(handler.get_content(), "");
    }

    #[test]
    fn test_input_handler_cursor_movement() {
        let mut handler = InputHandler::new();
        handler.insert_char('h');
        handler.insert_char('i');

        handler.move_cursor_left();
        assert_eq!(handler.get_cursor_position(), 1);

        handler.move_cursor_right();
        assert_eq!(handler.get_cursor_position(), 2);

        handler.move_cursor_to_start();
        assert_eq!(handler.get_cursor_position(), 0);

        handler.move_cursor_to_end();
        assert_eq!(handler.get_cursor_position(), 2);
    }

    #[test]
    fn test_completion_engine_command_completion() {
        let engine = CompletionEngine::new();
        let completions = engine.get_completions("/h");
        assert!(!completions.is_empty());
    }

    #[test]
    fn test_completion_engine_skill_completion() {
        let mut engine = CompletionEngine::new();
        engine.register_skill(SkillDefinition {
            name: "test-skill".to_string(),
            description: "Test skill description".to_string(),
            trigger: "test".to_string(),
        });

        let completions = engine.get_completions("@test");
        assert!(!completions.is_empty());
    }
}

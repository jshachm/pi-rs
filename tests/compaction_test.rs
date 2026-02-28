#[cfg(test)]
mod compaction_tests {
    use pi_rs::compaction::ContextCompactor;
    use pi_rs::Message;

    #[test]
    fn test_compactor_create() {
        let compactor = ContextCompactor::new(50, 40);
        // Default creation should work
        assert!(true);
    }

    #[test]
    fn test_compactor_should_compact_false() {
        let compactor = ContextCompactor::new(50, 40);

        let messages = vec![
            Message::user("Hello"),
            Message::assistant("Hi there!", None, None),
        ];

        assert!(!compactor.should_compact(&messages));
    }

    #[test]
    fn test_compactor_should_compact_true() {
        let compactor = ContextCompactor::new(10, 5);

        let messages: Vec<Message> = (0..15)
            .map(|i| Message::user(format!("Message {}", i)))
            .collect();

        assert!(compactor.should_compact(&messages));
    }

    #[test]
    fn test_compactor_compact_noop_when_small() {
        let compactor = ContextCompactor::new(50, 40);

        let messages = vec![
            Message::user("Hello"),
            Message::assistant("Hi there!", None, None),
        ];

        let result = compactor.compact(messages.clone(), None, None);

        assert!(result.summary.is_empty());
        assert_eq!(result.compacted_messages.len(), 2);
        assert_eq!(result.removed_count, 0);
    }

    #[test]
    fn test_compactor_compact_reduces_messages() {
        let compactor = ContextCompactor::new(5, 3);

        let messages: Vec<Message> = (0..10)
            .map(|i| Message::user(format!("Message {}", i)))
            .collect();

        let result = compactor.compact(messages, None, None);

        assert!(!result.summary.is_empty());
        assert!(result.compacted_messages.len() < 10);
        assert!(result.removed_count > 0);
    }

    #[test]
    fn test_compactor_summarize() {
        let compactor = ContextCompactor::default();

        let messages = vec![
            Message::user("Write a function"),
            Message::user("Now add tests"),
            Message::user("Fix the bug"),
        ];

        let summary = compactor.summarize(&messages);

        assert!(!summary.is_empty());
    }

    #[test]
    fn test_compactor_estimate_tokens() {
        let compactor = ContextCompactor::default();

        let tokens = compactor.estimate_tokens("Hello world");
        assert!(tokens > 0);
    }
}

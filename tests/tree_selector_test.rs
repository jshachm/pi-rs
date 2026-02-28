#[cfg(test)]
mod tree_selector_tests {
    use pi_rs::Message;
    use pi_rs::SessionManager;

    #[test]
    fn test_tree_selector_create() {
        let session = SessionManager::in_memory("/tmp");
        let id = session.get_session_id();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_tree_selector_build_tree() {
        let mut session = SessionManager::in_memory("/tmp");

        session.append_message(Message::user("Message 1"));
        session.append_message(Message::assistant("Response 1", None, None));

        let entries = session.get_branch(None);

        // Should have entries
        assert!(entries.len() >= 2);
    }

    #[test]
    fn test_tree_selector_navigate() {
        let mut session = SessionManager::in_memory("/tmp");

        session.append_message(Message::user("First"));

        // Get branch from specific point
        let entries = session.get_branch(None);
        assert!(!entries.is_empty());
    }
}

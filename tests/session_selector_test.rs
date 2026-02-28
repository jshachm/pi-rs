#[cfg(test)]
mod session_selector_tests {
    use pi_rs::Message;
    use pi_rs::SessionManager;

    #[test]
    fn test_session_selector_list_sessions() {
        let session = SessionManager::in_memory("/tmp");
        let _id = session.get_session_id();

        // Should have a valid session ID
        assert!(!_id.is_empty());
    }

    #[test]
    fn test_session_selector_get_current() {
        let session = SessionManager::in_memory("/tmp");

        let id = session.get_session_id();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_session_selector_branch_info() {
        let mut session = SessionManager::in_memory("/tmp");

        session.append_message(Message::user("Hello"));
        session.append_message(Message::assistant("Hi", None, None));

        let branch = session.get_branch(None);
        assert!(!branch.is_empty());
    }

    #[test]
    fn test_session_selector_get_entries() {
        let mut session = SessionManager::in_memory("/tmp");

        session.append_message(Message::user("First"));
        session.append_message(Message::user("Second"));

        let context = session.build_session_context();
        assert_eq!(context.messages.len(), 2);
    }
}

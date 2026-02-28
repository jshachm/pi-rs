#[cfg(test)]
mod session_tests {
    use pi_rs::Message;
    use pi_rs::SessionManager;

    #[test]
    fn test_session_create() {
        let session = SessionManager::in_memory("/tmp");
        assert!(!session.get_session_id().is_empty());
    }

    #[test]
    fn test_session_append_message() {
        let mut session = SessionManager::in_memory("/tmp");
        let msg = Message::user("Hello");
        let id = session.append_message(msg);
        assert!(!id.is_empty());
    }

    #[test]
    fn test_session_get_messages() {
        let mut session = SessionManager::in_memory("/tmp");
        session.append_message(Message::user("Hello"));
        session.append_message(Message::assistant("Hi there!", None, None));

        let context = session.build_session_context();
        assert_eq!(context.messages.len(), 2);
    }

    #[test]
    fn test_session_branch() {
        let mut session = SessionManager::in_memory("/tmp");
        session.append_message(Message::user("Hello"));

        let entries = session.get_branch(None);
        assert!(!entries.is_empty());
    }
}

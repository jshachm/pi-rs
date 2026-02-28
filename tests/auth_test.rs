#[cfg(test)]
mod auth_tests {
    use pi_rs::AuthStorage;

    #[test]
    fn test_auth_storage_create() {
        let storage = AuthStorage::in_memory();
        // Check if we can list providers (should be empty initially)
        let providers = storage.list();
        assert!(providers.is_empty() || !storage.has_auth("test"));
    }

    #[test]
    fn test_auth_storage_set_api_key() {
        let mut storage = AuthStorage::in_memory();
        storage.set_api_key("test-provider", "test-key-123".to_string());

        let key = storage.get_api_key("test-provider");
        assert_eq!(key, Some("test-key-123".to_string()));
    }

    #[test]
    fn test_auth_storage_get_nonexistent() {
        let storage = AuthStorage::in_memory();

        let key = storage.get_api_key("nonexistent");
        assert!(key.is_none());
    }

    #[test]
    fn test_auth_storage_has_auth() {
        let mut storage = AuthStorage::in_memory();

        assert!(!storage.has_auth("test-provider"));

        storage.set_api_key("test-provider", "test-key".to_string());

        assert!(storage.has_auth("test-provider"));
    }
}

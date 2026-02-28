#[cfg(test)]
mod provider_tests {
    use pi_rs::providers::ModelRegistry;

    #[test]
    fn test_registry_create() {
        let registry = ModelRegistry::new();
        let providers = registry.list_providers();
        assert!(!providers.is_empty()); // Should have at least ollama registered
    }

    #[test]
    fn test_registry_get_provider() {
        let registry = ModelRegistry::new();

        // Should have ollama by default
        let provider = registry.get_provider("ollama");
        assert!(provider.is_some());
    }

    #[test]
    fn test_registry_get_models() {
        let registry = ModelRegistry::new();
        let models = registry.get_all_models();
        assert!(!models.is_empty());
    }

    #[test]
    fn test_registry_get_model() {
        let registry = ModelRegistry::new();

        // Check if any model exists
        let models = registry.get_all_models();
        if !models.is_empty() {
            let model = registry.get_model(&models[0].id);
            assert!(model.is_some());
        }
    }

    #[test]
    fn test_registry_provider_for_model() {
        let registry = ModelRegistry::new();

        let models = registry.get_all_models();
        if !models.is_empty() {
            let provider = registry.get_provider_for_model(&models[0].id);
            assert!(provider.is_some());
        }
    }
}

#[cfg(test)]
mod model_resolver_tests {
    use pi_rs::providers::ModelRegistry;

    #[test]
    fn test_model_resolver_by_id() {
        let registry = ModelRegistry::new();

        // Should be able to get model by known ID
        let models = registry.get_all_models();
        if !models.is_empty() {
            let first_model_id = &models[0].id;
            let model = registry.get_model(first_model_id);
            assert!(model.is_some());
        }
    }

    #[test]
    fn test_model_resolver_by_provider() {
        let registry = ModelRegistry::new();

        // Should get models for a provider
        let provider = registry.list_providers().first().cloned();
        if let Some(p) = provider {
            let models = registry.get_models_for_provider(&p);
            assert!(models.is_some());
        }
    }

    #[test]
    fn test_model_resolver_provider_for_model() {
        let registry = ModelRegistry::new();

        let models = registry.get_all_models();
        if !models.is_empty() {
            let provider = registry.get_provider_for_model(&models[0].id);
            assert!(provider.is_some());
        }
    }
}

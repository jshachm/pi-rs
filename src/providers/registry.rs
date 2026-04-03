//! Model registry - manages available models and providers

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::core::Model;
use crate::providers::{
    AnthropicProvider, AzureProvider, GoogleProvider, GroqProvider, MistralProvider,
    MoonshotProvider, OllamaProvider, OpenAIProvider, Provider,
};

/// Provider configuration overrides from CLI
#[derive(Clone, Default)]
pub struct ProviderOverride {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

/// Model registry
pub struct ModelRegistry {
    providers: RwLock<HashMap<String, Arc<dyn Provider>>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::new_with_overrides(Default::default())
    }

    /// Create registry with CLI overrides for specific providers.
    /// Overrides take precedence over environment variables.
    pub fn new_with_overrides(overrides: HashMap<String, ProviderOverride>) -> Self {
        let mut registry = Self {
            providers: RwLock::new(HashMap::new()),
        };

        registry.register_default_providers(overrides);

        registry
    }

    fn register_default_providers(&mut self, overrides: HashMap<String, ProviderOverride>) {
        // Helper: get api_key from override or env var
        let get_key = |name: &str, env_key: &str, overrides: &HashMap<String, ProviderOverride>| -> Option<String> {
            overrides.get(name)
                .and_then(|o| o.api_key.clone())
                .or_else(|| std::env::var(env_key).ok().filter(|k| !k.is_empty()))
        };

        // Helper: get base_url from override, env var, or default
        let get_base_url = |name: &str, env_key: Option<&str>, default: &str, overrides: &HashMap<String, ProviderOverride>| -> String {
            overrides.get(name)
                .and_then(|o| o.base_url.as_ref())
                .filter(|u| !u.is_empty())
                .cloned()
                .or_else(|| env_key.and_then(|k| std::env::var(k).ok().filter(|u| !u.is_empty())))
                .unwrap_or_else(|| default.to_string())
        };

        // Anthropic
        if let Some(key) = get_key("anthropic", "ANTHROPIC_API_KEY", &overrides) {
            let base_url = get_base_url("anthropic", None, "https://api.anthropic.com", &overrides);
            self.register_provider(
                "anthropic",
                Arc::new(AnthropicProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }

        // OpenAI
        if let Some(key) = get_key("openai", "OPENAI_API_KEY", &overrides) {
            let base_url = get_base_url("openai", None, "https://api.openai.com", &overrides);
            self.register_provider(
                "openai",
                Arc::new(OpenAIProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }

        // Google
        if let Some(key) = get_key("google", "GOOGLE_API_KEY", &overrides) {
            let base_url = get_base_url("google", None, "https://generativelanguage.googleapis.com", &overrides);
            self.register_provider(
                "google",
                Arc::new(GoogleProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }

        // Moonshot
        if let Some(key) = get_key("moonshot", "MOONSHOT_API_KEY", &overrides) {
            let base_url = get_base_url("moonshot", None, "https://api.moonshot.cn/v1", &overrides);
            self.register_provider(
                "moonshot",
                Arc::new(MoonshotProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }

        // Ollama (always registered; base_url only)
        let ollama_base_url = get_base_url("ollama", Some("OLLAMA_BASE_URL"), "http://localhost:11434", &overrides);
        self.register_provider(
            "ollama",
            Arc::new(OllamaProvider::new(ollama_base_url)) as Arc<dyn Provider>,
        );

        // Azure OpenAI
        let azure_key = get_key("azure", "AZURE_OPENAI_API_KEY", &overrides);
        let azure_endpoint = overrides.get("azure")
            .and_then(|o| o.base_url.as_ref())
            .filter(|u| !u.is_empty())
            .cloned()
            .or_else(|| std::env::var("AZURE_OPENAI_ENDPOINT").ok().filter(|u| !u.is_empty()));
        if let (Some(key), Some(endpoint)) = (azure_key, azure_endpoint) {
            self.register_provider(
                "azure",
                Arc::new(AzureProvider::new(key, endpoint)) as Arc<dyn Provider>,
            );
        }

        // Mistral
        if let Some(key) = get_key("mistral", "MISTRAL_API_KEY", &overrides) {
            let base_url = get_base_url("mistral", None, "https://api.mistral.ai", &overrides);
            self.register_provider(
                "mistral",
                Arc::new(MistralProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }

        // Groq
        if let Some(key) = get_key("groq", "GROQ_API_KEY", &overrides) {
            let base_url = get_base_url("groq", None, "https://api.groq.com", &overrides);
            self.register_provider(
                "groq",
                Arc::new(GroqProvider::with_base_url(key, base_url)) as Arc<dyn Provider>,
            );
        }
    }

    /// Register a provider
    pub fn register_provider(&self, name: impl Into<String>, provider: Arc<dyn Provider>) {
        let mut providers = self.providers.write().unwrap();
        providers.insert(name.into(), provider);
    }

    /// Get a provider by name
    pub fn get_provider(&self, name: &str) -> Option<Arc<dyn Provider>> {
        let providers = self.providers.read().unwrap();
        providers.get(name).cloned()
    }

    /// Get all models from all providers
    pub fn get_all_models(&self) -> Vec<Model> {
        let providers = self.providers.read().unwrap();
        let mut models = Vec::new();

        for provider in providers.values() {
            models.extend(provider.models());
        }

        models
    }

    /// Get models for a specific provider
    pub fn get_models_for_provider(&self, provider: &str) -> Option<Vec<Model>> {
        let providers = self.providers.read().unwrap();
        providers.get(provider).map(|p| p.models())
    }

    /// Get a model by ID
    pub fn get_model(&self, model_id: &str) -> Option<Model> {
        self.get_all_models().into_iter().find(|m| m.id == model_id)
    }

    /// Get provider name for a model
    pub fn get_provider_for_model(&self, model_id: &str) -> Option<String> {
        for (name, provider) in self.providers.read().unwrap().iter() {
            if provider.models().iter().any(|m| m.id == model_id) {
                return Some(name.clone());
            }
        }
        None
    }

    /// List available providers
    pub fn list_providers(&self) -> Vec<String> {
        let providers = self.providers.read().unwrap();
        providers.keys().cloned().collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

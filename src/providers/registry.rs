//! Model registry - manages available models and providers

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::core::Model;
use crate::providers::{
    AnthropicProvider, AzureProvider, GoogleProvider, GroqProvider, MistralProvider,
    MoonshotProvider, OllamaProvider, OpenAIProvider, Provider,
};

/// Model registry
pub struct ModelRegistry {
    providers: RwLock<HashMap<String, Arc<dyn Provider>>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let registry = Self {
            providers: RwLock::new(HashMap::new()),
        };

        // Register default providers
        registry.register_default_providers();

        registry
    }

    fn register_default_providers(&self) {
        // Try to register providers with API keys from environment

        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "anthropic",
                    Arc::new(AnthropicProvider::new(key)) as Arc<dyn Provider>,
                );
            }
        }

        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "openai",
                    Arc::new(OpenAIProvider::new(key)) as Arc<dyn Provider>,
                );
            }
        }

        if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "google",
                    Arc::new(GoogleProvider::new(key)) as Arc<dyn Provider>,
                );
            }
        }

        // Moonshot (Kimi)
        if let Ok(key) = std::env::var("MOONSHOT_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "moonshot",
                    Arc::new(MoonshotProvider::new(key)) as Arc<dyn Provider>,
                );
            }
        }

        // Ollama (local models) - register if OLLAMA_BASE_URL is set
        if let Ok(base_url) = std::env::var("OLLAMA_BASE_URL") {
            if !base_url.is_empty() {
                self.register_provider(
                    "ollama",
                    Arc::new(OllamaProvider::new(base_url)) as Arc<dyn Provider>,
                );
            }
        } else {
            // Try default URL
            self.register_provider(
                "ollama",
                Arc::new(OllamaProvider::new("http://localhost:11434")) as Arc<dyn Provider>,
            );
        }

        // Azure OpenAI
        if let (Ok(key), Ok(endpoint)) = (
            std::env::var("AZURE_OPENAI_API_KEY"),
            std::env::var("AZURE_OPENAI_ENDPOINT"),
        ) {
            if !key.is_empty() && !endpoint.is_empty() {
                self.register_provider(
                    "azure",
                    Arc::new(AzureProvider::new(key, endpoint)) as Arc<dyn Provider>,
                );
            }
        }

        // Mistral
        if let Ok(key) = std::env::var("MISTRAL_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "mistral",
                    Arc::new(MistralProvider::new(key)) as Arc<dyn Provider>,
                );
            }
        }

        // Groq
        if let Ok(key) = std::env::var("GROQ_API_KEY") {
            if !key.is_empty() {
                self.register_provider(
                    "groq",
                    Arc::new(GroqProvider::new(key)) as Arc<dyn Provider>,
                );
            }
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

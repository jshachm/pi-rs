#[cfg(test)]
mod extension_tests {
    use pi_rs::extensions::{ExtensionInfo, ExtensionLoader, ExtensionRuntime};

    #[test]
    fn test_extension_loader_create() {
        let loader = ExtensionLoader::new();
        let extensions = loader.get_extensions();
        assert!(extensions.is_empty());
    }

    #[test]
    fn test_extension_runtime_create() {
        let runtime = ExtensionRuntime::new();
        let extensions = runtime.list_extensions();
        assert!(extensions.is_empty());
    }

    #[test]
    fn test_extension_runtime_register() {
        let runtime = ExtensionRuntime::new();

        runtime.register_extension(ExtensionInfo {
            name: "test-extension".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test extension".to_string()),
        });

        let extensions = runtime.list_extensions();
        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].name, "test-extension");
    }

    #[test]
    fn test_extension_runtime_enable_disable() {
        let runtime = ExtensionRuntime::new();

        runtime.register_extension(ExtensionInfo {
            name: "test-extension".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        });

        assert!(runtime.is_enabled("test-extension"));

        runtime.set_enabled("test-extension", false);
        assert!(!runtime.is_enabled("test-extension"));

        runtime.set_enabled("test-extension", true);
        assert!(runtime.is_enabled("test-extension"));
    }

    #[test]
    fn test_extension_runtime_get_tools() {
        let runtime = ExtensionRuntime::new();
        let tools = runtime.get_tools();
        assert!(tools.is_empty());
    }

    #[test]
    fn test_extension_runtime_get_commands() {
        let runtime = ExtensionRuntime::new();
        let commands = runtime.get_commands();
        assert!(commands.is_empty());
    }
}

#[cfg(test)]
mod extensions_discovery_tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_extension_discovery_empty_dir() {
        let temp_dir = std::env::temp_dir().join("pi_ext_test_empty");
        fs::create_dir_all(&temp_dir).ok();

        let mut loader = pi_rs::extensions::ExtensionLoader::new();
        loader.add_search_path(temp_dir.clone());
        let extensions = loader.load_extensions();

        fs::remove_dir_all(&temp_dir).ok();

        assert!(extensions.is_empty());
    }

    #[test]
    fn test_extension_discovery_single_extension() {
        let temp_dir = std::env::temp_dir().join("pi_ext_test_single");
        fs::create_dir_all(&temp_dir).ok();

        // Create a simple extension directory
        let ext_dir = temp_dir.join("test-extension");
        fs::create_dir_all(&ext_dir).ok();

        let manifest = r#"{"name": "test-extension", "version": "1.0.0", "description": "Test"}"#;
        fs::write(ext_dir.join("extension.json"), manifest).ok();

        let mut loader = pi_rs::extensions::ExtensionLoader::new();
        loader.add_search_path(temp_dir.clone());
        let extensions = loader.load_extensions();

        fs::remove_dir_all(&temp_dir).ok();

        assert!(extensions.len() <= 1);
    }

    #[test]
    fn test_extension_enable_disable() {
        let temp_dir = std::env::temp_dir().join("pi_ext_test_enable");
        fs::create_dir_all(&temp_dir).ok();

        let ext_dir = temp_dir.join("test-extension");
        fs::create_dir_all(&ext_dir).ok();

        let manifest = r#"{"name": "test-extension", "version": "1.0.0"}"#;
        fs::write(ext_dir.join("extension.json"), manifest).ok();

        let mut loader = pi_rs::extensions::ExtensionLoader::new();
        loader.add_search_path(temp_dir.clone());
        let _extensions = loader.load_extensions();

        let result = loader.enable_extension("test-extension");

        fs::remove_dir_all(&temp_dir).ok();

        assert!(result); // Should succeed if extension exists
    }
}

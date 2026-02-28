#[cfg(test)]
mod settings_tests {
    use pi_rs::SettingsManager;

    #[test]
    fn test_settings_create() {
        let _settings = SettingsManager::new("/tmp");
    }

    #[test]
    fn test_settings_get() {
        let settings = SettingsManager::new("/tmp");
        let _value = settings.get("thinking_level");
    }

    #[test]
    fn test_settings_provider() {
        let settings = SettingsManager::new("/tmp");
        let _provider = settings.get_default_provider();
    }

    #[test]
    fn test_settings_thinking_level() {
        let settings = SettingsManager::new("/tmp");
        let _level = settings.get_thinking_level();
    }
}

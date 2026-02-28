#[cfg(test)]
mod resource_loader_tests {
    use std::path::PathBuf;

    #[test]
    fn test_resource_loader_empty_dirs() {
        // Test with non-existent directories
        let temp = std::env::temp_dir().join("pi_test_nonexistent");

        // Skills
        let mut loader = pi_rs::skills::SkillLoader::new(temp.clone());
        let skills = loader.load_skills();
        assert!(skills.is_empty());

        // Prompts
        let mut loader2 = pi_rs::prompts::PromptLoader::new(temp.clone());
        let prompts = loader2.load_prompts();
        assert!(prompts.is_empty());

        // Extensions
        let mut loader3 = pi_rs::extensions::ExtensionLoader::new();
        loader3.add_search_path(temp.clone());
        let extensions = loader3.load_extensions();
        assert!(extensions.is_empty());
    }

    #[test]
    fn test_resource_loader_skills_dir() {
        let temp = std::env::temp_dir().join("pi_test_skills");
        std::fs::create_dir_all(&temp).ok();

        let mut loader = pi_rs::skills::SkillLoader::new(temp.clone());
        let skills = loader.load_skills();

        std::fs::remove_dir_all(&temp).ok();

        assert!(skills.is_empty());
    }

    #[test]
    fn test_resource_loader_prompts_dir() {
        let temp = std::env::temp_dir().join("pi_test_prompts");
        std::fs::create_dir_all(&temp).ok();

        let mut loader = pi_rs::prompts::PromptLoader::new(temp.clone());
        let prompts = loader.load_prompts();

        std::fs::remove_dir_all(&temp).ok();

        assert!(prompts.is_empty());
    }
}

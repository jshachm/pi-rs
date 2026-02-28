#[cfg(test)]
mod prompts_tests {
    use pi_rs::prompts::PromptLoader;
    use std::path::PathBuf;

    #[test]
    fn test_prompt_loader_create() {
        let loader = PromptLoader::new(PathBuf::from("/tmp/prompts"));
        let prompts = loader.get_prompts();
        assert!(prompts.is_empty()); // No prompts in /tmp
    }

    #[test]
    fn test_prompt_loader_get_nonexistent() {
        let loader = PromptLoader::new(PathBuf::from("/tmp/prompts"));
        let prompt = loader.get_prompt("nonexistent");
        assert!(prompt.is_none());
    }

    #[test]
    fn test_prompt_loader_categories() {
        let loader = PromptLoader::new(PathBuf::from("/tmp/prompts"));
        let categories = loader.categories();
        assert!(categories.is_empty());
    }

    #[test]
    fn test_prompt_loader_get_by_category() {
        let loader = PromptLoader::new(PathBuf::from("/tmp/prompts"));
        let prompts = loader.get_prompts_by_category("general");
        assert!(prompts.is_empty());
    }
}

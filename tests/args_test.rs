#[cfg(test)]
mod args_tests {
    use clap::Parser;
    use pi_rs::cli::args::Args;

    #[test]
    fn test_provider_flag() {
        let args = Args::parse_from(&["pi", "--provider", "openai"]);
        assert_eq!(args.provider, Some("openai".to_string()));
    }

    #[test]
    fn test_model_flag() {
        let args = Args::parse_from(&["pi", "--model", "gpt-4o"]);
        assert_eq!(args.model, Some("gpt-4o".to_string()));
    }

    #[test]
    fn test_api_key_flag() {
        let args = Args::parse_from(&["pi", "--api-key", "sk-test-key"]);
        assert_eq!(args.api_key, Some("sk-test-key".to_string()));
    }

    #[test]
    fn test_thinking_flag() {
        let args = Args::parse_from(&["pi", "--thinking", "high"]);
        assert_eq!(args.thinking, Some("high".to_string()));
    }

    #[test]
    fn test_session_flag() {
        let args = Args::parse_from(&["pi", "--session", "/path/to/session.jsonl"]);
        assert_eq!(args.session, Some("/path/to/session.jsonl".to_string()));
    }

    #[test]
    fn test_continue_flag() {
        let args = Args::parse_from(&["pi", "--continue"]);
        assert!(args.continue_session);
    }

    #[test]
    fn test_resume_flag() {
        let args = Args::parse_from(&["pi", "--resume"]);
        assert!(args.resume);
    }

    #[test]
    fn test_no_session_flag() {
        let args = Args::parse_from(&["pi", "--no-session"]);
        assert!(args.no_session);
    }

    #[test]
    fn test_list_models_flag() {
        let args = Args::parse_from(&["pi", "--list-models"]);
        assert!(args.list_models);
    }

    #[test]
    fn test_tools_flag() {
        let args = Args::parse_from(&["pi", "--tools", "read,bash,write"]);
        assert_eq!(args.tools, Some("read,bash,write".to_string()));
    }

    #[test]
    fn test_no_tools_flag() {
        let args = Args::parse_from(&["pi", "--no-tools"]);
        assert!(args.no_tools);
    }

    #[test]
    fn test_extension_flag() {
        let args = Args::parse_from(&[
            "pi",
            "--extension",
            "/path/to/ext1",
            "--extension",
            "/path/to/ext2",
        ]);
        assert_eq!(args.extensions.len(), 2);
    }

    #[test]
    fn test_no_extensions_flag() {
        let args = Args::parse_from(&["pi", "--no-extensions"]);
        assert!(args.no_extensions);
    }

    #[test]
    fn test_skill_flag() {
        let args = Args::parse_from(&[
            "pi",
            "--skill",
            "/path/to/skill1",
            "--skill",
            "/path/to/skill2",
        ]);
        assert_eq!(args.skills.len(), 2);
    }

    #[test]
    fn test_no_skills_flag() {
        let args = Args::parse_from(&["pi", "--no-skills"]);
        assert!(args.no_skills);
    }

    #[test]
    fn test_theme_flag() {
        let args = Args::parse_from(&["pi", "--theme", "dark"]);
        assert_eq!(args.theme, Some("dark".to_string()));
    }

    #[test]
    fn test_message_argument() {
        let args = Args::parse_from(&["pi", "Hello, world!"]);
        assert_eq!(args.message, "Hello, world!");
    }

    #[test]
    fn test_message_with_flags() {
        let args = Args::parse_from(&["pi", "--provider", "openai", "Hello, world!"]);
        assert_eq!(args.message, "Hello, world!");
        assert_eq!(args.provider, Some("openai".to_string()));
    }
}

#[cfg(test)]
mod system_prompt_tests {
    #[test]
    fn test_build_system_prompt_empty_tools() {
        let prompt = build_system_prompt(&[], &[], &[]);
        assert!(prompt.contains("Available tools"));
    }

    #[test]
    fn test_build_system_prompt_default_tools() {
        let prompt = build_system_prompt(&["read", "bash", "edit", "write"], &[], &[]);
        assert!(prompt.contains("read"));
        assert!(prompt.contains("bash"));
        assert!(prompt.contains("edit"));
        assert!(prompt.contains("write"));
    }

    #[test]
    fn test_build_system_prompt_with_context_files() {
        let context_files = vec![
            "/path/to/file1.md".to_string(),
            "/path/to/file2.md".to_string(),
        ];
        let prompt = build_system_prompt(&[], &context_files, &[]);
        assert!(prompt.contains("file1.md") || prompt.contains("Context"));
    }

    #[test]
    fn test_build_system_prompt_with_skills() {
        let skills = vec!["skill1".to_string(), "skill2".to_string()];
        let prompt = build_system_prompt(&[], &[], &skills);
        assert!(prompt.contains("skill") || prompt.contains("Skills"));
    }

    fn build_system_prompt(tools: &[&str], context_files: &[String], skills: &[String]) -> String {
        let mut prompt = String::new();

        prompt.push_str("You are a helpful coding assistant.\n\n");

        prompt.push_str("Available tools:\n");
        if tools.is_empty() {
            prompt.push_str("(none)\n");
        } else {
            for tool in tools {
                prompt.push_str(&format!("- {}:\n", tool));
            }
        }

        if !context_files.is_empty() {
            prompt.push_str("\nContext files:\n");
            for file in context_files {
                prompt.push_str(&format!("- {}\n", file));
            }
        }

        if !skills.is_empty() {
            prompt.push_str("\nSkills:\n");
            for skill in skills {
                prompt.push_str(&format!("- {}\n", skill));
            }
        }

        prompt
    }
}

#[cfg(test)]
mod skills_tests {
    use pi_rs::skills::SkillLoader;
    use std::path::PathBuf;

    #[test]
    fn test_skill_loader_create() {
        let loader = SkillLoader::new(PathBuf::from("/tmp/skills"));
        let skills = loader.get_skills();
        assert!(skills.is_empty()); // No skills in /tmp
    }

    #[test]
    fn test_skill_loader_get_nonexistent() {
        let loader = SkillLoader::new(PathBuf::from("/tmp/skills"));
        let skill = loader.get_skill("nonexistent");
        assert!(skill.is_none());
    }

    #[test]
    fn test_skill_loader_get_by_trigger_nonexistent() {
        let loader = SkillLoader::new(PathBuf::from("/tmp/skills"));
        let skill = loader.get_skill_by_trigger("@nonexistent");
        assert!(skill.is_none());
    }
}

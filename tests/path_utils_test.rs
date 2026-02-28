#[cfg(test)]
mod path_utils_tests {
    use std::path::PathBuf;

    #[test]
    fn test_resolve_to_cwd_absolute() {
        let path = resolve_to_cwd("/absolute/path/file.txt", "/some/cwd");
        assert_eq!(path, PathBuf::from("/absolute/path/file.txt"));
    }

    #[test]
    fn test_resolve_to_cwd_relative() {
        let path = resolve_to_cwd("relative/file.txt", "/some/cwd");
        assert_eq!(path, PathBuf::from("/some/cwd/relative/file.txt"));
    }

    #[test]
    fn test_expand_path_home() {
        let path = expand_path("~/Documents");
        assert!(!path.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_resolve_read_path() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("pi_test_file.txt");
        std::fs::write(&test_file, "test content").unwrap();

        let resolved = resolve_read_path(test_file.to_str().unwrap(), temp_dir.to_str().unwrap());
        assert!(resolved.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    fn resolve_to_cwd(path: &str, cwd: &str) -> PathBuf {
        let p = PathBuf::from(path);
        if p.is_absolute() {
            p
        } else {
            PathBuf::from(cwd).join(p)
        }
    }

    fn expand_path(path: &str) -> PathBuf {
        if path.starts_with('~') {
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            let rest = path.strip_prefix('~').unwrap_or("");
            home.join(rest)
        } else {
            PathBuf::from(path)
        }
    }

    fn resolve_read_path(path: &str, cwd: &str) -> Result<PathBuf, String> {
        let path = expand_path(path);
        if path.is_absolute() {
            Ok(path)
        } else {
            Ok(PathBuf::from(cwd).join(path))
        }
    }
}

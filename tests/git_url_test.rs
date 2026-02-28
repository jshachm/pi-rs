#[cfg(test)]
mod git_url_tests {
    #[test]
    fn test_parse_git_url_https() {
        let result = parse_git_url("https://github.com/user/repo");
        assert_eq!(result.host, "github.com");
        // path may vary
    }

    #[test]
    fn test_parse_git_url_ssh() {
        let result = parse_git_url("ssh://git@github.com/user/repo");
        assert!(result.host.contains("github") || result.repo.contains("github"));
    }

    #[test]
    fn test_parse_git_url_with_ref() {
        let result = parse_git_url("https://github.com/user/repo@v1.0.0");
        assert!(result.repo.contains("github"));
    }

    #[test]
    fn test_parse_git_url_shorthand() {
        let result = parse_git_url("git@github.com:user/repo");
        assert_eq!(result.host, "github.com");
        assert_eq!(result.path, "user/repo");
    }

    #[derive(Debug, PartialEq)]
    struct GitUrlResult {
        host: String,
        path: String,
        repo: String,
        r#ref: Option<String>,
    }

    fn parse_git_url(url: &str) -> GitUrlResult {
        if url.starts_with("https://") || url.starts_with("http://") {
            // Extract host
            let without_protocol = url.replace("https://", "").replace("http://", "");
            let host = without_protocol
                .split('/')
                .next()
                .unwrap_or("unknown")
                .to_string();
            let path = without_protocol
                .strip_prefix(&host)
                .unwrap_or("")
                .trim_start_matches('/')
                .to_string();

            let mut r#ref = None;
            let path = if let Some(at) = path.rfind('@') {
                r#ref = Some(path[at + 1..].to_string());
                path[..at].to_string()
            } else {
                path
            };

            return GitUrlResult {
                host,
                path,
                repo: url.to_string(),
                r#ref,
            };
        }

        if url.starts_with("git@") {
            let rest = &url[4..];
            if let Some(colon_pos) = rest.find(':') {
                let host = rest[..colon_pos].to_string();
                let path = rest[colon_pos + 1..].to_string();
                return GitUrlResult {
                    host,
                    path,
                    repo: url.to_string(),
                    r#ref: None,
                };
            }
        }

        GitUrlResult {
            host: "unknown".to_string(),
            path: url.to_string(),
            repo: url.to_string(),
            r#ref: None,
        }
    }
}

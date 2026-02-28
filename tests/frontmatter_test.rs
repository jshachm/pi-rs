#[cfg(test)]
mod frontmatter_tests {
    #[test]
    fn test_parse_frontmatter_simple() {
        let input = r#"---
title: Test
---
Content here"#;
        let result = parse_frontmatter(input);
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let input = "Just content without frontmatter";
        let result = parse_frontmatter(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_frontmatter_extract_content() {
        let input = r#"---
title: Test
---
Content here"#;
        let result = parse_frontmatter(input);
        assert!(result.is_some());
        let (_metadata, content) = result.unwrap();
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_parse_frontmatter_multiple_keys() {
        let input = r#"---
title: Test
author: John
date: 2024-01-01
---
Content"#;
        let result = parse_frontmatter(input);
        assert!(result.is_some());
        let (metadata, _) = result.unwrap();
        assert!(metadata.contains_key("title"));
        assert!(metadata.contains_key("author"));
    }

    fn parse_frontmatter(
        input: &str,
    ) -> Option<(std::collections::HashMap<String, String>, String)> {
        let trimmed = input.trim();
        if !trimmed.starts_with("---") {
            return None;
        }

        let mut lines = trimmed.lines();
        lines.next(); // Skip first ---

        let mut metadata = std::collections::HashMap::new();

        for line in lines.by_ref() {
            if line.trim() == "---" {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                metadata.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let content = lines.collect::<Vec<_>>().join("\n");
        Some((metadata, content))
    }
}

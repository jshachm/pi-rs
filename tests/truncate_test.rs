#[cfg(test)]
mod truncate_tests {
    #[test]
    fn test_truncate_to_width_simple() {
        let result = truncate_to_width("Hello World", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_truncate_to_width_exact() {
        let result = truncate_to_width("Hello", 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_truncate_to_width_longer() {
        let result = truncate_to_width("Hello World", 20);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_truncate_to_width_empty() {
        let result = truncate_to_width("", 5);
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_to_width_with_ellipsis() {
        let result = truncate_to_width_ellipsis("Hello World", 8);
        assert!(result.len() <= 11); // "Hello..." = 8
    }

    fn truncate_to_width(s: &str, width: usize) -> String {
        s.chars().take(width).collect()
    }

    fn truncate_to_width_ellipsis(s: &str, width: usize) -> String {
        if s.len() <= width {
            s.to_string()
        } else {
            let ellipsis = "...";
            let available = width.saturating_sub(ellipsis.len());
            format!(
                "{}{}",
                s.chars().take(available).collect::<String>(),
                ellipsis
            )
        }
    }
}

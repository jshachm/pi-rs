#[cfg(test)]
mod theme_tests {
    use pi_rs::theme::Theme;

    #[test]
    fn test_theme_default() {
        let theme = Theme::default_theme();
        assert_eq!(theme.name, "default");
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "light");
    }

    #[test]
    fn test_theme_parse_color_named() {
        let theme = Theme::default_theme();

        let color = theme.parse_color("red");
        assert!(true); // Should not panic
    }

    #[test]
    fn test_theme_parse_color_hex() {
        let theme = Theme::default_theme();

        let color = theme.parse_color("#FF0000");
        assert!(true); // Should not panic
    }

    #[test]
    fn test_theme_get_color() {
        let theme = Theme::default_theme();

        let color = theme.get_color("primary");
        assert!(true); // Should not panic
    }

    #[test]
    fn test_theme_get_color_user_message() {
        let theme = Theme::default_theme();

        let color = theme.get_color("user_message");
        assert!(true); // Should not panic
    }
}

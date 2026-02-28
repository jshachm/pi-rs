#[cfg(test)]
mod package_command_tests {
    #[test]
    fn test_parse_package_command_npm() {
        let result = parse_package_command("npm install lodash");
        assert!(result.is_some());
        let (_cmd, args) = result.unwrap();
        assert!(args.contains(&"install".to_string()));
    }

    #[test]
    fn test_parse_package_command_yarn() {
        let result = parse_package_command("yarn add lodash");
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_package_command_pnpm() {
        let result = parse_package_command("pnpm install lodash");
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_package_command_invalid() {
        let result = parse_package_command("not-a-package-command");
        assert!(result.is_none());
    }

    fn parse_package_command(input: &str) -> Option<(&str, Vec<String>)> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let cmd = parts[0];
        let valid_commands = ["npm", "yarn", "pnpm", "bun", "deno"];

        if !valid_commands.contains(&cmd) {
            return None;
        }

        Some((cmd, parts[1..].iter().map(|s| s.to_string()).collect()))
    }
}

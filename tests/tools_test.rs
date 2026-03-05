#[cfg(test)]
mod tools_tests {
    use pi_rs::tools::{bash_tool, coding_tools, epkg_tool, ls_tool, read_tool, write_tool};

    #[test]
    fn test_read_tool_name() {
        let tool = read_tool();
        assert_eq!(tool.name, "read");
    }

    #[test]
    fn test_write_tool_name() {
        let tool = write_tool();
        assert_eq!(tool.name, "write");
    }

    #[test]
    fn test_bash_tool_name() {
        let tool = bash_tool();
        assert_eq!(tool.name, "bash");
    }

    #[test]
    fn test_ls_tool_name() {
        let tool = ls_tool();
        assert_eq!(tool.name, "ls");
    }

    #[test]
    fn test_epkg_tool_name() {
        let tool = epkg_tool();
        assert_eq!(tool.name, "epkg");
    }

    #[test]
    fn test_epkg_tool_schema() {
        let tool = epkg_tool();
        assert_eq!(tool.schema.r#type, "object");
        assert!(tool.schema.required.contains(&"command".to_string()));
    }

    #[test]
    fn test_tool_schemas() {
        let tool = read_tool();
        assert!(!tool.schema.properties.is_null());
    }

    #[test]
    fn test_coding_tools() {
        let tools = coding_tools();
        assert!(!tools.is_empty());

        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"read"));
        assert!(names.contains(&"write"));
        assert!(names.contains(&"edit"));
        assert!(names.contains(&"bash"));
    }
}

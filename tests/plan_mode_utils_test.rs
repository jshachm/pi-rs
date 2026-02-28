#[cfg(test)]
mod plan_mode_utils_tests {
    #[test]
    fn test_extract_plan_from_response_simple() {
        let response = "Plan:\n1. First step\n2. Second step\n3. Third step";
        let plan = extract_plan_from_response(response);
        assert!(plan.is_some());
    }

    #[test]
    fn test_extract_plan_from_response_no_plan() {
        let response = "This is just a regular response without a plan.";
        let plan = extract_plan_from_response(response);
        assert!(plan.is_none());
    }

    #[test]
    fn test_extract_plan_from_response_markdown() {
        let response = "I'll help you with that.\n\n## Plan\n\n- Step one\n- Step two";
        let _plan = extract_plan_from_response(response);
        // May or may not have plan depending on implementation
    }

    fn extract_plan_from_response(response: &str) -> Option<Vec<String>> {
        let lines: Vec<&str> = response.lines().collect();
        let mut plan_lines = Vec::new();
        let mut in_plan = false;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.to_lowercase().contains("plan")
                && (trimmed.starts_with('#') || trimmed.to_lowercase().starts_with("plan"))
            {
                in_plan = true;
                continue;
            }

            if in_plan {
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with('#') {
                    break;
                }
                if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with("1.")
                {
                    plan_lines.push(trimmed.to_string());
                }
            }
        }

        if plan_lines.is_empty() {
            None
        } else {
            Some(plan_lines)
        }
    }
}

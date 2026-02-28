#[cfg(test)]
mod rpc_tests {
    #[test]
    fn test_rpc_format_request() {
        let request = format_rpc_request("method", serde_json::json!({"param": "value"}));
        assert!(request.contains("method"));
        assert!(request.contains("jsonrpc"));
    }

    #[test]
    fn test_rpc_format_response_success() {
        let response = format_rpc_response(1, serde_json::json!({"result": "ok"}), None);
        assert!(response.contains("result"));
    }

    #[test]
    fn test_rpc_format_response_error() {
        let response = format_rpc_response(1, serde_json::Value::Null, Some(-32600));
        assert!(response.contains("error"));
    }

    #[test]
    fn test_rpc_parse_response() {
        let json = r#"{"jsonrpc": "2.0", "id": 1, "result": "ok"}"#;
        let parsed = parse_rpc_response(json);
        assert!(parsed.is_ok());
    }

    fn format_rpc_request(method: &str, params: serde_json::Value) -> String {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        })
        .to_string()
    }

    fn format_rpc_response(id: u64, result: serde_json::Value, error: Option<i32>) -> String {
        if let Some(code) = error {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": code,
                    "message": "Error"
                }
            })
            .to_string()
        } else {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": result
            })
            .to_string()
        }
    }

    fn parse_rpc_response(json: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

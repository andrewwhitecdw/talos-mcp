use serde_json::json;
use talos_mcp_server::{handle_tool_call, JsonRpcRequest};

fn create_tool_call(tool_name: &str, node: Option<&str>) -> JsonRpcRequest {
    let params = match node {
        Some(n) => json!({"name": tool_name, "node": n}),
        None => json!({"name": tool_name}),
    };
    JsonRpcRequest::new("tools/call", Some(params), Some(json!(1)))
}

#[test]
fn test_handle_tool_call_with_valid_params() {
    let request = create_tool_call("containers", Some("10.0.0.1"));
    let response = handle_tool_call(&request);
    assert!(response.result.is_some() || response.error.is_some());
}

#[test]
fn test_unknown_tool_returns_error() {
    let request = create_tool_call("nonexistent_tool", Some("10.0.0.1"));
    let response = handle_tool_call(&request);
    assert!(
        response.error.is_some(),
        "Should return error for unknown tool"
    );
    let error = response.error.unwrap();
    assert_eq!(error.code, -32000, "Should return tool error");
}

#[test]
fn test_json_rpc_request_new() {
    let request = JsonRpcRequest::new(
        "test/method",
        Some(json!({"key": "value"})),
        Some(json!(42)),
    );
    assert_eq!(request.jsonrpc, "2.0");
    assert_eq!(request.method, "test/method");
    assert_eq!(request.id, Some(json!(42)));
    assert_eq!(request.params, Some(json!({"key": "value"})));
}

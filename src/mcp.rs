//! MCP Protocol types for JSON-RPC 2.0 communication

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

impl JsonRpcRequest {
    #[allow(unused)]
    pub fn new(
        method: &str,
        params: Option<serde_json::Value>,
        id: Option<serde_json::Value>,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<serde_json::Value>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    pub fn parse_error() -> Self {
        Self {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        }
    }

    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: -32601,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }

    pub fn invalid_params(message: &str) -> Self {
        Self {
            code: -32602,
            message: format!("Invalid params: {}", message),
            data: None,
        }
    }

    pub fn tool_error(message: &str) -> Self {
        Self {
            code: -32000,
            message: message.to_string(),
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_deserialize() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"test","params":{"foo":"bar"}}"#;
        let req: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, Some(serde_json::json!(1)));
        assert_eq!(req.method, "test");
        assert!(req.params.is_some());
    }

    #[test]
    fn test_json_rpc_request_no_params() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"test"}"#;
        let req: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.params, None);
    }

    #[test]
    fn test_json_rpc_response_success() {
        let result = serde_json::json!({"status": "ok"});
        let resp = JsonRpcResponse::success(Some(serde_json::json!(1)), result.clone());
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, Some(serde_json::json!(1)));
        assert_eq!(resp.result, Some(result));
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_json_rpc_response_error() {
        let error = JsonRpcError::method_not_found("unknown");
        let resp = JsonRpcResponse::error(Some(serde_json::json!(1)), error.clone());
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, Some(serde_json::json!(1)));
        assert!(resp.result.is_none());
        assert_eq!(resp.error, Some(error));
    }

    #[test]
    fn test_json_rpc_response_serialize() {
        let result = serde_json::json!({"status": "ok"});
        let resp = JsonRpcResponse::success(Some(serde_json::json!(1)), result);
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"result\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_json_rpc_error_codes() {
        let e = JsonRpcError::parse_error();
        assert_eq!(e.code, -32700);
        assert_eq!(e.message, "Parse error");

        let e = JsonRpcError::method_not_found("foo");
        assert_eq!(e.code, -32601);
        assert!(e.message.contains("foo"));

        let e = JsonRpcError::invalid_params("missing field");
        assert_eq!(e.code, -32602);
        assert!(e.message.contains("missing field"));

        let e = JsonRpcError::tool_error("something went wrong");
        assert_eq!(e.code, -32000);
        assert_eq!(e.message, "something went wrong");
    }

    #[test]
    fn test_json_rpc_error_serialize_no_data() {
        let e = JsonRpcError::parse_error();
        let json = serde_json::to_string(&e).unwrap();
        assert!(!json.contains("\"data\""));
    }
}

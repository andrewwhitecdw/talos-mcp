//! Talos MCP Server - Model Context Protocol server for Talos OS cluster management

mod mcp;
mod tools;
mod handlers;

use anyhow::Result;
use std::io::{self, BufRead, Write};
use tracing::{info, error, debug};
use tracing_subscriber::EnvFilter;

use crate::mcp::{JsonRpcRequest, JsonRpcResponse, JsonRpcError};
use crate::tools::get_all_tool_schemas;
use crate::handlers::handle_tool_call;

const SERVER_NAME: &str = "talos-mcp";
const SERVER_VERSION: &str = "1.0.0";

fn main() -> Result<()> {
    // Initialize logging to stderr (stdout is for MCP communication)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .with_writer(io::stderr)
        .init();

    info!("Starting Talos MCP Server v{}", SERVER_VERSION);
    
    // Check for TALOSCONFIG
    if std::env::var("TALOSCONFIG").is_err() {
        error!("TALOSCONFIG environment variable not set");
        eprintln!("Warning: TALOSCONFIG environment variable not set. Some commands may fail.");
    }

    run_server()
}

fn run_server() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    info!("Server running, waiting for requests on stdin...");

    for line in stdin.lock().lines() {
        let line = line?;
        
        if line.is_empty() {
            continue;
        }

        debug!("Received: {}", line);

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to parse request: {}", e);
                let error_response = JsonRpcResponse::error(
                    None,
                    JsonRpcError::parse_error(),
                );
                send_response(&mut stdout_lock, &error_response)?;
                continue;
            }
        };

        let response = handle_request(&request);
        send_response(&mut stdout_lock, &response)?;
    }

    info!("Server shutting down");
    Ok(())
}

fn send_response<W: Write>(writer: &mut W, response: &JsonRpcResponse) -> Result<()> {
    let json = serde_json::to_string(response)?;
    debug!("Sending: {}", json);
    writeln!(writer, "{}", json)?;
    writer.flush()?;
    Ok(())
}

fn handle_request(request: &JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => handle_initialize(request),
        "initialized" => JsonRpcResponse::success(request.id.clone(), serde_json::Value::Null),
        "tools/list" => handle_tools_list(request),
        "tools/call" => handle_tool_call(request),
        "ping" => JsonRpcResponse::success(request.id.clone(), serde_json::json!({})),
        _ => JsonRpcResponse::error(
            request.id.clone(),
            JsonRpcError::method_not_found(&request.method),
        ),
    }
}

fn handle_initialize(request: &JsonRpcRequest) -> JsonRpcResponse {
    let result = serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {
                "listChanged": false
            }
        },
        "serverInfo": {
            "name": SERVER_NAME,
            "version": SERVER_VERSION
        }
    });
    
    JsonRpcResponse::success(request.id.clone(), result)
}

fn handle_tools_list(request: &JsonRpcRequest) -> JsonRpcResponse {
    let tools = get_all_tool_schemas();
    let result = serde_json::json!({
        "tools": tools
    });
    
    JsonRpcResponse::success(request.id.clone(), result)
}

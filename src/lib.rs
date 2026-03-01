//! MCP server for Talos OS cluster management
//!
//! This library provides an MCP (Model Context Protocol) server that allows
//! AI assistants to interact with Talos OS clusters via talosctl commands.

pub mod handlers;
pub mod mcp;
pub mod tools;

pub use handlers::handle_tool_call;
pub use mcp::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

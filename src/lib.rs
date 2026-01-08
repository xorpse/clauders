//! Rust bindings for the Claude Code CLI.
//!
//! This crate provides a type-safe interface to interact with the Claude Code CLI,
//! enabling programmatic interaction with Claude through JSON streaming.
//!
//! # Features
//!
//! - Bidirectional message streaming
//! - Custom tool definitions via in-process MCP servers
//! - Hook-based interception of tool execution
//! - Type-safe message handling with comprehensive error types
//!
//! # Example
//!
//! ```no_run
//! use claudio::{Client, Options};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), claudio::Error> {
//!     let client = Client::new(Options::new()).await?;
//!     client.query("Hello, Claude!").await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod handler;
pub mod hooks;
pub mod mcp_server;
pub mod model;
pub mod options;
pub mod permissions;
pub mod proto;
pub mod response;
pub mod tool;
pub mod tool_input;
pub mod transport;
mod util;

pub use client::Client;
pub use error::Error;
pub use handler::{DefaultHandler, Handler, dispatch};
pub use hooks::Hooks;
pub use mcp_server::McpServer;
pub use model::Model;
pub use options::Options;
pub use permissions::{
    Callback as PermissionCallback, Decision, PermissionContext, PermissionMode, PermissionRule,
};
pub use proto::message::{AssistantError, Usage};
pub use response::{
    CompleteResponse, ErrorResponse, InitResponse, Response, Responses, TextResponse,
    ThinkingResponse, ToolResultResponse, ToolUseResponse,
};
pub use tool::{Tool, ToolError};
pub use tool_input::ToolInput;

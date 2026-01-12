use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Claude Code not found: {0}")]
    CliNotFound(String),
    #[error("connection error: {0}")]
    ConnectionError(String),
    #[error("control error (request_id={request_id}): {message}")]
    ControlError { request_id: String, message: String },
    #[error("hook error (callback_id={callback_id}): {message}")]
    HookError {
        callback_id: String,
        message: String,
    },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("no output schema configured; use Options::with_json_schema::<T>() when creating the client")]
    NoSchemaConfigured,
    #[error("permission denied for tool '{tool_name}': {message}")]
    PermissionDenied { tool_name: String, message: String },
    #[error("process error: {0}")]
    ProcessError(String),
    #[error("protocol error: {0}")]
    ProtocolError(String),
    #[error("schema mismatch: configured schema does not match requested type")]
    SchemaMismatch { expected: String, configured: String },
    #[error("timeout: {0}")]
    Timeout(String),
}

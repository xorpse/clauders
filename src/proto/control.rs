use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Control protocol request types.
///
/// These match the Python SDK's SDKControl*Request types exactly.
/// All field names use snake_case to match the CLI wire format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum Request {
    Interrupt,
    #[serde(rename = "can_use_tool")]
    CanUseTool(PermissionRequest),
    Initialize(InitializeRequest),
    SetPermissionMode(SetPermissionModeRequest),
    HookCallback(HookCallbackRequest),
    McpMessage(McpMessageRequest),
    SetModel(SetModelRequest),
    GetServerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    tool_name: String,
    input: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_suggestions: Option<Vec<PermissionUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_path: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl PermissionRequest {
    pub fn new(tool_name: impl Into<String>, input: Value) -> Self {
        Self {
            tool_name: tool_name.into(),
            input,
            permission_suggestions: None,
            blocked_path: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn permission_suggestions(&self) -> Option<&[PermissionUpdate]> {
        self.permission_suggestions.as_deref()
    }

    pub fn blocked_path(&self) -> Option<&str> {
        self.blocked_path.as_deref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_tool_name(&mut self, tool_name: impl Into<String>) {
        self.tool_name = tool_name.into();
    }

    pub fn set_input(&mut self, input: Value) {
        self.input = input;
    }

    pub fn set_permission_suggestions(&mut self, suggestions: Option<Vec<PermissionUpdate>>) {
        self.permission_suggestions = suggestions;
    }

    pub fn set_blocked_path(&mut self, path: Option<String>) {
        self.blocked_path = path;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.set_tool_name(tool_name);
        self
    }

    pub fn with_input(mut self, input: Value) -> Self {
        self.set_input(input);
        self
    }

    pub fn with_permission_suggestions(mut self, suggestions: Vec<PermissionUpdate>) -> Self {
        self.set_permission_suggestions(Some(suggestions));
        self
    }

    pub fn with_blocked_path(mut self, path: impl Into<String>) -> Self {
        self.set_blocked_path(Some(path.into()));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionUpdate {
    tool_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl PermissionUpdate {
    pub fn new(tool_name: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            rule: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_tool_name(&mut self, tool_name: impl Into<String>) {
        self.tool_name = tool_name.into();
    }

    pub fn set_rule(&mut self, rule: Option<String>) {
        self.rule = rule;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.set_tool_name(tool_name);
        self
    }

    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.set_rule(Some(rule.into()));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    hooks: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl InitializeRequest {
    pub fn new() -> Self {
        Self {
            hooks: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn hooks(&self) -> Option<&HashMap<String, Value>> {
        self.hooks.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_hooks(&mut self, hooks: Option<HashMap<String, Value>>) {
        self.hooks = hooks;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_hooks(mut self, hooks: HashMap<String, Value>) -> Self {
        self.set_hooks(Some(hooks));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl Default for InitializeRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPermissionModeRequest {
    mode: PermissionMode,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl SetPermissionModeRequest {
    pub fn new(mode: PermissionMode) -> Self {
        Self {
            mode,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn mode(&self) -> PermissionMode {
        self.mode
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_mode(&mut self, mode: PermissionMode) {
        self.mode = mode;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_mode(mut self, mode: PermissionMode) -> Self {
        self.set_mode(mode);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

/// Permission mode values use camelCase as sent by CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    Default,
    AcceptEdits,
    Plan,
    BypassPermissions,
}

impl std::fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PermissionMode::Default => "default",
            PermissionMode::AcceptEdits => "acceptEdits",
            PermissionMode::Plan => "plan",
            PermissionMode::BypassPermissions => "bypassPermissions",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCallbackRequest {
    callback_id: String,
    input: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_use_id: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl HookCallbackRequest {
    pub fn new(callback_id: impl Into<String>, input: Value) -> Self {
        Self {
            callback_id: callback_id.into(),
            input,
            tool_use_id: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn callback_id(&self) -> &str {
        &self.callback_id
    }

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn tool_use_id(&self) -> Option<&str> {
        self.tool_use_id.as_deref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_callback_id(&mut self, callback_id: impl Into<String>) {
        self.callback_id = callback_id.into();
    }

    pub fn set_input(&mut self, input: Value) {
        self.input = input;
    }

    pub fn set_tool_use_id(&mut self, tool_use_id: Option<String>) {
        self.tool_use_id = tool_use_id;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_callback_id(mut self, callback_id: impl Into<String>) -> Self {
        self.set_callback_id(callback_id);
        self
    }

    pub fn with_input(mut self, input: Value) -> Self {
        self.set_input(input);
        self
    }

    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.set_tool_use_id(Some(tool_use_id.into()));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMessageRequest {
    server_name: String,
    message: Value,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl McpMessageRequest {
    pub fn new(server_name: impl Into<String>, message: Value) -> Self {
        Self {
            server_name: server_name.into(),
            message,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    pub fn message(&self) -> &Value {
        &self.message
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_server_name(&mut self, server_name: impl Into<String>) {
        self.server_name = server_name.into();
    }

    pub fn set_message(&mut self, message: Value) {
        self.message = message;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_server_name(mut self, server_name: impl Into<String>) -> Self {
        self.set_server_name(server_name);
        self
    }

    pub fn with_message(mut self, message: Value) -> Self {
        self.set_message(message);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetModelRequest {
    model: String,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl SetModelRequest {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            extra: Map::new(),
        }
    }

    // Getters
    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.set_model(model);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

/// Control protocol response types.
///
/// Response subtype uses snake_case: "success" or "error".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum Response {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

/// Success response - all fields use snake_case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Value>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl SuccessResponse {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            response: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn response(&self) -> Option<&Value> {
        self.response.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_request_id(&mut self, request_id: impl Into<String>) {
        self.request_id = request_id.into();
    }

    pub fn set_response(&mut self, response: Option<Value>) {
        self.response = response;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.set_request_id(request_id);
        self
    }

    pub fn with_response(mut self, response: Value) -> Self {
        self.set_response(Some(response));
        self
    }

    pub fn with_response_opt(mut self, response: Option<Value>) -> Self {
        self.set_response(response);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

/// Error response - all fields use snake_case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    request_id: String,
    error: ErrorDetail,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ErrorResponse {
    pub fn new(request_id: impl Into<String>, error: ErrorDetail) -> Self {
        Self {
            request_id: request_id.into(),
            error,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn error(&self) -> &ErrorDetail {
        &self.error
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_request_id(&mut self, request_id: impl Into<String>) {
        self.request_id = request_id.into();
    }

    pub fn set_error(&mut self, error: ErrorDetail) {
        self.error = error;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.set_request_id(request_id);
        self
    }

    pub fn with_error(mut self, error: ErrorDetail) -> Self {
        self.set_error(error);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl ErrorDetail {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    // Getters
    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }

    // Setters
    pub fn set_code(&mut self, code: i32) {
        self.code = code;
    }

    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    pub fn set_data(&mut self, data: Option<Value>) {
        self.data = data;
    }

    // Builders
    pub fn with_code(mut self, code: i32) -> Self {
        self.set_code(code);
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.set_message(message);
        self
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.set_data(Some(data));
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    Custom(i32),
}

impl ErrorCode {
    pub fn to_i32(self) -> i32 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::Custom(n) => n,
        }
    }

    pub fn from_i32(code: i32) -> Self {
        match code {
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            n => Self::Custom(n),
        }
    }
}

/// Outgoing control request envelope (SDK → CLI).
///
/// Structure matches Python SDK exactly:
/// ```json
/// {
///   "type": "control_request",
///   "request_id": "...",
///   "request": { "subtype": "...", ... }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEnvelope {
    #[serde(rename = "type")]
    msg_type: String,
    request_id: String,
    request: Request,
}

/// Control response envelope (SDK ↔ CLI).
///
/// Structure matches Python SDK exactly:
/// ```json
/// {
///   "type": "control_response",
///   "response": { "subtype": "success", "request_id": "...", "response": {...} }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEnvelope {
    #[serde(rename = "type")]
    msg_type: String,
    response: Response,
}

impl RequestEnvelope {
    pub fn new(request: Request) -> Self {
        Self::new_with(uuid::Uuid::now_v7(), request)
    }

    pub fn new_with(request_id: impl std::fmt::Display, request: Request) -> Self {
        Self {
            msg_type: "control_request".to_owned(),
            request_id: request_id.to_string(),
            request,
        }
    }

    pub fn interrupt(request_id: impl std::fmt::Display) -> Self {
        Self::new_with(request_id, Request::Interrupt)
    }

    // Getters
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn request(&self) -> &Request {
        &self.request
    }

    // Setters
    pub fn set_msg_type(&mut self, msg_type: impl Into<String>) {
        self.msg_type = msg_type.into();
    }

    pub fn set_request_id(&mut self, request_id: impl Into<String>) {
        self.request_id = request_id.into();
    }

    pub fn set_request(&mut self, request: Request) {
        self.request = request;
    }

    // Builders
    pub fn with_msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.set_msg_type(msg_type);
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.set_request_id(request_id);
        self
    }

    pub fn with_request(mut self, request: Request) -> Self {
        self.set_request(request);
        self
    }
}

impl ResponseEnvelope {
    pub fn success(request_id: impl Into<String>, response: Option<Value>) -> Self {
        Self {
            msg_type: "control_response".to_owned(),
            response: Response::Success(
                SuccessResponse::new(request_id).with_response_opt(response),
            ),
        }
    }

    pub fn error(
        request_id: impl Into<String>,
        code: ErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            msg_type: "control_response".to_owned(),
            response: Response::Error(ErrorResponse::new(
                request_id,
                ErrorDetail::new(code.to_i32(), message),
            )),
        }
    }

    // Getters
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    // Setters
    pub fn set_msg_type(&mut self, msg_type: impl Into<String>) {
        self.msg_type = msg_type.into();
    }

    pub fn set_response(&mut self, response: Response) {
        self.response = response;
    }

    // Builders
    pub fn with_msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.set_msg_type(msg_type);
        self
    }

    pub fn with_response(mut self, response: Response) -> Self {
        self.set_response(response);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    version: String,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default)]
    commands: Vec<String>,
    #[serde(default, rename = "outputStyles")]
    output_styles: Vec<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ServerInfo {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            output_styles: Vec::new(),
            extra: Map::new(),
        }
    }

    // Getters
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    pub fn commands(&self) -> &[String] {
        &self.commands
    }

    pub fn output_styles(&self) -> &[String] {
        &self.output_styles
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_version(&mut self, version: impl Into<String>) {
        self.version = version.into();
    }

    pub fn set_capabilities(&mut self, capabilities: Vec<String>) {
        self.capabilities = capabilities;
    }

    pub fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
    }

    pub fn set_output_styles(&mut self, output_styles: Vec<String>) {
        self.output_styles = output_styles;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.set_version(version);
        self
    }

    pub fn with_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.set_capabilities(capabilities);
        self
    }

    pub fn with_commands(mut self, commands: Vec<String>) -> Self {
        self.set_commands(commands);
        self
    }

    pub fn with_output_styles(mut self, output_styles: Vec<String>) -> Self {
        self.set_output_styles(output_styles);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

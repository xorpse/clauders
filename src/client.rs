//! Client for interacting with the Claude Code CLI.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_stream::stream;
use futures::StreamExt;
use serde_json::{Value, json};
use tokio::sync::{Mutex, RwLock};
use tokio_stream::Stream;
use tracing::{debug, info, warn};

use crate::error::Error;
use crate::hooks::{Hooks, PostToolUseInput, PreToolUseInput, StopInput, UserPromptSubmitInput};
use crate::mcp_server::McpServer;
use crate::options::Options;
use crate::proto::{
    ContentBlock, Incoming, Message, OutgoingUserMessage, RequestEnvelope, UserContent,
    control::{HookCallbackRequest, Request, ResponseEnvelope},
};
use crate::response::Response;
use crate::transport::Transport;

/// Tracks which hook type and index a callback ID maps to.
#[derive(Debug, Clone)]
enum HookCallbackEntry {
    PreToolUse(usize),
    PostToolUse(usize),
    UserPromptSubmit(usize),
    Stop(usize),
}

/// Client for interacting with the Claude Code CLI.
///
/// Manages a subprocess running the Claude CLI and provides methods for
/// sending queries and receiving streaming responses.
///
/// # Example
///
/// ```no_run
/// use clauders::{Client, Options};
///
/// #[tokio::main]
/// async fn main() -> Result<(), clauders::Error> {
///     let client = Client::new(Options::new()).await?;
///     client.query("Hello, Claude!").await?;
///     Ok(())
/// }
/// ```
pub struct Client {
    transport: Mutex<Transport>,
    session_id: RwLock<Option<String>>,
    responded_tool_ids: Mutex<HashSet<String>>,
    mcp_servers: HashMap<String, Arc<McpServer>>,
    hooks: Option<Hooks>,
    hook_callbacks: HashMap<String, HookCallbackEntry>,
}

impl Client {
    /// Creates a new client with the given options.
    ///
    /// Spawns a Claude CLI subprocess and establishes communication channels.
    /// Sends an initialize control request to enable SDK MCP servers.
    pub async fn new(mut options: Options) -> Result<Self, Error> {
        let transport_options = options.to_transport_options();
        let transport = Transport::new(&transport_options).await?;

        let mcp_servers = options.mcp_servers().clone();
        let hooks = options.take_hooks();

        // Build hook callback map
        let hook_callbacks = Self::build_hook_callbacks(&hooks);

        let client = Self {
            transport: Mutex::new(transport),
            session_id: RwLock::new(None),
            responded_tool_ids: Mutex::new(HashSet::new()),
            mcp_servers,
            hooks,
            hook_callbacks,
        };

        // Send initialize control request to enable control protocol
        client.initialize().await?;

        Ok(client)
    }

    /// Builds a mapping from callback IDs to hook entries.
    fn build_hook_callbacks(hooks: &Option<Hooks>) -> HashMap<String, HookCallbackEntry> {
        let mut callbacks = HashMap::new();
        let Some(hooks) = hooks else {
            return callbacks;
        };

        let mut id = 0;

        for (idx, _) in hooks.pre_tool_use_hooks().enumerate() {
            callbacks.insert(format!("hook_{id}"), HookCallbackEntry::PreToolUse(idx));
            id += 1;
        }

        for (idx, _) in hooks.post_tool_use_hooks().enumerate() {
            callbacks.insert(format!("hook_{id}"), HookCallbackEntry::PostToolUse(idx));
            id += 1;
        }

        for (idx, _) in hooks.user_prompt_submit_hooks().enumerate() {
            callbacks.insert(
                format!("hook_{id}"),
                HookCallbackEntry::UserPromptSubmit(idx),
            );
            id += 1;
        }

        for (idx, _) in hooks.stop_hooks().enumerate() {
            callbacks.insert(format!("hook_{id}"), HookCallbackEntry::Stop(idx));
            id += 1;
        }

        callbacks
    }

    /// Sends the initialize control request to enable SDK features.
    async fn initialize(&self) -> Result<(), Error> {
        let hook_callback_ids = self.build_hook_callback_ids();

        let init_request = if hook_callback_ids.is_null() {
            crate::proto::control::InitializeRequest::new()
        } else {
            crate::proto::control::InitializeRequest::new().with_hooks(
                std::iter::once(("hookCallbackIds".to_owned(), hook_callback_ids)).collect(),
            )
        };

        let request = crate::proto::Request::Initialize(init_request);
        let envelope = RequestEnvelope::new(request);
        self.transport.lock().await.send_request(&envelope).await?;
        debug!("sent initialize control request");
        Ok(())
    }

    fn build_hook_callback_ids(&self) -> Value {
        let Some(hooks) = &self.hooks else {
            return Value::Null;
        };

        let mut result = json!({});

        // PreToolUse: [{ "matcher": "pattern", "callbackIds": ["hook_0"] }, ...]
        if hooks.has_pre_tool_use_hooks() {
            let mut pre_tool_use = Vec::new();
            for (id, (pattern, _)) in hooks.pre_tool_use_hooks().enumerate() {
                pre_tool_use.push(json!({
                    "matcher": pattern,
                    "callbackIds": [format!("hook_{id}")]
                }));
            }
            result["PreToolUse"] = json!(pre_tool_use);
        }

        // PostToolUse: [{ "matcher": "pattern", "callbackIds": ["hook_N"] }, ...]
        if hooks.has_post_tool_use_hooks() {
            let mut post_tool_use = Vec::new();
            let base_id = hooks.pre_tool_use_hooks().len();
            for (idx, (pattern, _)) in hooks.post_tool_use_hooks().enumerate() {
                post_tool_use.push(json!({
                    "matcher": pattern,
                    "callbackIds": [format!("hook_{}", base_id + idx)]
                }));
            }
            result["PostToolUse"] = json!(post_tool_use);
        }

        // UserPromptSubmit: ["hook_N", ...]
        if hooks.has_user_prompt_submit_hooks() {
            let base_id = hooks.pre_tool_use_hooks().len() + hooks.post_tool_use_hooks().len();
            let ids = (0..hooks.user_prompt_submit_hooks().len())
                .map(|i| format!("hook_{}", base_id + i))
                .collect::<Vec<_>>();
            result["UserPromptSubmit"] = json!(ids);
        }

        // Stop: ["hook_N", ...]
        if hooks.has_stop_hooks() {
            let base_id = hooks.pre_tool_use_hooks().len()
                + hooks.post_tool_use_hooks().len()
                + hooks.user_prompt_submit_hooks().len();
            let ids = (0..hooks.stop_hooks().len())
                .map(|i| format!("hook_{}", base_id + i))
                .collect::<Vec<_>>();
            result["Stop"] = json!(ids);
        }

        result
    }

    /// Returns the current session ID, if one has been established.
    pub async fn session_id(&self) -> Option<String> {
        self.session_id.read().await.clone()
    }

    /// Sends a text query to Claude.
    pub async fn query(&self, prompt: &str) -> Result<(), Error> {
        let msg = OutgoingUserMessage::text(prompt);
        let json = serde_json::to_value(&msg)?;
        self.transport.lock().await.send(&json).await
    }

    /// Sends a message with structured content to Claude.
    pub async fn send_message(&self, content: UserContent) -> Result<(), Error> {
        let msg = OutgoingUserMessage::new(content);
        let json = serde_json::to_value(&msg)?;
        self.transport.lock().await.send(&json).await
    }

    /// Responds to a tool use request from Claude.
    ///
    /// Each tool use ID can only be responded to once; subsequent calls are ignored.
    pub async fn respond_to_tool(
        &self,
        tool_use_id: &str,
        content: Value,
        is_error: bool,
    ) -> Result<(), Error> {
        let mut responded = self.responded_tool_ids.lock().await;
        if responded.contains(tool_use_id) {
            warn!(tool_use_id, "already responded to tool, skipping");
            return Ok(());
        }

        let tool_result = ContentBlock::ToolResult(
            crate::proto::content_block::ToolResult::new(tool_use_id)
                .with_content(content)
                .with_error(is_error),
        );

        let msg = OutgoingUserMessage::new(UserContent::Blocks(vec![tool_result]));
        let json = serde_json::to_value(&msg)?;
        self.transport.lock().await.send(&json).await?;
        responded.insert(tool_use_id.to_owned());
        Ok(())
    }

    /// Clears the set of tool IDs that have been responded to.
    pub async fn clear_tool_response_tracking(&self) {
        self.responded_tool_ids.lock().await.clear();
    }

    /// Returns a stream of responses from Claude.
    ///
    /// The stream ends when a [`Response::Complete`] is received or the connection closes.
    pub fn receive(&self) -> impl Stream<Item = Result<Response, Error>> + '_ {
        stream! {
            loop {
                let incoming = {
                    let mut transport = self.transport.lock().await;
                    transport.receive().await
                };

                match incoming {
                    Ok(Some(incoming)) => {
                        // Handle control requests
                        if let Some(ctrl) = incoming.as_control_request() {
                            let response = match ctrl.request() {
                                Request::McpMessage(mcp_req) => {
                                    self.handle_mcp_message(
                                        ctrl.request_id(),
                                        mcp_req.server_name(),
                                        mcp_req.message(),
                                    )
                                }
                                Request::HookCallback(hook_req) => {
                                    self.handle_hook_callback(ctrl.request_id(), hook_req)
                                }
                                _ => continue,
                            };
                            let mut transport = self.transport.lock().await;
                            if let Err(e) = transport.send_response(&response).await {
                                warn!(error = %e, "failed to send control response");
                            }
                            continue;
                        }

                        if let Some(msg) = incoming.to_message() {
                            if let Message::System(crate::proto::SystemMessage::Init(init)) = &msg
                                && let Some(sid) = init.session_id()
                            {
                                *self.session_id.write().await = Some(sid.to_owned());
                                debug!(session_id = %sid, "session initialized");
                            }

                            for response in Response::from_message(&msg) {
                                let is_complete = matches!(response, Response::Complete(_));
                                yield Ok(response);
                                if is_complete {
                                    return;
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        info!("stream ended (EOF)");
                        return;
                    }
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }
        }
    }

    fn handle_mcp_message(
        &self,
        request_id: &str,
        server_name: &str,
        message: &Value,
    ) -> ResponseEnvelope {
        debug!(server_name, "handling MCP message");

        match self.mcp_servers.get(server_name) {
            Some(server) => {
                let mcp_response = server.handle_json_message(message);
                // Wrap in mcp_response field as expected by Claude CLI
                let response_data = serde_json::json!({ "mcp_response": mcp_response });
                ResponseEnvelope::success(request_id, Some(response_data))
            }
            None => {
                warn!(server_name, "MCP server not found");
                let error_response = json!({
                    "mcp_response": {
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32601,
                            "message": format!("MCP server '{}' not found", server_name)
                        }
                    }
                });
                ResponseEnvelope::success(request_id, Some(error_response))
            }
        }
    }

    fn handle_hook_callback(
        &self,
        request_id: &str,
        hook_req: &HookCallbackRequest,
    ) -> ResponseEnvelope {
        let callback_id = hook_req.callback_id();
        let input = hook_req.input();

        debug!(callback_id, "handling hook callback");

        let Some(entry) = self.hook_callbacks.get(callback_id) else {
            warn!(callback_id, "hook callback not found");
            return ResponseEnvelope::success(request_id, Some(json!({})));
        };

        let Some(hooks) = &self.hooks else {
            warn!("hooks not available");
            return ResponseEnvelope::success(request_id, Some(json!({})));
        };

        let session_id = input["session_id"].as_str().unwrap_or_default();
        let transcript_path = input["transcript_path"].as_str().unwrap_or_default();

        let response_data = match entry {
            HookCallbackEntry::PreToolUse(idx) => {
                let tool_name = input["tool_name"].as_str().unwrap_or_default();
                let tool_input = input["tool_input"].clone();

                let hook_input =
                    PreToolUseInput::new(session_id, transcript_path, tool_name, tool_input.into());

                if let Some((_, callback)) = hooks.get_pre_tool_use_hook(*idx) {
                    let output = callback(hook_input);
                    output.to_hook_response()
                } else {
                    json!({})
                }
            }
            HookCallbackEntry::PostToolUse(idx) => {
                let tool_name = input["tool_name"].as_str().unwrap_or_default();
                let tool_input = input["tool_input"].clone();
                let tool_response = input["tool_response"].clone();

                let hook_input = PostToolUseInput::new(
                    session_id,
                    transcript_path,
                    tool_name,
                    tool_input.into(),
                    tool_response,
                );

                if let Some((_, callback)) = hooks.get_post_tool_use_hook(*idx) {
                    let output = callback(hook_input);
                    output.to_hook_response()
                } else {
                    json!({})
                }
            }
            HookCallbackEntry::UserPromptSubmit(idx) => {
                let prompt = input["prompt"].as_str().unwrap_or_default();

                let hook_input = UserPromptSubmitInput::new(session_id, transcript_path, prompt);

                if let Some(callback) = hooks.user_prompt_submit_hooks().nth(*idx) {
                    let output = callback(hook_input);
                    output.to_hook_response()
                } else {
                    json!({})
                }
            }
            HookCallbackEntry::Stop(idx) => {
                let stop_hook_active = input["stop_hook_active"].as_bool().unwrap_or_default();

                let hook_input = StopInput::new(session_id, transcript_path, stop_hook_active);

                if let Some(callback) = hooks.stop_hooks().nth(*idx) {
                    let output = callback(hook_input);
                    output.to_hook_response()
                } else {
                    json!({})
                }
            }
        };

        ResponseEnvelope::success(request_id, Some(response_data))
    }

    /// Receives all responses until completion, collecting them into a vector.
    pub async fn receive_all(&self) -> Result<Vec<Response>, Error> {
        let mut responses = Vec::new();
        let mut stream = std::pin::pin!(self.receive());
        while let Some(result) = stream.next().await {
            responses.push(result?);
        }
        Ok(responses)
    }

    /// Sends an interrupt signal to stop the current operation.
    pub async fn interrupt(&self) -> Result<(), Error> {
        self.transport.lock().await.interrupt().await
    }

    /// Sets the permission mode for tool execution.
    pub async fn set_permission_mode(
        &self,
        mode: crate::proto::PermissionMode,
    ) -> Result<(), Error> {
        let request = crate::proto::Request::SetPermissionMode(
            crate::proto::control::SetPermissionModeRequest::new(mode),
        );
        let envelope = RequestEnvelope::new(request);
        self.transport.lock().await.send_request(&envelope).await
    }

    /// Sets the Claude model to use for subsequent queries.
    pub async fn set_model(&self, model: &str) -> Result<(), Error> {
        let request =
            crate::proto::Request::SetModel(crate::proto::control::SetModelRequest::new(model));
        let envelope = RequestEnvelope::new(request);
        self.transport.lock().await.send_request(&envelope).await
    }

    /// Retrieves information about the Claude Code server.
    pub async fn get_server_info(&self) -> Result<crate::proto::ServerInfo, Error> {
        let request = crate::proto::Request::GetServerInfo;
        let envelope = RequestEnvelope::new(request);

        let mut transport = self.transport.lock().await;
        transport.send_request(&envelope).await?;

        loop {
            match transport.receive().await? {
                Some(Incoming::ControlResponse(resp)) => match resp.response() {
                    crate::proto::Response::Success(success) => {
                        if let Some(data) = success.response() {
                            let info =
                                serde_json::from_value::<crate::proto::ServerInfo>(data.clone())?;
                            return Ok(info);
                        }
                        return Err(Error::ProtocolError("empty response".to_owned()));
                    }
                    crate::proto::Response::Error(err) => {
                        return Err(Error::ControlError {
                            request_id: err.request_id().to_owned(),
                            message: err.error().message().to_owned(),
                        });
                    }
                },
                Some(_) => continue,
                None => return Err(Error::ConnectionError("stream ended".to_owned())),
            }
        }
    }
}

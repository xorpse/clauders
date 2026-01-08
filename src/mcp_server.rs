use std::collections::HashMap;

use serde_json::{Value, json};

use crate::tool::Tool;
use crate::tool_input::ToolInput;

#[derive(Debug)]
pub struct McpServer {
    name: String,
    version: String,
    tools: Vec<Tool>,
    tool_map: HashMap<String, usize>,
}

impl McpServer {
    pub fn new(name: impl Into<String>, tools: Vec<Tool>) -> Self {
        Self::with_version(name, "1.0.0", tools)
    }

    pub fn with_version(
        name: impl Into<String>,
        version: impl Into<String>,
        tools: Vec<Tool>,
    ) -> Self {
        let tool_map = tools
            .iter()
            .enumerate()
            .map(|(i, t)| (t.name().to_owned(), i))
            .collect::<HashMap<_, _>>();

        Self {
            name: name.into(),
            version: version.into(),
            tools,
            tool_map,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }

    fn jsonrpc_success(id: &Value, result: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        })
    }

    fn jsonrpc_error(id: &Value, code: i32, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        })
    }

    fn handle_initialize(&self, id: &Value) -> Value {
        Self::jsonrpc_success(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": self.name,
                    "version": self.version
                }
            }),
        )
    }

    fn handle_tools_list(&self, id: &Value) -> Value {
        let tools_json = self
            .tools
            .iter()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect::<Vec<_>>();

        Self::jsonrpc_success(id, json!({ "tools": tools_json }))
    }

    fn handle_tools_call(&self, id: &Value, params: &Value) -> Value {
        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => return Self::jsonrpc_error(id, -32602, "missing 'name' parameter"),
        };

        let tool_idx = match self.tool_map.get(tool_name) {
            Some(&idx) => idx,
            None => {
                return Self::jsonrpc_error(id, -32601, &format!("tool '{}' not found", tool_name));
            }
        };

        let tool = &self.tools[tool_idx];
        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or_else(|| json!({}));
        let input = ToolInput::new(arguments);

        match tool.call(input) {
            Ok(content) => Self::jsonrpc_success(id, json!({ "content": content })),
            Err(err) => Self::jsonrpc_success(
                id,
                json!({
                    "content": [{"type": "text", "text": err.to_string()}],
                    "isError": true
                }),
            ),
        }
    }

    pub fn handle_json_message(&self, msg: &Value) -> Value {
        let method = msg
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let params = msg.get("params").cloned().unwrap_or_else(|| json!({}));
        let id = msg.get("id").cloned().unwrap_or(Value::Null);

        match method {
            "initialize" => self.handle_initialize(&id),
            "tools/list" => self.handle_tools_list(&id),
            "tools/call" => self.handle_tools_call(&id, &params),
            // Handle initialized notification - just acknowledge it
            "notifications/initialized" => json!({"jsonrpc": "2.0", "result": {}}),
            _ => Self::jsonrpc_error(&id, -32601, &format!("method '{}' not found", method)),
        }
    }
}

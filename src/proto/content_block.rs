use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(Text),
    ToolUse(ToolUse),
    ToolResult(ToolResult),
    Thinking(Thinking),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    text: String,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    id: String,
    name: String,
    input: Value,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thinking {
    thinking: String,
    signature: String,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            extra: Map::new(),
        }
    }

    // Getters
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.set_text(text);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl ToolUse {
    pub fn new(id: impl Into<String>, name: impl Into<String>, input: Value) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            input,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_id(&mut self, id: impl Into<String>) {
        self.id = id.into();
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_input(&mut self, input: Value) {
        self.input = input;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.set_id(id);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    pub fn with_input(mut self, input: Value) -> Self {
        self.set_input(input);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl ToolResult {
    pub fn new(tool_use_id: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: None,
            is_error: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn tool_use_id(&self) -> &str {
        &self.tool_use_id
    }

    pub fn content(&self) -> Option<&Value> {
        self.content.as_ref()
    }

    pub fn is_error(&self) -> Option<bool> {
        self.is_error
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_tool_use_id(&mut self, tool_use_id: impl Into<String>) {
        self.tool_use_id = tool_use_id.into();
    }

    pub fn set_content(&mut self, content: Option<Value>) {
        self.content = content;
    }

    pub fn set_is_error(&mut self, is_error: Option<bool>) {
        self.is_error = is_error;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.set_tool_use_id(tool_use_id);
        self
    }

    pub fn with_content(mut self, content: Value) -> Self {
        self.set_content(Some(content));
        self
    }

    pub fn with_error(mut self, is_error: bool) -> Self {
        self.set_is_error(Some(is_error));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl Thinking {
    pub fn new(thinking: impl Into<String>, signature: impl Into<String>) -> Self {
        Self {
            thinking: thinking.into(),
            signature: signature.into(),
            extra: Map::new(),
        }
    }

    // Getters
    pub fn thinking(&self) -> &str {
        &self.thinking
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_thinking(&mut self, thinking: impl Into<String>) {
        self.thinking = thinking.into();
    }

    pub fn set_signature(&mut self, signature: impl Into<String>) {
        self.signature = signature.into();
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_thinking(mut self, thinking: impl Into<String>) -> Self {
        self.set_thinking(thinking);
        self
    }

    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.set_signature(signature);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl ContentBlock {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(Text::new(text))
    }

    pub fn tool_use(id: impl Into<String>, name: impl Into<String>, input: Value) -> Self {
        Self::ToolUse(ToolUse::new(id, name, input))
    }

    pub fn tool_result(tool_use_id: impl Into<String>) -> Self {
        Self::ToolResult(ToolResult::new(tool_use_id))
    }

    pub fn thinking(thinking: impl Into<String>, signature: impl Into<String>) -> Self {
        Self::Thinking(Thinking::new(thinking, signature))
    }
}

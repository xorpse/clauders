use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(Text),
    ToolUse(ToolUse),
    ToolResult(ToolResult),
    Thinking(Thinking),
    Image(Image),
    Document(Document),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    source: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<Value>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    source: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    citations: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_control: Option<Value>,
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

impl Image {
    pub fn new(source: Value) -> Self {
        Self {
            source,
            cache_control: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn source(&self) -> &Value {
        &self.source
    }

    pub fn cache_control(&self) -> Option<&Value> {
        self.cache_control.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_source(&mut self, source: Value) {
        self.source = source;
    }

    pub fn set_cache_control(&mut self, cache_control: Option<Value>) {
        self.cache_control = cache_control;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_source(mut self, source: Value) -> Self {
        self.set_source(source);
        self
    }

    pub fn with_cache_control(mut self, cache_control: Value) -> Self {
        self.set_cache_control(Some(cache_control));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl Document {
    pub fn new(source: Value) -> Self {
        Self {
            source,
            title: None,
            context: None,
            citations: None,
            cache_control: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn source(&self) -> &Value {
        &self.source
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    pub fn citations(&self) -> Option<&Value> {
        self.citations.as_ref()
    }

    pub fn cache_control(&self) -> Option<&Value> {
        self.cache_control.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_source(&mut self, source: Value) {
        self.source = source;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_context(&mut self, context: Option<String>) {
        self.context = context;
    }

    pub fn set_citations(&mut self, citations: Option<Value>) {
        self.citations = citations;
    }

    pub fn set_cache_control(&mut self, cache_control: Option<Value>) {
        self.cache_control = cache_control;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_source(mut self, source: Value) -> Self {
        self.set_source(source);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.set_title(Some(title.into()));
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.set_context(Some(context.into()));
        self
    }

    pub fn with_citations(mut self, citations: Value) -> Self {
        self.set_citations(Some(citations));
        self
    }

    pub fn with_cache_control(mut self, cache_control: Value) -> Self {
        self.set_cache_control(Some(cache_control));
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

    pub fn image(source: Value) -> Self {
        Self::Image(Image::new(source))
    }

    pub fn document(source: Value) -> Self {
        Self::Document(Document::new(source))
    }
}

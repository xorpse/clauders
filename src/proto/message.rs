use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::content_block::ContentBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    User(UserEnvelope),
    Assistant(AssistantEnvelope),
    System(SystemMessage),
    Result(ResultMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEnvelope {
    message: UserMessageInner,
}

impl UserEnvelope {
    pub fn new(message: UserMessageInner) -> Self {
        Self { message }
    }

    // Getters
    pub fn message(&self) -> &UserMessageInner {
        &self.message
    }

    // Setters
    pub fn set_message(&mut self, message: UserMessageInner) {
        self.message = message;
    }

    // Builders
    pub fn with_message(mut self, message: UserMessageInner) -> Self {
        self.set_message(message);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageInner {
    content: UserContent,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl UserMessageInner {
    pub fn new(content: UserContent) -> Self {
        Self {
            content,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn content(&self) -> &UserContent {
        &self.content
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_content(&mut self, content: UserContent) {
        self.content = content;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_content(mut self, content: UserContent) -> Self {
        self.set_content(content);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEnvelope {
    message: AssistantMessageInner,
}

impl AssistantEnvelope {
    pub fn new(message: AssistantMessageInner) -> Self {
        Self { message }
    }

    // Getters
    pub fn message(&self) -> &AssistantMessageInner {
        &self.message
    }

    // Setters
    pub fn set_message(&mut self, message: AssistantMessageInner) {
        self.message = message;
    }

    // Builders
    pub fn with_message(mut self, message: AssistantMessageInner) -> Self {
        self.set_message(message);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessageInner {
    content: Vec<ContentBlock>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<AssistantError>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl AssistantMessageInner {
    pub fn new(content: Vec<ContentBlock>, model: impl Into<String>) -> Self {
        Self {
            content,
            model: model.into(),
            error: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn content(&self) -> &[ContentBlock] {
        &self.content
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn error(&self) -> Option<&AssistantError> {
        self.error.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_content(&mut self, content: Vec<ContentBlock>) {
        self.content = content;
    }

    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    pub fn set_error(&mut self, error: Option<AssistantError>) {
        self.error = error;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_content(mut self, content: Vec<ContentBlock>) -> Self {
        self.set_content(content);
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.set_model(model);
        self
    }

    pub fn with_error(mut self, error: AssistantError) -> Self {
        self.set_error(Some(error));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantError {
    AuthenticationFailed,
    BillingError,
    RateLimit,
    InvalidRequest,
    ServerError,
    Unknown,
}

impl std::fmt::Display for AssistantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AuthenticationFailed => "authentication failed",
            Self::BillingError => "billing error",
            Self::RateLimit => "rate limit exceeded",
            Self::InvalidRequest => "invalid request",
            Self::ServerError => "server error",
            Self::Unknown => "unknown error",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum SystemMessage {
    Init(InitMessage),
    Error(ErrorMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl InitMessage {
    pub fn new() -> Self {
        Self {
            session_id: None,
            model: None,
            cwd: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_session_id(&mut self, session_id: Option<String>) {
        self.session_id = session_id;
    }

    pub fn set_model(&mut self, model: Option<String>) {
        self.model = model;
    }

    pub fn set_cwd(&mut self, cwd: Option<String>) {
        self.cwd = cwd;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.set_session_id(Some(session_id.into()));
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.set_model(Some(model.into()));
        self
    }

    pub fn with_cwd(mut self, cwd: impl Into<String>) -> Self {
        self.set_cwd(Some(cwd.into()));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl Default for InitMessage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    error: String,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ErrorMessage {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            extra: Map::new(),
        }
    }

    // Getters
    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error = error.into();
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.set_error(error);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMessage {
    subtype: String,
    duration_ms: i64,
    duration_api_ms: i64,
    is_error: bool,
    num_turns: i32,
    session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_cost_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    structured_output: Option<Value>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ResultMessage {
    pub fn new(subtype: impl Into<String>, session_id: impl Into<String>) -> Self {
        Self {
            subtype: subtype.into(),
            duration_ms: 0,
            duration_api_ms: 0,
            is_error: false,
            num_turns: 0,
            session_id: session_id.into(),
            total_cost_usd: None,
            usage: None,
            result: None,
            structured_output: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn subtype(&self) -> &str {
        &self.subtype
    }

    pub fn duration_ms(&self) -> i64 {
        self.duration_ms
    }

    pub fn duration_api_ms(&self) -> i64 {
        self.duration_api_ms
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn num_turns(&self) -> i32 {
        self.num_turns
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn total_cost_usd(&self) -> Option<f64> {
        self.total_cost_usd
    }

    pub fn usage(&self) -> Option<&Usage> {
        self.usage.as_ref()
    }

    pub fn result(&self) -> Option<&str> {
        self.result.as_deref()
    }

    pub fn structured_output(&self) -> Option<&Value> {
        self.structured_output.as_ref()
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_subtype(&mut self, subtype: impl Into<String>) {
        self.subtype = subtype.into();
    }

    pub fn set_duration_ms(&mut self, duration_ms: i64) {
        self.duration_ms = duration_ms;
    }

    pub fn set_duration_api_ms(&mut self, duration_api_ms: i64) {
        self.duration_api_ms = duration_api_ms;
    }

    pub fn set_is_error(&mut self, is_error: bool) {
        self.is_error = is_error;
    }

    pub fn set_num_turns(&mut self, num_turns: i32) {
        self.num_turns = num_turns;
    }

    pub fn set_session_id(&mut self, session_id: impl Into<String>) {
        self.session_id = session_id.into();
    }

    pub fn set_total_cost_usd(&mut self, total_cost_usd: Option<f64>) {
        self.total_cost_usd = total_cost_usd;
    }

    pub fn set_usage(&mut self, usage: Option<Usage>) {
        self.usage = usage;
    }

    pub fn set_result(&mut self, result: Option<String>) {
        self.result = result;
    }

    pub fn set_structured_output(&mut self, structured_output: Option<Value>) {
        self.structured_output = structured_output;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_subtype(mut self, subtype: impl Into<String>) -> Self {
        self.set_subtype(subtype);
        self
    }

    pub fn with_duration_ms(mut self, duration_ms: i64) -> Self {
        self.set_duration_ms(duration_ms);
        self
    }

    pub fn with_duration_api_ms(mut self, duration_api_ms: i64) -> Self {
        self.set_duration_api_ms(duration_api_ms);
        self
    }

    pub fn with_is_error(mut self, is_error: bool) -> Self {
        self.set_is_error(is_error);
        self
    }

    pub fn with_num_turns(mut self, num_turns: i32) -> Self {
        self.set_num_turns(num_turns);
        self
    }

    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.set_session_id(session_id);
        self
    }

    pub fn with_total_cost_usd(mut self, total_cost_usd: f64) -> Self {
        self.set_total_cost_usd(Some(total_cost_usd));
        self
    }

    pub fn with_usage(mut self, usage: Usage) -> Self {
        self.set_usage(Some(usage));
        self
    }

    pub fn with_result(mut self, result: impl Into<String>) -> Self {
        self.set_result(Some(result.into()));
        self
    }

    pub fn with_structured_output(mut self, structured_output: Value) -> Self {
        self.set_structured_output(Some(structured_output));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_creation_input_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_read_input_tokens: Option<i64>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl Usage {
    pub fn new() -> Self {
        Self {
            input_tokens: None,
            output_tokens: None,
            total_tokens: None,
            cache_creation_input_tokens: None,
            cache_read_input_tokens: None,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn input_tokens(&self) -> Option<i64> {
        self.input_tokens
    }

    pub fn output_tokens(&self) -> Option<i64> {
        self.output_tokens
    }

    pub fn total_tokens(&self) -> Option<i64> {
        self.total_tokens
    }

    pub fn cache_creation_input_tokens(&self) -> Option<i64> {
        self.cache_creation_input_tokens
    }

    pub fn cache_read_input_tokens(&self) -> Option<i64> {
        self.cache_read_input_tokens
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    pub fn input_tokens_or(&self, default: i64) -> i64 {
        self.input_tokens.unwrap_or(default)
    }

    pub fn output_tokens_or(&self, default: i64) -> i64 {
        self.output_tokens.unwrap_or(default)
    }

    pub fn total_tokens_or(&self, default: i64) -> i64 {
        self.total_tokens.unwrap_or(default)
    }

    // Setters
    pub fn set_input_tokens(&mut self, input_tokens: Option<i64>) {
        self.input_tokens = input_tokens;
    }

    pub fn set_output_tokens(&mut self, output_tokens: Option<i64>) {
        self.output_tokens = output_tokens;
    }

    pub fn set_total_tokens(&mut self, total_tokens: Option<i64>) {
        self.total_tokens = total_tokens;
    }

    pub fn set_cache_creation_input_tokens(&mut self, tokens: Option<i64>) {
        self.cache_creation_input_tokens = tokens;
    }

    pub fn set_cache_read_input_tokens(&mut self, tokens: Option<i64>) {
        self.cache_read_input_tokens = tokens;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_input_tokens(mut self, input_tokens: i64) -> Self {
        self.set_input_tokens(Some(input_tokens));
        self
    }

    pub fn with_output_tokens(mut self, output_tokens: i64) -> Self {
        self.set_output_tokens(Some(output_tokens));
        self
    }

    pub fn with_total_tokens(mut self, total_tokens: i64) -> Self {
        self.set_total_tokens(Some(total_tokens));
        self
    }

    pub fn with_cache_creation_input_tokens(mut self, tokens: i64) -> Self {
        self.set_cache_creation_input_tokens(Some(tokens));
        self
    }

    pub fn with_cache_read_input_tokens(mut self, tokens: i64) -> Self {
        self.set_cache_read_input_tokens(Some(tokens));
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

impl Default for Usage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingUserMessage {
    #[serde(rename = "type")]
    msg_type: String,
    message: OutgoingUserInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingUserInner {
    role: String,
    content: UserContent,
}

impl OutgoingUserInner {
    pub fn new(role: impl Into<String>, content: UserContent) -> Self {
        Self {
            role: role.into(),
            content,
        }
    }

    // Getters
    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn content(&self) -> &UserContent {
        &self.content
    }

    // Setters
    pub fn set_role(&mut self, role: impl Into<String>) {
        self.role = role.into();
    }

    pub fn set_content(&mut self, content: UserContent) {
        self.content = content;
    }

    // Builders
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.set_role(role);
        self
    }

    pub fn with_content(mut self, content: UserContent) -> Self {
        self.set_content(content);
        self
    }
}

impl OutgoingUserMessage {
    pub fn new(content: UserContent) -> Self {
        Self {
            msg_type: "user".to_owned(),
            message: OutgoingUserInner::new("user", content),
        }
    }

    pub fn text(s: impl Into<String>) -> Self {
        Self::new(UserContent::Text(s.into()))
    }

    pub fn blocks(blocks: Vec<ContentBlock>) -> Self {
        Self::new(UserContent::Blocks(blocks))
    }

    // Getters
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }

    pub fn message(&self) -> &OutgoingUserInner {
        &self.message
    }

    // Setters
    pub fn set_msg_type(&mut self, msg_type: impl Into<String>) {
        self.msg_type = msg_type.into();
    }

    pub fn set_message(&mut self, message: OutgoingUserInner) {
        self.message = message;
    }

    // Builders
    pub fn with_msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.set_msg_type(msg_type);
        self
    }

    pub fn with_message(mut self, message: OutgoingUserInner) -> Self {
        self.set_message(message);
        self
    }
}

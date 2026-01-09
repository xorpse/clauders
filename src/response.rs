use std::borrow::Cow;

use serde_json::Value;

use crate::proto::Message;
use crate::proto::content_block::{
    Text as ProtoText, Thinking as ProtoThinking, ToolResult as ProtoToolResult,
    ToolUse as ProtoToolUse,
};
use crate::proto::message::{AssistantError, InitMessage, ResultMessage, SystemMessage, Usage};

#[derive(Debug, Clone)]
pub enum Response {
    Text(TextResponse),
    ToolUse(ToolUseResponse),
    ToolResult(ToolResultResponse),
    Thinking(ThinkingResponse),
    Init(InitResponse),
    Error(ErrorResponse),
    Complete(CompleteResponse),
}

#[derive(Debug, Clone)]
pub struct TextResponse(pub(crate) ProtoText);

impl TextResponse {
    pub fn content(&self) -> &str {
        self.0.text()
    }
}

#[derive(Debug, Clone)]
pub struct ToolUseResponse(pub(crate) ProtoToolUse);

impl ToolUseResponse {
    pub fn id(&self) -> &str {
        self.0.id()
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn input(&self) -> &Value {
        self.0.input()
    }
}

#[derive(Debug, Clone)]
pub struct ToolResultResponse(pub(crate) ProtoToolResult);

impl ToolResultResponse {
    pub fn tool_use_id(&self) -> &str {
        self.0.tool_use_id()
    }

    pub fn content(&self) -> Option<&Value> {
        self.0.content()
    }

    pub fn is_error(&self) -> bool {
        self.0.is_error().unwrap_or(false)
    }
}

#[derive(Debug, Clone)]
pub struct ThinkingResponse(pub(crate) ProtoThinking);

impl ThinkingResponse {
    pub fn content(&self) -> &str {
        self.0.thinking()
    }

    pub fn signature(&self) -> &str {
        self.0.signature()
    }
}

#[derive(Debug, Clone)]
pub struct InitResponse(pub(crate) InitMessage);

impl InitResponse {
    pub fn session_id(&self) -> Option<&str> {
        self.0.session_id()
    }

    pub fn model(&self) -> Option<&str> {
        self.0.model()
    }

    pub fn cwd(&self) -> Option<&str> {
        self.0.cwd()
    }
}

#[derive(Debug, Clone)]
pub enum ErrorResponse {
    System(String),
    Assistant(AssistantError),
}

impl ErrorResponse {
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            Self::System(msg) => Cow::Borrowed(msg),
            Self::Assistant(err) => Cow::Owned(err.to_string()),
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Self::System(_))
    }

    pub fn is_assistant(&self) -> bool {
        matches!(self, Self::Assistant(_))
    }

    pub fn as_system(&self) -> Option<&str> {
        match self {
            Self::System(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_assistant(&self) -> Option<&AssistantError> {
        match self {
            Self::Assistant(err) => Some(err),
            _ => None,
        }
    }

    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::Assistant(AssistantError::RateLimit))
    }

    pub fn is_authentication_failed(&self) -> bool {
        matches!(self, Self::Assistant(AssistantError::AuthenticationFailed))
    }

    pub fn is_billing_error(&self) -> bool {
        matches!(self, Self::Assistant(AssistantError::BillingError))
    }
}

#[derive(Debug, Clone)]
pub struct CompleteResponse(pub(crate) ResultMessage);

impl CompleteResponse {
    pub fn subtype(&self) -> &str {
        self.0.subtype()
    }

    pub fn duration_ms(&self) -> i64 {
        self.0.duration_ms()
    }

    pub fn duration_api_ms(&self) -> i64 {
        self.0.duration_api_ms()
    }

    pub fn num_turns(&self) -> i32 {
        self.0.num_turns()
    }

    pub fn session_id(&self) -> &str {
        self.0.session_id()
    }

    pub fn total_cost_usd(&self) -> Option<f64> {
        self.0.total_cost_usd()
    }

    pub fn usage(&self) -> Option<&Usage> {
        self.0.usage()
    }

    pub fn result_text(&self) -> Option<&str> {
        self.0.result()
    }

    pub fn structured_output(&self) -> Option<&Value> {
        self.0.structured_output()
    }

    pub fn is_error(&self) -> bool {
        self.0.is_error()
    }
}

impl Response {
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub fn is_tool_use(&self) -> bool {
        matches!(self, Self::ToolUse(_))
    }

    pub fn is_tool_result(&self) -> bool {
        matches!(self, Self::ToolResult(_))
    }

    pub fn is_thinking(&self) -> bool {
        matches!(self, Self::Thinking(_))
    }

    pub fn is_init(&self) -> bool {
        matches!(self, Self::Init(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete(_))
    }

    pub fn as_text(&self) -> Option<&TextResponse> {
        match self {
            Self::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_tool_use(&self) -> Option<&ToolUseResponse> {
        match self {
            Self::ToolUse(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_tool_result(&self) -> Option<&ToolResultResponse> {
        match self {
            Self::ToolResult(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_thinking(&self) -> Option<&ThinkingResponse> {
        match self {
            Self::Thinking(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_init(&self) -> Option<&InitResponse> {
        match self {
            Self::Init(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_error(&self) -> Option<&ErrorResponse> {
        match self {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_complete(&self) -> Option<&CompleteResponse> {
        match self {
            Self::Complete(c) => Some(c),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<TextResponse> {
        match self {
            Self::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn into_tool_use(self) -> Option<ToolUseResponse> {
        match self {
            Self::ToolUse(t) => Some(t),
            _ => None,
        }
    }

    pub fn into_tool_result(self) -> Option<ToolResultResponse> {
        match self {
            Self::ToolResult(t) => Some(t),
            _ => None,
        }
    }

    pub fn into_thinking(self) -> Option<ThinkingResponse> {
        match self {
            Self::Thinking(t) => Some(t),
            _ => None,
        }
    }

    pub fn into_init(self) -> Option<InitResponse> {
        match self {
            Self::Init(i) => Some(i),
            _ => None,
        }
    }

    pub fn into_error(self) -> Option<ErrorResponse> {
        match self {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }

    pub fn into_complete(self) -> Option<CompleteResponse> {
        match self {
            Self::Complete(c) => Some(c),
            _ => None,
        }
    }

    pub fn from_message(msg: &Message) -> Vec<Self> {
        match msg {
            Message::User(_) => vec![],
            Message::Assistant(envelope) => {
                if let Some(err) = envelope.message().error() {
                    return vec![Self::Error(ErrorResponse::Assistant(err.clone()))];
                }
                envelope
                    .message()
                    .content()
                    .iter()
                    .map(|block| match block {
                        crate::proto::ContentBlock::Text(t) => Self::Text(TextResponse(t.clone())),
                        crate::proto::ContentBlock::ToolUse(t) => {
                            Self::ToolUse(ToolUseResponse(t.clone()))
                        }
                        crate::proto::ContentBlock::ToolResult(t) => {
                            Self::ToolResult(ToolResultResponse(t.clone()))
                        }
                        crate::proto::ContentBlock::Thinking(t) => {
                            Self::Thinking(ThinkingResponse(t.clone()))
                        }
                    })
                    .collect()
            }
            Message::System(sys) => match sys {
                SystemMessage::Init(init) => vec![Self::Init(InitResponse(init.clone()))],
                SystemMessage::Error(err) => {
                    vec![Self::Error(ErrorResponse::System(err.error().to_owned()))]
                }
            },
            Message::Result(result) => vec![Self::Complete(CompleteResponse(result.clone()))],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Responses(Vec<Response>);

impl Responses {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(self) -> Vec<Response> {
        self.into()
    }

    pub fn as_slice(&self) -> &[Response] {
        &self.0
    }

    pub fn push(&mut self, response: Response) {
        self.0.push(response);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Response> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn text_content(&self) -> String {
        self.0
            .iter()
            .filter_map(|r| r.as_text())
            .map(|t| t.content())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn thinking_content(&self) -> String {
        self.0
            .iter()
            .filter_map(|r| r.as_thinking())
            .map(|t| t.content())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn texts(&self) -> impl Iterator<Item = &TextResponse> {
        self.0.iter().filter_map(|r| r.as_text())
    }

    pub fn tool_uses(&self) -> impl Iterator<Item = &ToolUseResponse> {
        self.0.iter().filter_map(|r| r.as_tool_use())
    }

    pub fn tool_results(&self) -> impl Iterator<Item = &ToolResultResponse> {
        self.0.iter().filter_map(|r| r.as_tool_result())
    }

    pub fn thinkings(&self) -> impl Iterator<Item = &ThinkingResponse> {
        self.0.iter().filter_map(|r| r.as_thinking())
    }

    pub fn errors(&self) -> impl Iterator<Item = &ErrorResponse> {
        self.0.iter().filter_map(|r| r.as_error())
    }

    pub fn tool_use_by_name(&self, name: &str) -> Option<&ToolUseResponse> {
        self.tool_uses().find(|t| t.name() == name)
    }

    pub fn tool_uses_by_name(&self, name: &str) -> impl Iterator<Item = &ToolUseResponse> {
        self.tool_uses().filter(move |t| t.name() == name)
    }

    pub fn completion(&self) -> Option<&CompleteResponse> {
        self.0.iter().filter_map(|r| r.as_complete()).next_back()
    }

    pub fn init(&self) -> Option<&InitResponse> {
        self.0.iter().filter_map(|r| r.as_init()).next()
    }

    pub fn has_error(&self) -> bool {
        self.0.iter().any(|r| r.is_error())
    }

    pub fn first_error(&self) -> Option<&ErrorResponse> {
        self.0.iter().filter_map(|r| r.as_error()).next()
    }
}

impl From<Vec<Response>> for Responses {
    fn from(responses: Vec<Response>) -> Self {
        Self(responses)
    }
}

impl From<Responses> for Vec<Response> {
    fn from(responses: Responses) -> Self {
        responses.0
    }
}

impl IntoIterator for Responses {
    type Item = Response;
    type IntoIter = std::vec::IntoIter<Response>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Responses {
    type Item = &'a Response;
    type IntoIter = std::slice::Iter<'a, Response>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl std::ops::Index<usize> for Responses {
    type Output = Response;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

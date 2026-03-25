use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::control::{Request, Response};
use super::message::Message;

/// Incoming messages from CLI.
///
/// The `type` field determines which variant to parse.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Incoming {
    User(super::message::UserEnvelope),
    Assistant(super::message::AssistantEnvelope),
    System(super::message::SystemMessage),
    Result(super::message::ResultMessage),
    ControlRequest(ControlRequestEnvelope),
    ControlResponse(ControlResponseEnvelope),
    RateLimitEvent(RateLimitEvent),
}

/// Incoming control request envelope (CLI → SDK).
///
/// Structure matches Python SDK's SDKControlRequest:
/// ```json
/// {
///   "type": "control_request",
///   "request_id": "...",
///   "request": { "subtype": "mcp_message", "server_name": "...", "message": {...} }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRequestEnvelope {
    request_id: String,
    request: Request,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ControlRequestEnvelope {
    pub fn new(request_id: impl Into<String>, request: Request) -> Self {
        Self {
            request_id: request_id.into(),
            request,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn request(&self) -> &Request {
        &self.request
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_request_id(&mut self, request_id: impl Into<String>) {
        self.request_id = request_id.into();
    }

    pub fn set_request(&mut self, request: Request) {
        self.request = request;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.set_request_id(request_id);
        self
    }

    pub fn with_request(mut self, request: Request) -> Self {
        self.set_request(request);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

/// Incoming control response envelope (CLI → SDK).
///
/// Structure matches Python SDK's SDKControlResponse:
/// ```json
/// {
///   "type": "control_response",
///   "response": { "subtype": "success", "request_id": "...", "response": {...} }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlResponseEnvelope {
    response: Response,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl ControlResponseEnvelope {
    pub fn new(response: Response) -> Self {
        Self {
            response,
            extra: Map::new(),
        }
    }

    // Getters
    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }

    // Setters
    pub fn set_response(&mut self, response: Response) {
        self.response = response;
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self.extra = extra;
    }

    // Builders
    pub fn with_response(mut self, response: Response) -> Self {
        self.set_response(response);
        self
    }

    pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
        self.set_extra(extra);
        self
    }
}

/// The status of a rate limit check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitStatus {
    Allowed,
    AllowedWarning,
    Rejected,
}

impl std::fmt::Display for RateLimitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Allowed => "allowed",
            Self::AllowedWarning => "allowed_warning",
            Self::Rejected => "rejected",
        })
    }
}

/// Rate limit info nested within the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RateLimitInfo {
    status: RateLimitStatus,
    #[serde(skip_serializing_if = "Option::is_none", rename = "resetsAt")]
    resets_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilization: Option<f64>,
}

/// A rate limit event from the CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitEvent {
    rate_limit_info: RateLimitInfo,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

impl RateLimitEvent {
    pub fn status(&self) -> &RateLimitStatus {
        &self.rate_limit_info.status
    }

    pub fn resets_at(&self) -> Option<f64> {
        self.rate_limit_info.resets_at
    }

    pub fn utilization(&self) -> Option<f64> {
        self.rate_limit_info.utilization
    }

    pub fn extra(&self) -> &Map<String, Value> {
        &self.extra
    }
}

impl Incoming {
    pub fn to_message(&self) -> Option<Message> {
        match self {
            Self::User(u) => Some(Message::User(u.clone())),
            Self::Assistant(a) => Some(Message::Assistant(a.clone())),
            Self::System(s) => Some(Message::System(s.clone())),
            Self::Result(r) => Some(Message::Result(r.clone())),
            _ => None,
        }
    }

    pub fn as_control_request(&self) -> Option<&ControlRequestEnvelope> {
        match self {
            Self::ControlRequest(r) => Some(r),
            _ => None,
        }
    }

    pub fn as_control_response(&self) -> Option<&ControlResponseEnvelope> {
        match self {
            Self::ControlResponse(r) => Some(r),
            _ => None,
        }
    }

    pub fn as_rate_limit_event(&self) -> Option<&RateLimitEvent> {
        match self {
            Self::RateLimitEvent(r) => Some(r),
            _ => None,
        }
    }
}

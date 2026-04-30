use async_trait::async_trait;

use crate::response::{
    ApiRetryResponse, CompleteResponse, ErrorResponse, HookLifecycleResponse, InitResponse,
    NotificationResponse, RateLimitResponse, Response, TaskNotificationResponse,
    TaskProgressResponse, TaskStartedResponse, TaskUpdatedResponse, TextResponse, ThinkingResponse,
    ToolResultResponse, ToolUseResponse,
};

#[async_trait]
pub trait Handler: Send + Sync {
    async fn on_text(&self, _text: &TextResponse) {}
    async fn on_tool_use(&self, _tool_use: &ToolUseResponse) {}
    async fn on_tool_result(&self, _tool_result: &ToolResultResponse) {}
    async fn on_thinking(&self, _thinking: &ThinkingResponse) {}
    async fn on_init(&self, _init: &InitResponse) {}
    async fn on_error(&self, _error: &ErrorResponse) {}
    async fn on_rate_limit(&self, _rate_limit: &RateLimitResponse) {}
    async fn on_hook_started(&self, _hook: &HookLifecycleResponse) {}
    async fn on_hook_progress(&self, _hook: &HookLifecycleResponse) {}
    async fn on_hook_response(&self, _hook: &HookLifecycleResponse) {}
    async fn on_task_started(&self, _task: &TaskStartedResponse) {}
    async fn on_task_progress(&self, _task: &TaskProgressResponse) {}
    async fn on_task_updated(&self, _task: &TaskUpdatedResponse) {}
    async fn on_task_notification(&self, _task: &TaskNotificationResponse) {}
    async fn on_notification(&self, _notification: &NotificationResponse) {}
    async fn on_api_retry(&self, _retry: &ApiRetryResponse) {}
    async fn on_complete(&self, _complete: &CompleteResponse) {}
}

pub struct DefaultHandler;

#[async_trait]
impl Handler for DefaultHandler {}

pub async fn dispatch<H: Handler + ?Sized>(handler: &H, response: &Response) {
    match response {
        Response::Text(t) => handler.on_text(t).await,
        Response::ToolUse(t) => handler.on_tool_use(t).await,
        Response::ToolResult(t) => handler.on_tool_result(t).await,
        Response::Thinking(t) => handler.on_thinking(t).await,
        Response::Init(i) => handler.on_init(i).await,
        Response::Error(e) => handler.on_error(e).await,
        Response::RateLimit(r) => handler.on_rate_limit(r).await,
        Response::HookStarted(h) => handler.on_hook_started(h).await,
        Response::HookProgress(h) => handler.on_hook_progress(h).await,
        Response::HookResponse(h) => handler.on_hook_response(h).await,
        Response::TaskStarted(t) => handler.on_task_started(t).await,
        Response::TaskProgress(t) => handler.on_task_progress(t).await,
        Response::TaskUpdated(t) => handler.on_task_updated(t).await,
        Response::TaskNotification(t) => handler.on_task_notification(t).await,
        Response::Notification(n) => handler.on_notification(n).await,
        Response::ApiRetry(r) => handler.on_api_retry(r).await,
        Response::Complete(c) => handler.on_complete(c).await,
    }
}

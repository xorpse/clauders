use async_trait::async_trait;

use crate::response::{
    CompleteResponse, ErrorResponse, InitResponse, Response, TextResponse, ThinkingResponse,
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
        Response::Complete(c) => handler.on_complete(c).await,
    }
}

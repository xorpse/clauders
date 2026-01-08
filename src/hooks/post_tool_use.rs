use std::sync::Arc;

use serde_json::Value;

use crate::tool_input::ToolInput;

#[derive(Debug, Clone)]
pub struct PostToolUseInput {
    session_id: String,
    transcript_path: String,
    tool_name: String,
    tool_input: ToolInput,
    tool_response: Value,
}

impl PostToolUseInput {
    pub fn new(
        session_id: impl Into<String>,
        transcript_path: impl Into<String>,
        tool_name: impl Into<String>,
        tool_input: ToolInput,
        tool_response: Value,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            transcript_path: transcript_path.into(),
            tool_name: tool_name.into(),
            tool_input,
            tool_response,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn transcript_path(&self) -> &str {
        &self.transcript_path
    }

    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn tool_input(&self) -> &ToolInput {
        &self.tool_input
    }

    pub fn tool_response(&self) -> &Value {
        &self.tool_response
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostToolUseDecision {
    Continue,
    Block,
}

#[derive(Debug, Clone, Default)]
pub struct PostToolUseOutput {
    decision: Option<PostToolUseDecision>,
    reason: Option<String>,
    additional_context: Option<String>,
}

impl PostToolUseOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pass() -> Self {
        Self::default()
    }

    pub fn block(reason: impl Into<String>) -> Self {
        Self {
            decision: Some(PostToolUseDecision::Block),
            reason: Some(reason.into()),
            ..Default::default()
        }
    }

    pub fn continue_with_context(context: impl Into<String>) -> Self {
        Self {
            decision: Some(PostToolUseDecision::Continue),
            additional_context: Some(context.into()),
            ..Default::default()
        }
    }

    pub fn decision(&self) -> Option<PostToolUseDecision> {
        self.decision
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn additional_context(&self) -> Option<&str> {
        self.additional_context.as_deref()
    }

    pub fn set_decision(&mut self, decision: PostToolUseDecision) {
        self.decision = Some(decision);
    }

    pub fn set_reason(&mut self, reason: impl Into<String>) {
        self.reason = Some(reason.into());
    }

    pub fn set_additional_context(&mut self, context: impl Into<String>) {
        self.additional_context = Some(context.into());
    }

    pub fn with_decision(mut self, decision: PostToolUseDecision) -> Self {
        self.decision = Some(decision);
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_additional_context(mut self, context: impl Into<String>) -> Self {
        self.additional_context = Some(context.into());
        self
    }
}

pub type PostToolUseCallback = Arc<dyn Fn(PostToolUseInput) -> PostToolUseOutput + Send + Sync>;

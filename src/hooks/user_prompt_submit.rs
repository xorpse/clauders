use std::sync::Arc;

use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct UserPromptSubmitInput {
    session_id: String,
    transcript_path: String,
    prompt: String,
}

impl UserPromptSubmitInput {
    pub fn new(
        session_id: impl Into<String>,
        transcript_path: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            transcript_path: transcript_path.into(),
            prompt: prompt.into(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn transcript_path(&self) -> &str {
        &self.transcript_path
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserPromptSubmitDecision {
    Continue,
    Block,
}

#[derive(Debug, Clone, Default)]
pub struct UserPromptSubmitOutput {
    decision: Option<UserPromptSubmitDecision>,
    reason: Option<String>,
    additional_context: Option<String>,
}

impl UserPromptSubmitOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pass() -> Self {
        Self::default()
    }

    pub fn block(reason: impl Into<String>) -> Self {
        Self {
            decision: Some(UserPromptSubmitDecision::Block),
            reason: Some(reason.into()),
            ..Default::default()
        }
    }

    pub fn decision(&self) -> Option<UserPromptSubmitDecision> {
        self.decision
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn additional_context(&self) -> Option<&str> {
        self.additional_context.as_deref()
    }

    pub fn set_decision(&mut self, decision: UserPromptSubmitDecision) {
        self.decision = Some(decision);
    }

    pub fn set_reason(&mut self, reason: impl Into<String>) {
        self.reason = Some(reason.into());
    }

    pub fn set_additional_context(&mut self, context: impl Into<String>) {
        self.additional_context = Some(context.into());
    }

    pub fn with_decision(mut self, decision: UserPromptSubmitDecision) -> Self {
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

    pub fn to_hook_response(&self) -> Value {
        let mut result = json!({});

        if let Some(decision) = self.decision()
            && decision == UserPromptSubmitDecision::Block
        {
            result["decision"] = json!("block");
        }

        if let Some(reason) = self.reason() {
            result["reason"] = json!(reason);
        }

        let mut hook_specific = json!({
            "hookEventName": "UserPromptSubmit"
        });

        if let Some(context) = self.additional_context() {
            hook_specific["additionalContext"] = json!(context);
        }

        result["hookSpecificOutput"] = hook_specific;
        result
    }
}

pub type UserPromptSubmitCallback =
    Arc<dyn Fn(UserPromptSubmitInput) -> UserPromptSubmitOutput + Send + Sync>;

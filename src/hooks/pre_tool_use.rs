use std::sync::Arc;

use serde_json::{Value, json};

use crate::tool_input::ToolInput;

#[derive(Debug, Clone)]
pub struct PreToolUseInput {
    session_id: String,
    transcript_path: String,
    tool_name: String,
    tool_input: ToolInput,
}

impl PreToolUseInput {
    pub fn new(
        session_id: impl Into<String>,
        transcript_path: impl Into<String>,
        tool_name: impl Into<String>,
        tool_input: ToolInput,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            transcript_path: transcript_path.into(),
            tool_name: tool_name.into(),
            tool_input,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreToolUseDecision {
    Allow,
    Deny,
    Ask,
}

impl std::fmt::Display for PreToolUseDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PreToolUseDecision::Allow => "allow",
            PreToolUseDecision::Deny => "deny",
            PreToolUseDecision::Ask => "ask",
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct PreToolUseOutput {
    decision: Option<PreToolUseDecision>,
    reason: Option<String>,
    updated_input: Option<ToolInput>,
}

impl PreToolUseOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow() -> Self {
        Self {
            decision: Some(PreToolUseDecision::Allow),
            ..Default::default()
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: Some(PreToolUseDecision::Deny),
            reason: Some(reason.into()),
            ..Default::default()
        }
    }

    pub fn ask(reason: impl Into<String>) -> Self {
        Self {
            decision: Some(PreToolUseDecision::Ask),
            reason: Some(reason.into()),
            ..Default::default()
        }
    }

    pub fn pass() -> Self {
        Self::default()
    }

    pub fn decision(&self) -> Option<PreToolUseDecision> {
        self.decision
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn updated_input(&self) -> Option<&ToolInput> {
        self.updated_input.as_ref()
    }

    pub fn set_decision(&mut self, decision: PreToolUseDecision) {
        self.decision = Some(decision);
    }

    pub fn set_reason(&mut self, reason: impl Into<String>) {
        self.reason = Some(reason.into());
    }

    pub fn set_updated_input(&mut self, input: ToolInput) {
        self.updated_input = Some(input);
    }

    pub fn with_decision(mut self, decision: PreToolUseDecision) -> Self {
        self.decision = Some(decision);
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_updated_input(mut self, input: ToolInput) -> Self {
        self.updated_input = Some(input);
        self
    }

    pub fn to_hook_response(&self) -> Value {
        let mut hook_specific = json!({
            "hookEventName": "PreToolUse"
        });

        if let Some(decision) = self.decision() {
            hook_specific["permissionDecision"] = json!(decision.to_string());
        }

        if let Some(reason) = self.reason() {
            hook_specific["permissionDecisionReason"] = json!(reason);
        }

        if let Some(updated_input) = self.updated_input() {
            hook_specific["updatedInput"] = updated_input.as_value().clone();
        }

        json!({ "hookSpecificOutput": hook_specific })
    }
}

pub type PreToolUseCallback = Arc<dyn Fn(PreToolUseInput) -> PreToolUseOutput + Send + Sync>;

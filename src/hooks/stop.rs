use std::sync::Arc;

use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct StopInput {
    session_id: String,
    transcript_path: String,
    stop_hook_active: bool,
}

impl StopInput {
    pub fn new(
        session_id: impl Into<String>,
        transcript_path: impl Into<String>,
        stop_hook_active: bool,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            transcript_path: transcript_path.into(),
            stop_hook_active,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn transcript_path(&self) -> &str {
        &self.transcript_path
    }

    pub fn stop_hook_active(&self) -> bool {
        self.stop_hook_active
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopDecision {
    Continue,
    Block,
}

#[derive(Debug, Clone, Default)]
pub struct StopOutput {
    decision: Option<StopDecision>,
    reason: Option<String>,
}

impl StopOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pass() -> Self {
        Self::default()
    }

    pub fn block(reason: impl Into<String>) -> Self {
        Self {
            decision: Some(StopDecision::Block),
            reason: Some(reason.into()),
        }
    }

    pub fn decision(&self) -> Option<StopDecision> {
        self.decision
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn set_decision(&mut self, decision: StopDecision) {
        self.decision = Some(decision);
    }

    pub fn set_reason(&mut self, reason: impl Into<String>) {
        self.reason = Some(reason.into());
    }

    pub fn with_decision(mut self, decision: StopDecision) -> Self {
        self.decision = Some(decision);
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_hook_response(&self) -> Value {
        let mut result = json!({});

        if let Some(decision) = self.decision()
            && decision == StopDecision::Block
        {
            result["decision"] = json!("block");
        }

        if let Some(reason) = self.reason() {
            result["reason"] = json!(reason);
        }

        result["hookSpecificOutput"] = json!({
            "hookEventName": "Stop"
        });

        result
    }
}

pub type StopCallback = Arc<dyn Fn(StopInput) -> StopOutput + Send + Sync>;

use std::sync::Arc;

use crate::tool::ToolInput;

pub use crate::proto::PermissionMode;

#[derive(Debug, Clone)]
pub struct PermissionContext {
    tool_name: String,
    input: ToolInput,
    suggested_rules: Vec<PermissionRule>,
}

impl PermissionContext {
    pub fn new(
        tool_name: impl Into<String>,
        input: ToolInput,
        suggested_rules: Vec<PermissionRule>,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            input,
            suggested_rules,
        }
    }

    // Getters
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn input(&self) -> &ToolInput {
        &self.input
    }

    pub fn suggested_rules(&self) -> &[PermissionRule] {
        &self.suggested_rules
    }

    // Setters
    pub fn set_tool_name(&mut self, tool_name: impl Into<String>) {
        self.tool_name = tool_name.into();
    }

    pub fn set_input(&mut self, input: ToolInput) {
        self.input = input;
    }

    pub fn set_suggested_rules(&mut self, suggested_rules: Vec<PermissionRule>) {
        self.suggested_rules = suggested_rules;
    }

    // Builders
    pub fn with_tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.set_tool_name(tool_name);
        self
    }

    pub fn with_input(mut self, input: ToolInput) -> Self {
        self.set_input(input);
        self
    }

    pub fn with_suggested_rules(mut self, suggested_rules: Vec<PermissionRule>) -> Self {
        self.set_suggested_rules(suggested_rules);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PermissionRule {
    tool_name: String,
    rule: Option<String>,
}

impl PermissionRule {
    pub fn new(tool_name: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            rule: None,
        }
    }

    // Getters
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }

    // Setters
    pub fn set_tool_name(&mut self, tool_name: impl Into<String>) {
        self.tool_name = tool_name.into();
    }

    pub fn set_rule(&mut self, rule: Option<String>) {
        self.rule = rule;
    }

    // Builders
    pub fn with_tool_name(mut self, tool_name: impl Into<String>) -> Self {
        self.set_tool_name(tool_name);
        self
    }

    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.set_rule(Some(rule.into()));
        self
    }
}

#[derive(Debug, Clone)]
pub enum Decision {
    Allow { updated_input: Option<ToolInput> },
    Deny { message: String, interrupt: bool },
}

impl Decision {
    pub fn allow() -> Self {
        Self::Allow {
            updated_input: None,
        }
    }

    pub fn allow_with_input(input: ToolInput) -> Self {
        Self::Allow {
            updated_input: Some(input),
        }
    }

    pub fn deny(message: impl Into<String>) -> Self {
        Self::Deny {
            message: message.into(),
            interrupt: false,
        }
    }

    pub fn deny_and_interrupt(message: impl Into<String>) -> Self {
        Self::Deny {
            message: message.into(),
            interrupt: true,
        }
    }
}

pub type Callback = Arc<dyn Fn(PermissionContext) -> Decision + Send + Sync>;

pub fn default_allow(_ctx: PermissionContext) -> Decision {
    Decision::allow()
}

pub fn default_deny(ctx: PermissionContext) -> Decision {
    Decision::deny(format!("Tool '{}' not allowed", ctx.tool_name()))
}

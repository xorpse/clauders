//! Agent configuration for Claude Code subagents.

use serde::Serialize;

use crate::model::Model;

/// Configuration for a custom subagent.
///
/// Agents allow you to define specialised assistants with custom prompts,
/// models, and tool access. They are passed to the Claude CLI via the
/// `--agents` flag.
///
/// # Example
///
/// ```
/// use clauders::{Agent, Model};
///
/// let agent = Agent::new("Reviews code for issues", "You are a code reviewer")
///     .model(Model::Sonnet)
///     .tools(["Read", "Grep"]);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Agent {
    description: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<String>,
}

impl Agent {
    /// Creates a new agent with the given description and system prompt.
    pub fn new(description: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            prompt: prompt.into(),
            model: None,
            tools: Vec::new(),
        }
    }

    /// Sets the model for this agent.
    #[must_use]
    pub fn model(mut self, model: impl Into<Model>) -> Self {
        self.model = Some(model.into().to_string());
        self
    }

    /// Sets the tools this agent can use.
    #[must_use]
    pub fn tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tools = tools.into_iter().map(|s| s.into()).collect();
        self
    }
}

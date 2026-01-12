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
///     .with_model(Model::Sonnet)
///     .with_tools(["Read", "Grep"]);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Agent {
    description: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<Model>,
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

    /// The description of this agent.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// The prompt of this agent.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// The model of this agent, if set.
    pub fn model(&self) -> Option<&Model> {
        self.model.as_ref()
    }

    /// Sets the model for this agent.
    pub fn set_model(&mut self, model: impl Into<Model>) {
        self.model = Some(model.into());
    }

    /// Sets the model for this agent.
    #[must_use]
    pub fn with_model(mut self, model: impl Into<Model>) -> Self {
        self.set_model(model);
        self
    }

    /// The tools this agent can use.
    pub fn tools(&self) -> &[String] {
        &self.tools
    }

    /// Sets the tools this agent can use.
    pub fn set_tools(&mut self, tools: impl IntoIterator<Item = impl Into<String>>) {
        self.tools = tools.into_iter().map(|s| s.into()).collect();
    }

    /// Sets the tools this agent can use.
    #[must_use]
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.set_tools(tools);
        self
    }
}

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use schemars::JsonSchema;

use crate::agent::Agent;
use crate::hooks::Hooks;
use crate::mcp_server::McpServer;
use crate::model::Model;
use crate::proto::PermissionMode;
use crate::transport::TransportOptions;
use crate::util;

#[derive(Debug, Clone, Default)]
pub struct Options {
    allowed_tools: Vec<String>,
    disallowed_tools: Vec<String>,
    max_thinking_tokens: i32,
    system_prompt: Option<String>,
    append_system_prompt: Option<String>,
    permission_mode: Option<PermissionMode>,
    model: Option<Model>,
    fallback_model: Option<Model>,
    debug: bool,
    cwd: Option<PathBuf>,
    env: Vec<(String, String)>,
    max_budget_usd: Option<f64>,
    json_schema: Option<String>,
    mcp_servers: HashMap<String, Arc<McpServer>>,
    agents: HashMap<String, Agent>,
    hooks: Option<Hooks>,
}

impl Options {
    pub fn new() -> Self {
        Self {
            max_thinking_tokens: 8000,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn allowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.push(tool.into());
        self
    }

    #[must_use]
    pub fn allowed_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowed_tools = tools.into_iter().map(|s| s.into()).collect();
        self
    }

    #[must_use]
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = tools;
        self
    }

    #[must_use]
    pub fn disallowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.disallowed_tools.push(tool.into());
        self
    }

    #[must_use]
    pub fn disallowed_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.disallowed_tools = tools.into_iter().map(|s| s.into()).collect();
        self
    }

    #[must_use]
    pub fn with_disallowed_tools(mut self, tools: Vec<String>) -> Self {
        self.disallowed_tools = tools;
        self
    }

    #[must_use]
    pub fn max_thinking_tokens(mut self, tokens: i32) -> Self {
        self.max_thinking_tokens = tokens.max(1);
        self
    }

    #[must_use]
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    #[must_use]
    pub fn append_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.append_system_prompt = Some(prompt.into());
        self
    }

    #[must_use]
    pub fn permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_mode = Some(mode);
        self
    }

    #[must_use]
    pub fn model(mut self, model: impl Into<Model>) -> Self {
        self.model = Some(model.into());
        self
    }

    #[must_use]
    pub fn fallback_model(mut self, model: impl Into<Model>) -> Self {
        self.fallback_model = Some(model.into());
        self
    }

    #[must_use]
    pub fn cwd(mut self, path: impl AsRef<Path>) -> Self {
        self.cwd = Some(path.as_ref().to_path_buf());
        self
    }

    #[must_use]
    pub fn env(
        mut self,
        vars: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.env = vars
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    #[must_use]
    pub fn max_budget_usd(mut self, budget: f64) -> Self {
        self.max_budget_usd = if budget > 0.0 { Some(budget) } else { None };
        self
    }

    pub(crate) fn json_schema(&self) -> Option<&str> {
        self.json_schema.as_deref()
    }

    #[must_use]
    pub fn with_json_schema<T: JsonSchema>(mut self) -> Self {
        self.json_schema = Some(util::schema_for_structured_output::<T>().to_string());
        self
    }

    #[must_use]
    pub fn with_mcp_server(mut self, name: impl Into<String>, server: Arc<McpServer>) -> Self {
        self.mcp_servers.insert(name.into(), server);
        self
    }

    #[must_use]
    pub fn with_agent(mut self, name: impl Into<String>, agent: Agent) -> Self {
        self.agents.insert(name.into(), agent);
        self
    }

    #[must_use]
    pub fn with_agents(
        mut self,
        iter: impl IntoIterator<Item = (impl Into<String>, Agent)>,
    ) -> Self {
        self.agents
            .extend(iter.into_iter().map(|(name, agent)| (name.into(), agent)));
        self
    }

    #[must_use]
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    #[must_use]
    pub fn hooks(mut self, hooks: impl Into<Hooks>) -> Self {
        self.hooks = Some(hooks.into());
        self
    }

    pub(crate) fn mcp_servers(&self) -> &HashMap<String, Arc<McpServer>> {
        &self.mcp_servers
    }

    pub(crate) fn take_hooks(&mut self) -> Option<Hooks> {
        self.hooks.take()
    }

    pub(crate) fn to_transport_options(&self) -> TransportOptions {
        use crate::transport::TransportOptionsBuilder;

        let mut allowed = self.allowed_tools.clone();
        for (server_name, server) in &self.mcp_servers {
            for tool in server.tools() {
                allowed.push(format!("mcp__{}__{}", server_name, tool.name()));
            }
        }

        let mut builder = TransportOptionsBuilder::default();
        builder
            .allowed_tools(allowed)
            .disallowed_tools(self.disallowed_tools.clone())
            .mcp_server_names(self.mcp_servers.keys().cloned().collect::<Vec<_>>())
            .env(self.env.clone());

        if let Some(m) = &self.model {
            builder.model(m.to_string());
        }
        if let Some(m) = &self.fallback_model {
            builder.fallback_model(m.to_string());
        }
        if let Some(p) = &self.system_prompt {
            builder.system_prompt(p.clone());
        }
        if let Some(p) = &self.append_system_prompt {
            builder.append_system_prompt(p.clone());
        }
        if let Some(m) = self.permission_mode {
            builder.permission_mode(m.to_string());
        }
        if let Some(b) = self.max_budget_usd {
            builder.max_budget_usd(b);
        }
        if let Some(c) = &self.cwd {
            builder.cwd(c.clone());
        }
        if let Some(s) = &self.json_schema {
            builder.json_schema(s.clone());
        }

        builder.agents(self.agents.clone());

        builder.build().expect("all fields have defaults")
    }
}

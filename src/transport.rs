use std::path::PathBuf;
use std::process::Stdio;

use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tracing::{debug, error, info, warn};

use crate::error::Error;
use crate::proto::{Incoming, RequestEnvelope, control::ResponseEnvelope};

pub struct Transport {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    stderr_task: tokio::task::JoinHandle<()>,
}

impl std::fmt::Debug for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transport")
            .field("pid", &self.child.id())
            .field("stdin", &self.stdin.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Default, derive_builder::Builder)]
#[builder(default, setter(into, strip_option))]
pub struct TransportOptions {
    allowed_tools: Vec<String>,
    disallowed_tools: Vec<String>,
    model: Option<String>,
    fallback_model: Option<String>,
    system_prompt: Option<String>,
    append_system_prompt: Option<String>,
    permission_mode: Option<String>,
    max_budget_usd: Option<f64>,
    debug: bool,
    cwd: Option<PathBuf>,
    env: Vec<(String, String)>,
    json_schema: Option<String>,
    mcp_server_names: Vec<String>,
}

impl TransportOptions {
    pub fn allowed_tools(&self) -> &[String] {
        &self.allowed_tools
    }

    pub fn disallowed_tools(&self) -> &[String] {
        &self.disallowed_tools
    }

    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    pub fn fallback_model(&self) -> Option<&str> {
        self.fallback_model.as_deref()
    }

    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    pub fn append_system_prompt(&self) -> Option<&str> {
        self.append_system_prompt.as_deref()
    }

    pub fn permission_mode(&self) -> Option<&str> {
        self.permission_mode.as_deref()
    }

    pub fn max_budget_usd(&self) -> Option<f64> {
        self.max_budget_usd
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn cwd(&self) -> Option<&PathBuf> {
        self.cwd.as_ref()
    }

    pub fn env(&self) -> &[(String, String)] {
        &self.env
    }

    pub fn json_schema(&self) -> Option<&str> {
        self.json_schema.as_deref()
    }

    pub fn mcp_server_names(&self) -> &[String] {
        &self.mcp_server_names
    }
}

impl Transport {
    pub async fn new(options: &TransportOptions) -> Result<Self, Error> {
        let cmd = Self::build_command(options);
        let env = Self::build_env(options);

        info!(cmd = ?cmd, "spawning claude CLI");

        let mut child = Command::new("claude")
            .args(&cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .envs(env)
            .current_dir(options.cwd.as_deref().unwrap_or_else(|| std::path::Path::new(".")))
            .spawn()
            .map_err(|e| {
                error!(error = %e, "failed to spawn claude CLI");
                Error::CliNotFound(format!(
                    "failed to spawn claude CLI: {e}; make sure 'claude' is installed and authenticated",
                ))
            })?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| Error::ProcessError("failed to get stdin handle".to_owned()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| Error::ProcessError("failed to get stdout handle".to_owned()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| Error::ProcessError("failed to get stderr handle".to_owned()))?;

        let stderr_task = tokio::spawn(Self::log_stderr(stderr));

        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout: BufReader::new(stdout),
            stderr_task,
        })
    }

    fn build_command(options: &TransportOptions) -> Vec<String> {
        let mut cmd = vec![
            "--output-format".to_owned(),
            "stream-json".to_owned(),
            "--verbose".to_owned(),
        ];

        if options.debug {
            cmd.push("--debug".to_owned());
        }

        if let Some(prompt) = &options.system_prompt {
            cmd.extend(["--system-prompt".to_owned(), prompt.clone()]);
        }

        if let Some(prompt) = &options.append_system_prompt {
            cmd.extend(["--append-system-prompt".to_owned(), prompt.clone()]);
        }

        if !options.allowed_tools.is_empty() {
            cmd.extend(["--allowedTools".to_owned(), options.allowed_tools.join(",")]);
        }

        if !options.disallowed_tools.is_empty() {
            cmd.extend([
                "--disallowedTools".to_owned(),
                options.disallowed_tools.join(","),
            ]);
        }

        if let Some(model) = &options.model {
            cmd.extend(["--model".to_owned(), model.clone()]);
        }

        if let Some(model) = &options.fallback_model {
            cmd.extend(["--fallback-model".to_owned(), model.clone()]);
        }

        if let Some(mode) = &options.permission_mode {
            cmd.extend(["--permission-mode".to_owned(), mode.clone()]);
        }

        if let Some(budget) = options.max_budget_usd {
            cmd.extend(["--max-budget-usd".to_owned(), budget.to_string()]);
        }

        if let Some(schema) = &options.json_schema {
            cmd.extend(["--json-schema".to_owned(), schema.clone()]);
        }

        if !options.mcp_server_names.is_empty() {
            let mut mcp_servers = serde_json::Map::new();
            for name in &options.mcp_server_names {
                let server_config = serde_json::json!({
                    "type": "sdk",
                    "name": name,
                });
                mcp_servers.insert(name.clone(), server_config);
            }
            let mcp_config = serde_json::json!({ "mcpServers": mcp_servers });
            cmd.extend([
                "--mcp-config".to_owned(),
                serde_json::to_string(&mcp_config).expect("MCP config serialization"),
            ]);
        }

        cmd.extend(["--input-format".to_owned(), "stream-json".to_owned()]);
        cmd
    }

    fn build_env(options: &TransportOptions) -> Vec<(String, String)> {
        let mut env = vec![("CLAUDE_CODE_ENTRYPOINT".to_owned(), "sdk-rust".to_owned())];

        for (k, v) in &options.env {
            env.push((k.clone(), v.clone()));
        }

        env
    }

    async fn log_stderr(stderr: ChildStderr) {
        let mut reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => warn!(target: "claude_cli", "{}", line.trim_end()),
                Err(e) => {
                    error!(error = %e, "failed to read stderr");
                    break;
                }
            }
        }
    }

    pub async fn send(&mut self, json: &Value) -> Result<(), Error> {
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| Error::ProcessError("stdin closed".to_owned()))?;
        let data = serde_json::to_string(json)?;
        debug!(data = %data, "sending");
        stdin.write_all(data.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;
        Ok(())
    }

    pub async fn send_request(&mut self, envelope: &RequestEnvelope) -> Result<(), Error> {
        let json = serde_json::to_value(envelope)?;
        self.send(&json).await
    }

    pub async fn send_response(&mut self, envelope: &ResponseEnvelope) -> Result<(), Error> {
        let json = serde_json::to_value(envelope)?;
        self.send(&json).await
    }

    pub async fn receive_line(&mut self) -> Result<Option<String>, Error> {
        let mut line = String::new();
        match self.stdout.read_line(&mut line).await? {
            0 => Ok(None),
            _ => {
                debug!(line = %line.trim(), "received");
                Ok(Some(line))
            }
        }
    }

    pub async fn receive(&mut self) -> Result<Option<Incoming>, Error> {
        match self.receive_line().await? {
            Some(line) => {
                let incoming = serde_json::from_str::<Incoming>(&line).map_err(|e| {
                    error!(line = %line, error = %e, "failed to parse incoming message");
                    Error::ProtocolError(format!("failed to parse: {e}"))
                })?;
                Ok(Some(incoming))
            }
            None => Ok(None),
        }
    }

    pub async fn interrupt(&mut self) -> Result<(), Error> {
        info!("sending interrupt signal");
        let envelope = RequestEnvelope::interrupt("");
        self.send_request(&envelope).await
    }

    pub async fn close(mut self) -> Result<(), Error> {
        self.stdin.take();
        self.child.wait().await?;
        Ok(())
    }
}

impl Drop for Transport {
    fn drop(&mut self) {
        self.stderr_task.abort();
        if let Err(e) = self.child.start_kill() {
            error!(error = %e, "failed to kill child process");
        }
    }
}

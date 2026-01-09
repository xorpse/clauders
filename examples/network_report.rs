use std::io::{self, Write};
use std::process::Command;
use std::sync::Arc;

use clauders::{Client, McpServer, Model, Options, Responses, Tool};
use futures::StreamExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Input for the ping tool
#[derive(Debug, JsonSchema, Deserialize)]
struct PingInput {
    /// The hostname or IP address to ping
    host: String,
    /// Number of packets to send (default: 4)
    #[serde(default = "default_count")]
    count: u32,
}

fn default_count() -> u32 {
    4
}

/// Input for the DNS lookup tool
#[derive(Debug, JsonSchema, Deserialize)]
struct DnsLookupInput {
    /// The hostname to look up
    host: String,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
struct DnsLookupOutput {
    #[schemars(description = "List of resolved DNS records")]
    records: Vec<String>,
}

/// Input for the traceroute tool
#[derive(Debug, JsonSchema, Deserialize)]
struct TracerouteInput {
    /// The hostname or IP address to trace
    host: String,
    /// Maximum number of hops (default: 15)
    #[serde(default = "default_max_hops")]
    max_hops: u32,
}

fn default_max_hops() -> u32 {
    15
}

fn ping_tool() -> Tool {
    Tool::unstructured(
        "ping",
        "Ping a host to check connectivity and measure latency",
        |input: PingInput| {
            let output = Command::new("ping")
                .args(["-c", &input.count.to_string(), &input.host])
                .output()
                .map_err(|e| clauders::ToolError::execution_failed(e.to_string()))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                Ok(Tool::text_result(&stdout))
            } else {
                Ok(Tool::text_result(&format!(
                    "Ping failed:\n{}\n{}",
                    stdout, stderr
                )))
            }
        },
    )
}

fn dns_lookup_tool() -> Tool {
    Tool::structured(
        "dns_lookup",
        "Perform DNS lookup for a hostname",
        |input: DnsLookupInput| {
            let output = Command::new("dig")
                .args([&input.host, "+short"])
                .output()
                .map_err(|e| clauders::ToolError::execution_failed(e.to_string()))?;

            let result = DnsLookupOutput {
                records: String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .lines()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>(),
            };

            Ok(result)
        },
    )
}

fn traceroute_tool() -> Tool {
    Tool::unstructured(
        "traceroute",
        "Trace the network path to a host",
        |input: TracerouteInput| {
            let output = Command::new("traceroute")
                .args(["-e", "-I", "-m", &input.max_hops.to_string(), &input.host])
                .output()
                .map_err(|e| clauders::ToolError::execution_failed(e.to_string()))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() || !stdout.is_empty() {
                Ok(Tool::text_result(&stdout))
            } else {
                Ok(Tool::text_result(&format!(
                    "Traceroute failed:\n{}",
                    stderr
                )))
            }
        },
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: network_report <hostname>");
        eprintln!("Example: network_report google.com");
        std::process::exit(1);
    });

    println!("Network Diagnostics Report for: {}", host);
    println!("{}", "=".repeat(50));
    println!();

    // Create MCP server with network diagnostic tools
    let network_server = Arc::new(McpServer::new(
        "network_tools",
        vec![ping_tool(), dns_lookup_tool(), traceroute_tool()],
    ));

    let client = Client::new(
        Options::new()
            .model(Model::Haiku)
            .debug(true)
            .with_mcp_server("network_tools", network_server)
            .system_prompt(
                "You are a network diagnostics assistant. You have access to network diagnostic \
                 tools via the MCP server 'network_tools'. Use these tools to analyze hosts:\n\n\
                 - mcp__network_tools__dns_lookup: Look up DNS records\n\
                 - mcp__network_tools__ping: Test connectivity and latency\n\
                 - mcp__network_tools__traceroute: Trace the network path\n\n\
                 After gathering data, provide a clear summary report including:\n\
                 - DNS resolution results\n\
                 - Connectivity status and latency statistics\n\
                 - Network path analysis\n\
                 - Any issues or anomalies detected",
            ),
    )
    .await?;

    let prompt = format!(
        "Please generate a comprehensive network diagnostics report for: {}\n\n\
         Run these diagnostics in order:\n\
         1. DNS lookup to resolve the hostname\n\
         2. Ping test to check connectivity (4 packets)\n\
         3. Traceroute to analyze the network path (max 15 hops)\n\n\
         After running these tools, summarize the findings in a clear report.",
        host
    );

    client.query(&prompt).await?;

    let mut stream = std::pin::pin!(client.receive());
    let mut responses = Responses::new();
    let mut current_tool = String::new();

    while let Some(result) = stream.next().await {
        let response = result?;

        if let Some(text) = response.as_text() {
            print!("{}", text.content());
            io::stdout().flush()?;
        }

        if let Some(tool_use) = response.as_tool_use() {
            current_tool = tool_use.name().to_string();
            println!();
            println!("[Tool: {}]", tool_use.name());
        }

        if let Some(tool_result) = response.as_tool_result() {
            if let Some(content) = tool_result.content() {
                if let Some(text) = extract_tool_text(content) {
                    let preview = truncate(&text.replace('\n', " "), 80);
                    if tool_result.is_error() {
                        println!("[Error: {}]", preview);
                    } else {
                        println!("[Output: {}]", preview);
                    }
                }
            }
            current_tool.clear();
        }

        if let Some(err) = response.as_error() {
            eprintln!("\nError: {}", err.message());
        }

        responses.push(response);
    }

    println!();

    if let Some(complete) = responses.completion() {
        println!();
        println!("{}", "=".repeat(50));
        println!(
            "Report completed in {:.2}s | {} turns",
            complete.duration_ms() as f64 / 1000.0,
            complete.num_turns()
        );
        if let Some(cost) = complete.total_cost_usd() {
            println!("Cost: ${:.4}", cost);
        }
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}

fn extract_tool_text(content: &serde_json::Value) -> Option<String> {
    content
        .as_array()
        .and_then(|a| a.first())
        .and_then(|v| v.get("text"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
}

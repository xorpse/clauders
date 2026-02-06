# clauders

Rust bindings for the Claude Code CLI.

## Dependencies

Requires the Claude Code CLI to be installed.

## Usage

### Simple Query

```rust
let client = Client::new(Options::new()).await?;
let (text, _) = client.query_once("Hello, Claude!").await?;
```

### Conversation with Streaming

```rust
let client = Client::new(Options::new()).await?;
let mut conv = client.conversation();

let answer = conv.say("What is Rust?").await?;

// With streaming callback
conv.turn("Tell me more")
    .on_text(|chunk| print!("{}", chunk))
    .send_text()
    .await?;
```

### Structured Output

```rust
#[derive(Deserialize, JsonSchema)]
struct Sentiment {
    classification: String,
    confidence: f64,
}

let client = Client::new(
    Options::new().with_json_schema::<Sentiment>()
).await?;

let (result, _) = client.query_once_as::<Sentiment>("Analyze: 'Great!'").await?;
```

### Custom Tools via MCP

```rust
#[derive(JsonSchema, Deserialize)]
struct PingInput { host: String }

let tool = Tool::unstructured("ping", "Ping a host", |input: PingInput| async move {
    Ok(Tool::text_result(format!("Pinging {}", input.host)))
});

let server = Arc::new(McpServer::new("tools", vec![tool]));
let client = Client::new(Options::new().with_mcp_server("tools", server)).await?;
```

### Hooks

```rust
let hooks = Hooks::new()
    .on_pre_tool_use(None, |input| async {
        println!("Running: {}", input.tool_name());
        PreToolUseOutput::allow()
    });

let client = Client::new(Options::new().hooks(hooks)).await?;
```

## Examples

```bash
cargo run --example conversation
cargo run --example sentiment_analysis
cargo run --example network_report
```

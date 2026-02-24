# Evo User Agent Template

Template repository for creating new user agents in the [Evo self-evolution agent system](https://github.com/ai-evo-agents). Clone this repo, customize the handler and soul, and deploy your own agent.

## Quick Start

```sh
# Clone the template
git clone https://github.com/ai-evo-agents/evo-user-agent-template my-agent
cd my-agent

# Build your agent binary
cargo build --release

# Set environment
export KING_ADDRESS=http://localhost:3000
export GATEWAY_ADDRESS=http://localhost:8080

# Run the agent (pass the agent directory as argument)
./target/release/evo-user-agent .
```

**Alternative: Use the pre-built generic runner** (no Rust toolchain required):
```sh
./download-runner.sh
./evo-runner .
```

## Project Structure

```
my-agent/
├── Cargo.toml              # Rust project manifest (rename your binary here)
├── src/
│   └── main.rs             # Your AgentHandler implementation
├── soul.md                 # Agent identity, role, and behavior rules
├── skills/                 # Skills this agent can use
├── mcp/                    # MCP server definitions
├── download-runner.sh      # Downloads pre-built evo-runner binary
├── .github/
│   └── workflows/
│       ├── ci.yml          # Lint, test, and build on push/PR
│       └── release.yml     # Cross-platform binary releases on git tag
└── .gitignore
```

## Customization

### 1. Rename your agent

In `Cargo.toml`, change the package name and binary name:
```toml
[package]
name = "my-cool-agent"

[[bin]]
name = "my-cool-agent"
```

Update `BINARY_NAME` in `.github/workflows/release.yml` to match.

### 2. Implement your handler

Edit `src/main.rs` to define your agent's pipeline logic:

```rust
use async_trait::async_trait;
use evo_agent_sdk::prelude::*;

struct MyAgent;

#[async_trait]
impl AgentHandler for MyAgent {
    async fn on_pipeline(&self, ctx: PipelineContext<'_>) -> anyhow::Result<serde_json::Value> {
        let response = ctx.gateway
            .chat_completion("gpt-4o-mini", &ctx.soul.behavior, "Hello", None, None)
            .await?;
        Ok(serde_json::json!({ "result": response }))
    }
}
```

The `PipelineContext` gives you access to:
- `ctx.gateway` -- call LLMs via evo-gateway
- `ctx.soul` -- agent identity and behavior rules from `soul.md`
- `ctx.skills` -- loaded skill definitions
- `ctx.run_id`, `ctx.stage`, `ctx.metadata` -- pipeline run context

### 3. Edit soul.md

Define your agent's identity and behavior:
```markdown
# My Agent

## Role
user

## Behavior
- Describe what this agent does
- Each rule on its own line
```

The `## Role` header must be `user` for user agents. The `## Behavior` section becomes the LLM system prompt.

## CI/CD

This template includes GitHub Actions workflows:

- **CI** (`ci.yml`): Runs on push to `main` and pull requests. Validates `soul.md`, checks formatting, runs clippy and tests.
- **Release** (`release.yml`): Triggered by git tags matching `v*`. Builds cross-platform binaries (Linux x86_64/ARM64, macOS Intel/Apple Silicon, Windows x86_64) and uploads them as GitHub Release assets.

### Creating a release

```sh
git tag v0.1.0
git push origin v0.1.0
```

This triggers the release workflow, which builds binaries for 5 platforms and attaches them to a GitHub Release.

## Dependencies

- [evo-agent-sdk](https://crates.io/crates/evo-agent-sdk) -- SDK for building Evo agents
- [evo-common](https://crates.io/crates/evo-common) -- Shared types and protocols (re-exported by SDK)

## License

MIT

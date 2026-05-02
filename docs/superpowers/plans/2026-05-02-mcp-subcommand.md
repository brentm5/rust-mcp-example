# MCP Subcommand Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an `mcp` subcommand that starts a compliant stdio MCP server with no tools, prompts, or resources.

**Architecture:** The `mcp` subcommand plugs into the existing `clap` CLI alongside `time`. It starts an async MCP server via the `rmcp` crate's `ServerHandler` trait, bridged from the synchronous `main()` using `tokio::runtime::Runtime::block_on`. The server handles the MCP initialization handshake and runs until the client disconnects.

**Tech Stack:** `rmcp 1.6.0` (features: `server`, `transport-io`), `tokio 1` (features: `full`), `clap 4` (existing)

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Modify | `Cargo.toml` | Add `rmcp` and `tokio` dependencies |
| Create | `src/commands/mcp.rs` | `McpServer` struct + `ServerHandler` impl + `run()` |
| Modify | `src/commands/mod.rs` | Expose `mcp` module |
| Modify | `src/main.rs` | Add `Mcp` variant to `Commands`, wire `run()` |

---

### Task 1: Add dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add rmcp and tokio to Cargo.toml**

Open `Cargo.toml` and add to `[dependencies]`:

```toml
rmcp = { version = "1.6.0", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
```

The full `[dependencies]` block should look like:

```toml
[dependencies]
clap = { version = "4.6.1", features = ["derive"] }
chrono = { version = "0.4", features = ["clock"] }
rmcp = { version = "1.6.0", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 2: Verify dependencies resolve**

```bash
cargo fetch
```

Expected: No errors. Dependencies download successfully.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: add rmcp and tokio dependencies"
```

---

### Task 2: Create the MCP server module

**Files:**
- Create: `src/commands/mcp.rs`

- [ ] **Step 1: Create src/commands/mcp.rs**

```rust
use rmcp::{ServerHandler, ServiceExt, transport::stdio};

#[derive(Clone)]
pub struct McpServer;

impl ServerHandler for McpServer {}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    if let Err(e) = rt.block_on(async {
        let service = McpServer.serve(stdio()).await?;
        service.waiting().await?;
        Ok::<_, Box<dyn std::error::Error>>(())
    }) {
        eprintln!("mcp server error: {e}");
        std::process::exit(1);
    }
}
```

- [ ] **Step 2: Verify it compiles in isolation**

```bash
cargo check
```

Expected: Errors about `mcp` module not being registered yet (that's fine — the next task fixes it). There should be NO errors inside `src/commands/mcp.rs` itself other than "file not found in module tree".

---

### Task 3: Register the mcp module

**Files:**
- Modify: `src/commands/mod.rs`

- [ ] **Step 1: Add pub mod mcp to src/commands/mod.rs**

The full file should be:

```rust
pub mod mcp;
pub mod time;
```

- [ ] **Step 2: Verify**

```bash
cargo check
```

Expected: `src/commands/mcp.rs` compiles without errors. Only errors at this point should relate to `main.rs` not yet using the new module.

---

### Task 4: Wire up the CLI subcommand

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update src/main.rs**

Replace the full contents of `src/main.rs` with:

```rust
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print the current time
    Time(commands::time::TimeArgs),
    /// Start the MCP stdio server
    Mcp,
}

fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => commands::time::run(&time_args),
        Commands::Mcp => commands::mcp::run(),
    }
}
```

- [ ] **Step 2: Verify full build**

```bash
cargo build
```

Expected: Compiles cleanly with no errors or warnings.

- [ ] **Step 3: Smoke test — confirm subcommand is registered**

```bash
cargo run -- --help
```

Expected output includes:

```
Commands:
  time  Print the current time
  mcp   Start the MCP stdio server
  help  Print this message or the help of the given subcommand(s)
```

- [ ] **Step 4: Smoke test — confirm server starts**

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}' | cargo run -- mcp
```

Expected: Process starts, receives the initialize request, responds with a JSON-RPC result containing `serverInfo` and empty capabilities, then exits cleanly when stdin closes. No panics.

- [ ] **Step 5: Commit**

```bash
git add src/commands/mcp.rs src/commands/mod.rs src/main.rs
git commit -m "feat: add mcp subcommand with stdio MCP server shell"
```

# MCP Subcommand Design

**Date:** 2026-05-02
**Status:** Draft

## Overview

Add an `mcp` subcommand to the `rust-mcp-example` CLI that starts a compliant stdio MCP server. The server handles the MCP initialization handshake but exposes no tools, prompts, or resources. It serves as a working shell for future capability additions.

## Architecture

The `mcp` subcommand integrates into the existing `clap`-based CLI alongside the `time` subcommand. When invoked, it starts a blocking stdio MCP server using the `rmcp` crate's `ServerHandler` trait.

Because the existing binary uses a synchronous `main()`, the `mcp` command bridges sync→async via `tokio::runtime::Runtime::block_on`. This avoids converting the entire binary to `#[tokio::main]` and keeps the `time` command unaffected.

The server runs until the MCP client disconnects or closes stdin, at which point `waiting()` resolves and the process exits cleanly.

## Components

### `Cargo.toml`
Add two dependencies:
- `rmcp = { version = "1.6.0", features = ["server"] }` — MCP protocol implementation and stdio transport
- `tokio = { version = "1", features = ["full"] }` — async runtime required by rmcp

### `src/commands/mcp.rs`
- Empty `McpServer` struct implementing `rmcp::ServerHandler` with default method implementations (returns empty lists for tools/prompts/resources; the SDK handles `initialize`/`initialized` automatically)
- `run()` function: builds a `tokio::runtime::Runtime`, calls `serve_server(McpServer, rmcp::stdio())`, and awaits `waiting()`

### `src/commands/mod.rs`
Add `pub mod mcp`.

### `src/main.rs`
- Add `Mcp` variant to the `Commands` enum (no args needed; `clap::Args` on an empty struct)
- Wire the `Commands::Mcp` arm to call `commands::mcp::run()`

## Data Flow

```
MCP host (e.g. Claude Desktop)
  └─ spawns: rust-mcp-example mcp
       └─ stdin/stdout (JSON-RPC 2.0)
            └─ rmcp stdio transport
                 └─ McpServer (ServerHandler)
                      ├─ initialize → ServerInfo (name, version)
                      ├─ tools/list → []
                      ├─ prompts/list → []
                      └─ resources/list → []
```

## Error Handling

- If `serve_server` or `waiting()` returns an error, `run()` propagates it and the process exits with a non-zero code via `eprintln!` + `std::process::exit(1)`.
- No special handling needed for clean disconnect — `waiting()` resolves normally when the client closes the connection.

## Testing

Manual verification: run `cargo run -- mcp` and confirm the process starts without panicking. Full protocol compliance can be tested by configuring the binary as an MCP server in Claude Desktop and verifying a successful `initialize` handshake with empty capability lists.

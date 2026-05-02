---
title: MCP Server Architecture
tags: [architecture, mcp, rmcp]
updated: 2026-05-03
sources: [src/commands/mcp.rs, src/mcp_tools/]
---

# MCP Server Architecture

The MCP server is started via the `mcp` CLI subcommand and runs as a stdio server using the `rmcp` crate.

## Entry point

`src/commands/mcp.rs` → `run()`:

1. Builds a tokio runtime (bridges sync `main()` to async rmcp)
2. Calls `McpServer.serve(stdio())` — completes the MCP initialize handshake, starts the message loop in a background task
3. Blocks on `service.waiting()` until the client disconnects

## McpServer

`McpServer` is an empty struct implementing `rmcp::ServerHandler`. It overrides three methods:

| Method | Purpose |
|--------|---------|
| `get_info()` | Advertises `tools` capability in the initialize response — required for hosts to call `tools/list` |
| `list_tools()` | Returns the list of available tool definitions |
| `call_tool()` | Dispatches by tool name to the appropriate handler in `mcp_tools/` |

`#[derive(Clone)]` is required because rmcp clones the handler for each concurrent request.

## Tool modules

Each tool lives in `src/mcp_tools/<name>.rs` and exposes:

| Export | Type | Purpose |
|--------|------|---------|
| `NAME` | `&str` const | Tool name — used in both `definition()` and the dispatch match |
| `definition()` | `fn() -> Tool` | Returns the tool schema (name, description, input JSON Schema) |
| `call()` | `fn(...) -> CallToolResult` | Executes the tool and returns the result |

The JSON Schema for each tool's input is auto-generated from a Rust struct using `schemars::JsonSchema`.

## Adding a tool

1. Create `src/mcp_tools/<name>.rs` — implement `NAME`, `definition()`, `call()`
2. Add `pub mod <name>;` to `src/mcp_tools/mod.rs`
3. Add `<name>::definition()` to the `list_tools` vec in `mcp.rs`
4. Add a match arm `<name>::NAME => ...` to `call_tool` in `mcp.rs`

## Transport

stdio — the MCP host spawns the binary and communicates over stdin/stdout via JSON-RPC 2.0. `rmcp::transport::stdio()` returns `(tokio::io::stdin(), tokio::io::stdout())`.

## Related

- [[architecture/overview]] — where MCP fits in the project
- [[concepts/mcp]] — what MCP is and how tool calls work
- [[mcp-tools/get_time]] — the first tool implementation

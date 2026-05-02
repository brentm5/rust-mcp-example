# rust-mcp-example

A playground for learning how to build MCP (Model Context Protocol) servers in Rust.

## Concept

The pattern here is simple:

1. Build CLI subcommands that do useful work
2. Expose those same subcommands as MCP tool calls

Logic lives in one place — the CLI is the source of truth. The MCP server is a thin wrapper that lets AI agents (like Claude) invoke those same commands via the MCP protocol.

## Usage

```bash
# Print the current local time
rust-mcp-example time

# Print UTC time
rust-mcp-example time --utc

# Start the MCP stdio server
rust-mcp-example mcp

# Global debug flag
rust-mcp-example --debug time
```

## MCP Server

The `mcp` subcommand starts a stdio MCP server compatible with Claude Desktop, Claude Code, and other MCP hosts. Configure it by pointing your MCP host at the binary:

```json
{
  "mcpServers": {
    "rust-mcp-example": {
      "command": "cargo",
      "args": ["run", "--", "mcp"],
      "cwd": "/path/to/rust-mcp-example"
    }
  }
}
```

### Available Tools

| Tool | Description | Arguments |
|------|-------------|-----------|
| `get_time` | Returns the current time | `utc?: boolean` — use UTC instead of local time |

## Project Structure

```text
src/
  main.rs              — CLI entry point, arg parsing, dispatch
  commands/
    mod.rs             — submodule declarations
    time.rs            — time subcommand (format_time shared with MCP tools)
    mcp.rs             — MCP server (ServerHandler, stdio transport, run loop)
  mcp_tools/
    mod.rs             — submodule declarations
    get_time.rs        — get_time tool (definition + call handler)
```

## Adding a CLI Command

1. Create `src/commands/<name>.rs` with `pub struct <Name>Args` and `pub fn run(args: &<Name>Args)`
2. Add `pub mod <name>;` to `src/commands/mod.rs`
3. Add a variant to `Commands` in `main.rs` and a match arm in `main()`

## Adding an MCP Tool

1. Create `src/mcp_tools/<name>.rs` with `pub const NAME`, `pub fn definition() -> Tool`, and `pub fn call(...) -> CallToolResult`
2. Add `pub mod <name>;` to `src/mcp_tools/mod.rs`
3. Add the tool to `list_tools` and a match arm to `call_tool` in `src/commands/mcp.rs`

## Tech

- [clap](https://docs.rs/clap) — CLI argument parsing
- [chrono](https://docs.rs/chrono) — date/time
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) — MCP protocol (stdio transport, ServerHandler)
- [schemars](https://docs.rs/schemars) — JSON Schema generation from Rust structs
- [serde](https://docs.rs/serde) / [serde_json](https://docs.rs/serde_json) — JSON serialization
- [tokio](https://docs.rs/tokio) — async runtime (used by rmcp)
- Rust 2024 edition, pinned to 1.85.0

---
title: MCP (Model Context Protocol)
tags: [concept, mcp]
updated: 2026-05-03
---

# MCP (Model Context Protocol)

MCP is a protocol that lets AI agents (like Claude) invoke external tools via a standardized interface. An MCP server exposes a set of named tools, each with an input schema. The AI calls a tool by name with structured arguments; the server executes the work and returns a result.

## Role in this project

Each MCP tool maps directly to a CLI command. The CLI does the real work; MCP exposes it to AI agents. Logic is never added directly to the MCP layer.

## How the mapping works

```
AI agent calls tool "get_time" with args {"utc": true}
        ↓
MCP server receives the call (src/commands/mcp.rs)
        ↓
Dispatches to mcp_tools::get_time::call()
        ↓
Calls commands::time::format_time(Utc::now())
        ↓
Returns formatted time string to AI agent
```

The MCP tools call the shared CLI logic directly (library linking) — no subprocess, no serialization round-trip for arguments.

## Transport

The server uses stdio transport — the MCP host (e.g. Claude Desktop) spawns the binary as a subprocess and communicates over stdin/stdout using JSON-RPC 2.0. The `rmcp` crate handles the protocol.

## Available Tools

| Tool | Arguments | Behavior |
|------|-----------|----------|
| `get_time` | `utc?: boolean` | Returns current local time, or UTC if `utc=true` |

## Related

- [[concepts/cli-as-source-of-truth]] — why CLI is the canonical layer
- [[architecture/overview]] — how MCP fits in the project
- [[architecture/mcp-server]] — implementation details of the MCP server

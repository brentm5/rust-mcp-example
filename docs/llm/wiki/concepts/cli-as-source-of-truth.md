---
title: CLI as Source of Truth
tags: [concept, architecture, mcp]
updated: 2026-05-03
---

# CLI as Source of Truth

The central design principle of this project: **all command logic lives in CLI subcommands. The MCP server is a thin wrapper that invokes those commands.**

## Why

If logic lives in the MCP layer, it becomes inaccessible from the terminal. Testing requires an MCP client. Debugging requires an AI agent. The command can't be composed into shell scripts.

If logic lives in the CLI layer, it gets two surfaces for free:
- Human-invocable from the terminal
- AI-agent-invocable via MCP tool calls

The MCP server becomes a translation layer: it receives a tool call, maps it to the right CLI command + args, runs it, and returns the output.

## Practical implication

When adding a new capability:

1. Implement it as a CLI subcommand first (`src/commands/<name>.rs`)
2. Wire it into MCP second (`src/mcp_tools/<name>.rs` + register in `mcp.rs`)

Never add logic directly to the MCP layer — the tool handler should only deserialize arguments and delegate to the command's shared function.

## Related

- [[architecture/overview]] — where this fits in the project
- [[concepts/mcp]] — what MCP is and how tool calls work
- [[architecture/command-module-pattern]] — how CLI commands are structured

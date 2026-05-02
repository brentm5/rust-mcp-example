---
title: Project Overview
tags: [architecture, mcp, cli]
updated: 2026-05-03
---

# Project Overview

**rust-mcp-example** is a playground for learning how to build MCP (Model Context Protocol) servers in Rust.

## Core Pattern

The project demonstrates one specific pattern:

> Build CLI subcommands that do useful work. Then expose those same subcommands as MCP tool calls.

The CLI is the source of truth. The MCP server is a thin wrapper. Logic lives in one place — a CLI subcommand — and gets two surfaces for free: direct human invocation and AI agent invocation via MCP.

See [[concepts/cli-as-source-of-truth]] for the rationale.

## Current State

The MCP server is fully implemented. What exists:

- A working CLI binary (`rust-mcp-example`) with a `time` subcommand
- An `mcp` subcommand that starts a stdio MCP server (compatible with Claude Desktop and Claude Code)
- A `get_time` MCP tool that mirrors the `time` command's behavior
- A modular structure for adding further commands and tools

## Architecture Layers

```
CLI (clap)                        MCP (rmcp)
─────────────────────             ─────────────────────────
src/commands/time.rs  ◄────────  src/mcp_tools/get_time.rs
  pub format_time()   shared
src/commands/mcp.rs              ← MCP server entry point
```

Both layers share `format_time()` from `commands/time.rs`. The MCP tool calls it directly — no subprocess overhead.

## Related

- [[architecture/command-module-pattern]] — how commands are organized
- [[architecture/dispatch-flow]] — how execution flows through the CLI
- [[architecture/mcp-server]] — how the MCP server is structured
- [[concepts/mcp]] — what MCP is

---
title: sys_info MCP tool
tags: [mcp-tool, sysinfo]
updated: 2026-05-03
sources: [src/mcp_tools/sys_info.rs, src/commands/sys_info.rs]
---

# `sys_info` MCP Tool

Returns a human-readable system snapshot as a text string. Mirrors the behavior of the `sys-info` CLI subcommand.

## Arguments

None — the tool takes no input parameters and always returns the full snapshot.

## Output

A single text content item with the formatted system info block:

```
Hostname:   my-macbook.local
OS:         macOS 15.3.1
Kernel:     24.3.0
Arch:       aarch64
CPU:        Apple M3 Pro (12 cores)
RAM:        36 GB
User:       brentm5
```

## Implementation

- File: `src/mcp_tools/sys_info.rs`
- `NAME = "sys_info"` — shared constant used in definition and dispatch
- `Args` struct — empty, `#[derive(Deserialize, JsonSchema, Default)]`, generates an empty-object JSON Schema
- `definition()` — constructs the `Tool` with name, description, and schema
- `call()` — takes no arguments, calls `format_sys_info(&collect_sys_info())` directly

Unlike `get_time::call()`, this function takes no `arguments` parameter since there are no inputs to deserialize.

## Shared logic

Uses `collect_sys_info()` and `format_sys_info()` from `src/commands/sys_info.rs` — the same functions used by the CLI `sys-info` command.

## Related

- [[commands/sys_info]] — the CLI command this tool mirrors
- [[architecture/mcp-server]] — how tools are registered and dispatched

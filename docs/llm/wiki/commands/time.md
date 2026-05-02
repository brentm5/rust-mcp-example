---
title: time command
tags: [command, cli, chrono]
updated: 2026-05-03
sources: [src/commands/time.rs, docs/superpowers/specs/2026-05-02-time-subcommand-design.md]
---

# `time` Command

Prints the current time — local by default, UTC with `--utc`.

## Usage

```bash
rust-mcp-example time           # local time
rust-mcp-example time --utc     # UTC time
rust-mcp-example --debug time   # debug flag + local time
```

## Output format

```
Saturday May 3 2026 10:45 AM
```

Format string: `%A %B %-d %Y %I:%M %p`
- `%A` — full weekday name
- `%B` — full month name
- `%-d` — day without leading zero
- `%Y` — 4-digit year
- `%I:%M %p` — 12-hour clock with AM/PM

## Implementation

- File: `src/commands/time.rs`
- `TimeArgs` struct: single `--utc: bool` flag
- `pub format_time<Tz>()` — public generic helper shared with the `get_time` MCP tool
- `run()` prints UTC time if `args.utc`, otherwise local time

## Shared logic

`format_time` is `pub` so `src/mcp_tools/get_time.rs` can reuse it directly. Both surfaces (CLI and MCP) format time the same way from the same function.

## Out of scope (per design spec)

- Timezone selection (`--tz`)
- Machine-readable output (ISO 8601, Unix timestamp, etc.)

## Related

- [[architecture/command-module-pattern]] — the pattern this command implements
- [[concepts/clap-derive-api]] — how `TimeArgs` is wired to clap
- [[mcp-tools/get_time]] — the MCP tool that mirrors this command

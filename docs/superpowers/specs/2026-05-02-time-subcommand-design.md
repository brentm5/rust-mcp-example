# Time Subcommand Design

**Date:** 2026-05-02
**Status:** Approved

## Overview

Add a `time` subcommand to the `rust-mcp-example` CLI that prints the current local time in a human-readable format. An optional `--utc` flag also prints the UTC equivalent.

## CLI Structure

The top-level CLI is refactored from a flat flags model to a subcommand model using clap's derive API.

```
rust-mcp-example [--debug] <COMMAND>

Commands:
  time    Print the current time

Options:
  --debug    Enable debug mode
  --help     Print help
  --version  Print version
```

`--debug` remains a global flag on the root `Args` struct.

## `time` Subcommand

```
rust-mcp-example time [--utc]

Options:
  --utc    Also print UTC time
```

**Without `--utc`:**
```
Saturday May 2 2026 05:30 PM
```

**With `--utc`:**
```
Saturday May 2 2026 05:30 PM
UTC: Saturday May 2 2026 09:30 PM
```

## Dependencies

Add to `Cargo.toml`:
```toml
chrono = { version = "0.4", features = ["clock"] }
```

`chrono` is the standard Rust crate for date/time. The `clock` feature is required for `Local::now()` and `Utc::now()`.

## Implementation Notes

- `Args` struct holds `--debug` and a `Commands` enum via `#[command(subcommand)]`
- `Commands` is an enum with a single variant `Time(TimeArgs)`
- `TimeArgs` holds `--utc: bool`
- Format string: `"%A %B %-d %Y %I:%M %p"` — produces `Saturday May 2 2026 05:30 PM` (no leading zero on day; leading zero on hour is fine)

## Out of Scope

- Timezone selection (e.g. `--tz America/New_York`)
- Machine-readable output formats
- Any other subcommands

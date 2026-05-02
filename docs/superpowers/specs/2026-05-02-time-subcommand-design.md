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
12:30 PM on Saturday, May 2nd
```

**With `--utc`:**
```
12:30 PM on Saturday, May 2nd
UTC: 7:30 PM on Saturday, May 2nd
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
- Format string: `"%-I:%M %p on %A, %B %-d"` — produces `12:30 PM on Saturday, May 2nd` (no leading zero on hour/day; chrono doesn't support ordinal suffixes natively, so the day will be numeric without `st/nd/rd/th`)
- Ordinal suffix (1st, 2nd, 3rd) is not supported natively by chrono's format strings; a small helper function will compute the suffix and interpolate it into the output string

## Out of Scope

- Timezone selection (e.g. `--tz America/New_York`)
- Machine-readable output formats
- Any other subcommands

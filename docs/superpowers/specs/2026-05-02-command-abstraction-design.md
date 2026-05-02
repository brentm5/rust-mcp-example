# Command Abstraction Design

**Date:** 2026-05-02
**Status:** Approved

## Overview

Refactor `src/main.rs` to move command logic into a `src/commands/` module, one file per command. This gives each command a clear, isolated home and makes adding future commands a consistent, low-friction pattern.

## File Structure

```
src/
  main.rs              — Cli struct, Commands enum, main() dispatch only
  commands/
    mod.rs             — declares submodules (pub mod time;)
    time.rs            — TimeArgs, format_time(), pub fn run(args: &TimeArgs)
```

## Responsibilities

**`src/main.rs`**
- Defines `Cli` (global `--debug` flag, `#[command(subcommand)] command: Commands`)
- Defines `Commands` enum with variants referencing each command's args type (e.g. `Time(commands::time::TimeArgs)`)
- `main()` parses args, checks `--debug`, dispatches to the appropriate `commands::<name>::run()`
- No business logic

**`src/commands/mod.rs`**
- Declares submodules: `pub mod time;`
- No logic

**`src/commands/time.rs`**
- `pub struct TimeArgs` with `--utc: bool`
- `fn format_time<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> String` (private helper)
- `pub fn run(args: &TimeArgs)` — prints local time, optionally UTC

## Adding a New Command (Pattern)

1. Create `src/commands/<name>.rs` with `pub struct <Name>Args`, any helpers, and `pub fn run(args: &<Name>Args)`
2. Add `pub mod <name>;` to `src/commands/mod.rs`
3. Add a variant to `Commands` in `main.rs` and a match arm in `main()`

## Out of Scope

- Trait-based command abstraction (YAGNI at this stage)
- Moving `Cli` or `Commands` out of `main.rs`
- Any behavioral changes to the `time` command

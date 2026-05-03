---
title: Command Module Pattern
tags: [architecture, commands, rust]
updated: 2026-05-02
sources: [docs/superpowers/specs/2026-05-02-command-abstraction-design.md]
---

# Command Module Pattern

Each CLI subcommand lives in its own file under `src/commands/`. The pattern is a naming convention, not a trait abstraction — kept intentionally simple (YAGNI).

## Structure

```text
src/
  main.rs           — Cli struct, Commands enum, dispatch only
  commands/
    mod.rs          — pub mod declarations, no logic
    time.rs         — TimeArgs, helpers, pub fn run()
    <name>.rs       — future commands follow the same shape
```

## Per-command file contract

Every `src/commands/<name>.rs` exports exactly two public items:

| Export | Type | Purpose |
|--------|------|---------|
| `<Name>Args` | `struct` with `#[derive(Args)]` | Holds clap-parsed flags for this command |
| `run(args: &<Name>Args)` | `fn` | Executes the command |

Private helpers (like `format_time` in `time.rs`) stay in the file — not exported.

## Wiring a new command

1. Create `src/commands/<name>.rs` with `pub struct <Name>Args` and `pub fn run`
2. Add `pub mod <name>;` to `src/commands/mod.rs`
3. Add `<Name>(<commands::name::NameArgs>)` variant to `Commands` enum in `main.rs`
4. Add a match arm in `main()`: `Commands::<Name>(args) => commands::<name>::run(&args)`

## Design decisions

- No `Command` trait — avoids generics/dyn overhead for a simple dispatch pattern
- `Cli` and `Commands` stay in `main.rs` — they're entry-point concerns, not reusable modules
- `run()` takes `&Args` not owned — args are only needed for the duration of the command

## Related

- [[architecture/dispatch-flow]] — how main.rs calls into these modules
- [[commands/time]] — the first implementation of this pattern
- Source: `docs/superpowers/specs/2026-05-02-command-abstraction-design.md`

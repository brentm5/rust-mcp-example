# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build                          # debug build
cargo build --release                # release build
cargo run -- <subcommand> [args]     # run the CLI
cargo run -- sys-info              # print system information
cargo clippy -- -D warnings          # lint (CI enforces zero warnings)
cargo test                           # run tests
cargo test <test_name>               # run a single test
```

CI runs clippy and a release build targeting `aarch64-apple-darwin`. Run clippy locally before pushing.

## Architecture

The goal of this project is to expose CLI subcommands as MCP tool calls — CLI is the source of truth, MCP is a thin layer on top.

**Dispatch flow:** `main.rs` owns the `Cli` struct (global `--debug` flag) and `Commands` enum. Each enum variant holds its command's `Args` struct. `main()` parses, checks `--debug`, then dispatches to `commands::<name>::run(&args)`.

**Command module pattern:** Each subcommand lives in `src/commands/<name>.rs` and exports two things: `pub struct <Name>Args` (clap `Args` derive) and `pub fn run(args: &<Name>Args)`. The module is declared in `src/commands/mod.rs`. No trait abstraction — just a consistent naming convention.

**Adding a command:**
1. Create `src/commands/<name>.rs` with `pub struct <Name>Args` and `pub fn run`
2. Add `pub mod <name>;` to `src/commands/mod.rs`
3. Add a variant to `Commands` and a match arm in `main()`

## Toolchain

Rust 1.85.0 (pinned via `rust-toolchain.toml` and `mise.toml`). Uses Rust 2024 edition.

## Maintenance

Keep this file current. When adding commands, changing the architecture, or updating the toolchain, update the relevant section here. Also update `docs/llm/wiki/` — ingest any significant changes as wiki pages and append an entry to `docs/llm/log.md`.

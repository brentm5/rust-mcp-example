---
title: clap Derive API
tags: [concept, rust, clap, cli]
updated: 2026-05-02
---

# clap Derive API

This project uses [clap](https://docs.rs/clap) 4.6.1 with the `derive` feature, which generates argument parsing code from annotated structs and enums.

## How it's used here

**Root struct** (`Cli` in `main.rs`):
```rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}
```
`#[derive(Parser)]` makes `Cli::parse()` work. `#[command(subcommand)]` signals that `command` is an enum of subcommands.

**Subcommand enum** (`Commands` in `main.rs`):
```rust
#[derive(Subcommand, Debug)]
enum Commands {
    Time(commands::time::TimeArgs),
}
```
Each variant is a subcommand. The tuple payload is the args struct for that subcommand.

**Per-command args** (e.g. `TimeArgs` in `commands/time.rs`):
```rust
#[derive(Args, Debug)]
pub struct TimeArgs {
    #[arg(long)]
    utc: bool,
}
```
`#[derive(Args)]` (not `Parser`) — this struct is composed into the parent, not parsed as a root.

## Key distinction

| Derive | Used on | When |
|--------|---------|------|
| `Parser` | Root CLI struct | Entry point for parsing |
| `Subcommand` | Enum of subcommands | Dispatches to subcommand args |
| `Args` | Per-subcommand struct | Composed into a subcommand variant |

## Related

- [[architecture/command-module-pattern]] — how Args structs fit the module pattern
- [[architecture/dispatch-flow]] — how parsed types flow to execution

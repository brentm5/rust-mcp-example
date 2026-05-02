# Command Abstraction Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move command logic out of `src/main.rs` into a `src/commands/` module, one file per command, with no behavioral changes.

**Architecture:** Create `src/commands/mod.rs` (module declarations only) and `src/commands/time.rs` (owns `TimeArgs`, `format_time`, and `run()`). `main.rs` is reduced to CLI struct, `Commands` enum, and dispatch in `main()`.

**Tech Stack:** Rust, clap 4.x (derive API), chrono 0.4

---

### Task 1: Create `src/commands/time.rs`

**Files:**
- Create: `src/commands/time.rs`

- [ ] **Step 1: Create `src/commands/time.rs` with this exact content**

```rust
use chrono::{Local, Utc};
use clap::Args;

#[derive(Args, Debug)]
pub struct TimeArgs {
    /// Also print UTC time
    #[arg(long)]
    pub utc: bool,
}

fn format_time<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format("%A %B %-d %Y %I:%M %p").to_string()
}

pub fn run(args: &TimeArgs) {
    println!("{}", format_time(Local::now()));
    if args.utc {
        println!("UTC: {}", format_time(Utc::now()));
    }
}
```

Note: `TimeArgs` and its `utc` field are `pub` so `main.rs` can reference the type in the `Commands` enum. `format_time` is private — it's an implementation detail of this module.

---

### Task 2: Create `src/commands/mod.rs`

**Files:**
- Create: `src/commands/mod.rs`

- [ ] **Step 1: Create `src/commands/mod.rs` with this exact content**

```rust
pub mod time;
```

---

### Task 3: Update `src/main.rs`

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace the entire contents of `src/main.rs` with**

```rust
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print the current time
    Time(commands::time::TimeArgs),
}

fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => commands::time::run(&time_args),
    }
}
```

- [ ] **Step 2: Build and verify no errors or warnings**

```bash
cargo build
```

Expected output (no errors, no warnings):
```
   Compiling rust-mcp-example v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in Xs
```

- [ ] **Step 3: Smoke test — local time**

```bash
cargo run -- time
```

Expected: one line like `Saturday May 2 2026 12:53 PM`

- [ ] **Step 4: Smoke test — UTC flag**

```bash
cargo run -- time --utc
```

Expected: two lines — local time, then `UTC: <utc time>`

- [ ] **Step 5: Smoke test — debug flag**

```bash
cargo run -- --debug time
```

Expected: `Debug mode is on` followed by local time

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/commands/mod.rs src/commands/time.rs
git commit -m "refactor: move command logic into src/commands/ module"
```

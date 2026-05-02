# Time Subcommand Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `time` subcommand to the CLI that prints the current local time in a human-readable format, with an optional `--utc` flag to also print UTC time.

**Architecture:** Refactor `Args` to use clap's subcommand model, keeping `--debug` as a global flag. Add a `Time` subcommand with `--utc`. Format time using the `chrono` crate with the pattern `%A %B %-d %Y %I:%M %p`.

**Tech Stack:** Rust, clap 4.x (derive API), chrono 0.4

---

### Task 1: Add chrono dependency

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add chrono to Cargo.toml**

Edit `Cargo.toml` so `[dependencies]` reads:

```toml
[dependencies]
clap = { version = "4.6.1", features = ["derive"] }
chrono = { version = "0.4", features = ["clock"] }
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build
```

Expected: compiles successfully (no errors).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat: add chrono dependency"
```

---

### Task 2: Refactor CLI to subcommand model and add `time` subcommand

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace the contents of src/main.rs**

```rust
use chrono::{Local, Utc};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print the current time
    Time(TimeArgs),
}

#[derive(Parser, Debug)]
struct TimeArgs {
    /// Also print UTC time
    #[arg(long)]
    utc: bool,
}

fn format_time<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format("%A %B %-d %Y %I:%M %p").to_string()
}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => {
            println!("{}", format_time(Local::now()));
            if time_args.utc {
                println!("UTC: {}", format_time(Utc::now()));
            }
        }
    }
}
```

- [ ] **Step 2: Build and verify it compiles**

```bash
cargo build
```

Expected: compiles successfully with no errors or warnings.

- [ ] **Step 3: Smoke test local time output**

```bash
cargo run -- time
```

Expected: output like `Saturday May 2 2026 05:30 PM` (actual current local time).

- [ ] **Step 4: Smoke test --utc flag**

```bash
cargo run -- time --utc
```

Expected: two lines — local time first, then `UTC: <utc time>`.

- [ ] **Step 5: Smoke test --debug flag still works**

```bash
cargo run -- --debug time
```

Expected: `Debug mode is on` followed by the current local time.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat: add time subcommand with --utc flag"
```

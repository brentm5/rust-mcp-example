---
title: Dispatch Flow
tags: [architecture, rust, clap]
updated: 2026-05-02
---

# Dispatch Flow

How a CLI invocation flows from binary entry to command execution.

## Flow

```
rust-mcp-example --debug time --utc
        │
        ▼
    Cli::parse()              ← clap parses argv into Cli { debug, command }
        │
        ▼
    args.debug check          ← prints "Debug mode is on" if set
        │
        ▼
    match args.command        ← pattern match on Commands enum
        │
        ▼
    Commands::Time(time_args) ← destructures the variant's Args struct
        │
        ▼
    commands::time::run(&time_args)   ← delegates to the command module
```

## Key types

| Type | Location | Role |
|------|----------|------|
| `Cli` | `src/main.rs` | Root struct — global flags + subcommand |
| `Commands` | `src/main.rs` | Enum of all subcommands |
| `<Name>Args` | `src/commands/<name>.rs` | Per-command flag struct |

## Global flags

`--debug` is the only global flag. It sits on `Cli`, not on any subcommand `Args`. It fires before dispatch. Commands don't receive a debug parameter — they have no awareness of it.

## Related

- [[architecture/command-module-pattern]] — structure of command modules
- [[concepts/clap-derive-api]] — how clap wires up the types

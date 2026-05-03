# Notes Feature — LanceDB Design

**Date:** 2026-05-03  
**Status:** Approved

## Overview

Add persistent note storage to `rust-mcp-example` using LanceDB as the embedded database. Notes can be saved, retrieved by ID, searched by keyword, and listed. The feature is exposed via both CLI subcommands and MCP tools, sharing the same `NoteStore` abstraction.

## Data Model

A `Note` has three fields:

| Field | Type | Description |
|---|---|---|
| `id` | `String` | UUID v4, auto-generated on save |
| `name` | `String` | Human-readable label (not unique) |
| `message` | `String` | Note content |

`name` is a descriptive label, not a unique key. `id` is the sole identity. Saving always creates a new note — there is no upsert.

## Storage

- **Database:** LanceDB (embedded, no server process)
- **Location:** `~/.local/state/rust-mcp-example/` — created on first use
- **Table name:** `notes`
- **Search:** LanceDB full-text search (FTS) index on `name` and `message` — no vector embeddings required

## Architecture: `NoteStore`

**File:** `src/notes.rs`

```rust
pub struct NoteStore {
    table: lancedb::Table,
}
```

`NoteStore::open(db_path: &Path) -> Result<Self>` opens or creates the LanceDB connection, opens or creates the `notes` table with the correct schema, and creates the FTS index if not already present.

**Methods:**

- `save(&self, name: &str, message: &str) -> Result<Note>` — generates UUID, inserts row, returns created `Note`
- `retrieve(&self, id: &str) -> Result<Option<Note>>` — fetches single note by exact `id`
- `search(&self, query: &str) -> Result<Vec<Note>>` — FTS query across `name` and `message`
- `list(&self) -> Result<Vec<Note>>` — returns all notes in insertion order

All methods return `Result<_, Box<dyn std::error::Error>>`.

## CLI Commands

New nested subcommand group `notes` added to `Commands` in `main.rs`. Implementation in `src/commands/notes.rs`.

The `notes` subcommand group accepts a shared `--db-path <path>` option that overrides the default storage location (`~/.local/state/rust-mcp-example/`). This applies to all four subcommands:

```
cargo run -- notes --db-path /tmp/my-notes save --name "foo" --message "bar"
cargo run -- notes --db-path /tmp/my-notes list
```

| Command | Args | Output |
|---|---|---|
| `notes save` | `--name <name> --message <msg>` | Prints generated UUID |
| `notes retrieve` | `--id <id>` | Prints name + message, or "not found" |
| `notes search` | `--query <q>` | Prints matching notes (id, name, message) |
| `notes list` | none | Prints all notes (id, name, message) |

`--db-path` is an optional clap arg on the `NotesArgs` parent struct (the subcommand group), defaulting to `~/.local/state/rust-mcp-example/`. Each subcommand receives the resolved path and passes it to `NoteStore::open()`. Output is plain text matching the style of existing commands (e.g., `sys-info`). CLI commands open a fresh `NoteStore` per invocation — appropriate for short-lived processes.

## MCP Tools

Four new tool files in `src/mcp_tools/`, each following the existing `definition()` + `call()` pattern:

| File | Tool name | Args | Returns |
|---|---|---|---|
| `notes_save.rs` | `notes_save` | `name`, `message` | New UUID |
| `notes_retrieve.rs` | `notes_retrieve` | `id` | Note or "not found" |
| `notes_search.rs` | `notes_search` | `query` | Matching notes as text |
| `notes_list.rs` | `notes_list` | none | All notes as text |

All four are registered in `mcp.rs` `list_tools()` and `call_tool()`.

**Persistent connection:** `McpServer` holds `Arc<NoteStore>`, opened once at startup in `mcp::run()` and shared across all request handler clones (rmcp requires `McpServer: Clone`; `Arc` makes this trivially cheap). LanceDB handles internal concurrency.

## Error Handling

- CLI: print error message to stderr, exit with non-zero code
- MCP tools: convert errors to `ErrorData::internal_error(...)`

## Dependencies

New entries in `Cargo.toml`:

| Crate | Purpose |
|---|---|
| `lancedb` | Embedded database |
| `arrow-array` | LanceDB row construction |
| `arrow-schema` | LanceDB schema definition |
| `uuid` (feature: `v4`) | UUID generation |
| `futures` | Collecting LanceDB async streams |
| `tempfile` (dev) | Temp dir for integration tests |

`tokio` is already present. CLI commands that call async `NoteStore` methods use a small `tokio::runtime::Runtime::new()` block, matching the pattern in `mcp.rs`.

## Testing

One integration test module in `src/notes.rs` using `tempfile::tempdir()`:

- Save a note → retrieve by returned ID → assert fields match
- Save two notes → search by keyword present in one → assert only correct note returned
- Save two notes → list → assert both present

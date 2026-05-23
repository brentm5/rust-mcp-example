# Notes Feature (LanceDB) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add persistent note storage (save, retrieve, search, list) backed by LanceDB, exposed via both CLI subcommands and MCP tools.

**Architecture:** A `NoteStore` struct in `src/notes.rs` owns all LanceDB interactions and is shared by CLI commands (fresh instance per invocation) and the MCP server (`Arc<NoteStore>` held in `McpServer`). CLI commands live in `src/commands/notes.rs` as a nested clap subcommand group with a `--db-path` override. Four MCP tool files in `src/mcp_tools/` wrap `NoteStore` methods.

**Tech Stack:** Rust 1.85 / 2024 edition, LanceDB 0.27, arrow-array, arrow-schema, uuid v4, futures, tokio (already present), tempfile (dev/test)

---

## File Map

| Action | Path | Responsibility |
|---|---|---|
| Create | `src/notes.rs` | `Note` struct, `NoteStore` struct and all DB methods, integration tests |
| Create | `src/commands/notes.rs` | CLI arg structs for `notes` subcommand group + four subcommands, `run()` |
| Modify | `src/commands/mod.rs` | Add `pub mod notes;` |
| Modify | `src/main.rs` | Add `Notes` variant to `Commands`, dispatch arm |
| Create | `src/mcp_tools/notes_save.rs` | MCP tool: `notes_save` |
| Create | `src/mcp_tools/notes_retrieve.rs` | MCP tool: `notes_retrieve` |
| Create | `src/mcp_tools/notes_search.rs` | MCP tool: `notes_search` |
| Create | `src/mcp_tools/notes_list.rs` | MCP tool: `notes_list` |
| Modify | `src/mcp_tools/mod.rs` | Add `pub mod` for the four new tool files |
| Modify | `src/commands/mcp.rs` | Add `Arc<NoteStore>` to `McpServer`, register four tools |
| Modify | `Cargo.toml` | Add lancedb, arrow-array, arrow-schema, uuid, futures, tempfile |

---

## Task 1: Add dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add new dependencies**

Open `Cargo.toml` and add to `[dependencies]`:

```toml
lancedb = "0.27"
arrow-array = "54"
arrow-schema = "54"
uuid = { version = "1", features = ["v4"] }
futures = "0.3"
```

And add a `[dev-dependencies]` section (or append to it if it exists):

```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build 2>&1 | head -30
```

Expected: No errors (warnings about unused deps are fine at this stage).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: add lancedb, arrow, uuid, futures dependencies"
```

---

## Task 2: Define `Note` struct and `NoteStore` skeleton

**Files:**
- Create: `src/notes.rs`
- Modify: `src/main.rs` (add `mod notes;`)

- [ ] **Step 1: Create `src/notes.rs` with the `Note` struct and `NoteStore` skeleton**

```rust
use std::path::Path;
use std::sync::Arc;

use arrow_array::{RecordBatch, RecordBatchIterator, StringArray};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::index::scalar::FtsIndexBuilder;
use lancedb::index::Index;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{Connection, Table};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub name: String,
    pub message: String,
}

fn note_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
    ]))
}

pub struct NoteStore {
    table: Table,
}

impl NoteStore {
    pub async fn open(db_path: &Path) -> Result<Self> {
        todo!()
    }

    pub async fn save(&self, name: &str, message: &str) -> Result<Note> {
        todo!()
    }

    pub async fn retrieve(&self, id: &str) -> Result<Option<Note>> {
        todo!()
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Note>> {
        todo!()
    }

    pub async fn list(&self) -> Result<Vec<Note>> {
        todo!()
    }
}

fn batches_to_notes(batches: Vec<RecordBatch>) -> Result<Vec<Note>> {
    let mut notes = Vec::new();
    for batch in batches {
        let ids = batch
            .column_by_name("id")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing id column")?;
        let names = batch
            .column_by_name("name")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing name column")?;
        let messages = batch
            .column_by_name("message")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing message column")?;
        for i in 0..batch.num_rows() {
            notes.push(Note {
                id: ids.value(i).to_string(),
                name: names.value(i).to_string(),
                message: messages.value(i).to_string(),
            });
        }
    }
    Ok(notes)
}
```

- [ ] **Step 2: Declare the module in `src/main.rs`**

Add `mod notes;` near the top with the other mod declarations:

```rust
mod commands;
mod mcp_tools;
mod notes;
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo build 2>&1 | head -30
```

Expected: Compiles (with warnings about `todo!()` dead code being fine).

- [ ] **Step 4: Commit**

```bash
git add src/notes.rs src/main.rs
git commit -m "feat: add Note struct and NoteStore skeleton"
```

---

## Task 3: Implement `NoteStore::open` and `NoteStore::save`

**Files:**
- Modify: `src/notes.rs`

- [ ] **Step 1: Write failing integration tests for `open` and `save`**

Add a `#[cfg(test)]` block at the bottom of `src/notes.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    #[test]
    fn test_save_returns_note_with_uuid() {
        rt().block_on(async {
            let dir = tempdir().unwrap();
            let store = NoteStore::open(dir.path()).await.unwrap();
            let note = store.save("test note", "hello world").await.unwrap();
            assert!(!note.id.is_empty());
            assert_eq!(note.name, "test note");
            assert_eq!(note.message, "hello world");
            // UUID format: 8-4-4-4-12
            assert_eq!(note.id.len(), 36);
        });
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test test_save_returns_note_with_uuid 2>&1 | tail -20
```

Expected: FAIL — panics at `todo!()`.

- [ ] **Step 3: Implement `NoteStore::open`**

Replace the `todo!()` in `open`:

```rust
pub async fn open(db_path: &Path) -> Result<Self> {
    std::fs::create_dir_all(db_path)?;
    let conn: Connection = lancedb::connect(db_path.to_str().ok_or("invalid path")?)
        .execute()
        .await?;

    let table = if conn.table_names().execute().await?.contains(&"notes".to_string()) {
        conn.open_table("notes").execute().await?
    } else {
        let schema = note_schema();
        let empty = RecordBatchIterator::new(std::iter::empty(), schema.clone());
        let tbl = conn
            .create_table("notes", Box::new(empty))
            .execute()
            .await?;
        tbl.create_index(&["name", "message"], Index::FTS(FtsIndexBuilder::default()))
            .execute()
            .await?;
        tbl
    };

    Ok(Self { table })
}
```

- [ ] **Step 4: Implement `NoteStore::save`**

Replace the `todo!()` in `save`:

```rust
pub async fn save(&self, name: &str, message: &str) -> Result<Note> {
    let id = Uuid::new_v4().to_string();
    let schema = note_schema();
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec![id.as_str()])),
            Arc::new(StringArray::from(vec![name])),
            Arc::new(StringArray::from(vec![message])),
        ],
    )?;
    let reader = RecordBatchIterator::new(std::iter::once(Ok(batch)), schema);
    self.table.add(Box::new(reader)).execute().await?;
    Ok(Note {
        id,
        name: name.to_string(),
        message: message.to_string(),
    })
}
```

- [ ] **Step 5: Run test to verify it passes**

```bash
cargo test test_save_returns_note_with_uuid 2>&1 | tail -20
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/notes.rs
git commit -m "feat: implement NoteStore::open and save"
```

---

## Task 4: Implement `NoteStore::retrieve`

**Files:**
- Modify: `src/notes.rs`

- [ ] **Step 1: Write failing test**

Add to the `tests` module in `src/notes.rs`:

```rust
#[test]
fn test_retrieve_by_id() {
    rt().block_on(async {
        let dir = tempdir().unwrap();
        let store = NoteStore::open(dir.path()).await.unwrap();
        let saved = store.save("my note", "content here").await.unwrap();
        let found = store.retrieve(&saved.id).await.unwrap();
        assert!(found.is_some());
        let note = found.unwrap();
        assert_eq!(note.id, saved.id);
        assert_eq!(note.name, "my note");
        assert_eq!(note.message, "content here");
    });
}

#[test]
fn test_retrieve_missing_id_returns_none() {
    rt().block_on(async {
        let dir = tempdir().unwrap();
        let store = NoteStore::open(dir.path()).await.unwrap();
        let result = store.retrieve("nonexistent-id").await.unwrap();
        assert!(result.is_none());
    });
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test test_retrieve 2>&1 | tail -20
```

Expected: FAIL — panics at `todo!()`.

- [ ] **Step 3: Implement `NoteStore::retrieve`**

Replace the `todo!()` in `retrieve`:

```rust
pub async fn retrieve(&self, id: &str) -> Result<Option<Note>> {
    let filter = format!("id = '{}'", id.replace('\'', "''"));
    let batches: Vec<RecordBatch> = self
        .table
        .query()
        .only_if(&filter)
        .execute()
        .await?
        .try_collect()
        .await?;
    let notes = batches_to_notes(batches)?;
    Ok(notes.into_iter().next())
}
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
cargo test test_retrieve 2>&1 | tail -20
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/notes.rs
git commit -m "feat: implement NoteStore::retrieve"
```

---

## Task 5: Implement `NoteStore::search` and `NoteStore::list`

**Files:**
- Modify: `src/notes.rs`

- [ ] **Step 1: Write failing tests**

Add to the `tests` module in `src/notes.rs`:

```rust
#[test]
fn test_search_returns_matching_notes() {
    rt().block_on(async {
        let dir = tempdir().unwrap();
        let store = NoteStore::open(dir.path()).await.unwrap();
        store.save("shopping list", "buy milk and eggs").await.unwrap();
        store.save("work todo", "finish the report").await.unwrap();
        let results = store.search("milk").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "shopping list");
    });
}

#[test]
fn test_list_returns_all_notes() {
    rt().block_on(async {
        let dir = tempdir().unwrap();
        let store = NoteStore::open(dir.path()).await.unwrap();
        store.save("note one", "first").await.unwrap();
        store.save("note two", "second").await.unwrap();
        let all = store.list().await.unwrap();
        assert_eq!(all.len(), 2);
    });
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test test_search test_list 2>&1 | tail -20
```

Expected: FAIL — panics at `todo!()`.

- [ ] **Step 3: Implement `NoteStore::search`**

Replace the `todo!()` in `search`. Note: FTS requires the index to exist — it was created in `open`. After adding rows, we need to optimize the index before searching so the new rows are indexed:

```rust
pub async fn search(&self, query: &str) -> Result<Vec<Note>> {
    use lancedb::index::scalar::FullTextSearchQuery;
    // Optimize ensures newly inserted rows are visible to FTS
    self.table.optimize(lancedb::table::OptimizeAction::All).await?;
    let batches: Vec<RecordBatch> = self
        .table
        .query()
        .full_text_search(FullTextSearchQuery::new(query.to_string()))
        .execute()
        .await?
        .try_collect()
        .await?;
    batches_to_notes(batches)
}
```

- [ ] **Step 4: Implement `NoteStore::list`**

Replace the `todo!()` in `list`:

```rust
pub async fn list(&self) -> Result<Vec<Note>> {
    let batches: Vec<RecordBatch> = self
        .table
        .query()
        .execute()
        .await?
        .try_collect()
        .await?;
    batches_to_notes(batches)
}
```

- [ ] **Step 5: Run all tests to verify they pass**

```bash
cargo test 2>&1 | tail -20
```

Expected: All tests PASS.

- [ ] **Step 6: Commit**

```bash
git add src/notes.rs
git commit -m "feat: implement NoteStore::search and list"
```

---

## Task 6: CLI commands

**Files:**
- Create: `src/commands/notes.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create `src/commands/notes.rs`**

```rust
use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::notes::NoteStore;

fn default_db_path() -> PathBuf {
    let mut p = dirs::state_dir()
        .unwrap_or_else(|| PathBuf::from(".local/state"));
    p.push("rust-mcp-example");
    p
}

fn format_note(note: &crate::notes::Note) -> String {
    format!("id:      {}\nname:    {}\nmessage: {}", note.id, note.name, note.message)
}

#[derive(Args, Debug)]
pub struct NotesArgs {
    /// Override the database directory (default: ~/.local/state/rust-mcp-example)
    #[arg(long, global = true)]
    pub db_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: NotesCommands,
}

#[derive(Subcommand, Debug)]
pub enum NotesCommands {
    /// Save a new note
    Save(SaveArgs),
    /// Retrieve a note by ID
    Retrieve(RetrieveArgs),
    /// Search notes by keyword
    Search(SearchArgs),
    /// List all notes
    List,
}

#[derive(Args, Debug)]
pub struct SaveArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub message: String,
}

#[derive(Args, Debug)]
pub struct RetrieveArgs {
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    #[arg(long)]
    pub query: String,
}

pub fn run(args: &NotesArgs) {
    let db_path = args.db_path.clone().unwrap_or_else(default_db_path);
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(async {
        let store = match NoteStore::open(&db_path).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error opening notes database: {e}");
                std::process::exit(1);
            }
        };
        match &args.command {
            NotesCommands::Save(a) => match store.save(&a.name, &a.message).await {
                Ok(note) => println!("{}", note.id),
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
            },
            NotesCommands::Retrieve(a) => match store.retrieve(&a.id).await {
                Ok(Some(note)) => println!("{}", format_note(&note)),
                Ok(None) => println!("not found"),
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
            },
            NotesCommands::Search(a) => match store.search(&a.query).await {
                Ok(notes) if notes.is_empty() => println!("no results"),
                Ok(notes) => {
                    for note in &notes {
                        println!("{}\n---", format_note(note));
                    }
                }
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
            },
            NotesCommands::List => match store.list().await {
                Ok(notes) if notes.is_empty() => println!("no notes"),
                Ok(notes) => {
                    for note in &notes {
                        println!("{}\n---", format_note(note));
                    }
                }
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
            },
        }
    });
}
```

Note: this uses the `dirs` crate for `state_dir()`. Add it to `Cargo.toml` in the next step.

- [ ] **Step 2: Add `dirs` dependency to `Cargo.toml`**

Add to `[dependencies]`:

```toml
dirs = "5"
```

- [ ] **Step 3: Add `pub mod notes;` to `src/commands/mod.rs`**

Append to the existing file:

```rust
pub mod notes;
```

- [ ] **Step 4: Add `Notes` variant to `Commands` in `src/main.rs`**

Add to the `Commands` enum:

```rust
/// Manage notes
Notes(commands::notes::NotesArgs),
```

Add to the `match` in `main()`:

```rust
Commands::Notes(notes_args) => commands::notes::run(&notes_args),
```

- [ ] **Step 5: Verify it compiles and the CLI help works**

```bash
cargo build 2>&1 | head -20
cargo run -- notes --help
cargo run -- notes save --help
```

Expected: Help text lists `save`, `retrieve`, `search`, `list` subcommands and `--db-path` option.

- [ ] **Step 6: Smoke test the CLI**

```bash
cargo run -- notes --db-path /tmp/notes-test save --name "hello" --message "world"
# Expected: prints a UUID like 550e8400-e29b-41d4-a716-446655440000

cargo run -- notes --db-path /tmp/notes-test list
# Expected: prints the note with id/name/message
```

- [ ] **Step 7: Commit**

```bash
git add src/commands/notes.rs src/commands/mod.rs src/main.rs Cargo.toml Cargo.lock
git commit -m "feat: add notes CLI subcommands (save, retrieve, search, list)"
```

---

## Task 7: MCP tool — `notes_save`

**Files:**
- Create: `src/mcp_tools/notes_save.rs`
- Modify: `src/mcp_tools/mod.rs`
- Modify: `src/commands/mcp.rs`

- [ ] **Step 1: Create `src/mcp_tools/notes_save.rs`**

```rust
use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::notes::NoteStore;

pub const NAME: &str = "notes_save";

#[derive(Deserialize, JsonSchema)]
pub struct Args {
    /// Human-readable label for the note
    pub name: String,
    /// The note content
    pub message: String,
}

pub fn definition() -> Tool {
    Tool::new(
        NAME,
        "Save a new note. Returns the auto-generated UUID of the created note.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub async fn call(store: &Arc<NoteStore>, args: &Args) -> CallToolResult {
    match store.save(&args.name, &args.message).await {
        Ok(note) => CallToolResult::success(vec![Content::text(note.id)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}
```

- [ ] **Step 2: Add `pub mod notes_save;` to `src/mcp_tools/mod.rs`**

```rust
pub mod notes_save;
```

- [ ] **Step 3: Update `McpServer` in `src/commands/mcp.rs` to hold `Arc<NoteStore>`**

Add imports at the top of `src/commands/mcp.rs`:

```rust
use std::sync::Arc;
use crate::notes::NoteStore;
```

Change the struct definition from:

```rust
#[derive(Clone)]
pub struct McpServer;
```

to:

```rust
#[derive(Clone)]
pub struct McpServer {
    pub notes: Arc<NoteStore>,
}
```

Update `list_tools` to include `notes_save`:

```rust
tools: vec![
    mcp_tools::get_time::definition(),
    mcp_tools::sys_info::definition(),
    mcp_tools::notes_save::definition(),
],
```

Update `call_tool` to handle `notes_save`:

```rust
mcp_tools::notes_save::NAME => {
    let args: mcp_tools::notes_save::Args = request
        .arguments
        .as_ref()
        .and_then(|v| serde_json::from_value(serde_json::Value::Object(v.clone())).ok())
        .ok_or_else(|| ErrorData::invalid_params("missing or invalid arguments", None))?;
    Ok(mcp_tools::notes_save::call(&self.notes, &args).await)
}
```

Update `run()` to open the `NoteStore` and construct `McpServer` with it:

```rust
pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    if let Err(e) = rt.block_on(async {
        eprintln!("mcp server starting on stdio");
        let db_path = {
            let mut p = dirs::state_dir()
                .unwrap_or_else(|| std::path::PathBuf::from(".local/state"));
            p.push("rust-mcp-example");
            p
        };
        let notes = Arc::new(NoteStore::open(&db_path).await?);
        let service = McpServer { notes }.serve(stdio()).await?;
        eprintln!("mcp server ready");
        service.waiting().await?;
        eprintln!("mcp server stopped");
        Ok::<_, Box<dyn std::error::Error>>(())
    }) {
        eprintln!("mcp server error: {e}");
        std::process::exit(1);
    }
}
```

- [ ] **Step 4: Verify it compiles**

```bash
cargo build 2>&1 | head -30
```

Expected: No errors.

- [ ] **Step 5: Commit**

```bash
git add src/mcp_tools/notes_save.rs src/mcp_tools/mod.rs src/commands/mcp.rs
git commit -m "feat: add notes_save MCP tool and NoteStore to McpServer"
```

---

## Task 8: MCP tools — `notes_retrieve`, `notes_search`, `notes_list`

**Files:**
- Create: `src/mcp_tools/notes_retrieve.rs`
- Create: `src/mcp_tools/notes_search.rs`
- Create: `src/mcp_tools/notes_list.rs`
- Modify: `src/mcp_tools/mod.rs`
- Modify: `src/commands/mcp.rs`

- [ ] **Step 1: Create `src/mcp_tools/notes_retrieve.rs`**

```rust
use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::notes::NoteStore;

pub const NAME: &str = "notes_retrieve";

#[derive(Deserialize, JsonSchema)]
pub struct Args {
    /// The UUID of the note to retrieve
    pub id: String,
}

pub fn definition() -> Tool {
    Tool::new(
        NAME,
        "Retrieve a note by its UUID. Returns the note's name and message, or 'not found'.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub async fn call(store: &Arc<NoteStore>, args: &Args) -> CallToolResult {
    match store.retrieve(&args.id).await {
        Ok(Some(note)) => CallToolResult::success(vec![Content::text(format!(
            "id:      {}\nname:    {}\nmessage: {}",
            note.id, note.name, note.message
        ))]),
        Ok(None) => CallToolResult::success(vec![Content::text("not found")]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}
```

- [ ] **Step 2: Create `src/mcp_tools/notes_search.rs`**

```rust
use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::notes::NoteStore;

pub const NAME: &str = "notes_search";

#[derive(Deserialize, JsonSchema)]
pub struct Args {
    /// Keyword to search across note names and messages
    pub query: String,
}

pub fn definition() -> Tool {
    Tool::new(
        NAME,
        "Search notes by keyword. Searches across both the name and message fields. Returns all matching notes.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub async fn call(store: &Arc<NoteStore>, args: &Args) -> CallToolResult {
    match store.search(&args.query).await {
        Ok(notes) if notes.is_empty() => {
            CallToolResult::success(vec![Content::text("no results")])
        }
        Ok(notes) => {
            let text = notes
                .iter()
                .map(|n| format!("id:      {}\nname:    {}\nmessage: {}", n.id, n.name, n.message))
                .collect::<Vec<_>>()
                .join("\n---\n");
            CallToolResult::success(vec![Content::text(text)])
        }
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}
```

- [ ] **Step 3: Create `src/mcp_tools/notes_list.rs`**

```rust
use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::notes::NoteStore;

pub const NAME: &str = "notes_list";

#[derive(Deserialize, JsonSchema, Default)]
pub struct Args {}

pub fn definition() -> Tool {
    Tool::new(
        NAME,
        "List all saved notes. Returns every note with its id, name, and message.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub async fn call(store: &Arc<NoteStore>) -> CallToolResult {
    match store.list().await {
        Ok(notes) if notes.is_empty() => {
            CallToolResult::success(vec![Content::text("no notes")])
        }
        Ok(notes) => {
            let text = notes
                .iter()
                .map(|n| format!("id:      {}\nname:    {}\nmessage: {}", n.id, n.name, n.message))
                .collect::<Vec<_>>()
                .join("\n---\n");
            CallToolResult::success(vec![Content::text(text)])
        }
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}
```

- [ ] **Step 4: Add `pub mod` declarations to `src/mcp_tools/mod.rs`**

```rust
pub mod notes_retrieve;
pub mod notes_search;
pub mod notes_list;
```

- [ ] **Step 5: Register all three tools in `src/commands/mcp.rs`**

In `list_tools`, add:

```rust
mcp_tools::notes_retrieve::definition(),
mcp_tools::notes_search::definition(),
mcp_tools::notes_list::definition(),
```

In `call_tool`, add three match arms:

```rust
mcp_tools::notes_retrieve::NAME => {
    let args: mcp_tools::notes_retrieve::Args = request
        .arguments
        .as_ref()
        .and_then(|v| serde_json::from_value(serde_json::Value::Object(v.clone())).ok())
        .ok_or_else(|| ErrorData::invalid_params("missing or invalid arguments", None))?;
    Ok(mcp_tools::notes_retrieve::call(&self.notes, &args).await)
}
mcp_tools::notes_search::NAME => {
    let args: mcp_tools::notes_search::Args = request
        .arguments
        .as_ref()
        .and_then(|v| serde_json::from_value(serde_json::Value::Object(v.clone())).ok())
        .ok_or_else(|| ErrorData::invalid_params("missing or invalid arguments", None))?;
    Ok(mcp_tools::notes_search::call(&self.notes, &args).await)
}
mcp_tools::notes_list::NAME => {
    Ok(mcp_tools::notes_list::call(&self.notes).await)
}
```

- [ ] **Step 6: Verify all tests pass and it compiles cleanly**

```bash
cargo test 2>&1 | tail -20
cargo clippy -- -D warnings 2>&1 | tail -20
```

Expected: All tests PASS, no clippy warnings.

- [ ] **Step 7: Commit**

```bash
git add src/mcp_tools/notes_retrieve.rs src/mcp_tools/notes_search.rs src/mcp_tools/notes_list.rs src/mcp_tools/mod.rs src/commands/mcp.rs
git commit -m "feat: add notes_retrieve, notes_search, notes_list MCP tools"
```

---

## Task 9: Update docs and verify CI readiness

**Files:**
- Modify: `CLAUDE.md`
- Modify: `docs/llm/log.md` (if it exists)

- [ ] **Step 1: Check if `docs/llm/log.md` exists**

```bash
ls docs/llm/ 2>/dev/null || echo "no docs/llm directory"
```

- [ ] **Step 2: Update `CLAUDE.md` commands section**

Add to the Commands section in `CLAUDE.md`:

```markdown
cargo run -- notes save --name "title" --message "content"   # save a note, prints UUID
cargo run -- notes retrieve --id <uuid>                        # retrieve note by id
cargo run -- notes search --query <keyword>                    # full-text search
cargo run -- notes list                                        # list all notes
cargo run -- notes --db-path /custom/path list                 # override db location
```

- [ ] **Step 3: Run the full CI check locally**

```bash
cargo clippy -- -D warnings 2>&1 | tail -20
cargo build --release --target aarch64-apple-darwin 2>&1 | tail -20
cargo test 2>&1 | tail -20
```

Expected: All clean.

- [ ] **Step 4: Commit docs**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with notes commands"
```

---

## Task 10: Commit spec and plan to git

**Files:**
- `docs/superpowers/specs/2026-05-03-notes-lancedb-design.md`
- `docs/superpowers/plans/2026-05-03-notes-lancedb.md`

- [ ] **Step 1: Commit both documents**

```bash
git add docs/superpowers/specs/2026-05-03-notes-lancedb-design.md docs/superpowers/plans/2026-05-03-notes-lancedb.md
git commit -m "docs: add notes feature spec and implementation plan"
```

---

## Self-Review Notes

- **Spec coverage:** Data model ✓, NoteStore ✓, CLI with `--db-path` ✓, MCP tools ✓, `Arc<NoteStore>` in MCP server ✓, error handling ✓, deps ✓, tests ✓
- **FTS caveat:** LanceDB FTS requires `optimize()` before searching newly inserted rows in the same session. This is handled in `NoteStore::search`. If this turns out not to be required in 0.27, the call is a no-op and safe to remove.
- **`dirs` crate:** Used for `state_dir()` to resolve `~/.local/state`. Added to deps in Task 6.
- **Type consistency:** `NoteStore`, `Note`, `batches_to_notes` naming is consistent across all tasks.
- **No placeholders:** All code blocks are complete and self-contained.

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

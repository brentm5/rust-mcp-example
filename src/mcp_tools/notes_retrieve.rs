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
        Ok(Some(note)) => CallToolResult::success(vec![Content::text(note.to_string())]),
        Ok(None) => CallToolResult::success(vec![Content::text("not found")]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}

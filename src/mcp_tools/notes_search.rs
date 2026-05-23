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
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("\n---\n");
            CallToolResult::success(vec![Content::text(text)])
        }
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}

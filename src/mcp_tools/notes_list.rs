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
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("\n---\n");
            CallToolResult::success(vec![Content::text(text)])
        }
        Err(e) => CallToolResult::error(vec![Content::text(format!("error: {e}"))]),
    }
}

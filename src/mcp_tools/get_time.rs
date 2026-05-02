use chrono::{Local, Utc};
use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::commands::time::format_time;

// Used in both definition() and the call_tool dispatch match in mcp.rs to prevent drift.
pub const NAME: &str = "get_time";

#[derive(Deserialize, JsonSchema, Default)]
struct Args {
    /// Print UTC time instead of local time
    utc: Option<bool>,
}

pub fn definition() -> Tool {
    // schema_for_type generates a JSON Schema from the Args struct, including
    // field descriptions from doc comments above.
    Tool::new(
        NAME,
        "Returns the current time. Use utc=true for UTC, omit for local time.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub fn call(arguments: Option<&serde_json::Map<String, serde_json::Value>>) -> CallToolResult {
    // from_value consumes its argument, so the map must be cloned into a Value::Object first.
    let args: Args = arguments
        .and_then(|a| serde_json::from_value(serde_json::Value::Object(a.clone())).ok())
        .unwrap_or_default();

    let text = if args.utc.unwrap_or(false) {
        format_time(Utc::now())
    } else {
        format_time(Local::now())
    };
    CallToolResult::success(vec![Content::text(text)])
}

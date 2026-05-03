use rmcp::model::{CallToolResult, Content, Tool};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::commands::sys_info::{collect_sys_info, format_sys_info};

pub const NAME: &str = "sys_info";

#[derive(Deserialize, JsonSchema, Default)]
struct Args {}

pub fn definition() -> Tool {
    Tool::new(
        NAME,
        "Returns system information: hostname, OS, kernel, architecture, CPU, RAM, and current user.",
        rmcp::handler::server::tool::schema_for_type::<Args>(),
    )
}

pub fn call() -> CallToolResult {
    CallToolResult::success(vec![Content::text(format_sys_info(&collect_sys_info()))])
}

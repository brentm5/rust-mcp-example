use std::sync::Arc;

use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, ErrorData, ListToolsResult, PaginatedRequestParams,
        ServerCapabilities, ServerInfo, ToolsCapability,
    },
    service::RequestContext,
    transport::stdio,
    RoleServer,
};

use crate::mcp_tools;
use crate::notes::NoteStore;

// Clone is required by rmcp — the SDK clones the handler for each concurrent request.
#[derive(Clone)]
pub struct McpServer {
    pub notes: Arc<NoteStore>,
}

impl ServerHandler for McpServer {
    // Advertise capabilities during the MCP initialize handshake.
    // Without tools: Some(...), MCP hosts won't call tools/list.
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability { list_changed: None });
        ServerInfo::new(capabilities)
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>> + Send {
        std::future::ready(Ok(ListToolsResult {
            tools: vec![
                mcp_tools::get_time::definition(),
                mcp_tools::notes_list::definition(),
                mcp_tools::notes_retrieve::definition(),
                mcp_tools::notes_save::definition(),
                mcp_tools::notes_search::definition(),
                mcp_tools::sys_info::definition(),
            ],
            next_cursor: None,
            meta: None,
        }))
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>> + Send {
        let notes = Arc::clone(&self.notes);
        async move {
            match request.name.as_ref() {
                mcp_tools::get_time::NAME => {
                    Ok(mcp_tools::get_time::call(request.arguments.as_ref()))
                }
                mcp_tools::sys_info::NAME => Ok(mcp_tools::sys_info::call()),
                mcp_tools::notes_save::NAME => {
                    let args: mcp_tools::notes_save::Args = request
                        .arguments
                        .as_ref()
                        .and_then(|v| {
                            serde_json::from_value(serde_json::Value::Object(v.clone())).ok()
                        })
                        .ok_or_else(|| {
                            ErrorData::invalid_params("missing or invalid arguments", None)
                        })?;
                    Ok(mcp_tools::notes_save::call(&notes, &args).await)
                }
                mcp_tools::notes_retrieve::NAME => {
                    let args: mcp_tools::notes_retrieve::Args = request
                        .arguments
                        .as_ref()
                        .and_then(|v| {
                            serde_json::from_value(serde_json::Value::Object(v.clone())).ok()
                        })
                        .ok_or_else(|| {
                            ErrorData::invalid_params("missing or invalid arguments", None)
                        })?;
                    Ok(mcp_tools::notes_retrieve::call(&notes, &args).await)
                }
                mcp_tools::notes_search::NAME => {
                    let args: mcp_tools::notes_search::Args = request
                        .arguments
                        .as_ref()
                        .and_then(|v| {
                            serde_json::from_value(serde_json::Value::Object(v.clone())).ok()
                        })
                        .ok_or_else(|| {
                            ErrorData::invalid_params("missing or invalid arguments", None)
                        })?;
                    Ok(mcp_tools::notes_search::call(&notes, &args).await)
                }
                mcp_tools::notes_list::NAME => {
                    Ok(mcp_tools::notes_list::call(&notes).await)
                }
                _ => Err(ErrorData::invalid_params("unknown tool", None)),
            }
        }
    }
}

pub fn run() {
    // main() is synchronous, so we build a tokio runtime here to drive the async MCP server.
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    if let Err(e) = rt.block_on(async {
        eprintln!("mcp server starting on stdio");
        let db_path = crate::notes::default_db_path();
        let notes = Arc::new(crate::notes::NoteStore::open(&db_path).await?);
        // serve() completes the MCP initialize handshake and starts the message loop
        // in a background task, returning a handle to it.
        let service = McpServer { notes }.serve(stdio()).await?;
        eprintln!("mcp server ready");
        // Block until the client disconnects (stdin closes).
        service.waiting().await?;
        eprintln!("mcp server stopped");
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    }) {
        eprintln!("mcp server error: {e}");
        std::process::exit(1);
    }
}

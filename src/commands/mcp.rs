use rmcp::{ServerHandler, ServiceExt, transport::stdio};

#[derive(Clone)]
pub struct McpServer;

impl ServerHandler for McpServer {}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    if let Err(e) = rt.block_on(async {
        let service = McpServer.serve(stdio()).await?;
        service.waiting().await?;
        Ok::<_, Box<dyn std::error::Error>>(())
    }) {
        eprintln!("mcp server error: {e}");
        std::process::exit(1);
    }
}

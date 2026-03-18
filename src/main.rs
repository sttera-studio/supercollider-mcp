use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};

mod supercollider_model;
mod server;

use crate::server::SupercolliderMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging later if needed (tracing, etc.).
    let server_impl = SupercolliderMcpServer::new();

    // IMPORTANT: write human-readable startup logs to stderr so MCP JSON-RPC
    // traffic on stdio remains clean for clients.
    eprintln!("[supercollider-mcp] starting MCP server over stdio");
    eprintln!("[supercollider-mcp] available tool: ping_supercollider");

    // Use rmcp stdio transport so this can be launched as an MCP server by
    // tools like Claude Desktop or Cursor.
    // Serve the MCP server.
    let server = server_impl.serve(stdio()).await?;
    eprintln!("[supercollider-mcp] ready. waiting for MCP requests.");

    server.waiting().await?;

    Ok(())
}

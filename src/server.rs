use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::supercollider_model::SupercolliderServerState;

#[derive(Clone)]
pub struct SupercolliderMcpServer {
    tool_router: ToolRouter<Self>,
    state: SupercolliderServerState,
    // TODO: add SuperCollider connection / OSC config here.
}

impl SupercolliderMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            state: SupercolliderServerState::bootstrap_placeholder(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PingParams {
    /// Optional message to send with the ping.
    pub message: Option<String>,
}

#[tool_router]
impl SupercolliderMcpServer {
    /// Simple ping tool that will later check connectivity with SuperCollider.
    #[tool(description = "Ping the SuperCollider understanding backend (currently mocked).")]
    async fn ping_supercollider(&self, Parameters(params): Parameters<PingParams>) -> String {
        let msg = params
            .message
            .unwrap_or_else(|| "Hello from supercollider-mcp boilerplate!".to_string());
        let node_count = self.state.nodes.len();

        // TODO: in the future, actually check SC connectivity via OSC and include status.
        format!("SuperCollider MCP boilerplate is running. Message: {msg} (nodes in placeholder model: {node_count})")
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for SupercolliderMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "SuperCollider mega-understanding MCP server (boilerplate)".to_string(),
        )
    }
}

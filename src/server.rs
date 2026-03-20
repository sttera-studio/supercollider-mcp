use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::{sc_process, supercollider_model::SupercolliderServerState};

#[derive(Clone)]
pub struct SupercolliderMcpServer {
    tool_router: ToolRouter<Self>,
    #[allow(dead_code)]
    state: SupercolliderServerState,
    // TODO: graph tools + OSC will use `state`.
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
    /// Optional note (echoed in the reply).
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EmptyParams {}

#[tool_router]
impl SupercolliderMcpServer {
    /// Simple ping tool that will later check connectivity with SuperCollider.
    #[tool(
        description = "Ping SuperCollider on this machine: checks whether scsynth/sclang processes are running (no OSC). Also confirms this MCP server is up."
    )]
    async fn ping_supercollider(&self, Parameters(params): Parameters<PingParams>) -> String {
        eprintln!(
            "[supercollider-mcp] tool start: ping_supercollider params={params:?}"
        );

        let report = match tokio::task::spawn_blocking(sc_process::probe).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "[supercollider-mcp] tool error: ping_supercollider — spawn_blocking join failed: {e}"
                );
                return format!("ping_supercollider failed: {e}");
            }
        };

        let reply = match &params.message {
            Some(m) if !m.trim().is_empty() => format!("{report}\nNote: {m}"),
            _ => report,
        };

        eprintln!("[supercollider-mcp] tool ok: ping_supercollider — finished successfully");

        reply
    }

    #[tool(
        description = "List SuperCollider audio server processes (scsynth) on this machine with OS stats: RSS, CPU%, uptime, disk I/O counters. Logs one line per server to the supercollider-mcp terminal. Not OSC."
    )]
    async fn get_servers(&self, Parameters(_p): Parameters<EmptyParams>) -> String {
        eprintln!("[supercollider-mcp] tool start: get_servers");

        let report = match tokio::task::spawn_blocking(sc_process::get_servers).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "[supercollider-mcp] tool error: get_servers — spawn_blocking join failed: {e}"
                );
                return format!("get_servers failed: {e}");
            }
        };

        eprintln!("[supercollider-mcp] tool ok: get_servers — finished successfully");

        report
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for SupercolliderMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "Use ping_supercollider for a quick scsynth/sclang presence check; use get_servers for per-scsynth OS stats (RSS, CPU, uptime).".to_string(),
        )
    }
}

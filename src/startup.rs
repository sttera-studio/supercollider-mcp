//! Human-readable messages on stderr. For stdio transport, MCP JSON-RPC uses stdout only.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn stdio() {
    eprintln!(
        "[supercollider-mcp] MCP over stdio (default). JSON-RPC on stdout; logs on stderr. Use --http for Streamable HTTP."
    );
    eprintln!("[supercollider-mcp] tools: ping_supercollider, get_servers");
}

pub fn streamable_http(addr: SocketAddr) {
    eprintln!("[supercollider-mcp] listening on http://{addr}/mcp");
    if addr.ip() == IpAddr::V4(Ipv4Addr::UNSPECIFIED) {
        eprintln!(
            "[supercollider-mcp] Open WebUI URL: http://127.0.0.1:{}/mcp (use 127.0.0.1, not localhost, if tools fail)",
            addr.port()
        );
    }
    eprintln!("[supercollider-mcp] tools: ping_supercollider, get_servers · Ctrl+C to stop");
}

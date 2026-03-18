# supercollider-mcp

Rust MCP server boilerplate for SuperCollider.

## Build

```bash
# Development: Builds to 'target/debug/'
cargo run

# Distribution: Builds to 'target/release/'
cargo build --release
```

## Run as an MCP server

This server uses MCP over stdio, so MCP clients can launch it as a subprocess.
That includes tools like Cursor and Claude-compatible MCP hosts.

After launch, the first example tool is:

- `ping_supercollider` - confirms the MCP server is alive (currently mocked).

## Roadmap

- [ ] Add OSC integration with SuperCollider.
- [ ] Export node tree / graph model into `SupercolliderServerState`.
- [ ] Add tools like `get_graph`, `eval_code`, `explain_patch`, etc.
- [ ] Add a persistent connection manager for SC server lifecycle and reconnects.
- [ ] Implement request/response correlation for OSC round-trips and timeouts.
- [ ] Add a `health_check` tool reporting MCP + SuperCollider bridge status.
- [ ] Add `get_node_tree` and `get_node_detail` tools for graph introspection.
- [ ] Add `run_code` tool with safety guardrails and execution metadata.
- [ ] Add model diffing utilities to explain changes between graph snapshots.
- [ ] Add structured error mapping from SC/OSC failures into MCP tool errors.
- [ ] Add integration tests with a mock SC bridge and stdio MCP client harness.

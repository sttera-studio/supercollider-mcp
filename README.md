# supercollider-mcp

Rust MCP Server boilerplate for SuperCollider.

Fully local pipeline with MCP Client OpenWebUI and Ollama `gemma3:27b` model.

## Build

```bash
cargo run
cargo build --release
```

## Run as an MCP server

**stdio (default)** — subprocess clients (Cursor, ChatMCP, etc.): `cargo run`

**Streamable HTTP** — `cargo run -- --http` (default `0.0.0.0:8787`). Open WebUI: MCP (Streamable HTTP), URL `http://127.0.0.1:8787/mcp`, Auth None; enable tool in chat. Docker: `host.docker.internal`.

## MCP tools (callable)


| Tool                 | Args               | What it does                                                                                                                                                                                                                                         |
| -------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ping_supercollider` | `message` optional | Quick scan for **scsynth** / **sclang**; confirms MCP is up. Optional note echoed. Stderr: `tool ok: ping_supercollider`.                                                                                                                            |
| `get_servers`        | *(none)*           | Lists each **scsynth** (audio server) process with OS stats: RSS, vsize, CPU% (est.), uptime, disk I/O totals. Logs one `[supercollider-mcp] get_servers: …` line per server on stderr; returns the same text to the client. Not OSC / not DSP load. |


*Process names only; OSC would add real synth/node/audio stats.*

**Robustness:** `--bind` is validated at startup. Tools log `tool error:` on stderr if the background task fails; `tool ok:` only after a successful run. Process scans are panic-guarded so a bad `sysinfo` state returns a string error instead of crashing the MCP server.
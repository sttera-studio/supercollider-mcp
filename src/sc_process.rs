//! Detect SuperCollider-related processes on this machine (no OSC).

use std::fmt::Write as _;
use std::panic;
use std::thread;

use sysinfo::{DiskUsage, ProcessRefreshKind, ProcessesToUpdate, System};

struct ScsynthRow {
    pid: u32,
    name: String,
    rss: u64,
    vsz: u64,
    cpu_raw: f32,
    cpu_norm: f32,
    run_s: u64,
    disk: DiskUsage,
}

fn is_scsynth(name_lower: &str) -> bool {
    name_lower.contains("scsynth")
}

fn is_sclang(name_lower: &str) -> bool {
    name_lower.contains("sclang")
}

fn probe_impl() -> String {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut scsynth: Vec<String> = Vec::new();
    let mut sclang: Vec<String> = Vec::new();

    for (pid, proc) in sys.processes() {
        let name_lower = proc.name().to_string_lossy().to_lowercase();
        let disp = proc.name().to_string_lossy();
        if is_scsynth(&name_lower) {
            scsynth.push(format!("pid {} ({})", pid.as_u32(), disp));
        }
        if is_sclang(&name_lower) {
            sclang.push(format!("pid {} ({})", pid.as_u32(), disp));
        }
    }

    let mut out = String::new();
    out.push_str("SuperCollider on this machine (process scan, not OSC):\n");

    if scsynth.is_empty() {
        out.push_str("- scsynth: not found (audio server likely not running under that name)\n");
    } else {
        let _ = writeln!(out, "- scsynth: running — {}", scsynth.join(", "));
    }

    if sclang.is_empty() {
        out.push_str("- sclang: not found\n");
    } else {
        let _ = writeln!(out, "- sclang: running — {}", sclang.join(", "));
    }

    out.push_str("- supercollider-mcp: running\n");
    out
}

/// Scan running processes for scsynth / sclang (typical SuperCollider server + language).
pub fn probe() -> String {
    match panic::catch_unwind(probe_impl) {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "[supercollider-mcp] probe: internal panic while scanning processes (sysinfo/OS)"
            );
            "Process scan failed unexpectedly (panic). Check supercollider-mcp stderr.".to_string()
        }
    }
}

pub(crate) fn mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn fmt_disk(d: &DiskUsage) -> String {
    format!(
        "disk_read={} disk_written={} (bytes total since measure)",
        d.total_read_bytes, d.total_written_bytes
    )
}

fn get_servers_impl() -> String {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cpu().with_memory(),
    );

    let n_cpu = sys.cpus().len().max(1);

    let mut rows: Vec<ScsynthRow> = Vec::new();

    for (pid, proc) in sys.processes() {
        let name = proc.name().to_string_lossy();
        if !is_scsynth(&name.to_lowercase()) {
            continue;
        }
        let rss = proc.memory();
        let vsz = proc.virtual_memory();
        let cpu_raw = proc.cpu_usage();
        let cpu_norm = cpu_raw / n_cpu as f32;
        let run_s = proc.run_time();
        let disk = proc.disk_usage();
        rows.push(ScsynthRow {
            pid: pid.as_u32(),
            name: name.into_owned(),
            rss,
            vsz,
            cpu_raw,
            cpu_norm,
            run_s,
            disk,
        });
    }

    rows.sort_by_key(|r| r.pid);

    if rows.is_empty() {
        eprintln!(
            "[supercollider-mcp] get_servers: no scsynth processes found (SuperCollider server not running or different binary name)"
        );
        return "No **scsynth** (SuperCollider audio server) processes found on this machine.\n\
                Boot the server from sclang or your IDE, or the process name may not contain \"scsynth\".\n\
                (OS process stats only — not OSC / not node counts.)"
            .to_string();
    }

    let mut out = String::new();
    out.push_str(
        "SuperCollider **scsynth** server process(es) on this machine (OS stats via sysinfo, not OSC):\n\n",
    );

    for row in &rows {
        let line = format!(
            "pid={} name={} rss={:.1} MiB vsize={:.1} MiB cpu~={:.1}% (of one core, est.) cpu_raw={:.1}% uptime={}s {}",
            row.pid,
            row.name,
            mib(row.rss),
            mib(row.vsz),
            row.cpu_norm,
            row.cpu_raw,
            row.run_s,
            fmt_disk(&row.disk)
        );
        eprintln!("[supercollider-mcp] get_servers: {line}");
        let _ = writeln!(out, "- {line}");
    }

    let _ = writeln!(
        out,
        "\nCPU % is estimated from OS counters ({} logical CPUs); not SuperCollider DSP load.",
        n_cpu
    );
    out
}

/// **scsynth** processes = SuperCollider audio servers (OS-level stats only; not OSC / not synth counts).
pub fn get_servers() -> String {
    match panic::catch_unwind(get_servers_impl) {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "[supercollider-mcp] get_servers: internal panic while collecting stats (sysinfo/OS)"
            );
            "get_servers failed unexpectedly (panic). Check supercollider-mcp stderr.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mib_roundtrip_small() {
        assert!((mib(1024 * 1024) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn name_filters() {
        assert!(is_scsynth("scsynth.exe"));
        assert!(is_sclang("sclang"));
        assert!(!is_scsynth("notepad"));
    }
}

---
title: sys_info command
tags: [command, cli, sysinfo]
updated: 2026-05-03
sources: [src/commands/sys_info.rs, docs/superpowers/specs/2026-05-03-sys-info-design.md]
---

# `sys-info` Command

Prints a human-readable snapshot of the host machine: hostname, OS, kernel, architecture, CPU, RAM, and current user.

## Usage

```bash
rust-mcp-example sys-info
```

No arguments — always returns the full snapshot.

## Output format

```
Hostname:   my-macbook.local
OS:         macOS 15.3.1
Kernel:     24.3.0
Arch:       aarch64
CPU:        Apple M3 Pro (12 cores)
RAM:        36 GB
User:       brentm5
```

## Implementation

- File: `src/commands/sys_info.rs`
- `SysInfoArgs` struct: no fields (clap `Args` derive)
- `SysInfoData` struct: plain data struct holding all resolved fields as `String`/`usize`/`u64`
- `collect_sys_info() -> SysInfoData` — calls `sysinfo` crate APIs and resolves all fields
- `format_sys_info(data: &SysInfoData) -> String` — pure formatting function, shared with MCP tool
- `run()` calls both and prints the result

## Data collection

All fields sourced from the `sysinfo` crate (`sysinfo = "0.33"`) or the standard library:

| Field | Source |
|-------|--------|
| Hostname | `sysinfo::System::host_name()` |
| OS | `sysinfo::System::long_os_version()` |
| Kernel | `sysinfo::System::kernel_version()` |
| Arch | `std::env::consts::ARCH` |
| CPU brand | `sys.cpus().first().brand()` |
| CPU count | `sys.cpus().len()` |
| RAM | `sys.total_memory() / 1_073_741_824` (bytes → GB, truncated) |
| User | `$USER` env var, fallback to `$USERNAME`, fallback to `"unknown"` |

All `Option`-returning sysinfo calls fall back to `"unknown"` if `None`.

## Shared logic

`format_sys_info` is `pub` so `src/mcp_tools/sys_info.rs` can reuse it. The `SysInfoData` struct separation means the formatting function is pure and testable without any system calls.

## Testing

Tests construct a `SysInfoData` with known values and assert exact output — no real system calls in tests. `collect_sys_info()` is not unit tested as its output is environment-dependent.

## Out of scope (per design spec)

- Per-CPU stats, disk usage, network interfaces, load average, uptime
- Filtering or selecting individual fields

## Related

- [[architecture/command-module-pattern]] — the pattern this command implements
- [[mcp-tools/sys_info]] — the MCP tool that mirrors this command

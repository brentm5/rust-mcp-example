# sys_info — Design Spec

## Overview

Add a `sys_info` subcommand and matching MCP tool that returns a human-readable snapshot of the host machine. The primary consumer is a user running the CLI directly or querying through an MCP client.

## Fields

| Field     | Source                                      |
|-----------|---------------------------------------------|
| Hostname  | `hostname` crate                            |
| OS        | `sysinfo::System::long_os_version()`        |
| Kernel    | `sysinfo::System::kernel_version()`         |
| Arch      | `std::env::consts::ARCH`                    |
| CPU       | brand from `sysinfo` first CPU + core count |
| RAM       | `sysinfo::System::total_memory()` as GB     |
| User      | `std::env::var("USER")` / `"USERNAME"`      |

## Output Format

A single labeled text block — same style as `get_time`. Example:

```
Hostname:   my-macbook.local
OS:         macOS 15.3.1
Kernel:     24.3.0
Arch:       aarch64
CPU:        Apple M3 Pro (12 cores)
RAM:        36 GB
User:       brentm5
```

No args — always returns the full snapshot.

## Architecture

Follows the exact `get_time` pattern: a shared formatting function used by both the CLI command and MCP tool.

### New files

- `src/commands/sys_info.rs` — `pub struct SysInfoArgs` (no fields, clap `Args` derive) + `pub fn run(args: &SysInfoArgs)` that prints `format_sys_info()`
- `src/mcp_tools/sys_info.rs` — `pub const NAME`, `pub fn definition()`, `pub fn call()` that returns `CallToolResult::success(vec![Content::text(format_sys_info())])`

### Shared logic

Two functions live in `src/commands/sys_info.rs`:

- `pub fn collect_sys_info() -> SysInfoData` — constructs a `sysinfo::System`, reads all system APIs, and returns a plain data struct.
- `pub fn format_sys_info(data: &SysInfoData) -> String` — pure formatting function; takes the struct and returns the labeled text block.

Both `run()` and the MCP `call()` use `format_sys_info(&collect_sys_info())`.

`SysInfoData` is a plain struct with all string/numeric fields already resolved:

```rust
pub struct SysInfoData {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub arch: String,
    pub cpu_brand: String,
    pub cpu_count: usize,
    pub ram_gb: u64,
    pub user: String,
}
```

### Wiring

- `src/commands/mod.rs` — add `pub mod sys_info;`
- `src/main.rs` — add `SysInfo` variant to `Commands` enum and a match arm dispatching to `commands::sys_info::run(&args)`
- `src/mcp_tools/mod.rs` — add `pub mod sys_info;`
- `src/mcp_tools/get_time.rs` (MCP dispatch in `mcp.rs`) — add `sys_info::NAME` to tool list and `call()` dispatch

### Dependencies

Add to `Cargo.toml`:
```toml
sysinfo = "0.33"
```

The `hostname` crate is already present (used by `get_time` indirectly via `sysinfo`; if not, add `hostname = "0.4"`). Verify before adding.

## Error Handling

- `sysinfo` fields that return `Option` (e.g., `long_os_version`, `kernel_version`, CPU brand) fall back to `"unknown"` if `None`.
- `USER`/`USERNAME` env var falls back to `"unknown"` if unset.
- No panics — all fallbacks are handled inline in `format_sys_info()`.

## Testing

Because `format_sys_info()` is a pure function that takes a `SysInfoData` struct, tests construct known data and assert exact output — no system calls, no mocking needed.

```rust
#[test]
fn test_format_sys_info() {
    let data = SysInfoData {
        hostname: "test-host".into(),
        os: "macOS 15.3.1".into(),
        kernel: "24.3.0".into(),
        arch: "aarch64".into(),
        cpu_brand: "Apple M3 Pro".into(),
        cpu_count: 12,
        ram_gb: 36,
        user: "brentm5".into(),
    };
    let output = format_sys_info(&data);
    assert!(output.contains("Hostname:   test-host"));
    assert!(output.contains("OS:         macOS 15.3.1"));
    assert!(output.contains("CPU:        Apple M3 Pro (12 cores)"));
    assert!(output.contains("RAM:        36 GB"));
}
```

- `collect_sys_info()` is not unit tested — it calls real system APIs and its output is environment-dependent.
- Manual CLI smoke test: `cargo run -- sys-info`
- Manual MCP smoke test: run `cargo run -- mcp` and call `sys_info` tool via an MCP client.

## Out of Scope

- Per-CPU stats, disk usage, network interfaces, load average, uptime — not included in this iteration.
- Filtering or selecting individual fields — always returns the full snapshot.

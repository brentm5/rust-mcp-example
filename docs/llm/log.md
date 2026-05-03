# Wiki Log

Append-only record of wiki operations. Format: `## [YYYY-MM-DD] <type> | <title>`

---

## [2026-05-02] create | Initial wiki build

Bootstrapped wiki from full project scan. Sources ingested:
- `src/main.rs`, `src/commands/mod.rs`, `src/commands/time.rs`
- `Cargo.toml`, `rust-toolchain.toml`, `mise.toml`
- `CLAUDE.md`, `README.md`
- `docs/superpowers/specs/2026-05-02-time-subcommand-design.md`
- `docs/superpowers/specs/2026-05-02-command-abstraction-design.md`
- `docs/superpowers/plans/2026-05-02-time-subcommand.md`
- `docs/superpowers/plans/2026-05-02-command-abstraction.md`
- `.github/workflows/ci.yml`
- `docs/llm/llm-wiki.md`

Pages created: overview, command-module-pattern, dispatch-flow, time (command), cli-as-source-of-truth, mcp, clap-derive-api, ci, toolchain, index.

## 2026-05-03 — sys_info command and MCP tool

Added `sys-info` CLI subcommand and `sys_info` MCP tool. Returns hostname, OS, kernel version, architecture, CPU brand + core count, total RAM, and current user. Uses `sysinfo = "0.33"` crate. Shared `SysInfoData` struct + pure `format_sys_info()` function used by both CLI and MCP. Tests assert exact output using a constructed `SysInfoData` — no system calls in tests.

## 2026-05-03 — notes feature pre-merge cleanup

Cleaned up the notes module before merging `feat/notes-lancedb`. Removed the skeleton-phase `#![allow(unused_imports, dead_code, unused)]` from `src/notes.rs` and dropped the redundant `: Connection` type annotation. Extracted a shared `pub fn default_db_path() -> PathBuf` into `src/notes.rs` (using `dirs::state_dir()`) and replaced the duplicated inline path-building blocks in `src/commands/notes.rs` and `src/commands/mcp.rs`. Added a `std::fmt::Display` impl on `Note` and replaced the triplicated format string across `src/commands/notes.rs`, `src/mcp_tools/notes_retrieve.rs`, `src/mcp_tools/notes_search.rs`, and `src/mcp_tools/notes_list.rs` with `note.to_string()` / `n.to_string()`. All 11 tests pass; clippy clean at `-D warnings`.

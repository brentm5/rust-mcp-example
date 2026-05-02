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

---
title: Toolchain
tags: [rust, toolchain, mise]
updated: 2026-05-02
---

# Toolchain

Rust version is pinned to **1.85.0** in two places:

| File | Tool | Purpose |
|------|------|---------|
| `rust-toolchain.toml` | rustup | Pins version for local dev and CI via rustup |
| `mise.toml` | mise | Pins version for mise-managed environments |

Both point to 1.85.0. CI uses mise (`jdx/mise-action@v2`) to bootstrap, then `rustup install` to finalize.

## rust-toolchain.toml

```toml
[toolchain]
channel = "1.85.0"
components = ["clippy"]
targets = ["aarch64-apple-darwin"]
```

Clippy is declared as a required component. The explicit target matches CI's build target.

## Edition

`Cargo.toml` declares `edition = "2024"` — the Rust 2024 edition, available since Rust 1.85.0.

## Related

- [[architecture/ci]] — how toolchain setup fits into the CI pipeline

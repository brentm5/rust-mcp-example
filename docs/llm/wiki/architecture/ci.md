---
title: CI Pipeline
tags: [ci, github-actions, toolchain]
updated: 2026-05-02
sources: [.github/workflows/ci.yml]
---

# CI Pipeline

GitHub Actions workflow at `.github/workflows/ci.yml`. Runs on every pull request targeting `main`.

## Steps

1. **Checkout** — `actions/checkout@v4`
2. **Setup mise** — `jdx/mise-action@v2`, installs Rust 1.85.0 from `mise.toml`
3. **Install Rust toolchain** — `rustup install` (reads `rust-toolchain.toml`)
4. **Install Clippy** — `rustup component add clippy`
5. **Cache cargo registry** — caches `~/.cargo` and `target/` keyed on `Cargo.lock` hash
6. **Lint** — `cargo clippy --target aarch64-apple-darwin -- -D warnings` (zero warnings enforced)
7. **Build** — `cargo build --release --target aarch64-apple-darwin`

## What CI does NOT do

- No tests (`cargo test`) — there are no tests yet
- No formatting check (`cargo fmt`)
- No publish/deploy

## Local equivalent

```bash
cargo clippy -- -D warnings
cargo build --release
```

## Related

- [[architecture/toolchain]] — how Rust version is pinned

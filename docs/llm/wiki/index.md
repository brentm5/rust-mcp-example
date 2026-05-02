---
title: Wiki Index
updated: 2026-05-03
---

# rust-mcp-example Wiki

A compounding knowledge base for the rust-mcp-example project — built and maintained by an LLM, read by humans.

## Architecture

| Page | Summary |
|------|---------|
| [[architecture/overview]] | High-level project purpose and CLI→MCP pattern |
| [[architecture/command-module-pattern]] | How commands are structured and how to add new ones |
| [[architecture/dispatch-flow]] | How CLI args flow from entry point to execution |
| [[architecture/mcp-server]] | MCP server structure, ServerHandler, tool dispatch |

## Commands

| Page | Summary |
|------|---------|
| [[commands/time]] | `time` subcommand — prints local or UTC time |

## MCP Tools

| Page | Summary |
|------|---------|
| [[mcp-tools/get_time]] | `get_time` tool — returns current time, mirrors the `time` command |

## Concepts

| Page | Summary |
|------|---------|
| [[concepts/cli-as-source-of-truth]] | Why CLI is canonical and MCP is a thin layer on top |
| [[concepts/mcp]] | What MCP (Model Context Protocol) is and its role here |
| [[concepts/clap-derive-api]] | How clap's derive API is used in this project |

## Infrastructure

| Page | Summary |
|------|---------|
| [[architecture/ci]] | GitHub Actions CI — what it checks and why |
| [[architecture/toolchain]] | Rust version pinning and mise setup |

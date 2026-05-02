# LLM Wiki Schema

This document governs how the wiki in `docs/llm/wiki/` is structured and maintained. Read this before making any wiki changes.

## Directory layout

```text
docs/llm/
  SCHEMA.md         — this file (wiki operating instructions)
  log.md            — append-only activity log
  llm-wiki.md       — the idea doc that inspired this wiki (do not edit)
  raw/              — immutable source documents (LLM reads, never writes)
  wiki/
    index.md        — content catalog (update on every ingest)
    architecture/   — how the project is structured and why
    commands/       — one page per CLI subcommand
    concepts/       — key ideas referenced across the project
```

## Page format

Every wiki page starts with YAML frontmatter:

```yaml
---
title: <Page Title>
tags: [tag1, tag2]
updated: YYYY-MM-DD
sources: [optional list of source files this page was derived from]
---
```

Use `[[wikilinks]]` for cross-references. Links are relative to the `wiki/` root.

## Operations

### Ingest a new source
1. Drop the source into `docs/llm/raw/`
2. Read it and discuss key takeaways
3. Write or update relevant wiki pages
4. Update `wiki/index.md` with any new pages
5. Append an entry to `log.md`: `## [YYYY-MM-DD] ingest | <title>`

### Answer a query
1. Read `wiki/index.md` to find relevant pages
2. Read those pages and synthesize an answer
3. If the answer is valuable, file it as a new wiki page
4. Append to `log.md`: `## [YYYY-MM-DD] query | <question summary>`

### Lint the wiki
Look for: orphan pages (no inbound links), contradictions, stale claims, important concepts without their own page. Append to `log.md`: `## [YYYY-MM-DD] lint | <summary of findings>`

## Conventions

- LLM writes all wiki content. Humans read.
- `raw/` sources are never modified.
- `log.md` is append-only — never edit past entries.
- `index.md` must stay current — update it whenever pages are added or removed.
- Keep pages focused — one concept, command, or architectural concern per page.

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 02 — Install Skills

Skills are **project-local**. They live in `skills/<name>/SKILL.md` in this repository. You do not need a global skill registry to use them.

## Claude Code

Open the repository root in Claude Code. Skills may be mirrored under `.claude/skills/` depending on your local setup; the source of truth is `skills/`. Activate by name:

> Activate esp32-sdd-full-project-generator skill

## Grok and other agents

Any agent that can read Markdown can follow a skill:

1. Read `skills/<name>/SKILL.md`.
2. Read `shared-specs/AIGenLessonsLearned.md` first (mandatory).
3. Follow the workflow exactly.

Grok may also discover skills via `AGENTS.md` when that file is present in the agent workspace.

## Optional CLI install

If you use `@agentskills/cli` to publish or install skill packs:

```sh
npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills
```

This is optional. Opening the repo and pointing the agent at `skills/` is enough.

## Eleven skills

See [skills/README.md](../../skills/README.md) for the full table (10 original SDD skills plus `esp32-lp-core-engineer`).

Next: [03 — Run Your First Example](03-first-example.md).

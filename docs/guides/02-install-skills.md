<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 02 — Install Skills

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask the agent to follow it. It is not a shell command and not a Rust crate.

`skills/` is already in the clone. No install is required. How a spec drives firmware and docs is [Using With AI Agents](using-with-ai-agents.md). A **spec** is Markdown a person writes; that page defines the rest.

Activate phrases are said to the agent, not run in the shell. Example, typed in the agent chat:

> Activate esp32-sdd-full-project-generator

## Claude Code

Open the repository root in Claude Code. The source of truth is `skills/`. A local mirror under `.claude/skills/` is optional and is not a second catalog.

## Grok

Grok loads root `AGENTS.md` at session start. That file points at `skills/` and at `shared-specs/AIGenLessonsLearned.md`. Ask Grok to follow a skill by name. You do not run the skill in the terminal.

## Any agent that can read Markdown

1. Open this repository.
2. Ask the agent to read `skills/<name>/SKILL.md` and follow it.
3. The skill's first step is to read `shared-specs/AIGenLessonsLearned.md`.

The workflows do not depend on one vendor's API.

## Optional CLI install

If you use `@agentskills/cli` to install skill packs elsewhere, this command is optional:

```sh
npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills
```

Opening the repo and pointing the agent at `skills/` is enough. Do not treat that `npx` line as a required setup step.

Phrase list: [skill invocation](../reference/skill-invocation.md).

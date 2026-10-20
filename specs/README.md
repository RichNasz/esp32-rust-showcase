<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# specs/

**Human-written source of truth.** This is one of the only directories people edit directly (along with `shared-specs/` and `examples/*/specs/`). A **spec** is Markdown a person writes. The agent generates firmware and documentation from these files. The lesson, if you have not done that before, is [Using an AI coding agent](../docs/guides/using-with-ai-agents.md).

## Files

| File | Purpose |
|---|---|
| `showcase-master-spec.md` | Catalog of nine examples, ten skills, the C6 hardware bar, and the implementation-status table. As of 2026-10-20 all nine examples are **Firmware generated**. |
| `repo-overview-spec.md` | Repository purpose, features, and the two quick-start tasks (flash blinky, or change firmware with an agent) |
| `documentation-generation-spec.md` | Rules for generated Markdown: readers, file checklist, tone, Cargo + espflash |
| `contributing-spec.md` | What people may edit, how to ask the agent to regenerate, commit types |
| `example-project-structure-spec.md` | Permanent directory layout every example follows |

## The rule

> **Do not edit any file outside `specs/`, `shared-specs/`, or `examples/*/specs/{Functional,Coding,Test}Spec.md`.**
> Other files are agent-generated and will be overwritten on the next documentation-generator or full-project-generator run.

## Regenerating documentation

After editing a file in this directory, say this to the agent (not in the shell):

> Activate esp32-sdd-documentation-generator and rebuild all documentation.

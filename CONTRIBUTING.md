<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Contributing

You want to change this project, including by using an AI coding agent to change the ESP32 Rust. You do not need to already know spec-driven development (SDD).

A **spec** is a Markdown file a person writes. It is the source of truth for firmware and for documentation. The agent reads it and writes the Rust and the Markdown. Generated files (`src/`, `Cargo.toml`, READMEs, `/docs/`, and everything outside the table below) are overwritten the next time the agent generates. When output is wrong, change the spec and ask the agent to generate again.

The longer lesson is [Using an AI coding agent](docs/guides/using-with-ai-agents.md).

## What you may edit

| Path | You may edit? |
|---|---|
| `specs/*.md` | Yes — repo-wide rules |
| `shared-specs/*.md` | Yes — platform constraints shared by every example |
| `examples/*/specs/FunctionalSpec.md` | Yes — what the example does |
| `examples/*/specs/CodingSpec.md` | Yes — architecture and constraints, no code |
| `examples/*/specs/TestSpec.md` | Yes — how to check the example |
| Everything else | No — the agent generates it |

## Workflow

1. Edit a file in `specs/`, `shared-specs/`, or `examples/*/specs/`.
2. If the change affects documentation, ask the agent: "Activate esp32-sdd-documentation-generator and rebuild all documentation."
3. If the change affects a crate, ask the agent: "Activate esp32-sdd-full-project-generator".
4. Those phrases are typed to the agent, not in the shell.
5. Commit the spec edit and the files the agent regenerated. Open the pull request with both.

A pull request contains the spec edit and the regenerated files. Do not hand-edit generated files. Do not open a pull request that only tweaks generated Rust or Markdown. Do not drop the regenerated files to leave a spec-only diff.

People write specs. The agent writes the Rust and the Markdown.

License: MIT. Copyright holder: Richard Naszcyniec.

## Commit convention

All commits use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>
```

| Type | When to use |
|---|---|
| `feat` | New example, new skill, or new spec section |
| `fix` | Correction to spec content, with regenerated output in this commit or the next `gen` commit |
| `spec` | Spec-only change, only when generation has not been run yet |
| `gen` | Regenerated files that follow a spec change already committed |
| `spec+gen` | Spec edit and the regenerated files in one commit |
| `chore` | Maintenance of files that are not agent-generated (for example `.gitignore`) |
| `docs` | Do not use this type here. A documentation correction is a spec edit plus regeneration |

Scope is optional. Use the folder name (`blinky`, `skills`, `shared-specs`).

Summary: imperative mood, lowercase, no trailing period, 72 characters or fewer.

```
feat(blinky): add FunctionalSpec for LED blink example
gen: regenerate docs after contributing-spec update
fix(shared-specs): correct esp-hal version pin to 1.2.1
chore: update .gitignore to exclude target/
spec: retarget coding conventions to no_std
spec+gen: describe the spec workflow and regenerate the docs
```

For `gen` and `spec+gen` commits, append a trailer. Use the name of the agent that produced the commit.

```
gen: regenerate CONTRIBUTING.md after contributing-spec update

Generated-by: Grok
```

This matches the OpenSSF guidance (September 2025) for machine-readable AI attribution in git history.

## Code of Conduct

Participation is covered by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Do not put vulnerability details in a public issue. See [SECURITY.md](SECURITY.md).

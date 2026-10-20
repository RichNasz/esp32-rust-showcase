# Documentation Generation Specification (AI-First Rule)

This file is the single source of truth for ALL Markdown documentation.

## Core Rules

- Human input strictly limited to `/specs/`, `/shared-specs/`, and `examples/*/specs/{Functional,Coding,Test}Spec.md`.
- Generated files:
  - Root: README.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, AGENTS.md
  - GitHub templates: `.github/ISSUE_TEMPLATE/spec-improvement.md`, `.github/ISSUE_TEMPLATE/bug-report.md`, `.github/PULL_REQUEST_TEMPLATE.md`
  - Extended docs: `/docs/`, example READMEs, `board-specs/README.md`, `skills/README.md`, etc.
- Every generated `.md` file must carry the standard agent-generated header.
- Generated Rust files carry `// AGENT-GENERATED — DO NOT EDIT BY HAND` on line 1.
- Tone: professional, precise, enthusiastic, community-focused. On human-facing pages, define SDD, spec, skill, generate, and regenerate at first use, using the glossary in **Readers**. Do not open those pages with "100% AI-first" unless the next sentence says what that means: a person writes specs, and the agent writes the Rust and the Markdown.
- Every generated file that describes or depicts example directory layout must match `specs/example-project-structure-spec.md` exactly.
- **NEVER generate files inside `.github/workflows/`** — GitHub restricts access to this directory without workflow scope.
- Primary build / flash / monitor tool is **Cargo + espflash** (`cargo run --release`). Do **not** present `idf.py` or ESP-IDF as the build system.
- Purpose of this repository: Spec-Driven Development for **rust-on-metal ESP32** (`no_std` `esp-hal` on ESP32-C6).

### Chip vs board (required distinction)

**Chip (SoC)** and **board** are different axes. rust-on-metal does **not** port across ESP32 / S3 / C6 the way an ESP-IDF `idf.py set-target` project often does. `esp-hal` selects the SoC with a Cargo **chip feature**; peripherals, RTC GPIO maps, USB, LP-core vs ULP, and the rustc target are chip-gated.

| Axis | What it selects | v1 |
|---|---|---|
| **Chip (SoC)** | `esp-hal` chip feature, rustc target, RTC/LP GPIO set, LP-core, radio/`unstable` | **`esp32c6` only** |
| **Board** | Pins, LED type, USB-UART vs native USB, antenna, LiPo — via `board-specs/` → generated `src/board.rs` | `board-devkitc-c6` (default), optional `board-xiao-c6` |

v1 firmware is **one chip per example**. Multi-board on that chip is allowed. Do not generate `if S3 { … } else if C6 { … }` specs or READMEs. The board catalog is ESP32-C6 only. Do not add non-C6 board-specs. Switching SoC later means a new chip feature, a `rust-toolchain.md` row, new board-specs, and new example specs — not `set-target`.

Root README (project description or Features) and `docs/README.md` must each include one short paragraph stating this. `docs/guides/why-esp-idf-is-not-required.md` must include a **Chip vs board** subsection: IDF `idf.py set-target` is **not** equivalent to swapping `esp-hal` chip features without a new spec. `docs/reference/spec-format.md` Target Hardware must name **SoC** separately from the board checklist. `docs/guides/04-new-example.md` (Hardware bar) and `examples/README.md` (intro) must each include a **Chip ≠ board** one-liner (`esp32c6` vs board features; not `set-target`).

---

## Readers

Generated docs serve more than one reader. Keep the assumptions already stated elsewhere in this file:

- **Testing sections** — rustup and `espflash` are installed; the reader is not an embedded-Rust expert.
- **Limitations** — an engineer judging whether an example is fit for production use.
- **Why ESP-IDF is not required** — a C or C++ ESP32 developer who usually installs ESP-IDF.

**Also write for this reader:** someone looking for a way to apply an **AI coding agent** to Rust development for ESP32. They may already write Rust, or they may be arriving from C and ESP-IDF. They want the agent to help them produce `no_std` `esp-hal` firmware on ESP32-C6. They are not assumed to know this repository's workflow.

Two tasks, and generated docs must keep them separate:

| Task | What the reader needs first | Where they start |
|---|---|---|
| **A. Flash an existing example** | rustup and `espflash`, or the willingness to install them. No agent. | `docs/guides/01-toolchain-setup.md`, then `docs/guides/03-first-example.md` |
| **B. Change firmware or docs with an AI coding agent** | An agent that can read Markdown. Spec-driven development is not assumed. | `docs/guides/using-with-ai-agents.md`, then `docs/guides/04-new-example.md` |

A C or C++ reader still gets `docs/guides/why-esp-idf-is-not-required.md` before either task.

### Glossary

Use these meanings. Do not invent a second definition in a generated page.

| Term | Meaning on the page |
|---|---|
| **spec** | Markdown a person writes. The agent reads it and writes Rust and documentation. |
| **SDD** | Spec-driven development: specs are the source of truth for code and docs. Spell it out on first use. |
| **skill** | A Markdown workflow in `skills/<name>/SKILL.md`. The person asks the agent to follow it. It is not a shell command and not a Rust crate. |
| **generate** | The agent writes crate files or Markdown from the specs. |
| **regenerate** | Generate again. This overwrites the previous agent output and leaves the spec files in place. |
| **chip** | The SoC. v1 is ESP32-C6, Cargo feature `esp32c6`. |
| **board** | The module, pins, LED, and USB. v1 is DevKitC-C6 (default) and XIAO ESP32-C6. |

**Do not assume** this reader already knows spec-driven development. The first time a generated doc relies on any item below, define it in plain language, in this order, before skill names, directory trees, or the phrase "humans edit specs only":

1. **Specs are the source of truth** for both firmware and documentation. A spec is a Markdown file a person writes. It states what the firmware must do and what the docs must say. An AI coding agent reads those files and writes the Rust and the Markdown. People do not hand-edit `src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, READMEs, or `/docs/`. When generated output is wrong, change the spec and ask the agent to generate again.
2. **How to set up specs in a project.** Human files live in repository `specs/` and `shared-specs/`, and in `examples/<name>/specs/FunctionalSpec.md`, `CodingSpec.md`, and `TestSpec.md`. Say what each file is for, in one sentence: FunctionalSpec is observable behavior, CodingSpec is architecture and constraints with no code, TestSpec is how to check the example, `specs/` is repo-wide rules, `shared-specs/` is cross-example platform constraints. A new example starts by adding a row to `specs/showcase-master-spec.md` and creating those three example spec files. Point at `templates/FunctionalSpec.template.md` and `specs/example-project-structure-spec.md`.
3. **How to modify specs.** Behavior changes go in `FunctionalSpec.md`. Architecture, crates, and gotchas go in `CodingSpec.md`. Checks go in `TestSpec.md`. Repo-wide documentation rules go in `specs/`. Edit those files in the editor. Do not present a hand-edit of generated Rust or generated Markdown as the way to change behavior or docs.
4. **How to generate code and docs from specs.** Give the phrase to type and what the agent writes:
   - "Activate esp32-sdd-functional-spec-creator" — drafts a `FunctionalSpec.md` with the person.
   - "Activate esp32-sdd-full-project-generator" — writes the Cargo crate (`src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`) from the example specs. It leaves `specs/` in place.
   - "Activate esp32-sdd-documentation-generator and rebuild all documentation." — rewrites the Markdown listed under Generated files in Core Rules (root README, `CONTRIBUTING.md`, `/docs/`, example READMEs, and the rest of that list). Run this after a spec change.

   Say that generation overwrites the previous agent output.

### Where this explanation must appear

Do not add a new guide file for this reader. Teach the four items in the files below.

| File | What to add |
|---|---|
| Root `README.md` project description (required section 3, what + why + who) | Three to five sentences. Name this reader: they want an AI coding agent to help with ESP32 Rust, and they do not need prior experience with specs as the source of truth. Define SDD in that same paragraph. Link to `docs/guides/using-with-ai-agents.md`. |
| Root `README.md` Quick Start | Both tasks from the table above. Wording is in **Quick Start (root README)** below. Task A has no skill name. Task B defines spec in one sentence and says the activate phrases go to the agent, not the shell. |
| `CONTRIBUTING.md` | Opening paragraph: who the page is for, then spec vs generated file. A pull request contains the spec edit and the regenerated files. It is not a hand-edit of generated Rust or Markdown, and it is not a spec-only patch that drops the regenerated output. Link `docs/guides/using-with-ai-agents.md`. Source text: `specs/contributing-spec.md`. |
| Issue and PR templates | One sentence in the form: a spec is the human Markdown; generated files are fixed by changing that spec and regenerating. Keep the existing checklists. |
| `docs/README.md` | A **What a spec is** subsection immediately before "The SDD Workflow in Three Steps". Cover item 1. The three steps must not be the first mention of specs. |
| `docs/guides/using-with-ai-agents.md` | Open with why a reader would use an AI coding agent for ESP32 Rust, then items 1–4, then agent setup (Claude Code, Grok, other agents). Do not open on install steps. Keep the incremental workflow diagram after the four items. |
| `docs/guides/02-install-skills.md` | Define **skill** with the glossary sentence before Claude, Grok, or `npx` steps. State that `skills/` is already in the clone. Link items 1–4 to `using-with-ai-agents.md` instead of repeating them. |
| `docs/guides/03-first-example.md` | Task A only. Say no agent is required. The closing "if something looks wrong" note says not to edit `src/`, points at `examples/blinky/specs/`, and links `using-with-ai-agents.md`. |
| `docs/guides/04-new-example.md` | Before the numbered workflow, one paragraph on setting up the three spec files (item 2) and one paragraph on modifying them (item 3). Keep a one-line reason on each numbered step. |
| `docs/workflows/README.md` | Under the Core Principle diagram, one sentence that states item 1, not only the slogan. Workflows 1, 3, and 4 each start with one sentence saying which file the person edits and which files the agent writes. |
| `docs/reference/spec-format.md` | Before the section tables, items 2 and 3 in short form: which file the person creates, and which file they edit for which kind of change. |
| `docs/reference/skill-invocation.md` | Glossary definition of skill, then the phrase table. Say the phrases are typed to the agent, not run in a shell. |
| `docs/guides/README.md` | Recommended Reading Order shows task A and task B. A reader who has not used specs as the source of truth reads [Using With AI Agents](using-with-ai-agents.md) before guide 04. |
| Each `examples/<name>/README.md` | The first time the README says the crate or the page was generated from specs, add one sentence plus a link to `docs/guides/using-with-ai-agents.md`. Do not paste items 1–4 into every example. The **Spec Files** section names FunctionalSpec, CodingSpec, and TestSpec in one sentence each. |

Expand these terms on first use in each of those files: SDD (spec-driven development), spec, FunctionalSpec, CodingSpec, TestSpec, skill, generate, regenerate.

File-by-file content for the rest of `/docs/` is **Documentation set** at the end of this spec. That section is the checklist. Do not drop a file because another section already mentions it.

---

## Example implementation status

`specs/showcase-master-spec.md` section **Implementation status** is the only catalog of which demos have a crate. Each example `FunctionalSpec.md` repeats that example's row in an **Implementation status** section. Generated docs copy those facts. They do not invent hardware results, and they do not describe a **Specs only** example as if `cargo run` works today.

### Status labels

| Label | Meaning in the tree |
|---|---|
| **Firmware generated** | `examples/<name>/Cargo.toml` and `src/main.rs` exist. Build commands in the README apply to that crate. |
| **Specs only** | `specs/FunctionalSpec.md`, `CodingSpec.md`, and `TestSpec.md` exist. There is no crate. The README describes the spec. Generation has not been run. |

A label is not a pass/fail from a board. Do not write "tested", "verified on hardware", or "flashes successfully" unless the master-spec note says that, with a date.

### Where to render it

- **Root `README.md` examples table:** columns `#`, Example, Description, Status, Link. Description is the one-liner from the master catalog, not a retired claim (no "multi-year battery", no "signed production updates", no "Wi-Fi-free mesh"). The Features bullet must state how many crates exist and how many examples are specs only, using the counts in the status table.
- **`examples/README.md`:** the learning-progression table gains a Status column copied from the master spec. State the Firmware generated count and the Specs only count from that table. When Specs only is zero, say every example folder contains a generated crate and the locked directory tree is the layout on disk. When it is not zero, name those folders and say they contain `specs/` and `README.md` until `esp32-sdd-full-project-generator` runs. The build snippet stays on `blinky`.
- **`docs/README.md` examples catalog:** add a Status column. The opening paragraph must match the catalog (ESP-NOW is a sensor-to-gateway pair; OTA is a slot flip; LP-core is GPIO 0–7). Do not claim a mesh.
- **Each `examples/<name>/README.md`:** an **Implementation status** section immediately after the title and tagline, before Overview. Quote the label, the date, and the note from the master-spec row. **Specs only** READMEs put `cargo build` / `cargo run` under a heading **After the crate is generated**, and say in that section that those commands fail today because there is no `Cargo.toml`.
- **`specs/README.md`:** the `showcase-master-spec.md` row mentions the implementation-status table.
- **`docs/guides/07-secure-your-project.md`:** the OTA paragraph must match `examples/secure-ota-https/specs/FunctionalSpec.md` (TLS 1.3, CRC32, no stock-`espflash` auto-rollback) and the implementation-status row in `showcase-master-spec.md`. Do not write **Specs only** when that row says **Firmware generated**.
- **`docs/reference/README.md` chip-variant table:** ESP32-C6 capacitive touch is **No**. C6 has no touch peripheral. Do not point that row at `lp-core-wakeup`. That example is LP-core GPIO wakeup on RTC GPIO 0–7. Its status is the master-spec row, not a hardcoded **Specs only**.

When a FunctionalSpec implementation-status paragraph disagrees with the master-spec table, the table wins. Update the FunctionalSpec to match before generating docs. Do not publish both claims.

When any spec changes status, regenerate these files. Do not leave a README describing the previous design (S3 touch, IDF mesh, signed Secure Boot demo).

---

## Root README.md Required Sections

The root `README.md` must contain the following sections in this exact order (numbers 0 through 13):

| # | Section |
|---|---|
| 0 | Agent-generated header |
| 1 | Badge row |
| 2 | Title + tagline |
| 3 | Project description (what + why + who), 3–5 sentences, per **Readers**. Link to `docs/guides/using-with-ai-agents.md`. |
| 4 | Features: 4–6 bullets covering unique value props |
| 5 | Table of contents with jump links |
| 6 | Quick Start |
| 7 | Prerequisites (rustup stable, rust-src, riscv32imac-unknown-none-elf, espflash/cargo-espflash, ESP32-C6-DevKitC-1-N8 or XIAO ESP32-C6) |
| 8 | Examples table (all examples: name + one-line description + **implementation status** + link). Status comes from `showcase-master-spec.md`. See Example implementation status. |
| 9 | Skills table (skill name + purpose). Copy the purpose column from `specs/showcase-master-spec.md`. Do not invent a second purpose. |
| 10 | Full Documentation link table |
| 11 | Contributing (one paragraph + link to CONTRIBUTING.md) |
| 12 | License (MIT — see LICENSE) |
| 13 | Footer (generated-by credit using the **invoking agent name**, not a hardcoded vendor) |

### Badge Row (exact markdown)

```markdown
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.2-blue)
![Chip](https://img.shields.io/badge/chip-ESP32--C6-orange)
[![Made with Grok Build](https://img.shields.io/badge/Made%20with-Grok%20Build-black)](https://x.ai/cli)
```

The Grok Build badge is a static shields.io attribution (not a CI status). Link it to `https://x.ai/cli`. Do not add an ESP-IDF badge. Do not add a CI/build-status badge — `.github/workflows/` is off-limits and a broken badge is worse than no badge.

### Quick Start (root README)

Show both tasks from **Readers**. Do not start this section with a skill name.

**Task A — flash an example.** No AI agent required. From `examples/blinky`:

```sh
cargo run --release
```

Optional XIAO board:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

One sentence: this builds the crate already in the tree and flashes it. Link `docs/guides/01-toolchain-setup.md` and `docs/guides/03-first-example.md`.

**Task B — change firmware or docs with an AI coding agent.** Define spec in one sentence (glossary). Then:

1. Open this repository in the agent. Workflows are already in `skills/`. A separate skill install is not required.
2. Read `docs/guides/using-with-ai-agents.md` for how specs drive code and docs.
3. Edit a human spec. For the blink example, `examples/blinky/specs/FunctionalSpec.md` is what the LED does.
4. Ask the agent to generate. Crate phrase: "Activate esp32-sdd-full-project-generator". Markdown phrase: "Activate esp32-sdd-documentation-generator and rebuild all documentation."

Say those phrases are typed to the agent, not in the shell.

### Prerequisites (root README)

Must list:

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: ESP32-C6-DevKitC-1-N8 (required default) or XIAO ESP32-C6

Must **not** list ESP-IDF, `idf.py`, or `espup` as prerequisites.

After that list, add 2–4 sentences on why ESP-IDF, `espup`, and the `esp` rustup channel are unused, plus a link to `docs/guides/why-esp-idf-is-not-required.md`. Facts are in **Guide: Why ESP-IDF is not required**. "Not required" alone is not enough.

---

## Root AGENTS.md (required)

Grok (and compatible agents) auto-load root `AGENTS.md` at session start. It is the shareable
new-session context. Keep it **short and actionable**. Do **not** duplicate the root README,
skill catalog, or example table — link to those files instead.

Audience for this file is the agent that loads it at session start. A person who wants to apply an agent to ESP32 Rust starts at `docs/guides/using-with-ai-agents.md`, not here. Do not paste the Readers four-item lesson into `AGENTS.md`.

Must carry the standard agent-generated header, then these sections **in this order**:

1. **What this repo is** — Spec-Driven Development for rust-on-metal ESP32-C6 (`no_std`
   `esp-hal`). There is no ESP-IDF, no `idf.py`, no `sdkconfig`.
2. **Before any generate / edit / debug** — read `shared-specs/AIGenLessonsLearned.md`;
   then follow `skills/<name>/SKILL.md`.
3. **Role split** — humans edit only `specs/`, `shared-specs/`, and
   `examples/*/specs/{Functional,Coding,Test}Spec.md`. Never hand-edit generated files.
   If output is wrong, fix the spec and regenerate.
4. **Toolchain** — rustup **stable** (MSRV 1.95.0), target `riscv32imac-unknown-none-elf`,
   `espflash`. Not the `esp` rustup channel, not `espup`. Every firmware crate depends on
   `esp-bootloader-esp-idf` and invokes `esp_app_desc!()`. Build/flash from the example
   directory: `cargo run --release`.
5. **Chip vs boards** — chip feature `esp32c6` only (SoC ≠ board). GPIO from
   `board-specs/` via generated `src/board.rs`; never hardcode pins in `main.rs`.
   Default board feature `board-devkitc-c6`; optional `board-xiao-c6`. The board
   catalog is those two ESP32-C6 boards. Do not add other SoCs.
6. **Skills** — source of truth is `skills/` (not `.grok/skills/`). Invoke by name. Index:
   `skills/README.md`.
7. **Layout** — standalone Cargo crates under `examples/<name>/`. Locked structure:
   `specs/example-project-structure-spec.md`.

---

## New Generated Files

### CONTRIBUTING.md (root)

Source: `specs/contributing-spec.md`. Required sections, in this order:

1. Opening paragraph: who the page is for, and spec vs generated file (see **Readers**).
2. What a person may edit — the path table from contributing-spec.
3. Workflow: edit a spec, ask the agent to regenerate, commit the spec and the regenerated files together.
4. Commit convention: types, examples, and the `Generated-by` trailer.
5. Link to `CODE_OF_CONDUCT.md`.

A pull request includes the spec edit and the regenerated files. It does not hand-edit generated Rust or Markdown, and it does not drop the regenerated files to leave a spec-only diff.

### CODE_OF_CONDUCT.md (root)

Based on Contributor Covenant v2.1 adapted for AI-first projects. Source content from
`specs/contributing-spec.md` (Code of Conduct section). Must include:
- Our Pledge
- Expected Behaviour
- Prohibited Behaviour
- Enforcement
- Reporting instructions: open a GitHub Issue with the `security` label (do not disclose
  vulnerability details publicly); maintainer will follow up privately within 48 hours.
- Attribution to Contributor Covenant v2.1.

### SECURITY.md (root)

Security policy document. Must include:
- Scope: firmware generation and build configurations; chip-level hardware security is out of scope.
- Reporting process: open a GitHub Issue with the `security` label, state only that a security
  issue exists (no public details), maintainer acknowledges within 48 hours and arranges private
  follow-up.
- What is in scope vs out of scope.
- Note: no coordinated disclosure timeline is guaranteed for this project stage.

### .github/ISSUE_TEMPLATE/spec-improvement.md

Guided form for spec change requests. Must include:
- YAML front matter: `name`, `about`, `labels: ["spec"]`, `assignees: ""`
- One sentence before the fields: a spec is the human-written Markdown; to change generated output, edit that spec and regenerate.
- Sections: Which spec file, Current behaviour/content, Proposed change, Motivation/rationale,
  Checklist (human-authored spec only, no generated files edited, documentation-generator run
  after change).

### .github/ISSUE_TEMPLATE/bug-report.md

Guided form for reporting generated-code defects. Must include:
- YAML front matter: `name`, `about`, `labels: ["bug"]`, `assignees: ""`
- One sentence before the fields: if the bug is in generated code, the fix is a spec change and a regeneration, not a hand-edit of `src/`.
- Sections: Which example, rustc / `esp-hal` version, Board, Describe the bug, Steps to reproduce,
  Expected behaviour, Actual behaviour, `cargo build --release` log (code block), Checklist
  (confirmed not a hand-edit of generated files).
- Do **not** ask for an ESP-IDF version.

### .github/PULL_REQUEST_TEMPLATE.md

PR checklist enforcing the spec workflow. Must include:
- One sentence above the checklist: specs are the human Markdown; this PR should contain that edit plus the files the agent regenerated from it.
- Summary section.
- Checklist items:
  - [ ] This PR contains a spec change (file in `specs/`, `shared-specs/`, or example
        `FunctionalSpec.md` / `CodingSpec.md` / `TestSpec.md`)
  - [ ] `esp32-sdd-documentation-generator` was run after the spec change
  - [ ] No agent-generated files were edited by hand
  - [ ] Commit messages follow the Conventional Commits format (see CONTRIBUTING.md)
  - [ ] I have read and agree to the Code of Conduct

---

## CodingSpec.md Standard

CodingSpec.md is high-level guidance — architecture, constraints, preferred crates,
non-functional requirements, and gotchas. It must never contain code snippets, specific function
calls, or step-by-step implementation instructions. When generating or reviewing documentation
that describes CodingSpec.md, reflect this standard accurately.

---

## Example README required order

Every `examples/<name>/README.md` uses this order. Later sections in this file set the content of each block. They do not add a second order.

| # | Section | When |
|---|---|---|
| 0 | Agent-generated header | Always |
| 1 | Title + one-line tagline | Always. Tagline is the master-catalog one-liner. |
| 2 | Implementation status | Always, before Overview. Quote the master-spec row. |
| 3 | Overview | What a person can observe. First mention of specs: one sentence plus a link to `docs/guides/using-with-ai-agents.md`. |
| 4 | Prerequisites | Board, USB port, link to `docs/guides/01-toolchain-setup.md`. ESP-IDF is not a prerequisite. |
| 5 | Supported Boards | Always. Auto-discovered. One **Chip ≠ board** sentence in the intro. |
| 6 | Per-Board Behavior | Only when the Supported Boards table has two or more rows. Immediately after that table. |
| 7 | Build & Flash | Immediately after Per-Board Behavior, or after Supported Boards when Per-Board Behavior is omitted. Include the VS Code / Cursor "open only this folder" steps as a subsection. |
| 8 | Expected behavior | Only when FunctionalSpec or TestSpec states an observable result (LED, serial line, packet). Do not invent log strings. |
| 9 | Key Concepts | Always. |
| 10 | Limitations | Only when FunctionalSpec has a Limitations heading. Immediately after Key Concepts. |
| 11 | Testing | Only when TestSpec.md exists. After Limitations, or after Key Concepts when Limitations is omitted. Before Spec Files. |
| 12 | Spec Files | Always. One sentence each for FunctionalSpec (observable behavior), CodingSpec (architecture and constraints, no code), and TestSpec (how to check). Say edits go in those files, then `esp32-sdd-full-project-generator`. Link `docs/guides/using-with-ai-agents.md`. |

**Specs only** READMEs keep this order. Build & Flash is titled **After the crate is generated** and says `cargo build` / `cargo run` fail today because there is no `Cargo.toml`.

---

## Testing Section Generation

Every example README.md that has a TestSpec includes a **Testing** section generated from that file. Position is row 11 of **Example README required order**.

### Testing Philosophy (enforce in every generated Testing section)

- Automated testing using native Cargo / rustc tools is strongly preferred
  (`cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi).
- Additional tools are only included when TestSpec.md provides explicit justification and
  documented setup steps.
- Manual testing steps are only included when no practical automated alternative exists.
- Do not mention Unity, `idf.py`, or ESP-IDF test components.

### Generated Testing Section Structure

The Testing section sits where **Example README required order** places it (after Key Concepts, and after Limitations when that section exists, and before Spec Files). It must:

1. Open with the testing philosophy as a one-sentence note.
2. List **Automated Tests** first (subsection), with numbered steps. If none exist, omit the
   subsection rather than showing an empty list.
3. List **Manual Tests** second (subsection), clearly labelled "Manual (hardware required)".
   Only include if TestSpec.md specifies manual steps. Explain in a note why automation is not
   practical for each step.
4. Never mix automated and manual steps in the same numbered list.
5. Provide beginner-friendly wording — assume the reader has rustup + espflash installed but is
   not an embedded-Rust expert. Do not assume they know spec-driven development; follow **Readers**
   when the Testing section mentions specs, skills, or regeneration (one sentence plus a link is
   enough inside an example README).

### When TestSpec.md Is Absent

If `specs/TestSpec.md` does not exist for an example, omit the Testing section entirely from
the README. Do not generate a placeholder.

---

## Limitations Section Generation

When `specs/FunctionalSpec.md` includes a "Limitations and Topology Characterization"
section (or any section whose heading contains "Limitations"), the generated README.md
**must** include a corresponding **Limitations** section.

### Rules

- **Position**: immediately after "Key Concepts" and before "Testing" (or before
  "Spec Files" if Testing is absent).
- **Content**: reproduce the shortcomings and extension guidance from FunctionalSpec.md
  accurately. Never soften, omit, or reframe limitations to make the example appear
  more capable than it is.
- **Tone**: factual and constructive. State what the example does not do, then provide
  a concrete path to the capability if the reader needs it.
- **Tables**: preserve any "What this example is NOT" table and the extensions table
  from the FunctionalSpec verbatim in structure; adapt prose as needed for README style.
- **When absent**: if FunctionalSpec.md has no Limitations section, omit this section
  from the README entirely — do not generate a placeholder.

### Why this rule exists

Over-claiming example capabilities (e.g. calling a tree topology with partial recovery
a "self-healing mesh") misleads engineers evaluating the example for production use.
Documented limitations give readers an accurate mental model and a clear upgrade path
rather than discovering the gap at deployment time.

---

## Radio / `unstable` Honesty

Every example that enables `esp-hal` feature `unstable` (radio, some sleep / RTC APIs) **must**
state in its README that those APIs are unstable and may change with `esp-hal` minor releases.
`esp-radio` 1.0.0-beta.0 is a beta crate — radio example READMEs must say so.

---

## Supported Boards Auto-Discovery

When generating an example README.md, the Supported Boards section **must be
auto-generated by scanning `board-specs/`** — do not copy a hardcoded table from
FunctionalSpec.md. The FunctionalSpec provides only the compatibility requirements
(Board Compatibility Checklist) and the default board name; the generator derives
everything else from board-spec files.

v1 catalog is ESP32-C6 only. Default board: **Espressif ESP32-C6-DevKitC-1-N8**.
Also supported: **Seeed XIAO ESP32-C6**. The board catalog is those two boards. Do not add other SoCs.

### Discovery workflow

1. **Scan `board-specs/**/*.md`**, excluding any file named `README.md`.
2. **Apply the FunctionalSpec's Board Compatibility Checklist** to each board-spec.
   Required checks (any failure = exclude the board):
   - SoC is ESP32-C6 — check board-spec Module/SoC table. Non-C6 boards fail this check in v1.
   - Chip listed as **Required** (or supported) in `shared-specs/rust-toolchain.md` (ESP32-C6 required; others not in v1).
   - Example-specific radios (Wi-Fi, BLE, ESP-NOW) present when the FunctionalSpec requires them.
   - Deep sleep with RTC timer wakeup supported when the FunctionalSpec requires it — check board-spec Power Management.
   - At least one user-controllable LED documented in board-spec Onboard Peripherals.
3. **Derive Supported Boards table columns** from board-spec data:

   | Column | Source in board-spec |
   | --- | --- |
   | Board name | Product heading / name |
   | board-spec file | Relative path under `board-specs/` |
   | Chip feature | `esp32c6` (from SoC; v1 has no other chip feature) |
   | Board Cargo feature | DevKitC-C6 → `board-devkitc-c6`; XIAO C6 → `board-xiao-c6` |
   | LED GPIO | Onboard Peripherals table — user LED row, GPIO column |
   | LED polarity | Onboard Peripherals table — active HIGH / active LOW note |
   | LED type | Onboard Peripherals table — "Simple GPIO" or "WS2812 RGB" / "SK6812" |
   | USB console | USB Serial/Console section: UART bridge chip → "UART bridge (`espflash` on the UART-C port)"; Native USB Serial/JTAG → "USB Serial/JTAG"; Native USB CDC → "USB CDC". Do **not** emit `CONFIG_ESP_CONSOLE_*` sdkconfig lines — there is no sdkconfig. |

4. **Mark the default board** with `*(default)*` using the board name stated in
   FunctionalSpec (e.g. `> Default board: Espressif ESP32-C6-DevKitC-1-N8`).
5. **Add WS2812 footnote** automatically whenever any row in the table has LED type
   WS2812 or SK6812. Standard footnote text: "A GPIO level alone produces no
   visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform."
6. **Add an exclusion callout** only when a scanned board-spec fails a required
   check, and name the reason. The v1 catalog is the two C6 boards. A scan that
   passes both has no exclusion callout. Do not invent a list of boards that are
   not in `board-specs/`.

### Why auto-discovery, not FunctionalSpec tables

When a new board-spec is added to `board-specs/` (via `esp32-board-spec-generator`),
the auto-discovery rule ensures it appears in all compatible example READMEs the next
time the documentation generator runs — with no manual FunctionalSpec edit required.
The FunctionalSpec remains the source of truth for *what makes a board compatible*;
the board-spec files are the source of truth for *the hardware facts of each board*.
v1 still filters to ESP32-C6.

---

## Per-Board Behavior Section Generation

Every example README.md that supports more than one board must include a **Per-Board
Behavior** section. This section answers a different question than the supported boards
table: not *"what do I need to change?"* but *"what will I observe differently?"*

### When to include

Include this section whenever the auto-discovered Supported Boards table contains two
or more rows. If only one board is compatible, omit the section entirely.

### Required position

The Per-Board Behavior section must appear immediately after the Supported Boards table
and before the Build & Flash section.

### Required content

The section must contain a table with one row per supported board and one column per
behavioral dimension that varies across boards. Only include columns where at least one
board differs from the default. **Derive all values from board-spec files** using the
rules below — do not require the FunctionalSpec to maintain a separate Per-Board
Behavior table.

| Column | Derivation rule |
| --- | --- |
| Serial monitor during deep sleep | UART bridge chip (CP2104, CH343P, etc.) → "Stays connected throughout sleep"; Native USB Serial/JTAG → "Drops and reconnects on wake (USB Serial/JTAG)"; Native USB CDC → "Drops and reconnects on wake (USB CDC)" |
| LED feedback | WS2812/SK6812 → "Color: green = success, red = failure, off = sleep (WS2812)"; Simple GPIO active LOW → "On / off, active LOW"; Simple GPIO active HIGH → "On / off, active HIGH" |
| WiFi scan coverage | v1 C6 boards are 2.4 GHz Wi-Fi 6. Omit this column unless a C6 board-spec documents a different band. |
| Deep sleep floor | From board-spec Power Management table deep sleep current figure; use "See board-spec" if no figure is documented |

### Formatting rules

- Use spaced table separators `| --- | --- |`.
- Mark the default board row with `*(default)*`.
- Add a short paragraph after the table calling out the single most consequential
  behavioral difference for a first-time user (typically: UART-bridge boards keep the
  serial monitor connected through deep sleep; native USB boards drop and reconnect —
  this is expected, not a crash).
- If any board in the table has dual-band WiFi, add a sentence noting that it is the
  only board that can scan 5 GHz APs and may report RSSI values invisible to other
  boards.

---

## Build & Flash Section Generation

Example READMEs must document:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

and the XIAO feature variant. Do **not** document `idf.py build`, `idf.py flash`, or
`idf.py monitor` as the primary workflow.

---

## Guide: Why ESP-IDF is not required

C/C++ ESP32 developers typically install **ESP-IDF** (Espressif IoT Development
Framework): Python, CMake/Ninja, a GCC toolchain, `idf.py`, `sdkconfig` /
`menuconfig`, and IDF components. This repository does **not** use that toolkit.
Generated docs must explain **why**, not only state that it is absent.

### Required generated file

`docs/guides/why-esp-idf-is-not-required.md`

Must appear in:

- `docs/guides/README.md` (conceptual guide; list it with the other unnumbered
  guides, description: “Why C developers do not need ESP-IDF / `idf.py` for this
  rust-on-metal repo”)
- `docs/README.md` documentation-sections table
- Root README **Full Documentation** table
- `docs/guides/01-toolchain-setup.md` — next to the existing “no ESP-IDF install”
  statement, link to this guide
- Root README **Prerequisites** — after listing what *is* required, add 2–4
  sentences of *why* ESP-IDF / `espup` / the `esp` rustup channel are unused,
  plus a link to this guide. Do not stop at “not required.”

### Facts the guide must state (verified against Espressif / esp-rs, 2026-09)

There are **two** Rust-on-ESP32 ecosystems. Do not conflate them.

1. **`std` + ESP-IDF** (`esp-idf-sys`, `esp-idf-hal`, `esp-idf-svc`): Rust
   bindings around the C ESP-IDF SDK. This path **does** require an ESP-IDF
   install. Espressif marked these crates **community-supported** (Feb 2025
   `esp-hal` 1.0 beta announcement). This repository does **not** use this path.
2. **`no_std` + `esp-hal`** (this repository): Espressif’s official rust-on-metal
   HAL. It is a bare-metal (`no_std`) HAL that **does not use ESP-IDF**. Firmware
   talks to peripherals through Rust drivers (`esp-hal`, `esp-radio`,
   `esp-lp-hal`), not through IDF C drivers.

**Why ESP-IDF is not installed here:**

- **HAL is bare-metal.** `esp-hal` is documented as a `no_std` HAL for Espressif
  devices. GPIO, UART, SPI, I2C, RMT, AES, RTC sleep, and the LP core are driven
  from Rust. There is no `idf.py`, no `sdkconfig`, no IDF component registry.
- **ESP32-C6 is RISC-V.** The official *Rust on ESP Book* (Toolchain
  Installation): for RISC-V chips, install rustup **stable** (or nightly) with
  `rust-src` and `rustup target add riscv32imac-unknown-none-elf`. If you are
  **not** targeting ESP32 / ESP32-S2 / ESP32-S3 (Xtensa), you skip `espup`.
  `espup` and the `esp` rustup channel exist because Xtensa is not an upstream
  LLVM/Rust target yet. v1 of this showcase is **C6 only**.
- **Build is Cargo.** Chip (`esp32c6`) and board (`board-devkitc-c6` /
  `board-xiao-c6`) are Cargo features. Pins live in generated `src/board.rs`.
  There is no `menuconfig`.
- **Flash / monitor is `espflash`.** `cargo run --release` uses
  `runner = "espflash flash --monitor"`. That is the official `esp-rs` serial
  flasher, not `idf.py flash` / `idf.py monitor`.
- **Radio is `esp-radio`.** Wi-Fi / BLE / ESP-NOW use `esp-radio` on top of
  `esp-hal` plus Espressif **binary blobs**. That is not the ESP-IDF Wi-Fi/BT
  stacks. Radio APIs remain `unstable` / beta — document that honestly.

**C-developer mapping table** (required in the guide):

| ESP-IDF / C habit | This repository |
|---|---|
| `idf.py set-target esp32c6` | Cargo chip feature `esp32c6` — **v1 has no other chip**. A different SoC is a new spec, not a flag flip |
| `idf.py set-target esp32s3` (or any non-C6) | **Not in v1.** Do not swap chip features on an existing C6 example |
| `sdkconfig` / `menuconfig` | Cargo features + generated `src/board.rs` |
| `idf.py build` / `flash` / `monitor` | `cargo run --release` (`espflash`) |
| IDF components / `idf_component.yml` | crates (`esp-hal`, `esp-radio`, `esp-rtos`, …) |
| `nvs_flash_init` / IDF Wi-Fi/BT APIs | `esp-radio` (unstable beta) |

**Chip vs board** subsection (required in the same guide): rust-on-metal firmware is **not** portable across ESP32 / S3 / C6 via `set-target`. Chip = Cargo feature `esp32c6` (SoC). Board = pins/USB/LED via `board-specs/` and `board-devkitc-c6` / `board-xiao-c6`. Changing SoC is a new FunctionalSpec + toolchain row, not a feature flip on an existing example.

**Honest caveats** (required; do not over-claim):

- The crate `esp-bootloader-esp-idf` is **not** an ESP-IDF install. It only
  emits the ESP-IDF-*compatible application descriptor* (`esp_app_desc!()`)
  that current `espflash` and the chip’s second-stage bootloader expect.
- Espressif chips still use an ESP-IDF-format **second-stage bootloader** in
  ROM/flash (see *Rust on ESP Book* — Application Startup and Bootloader). You
  do not install ESP-IDF to use the bootloader already on a typical DevKit
  module.
- If a reader wants **`std` Rust wrapping ESP-IDF**, they would install ESP-IDF.
  That is a different ecosystem (`esp-idf-*` crates). Point them at
  [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/) and
  keep this repo on `esp-hal`.
- Xtensa chips (ESP32, S2, S3) need `espup` even for `no_std`. They are **not
  in v1**.

Tone: precise, welcoming to C developers, not dismissive of ESP-IDF.

Link official sources in the generated guide:

- [esp-hal](https://github.com/esp-rs/esp-hal) (`no_std` HAL)
- [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html)
- [esp-hal 1.0.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta)
- [Application Startup and Bootloader](https://docs.espressif.com/projects/rust/book/application-development/bootloader.html)

On first use in this guide, define `no_std` in one sentence (Rust without the standard library, running on the chip through `esp-hal`) so a reader who is not coming from C still knows which path they are on.

---

## Documentation set

Every path below is required. Do not add another guide for the AI-agent reader. **Readers** names which pages teach the four spec concepts. This section names what else each page must contain. Skill purposes and example one-liners are copied from `specs/showcase-master-spec.md`. Do not invent a second purpose or a second one-liner.

`AGENTS.md` stays the short agent context defined above. It is not part of the human getting-started path.

### Guides

| File | Required content |
|---|---|
| `docs/guides/README.md` | One row per guide in this table, with a task-shaped description (what the reader is trying to do). Recommended Reading Order shows task A and task B from **Readers**. C/C++ readers are pointed at the why-ESP-IDF guide first. Prerequisites match the root README list. |
| `docs/guides/01-toolchain-setup.md` | Install rustup stable, `rust-src`, clippy, rustfmt, target `riscv32imac-unknown-none-elf`, and `espflash`. MSRV comes from `shared-specs/rust-toolchain.md`. Link the why-ESP-IDF guide next to the "no ESP-IDF" statement. Name the DevKitC left UART-C port and the XIAO USB port. Close with one sentence: this page installs the compiler and flasher; the agent path is guide 02 and `using-with-ai-agents.md`. |
| `docs/guides/02-install-skills.md` | Audience rules in **Readers**. After the definition, cover Claude Code, Grok, and any agent that can read Markdown. Optional `npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills` stays optional. |
| `docs/guides/03-first-example.md` | Task A. Open only `examples/blinky/`. DevKitC left USB-C. `cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release`. XIAO feature line. Closing note per **Readers**. |
| `docs/guides/04-new-example.md` | Audience paragraphs from **Readers**, then the numbered workflow, then the locked layout from `specs/example-project-structure-spec.md` with each path labeled human-written or agent-written. **Chip ≠ board** one-liner in Hardware bar. |
| `docs/guides/05-add-board-spec.md` | One sentence: a board spec is the Markdown description of one board's pins, LED, USB, and power; the agent turns those facts into `src/board.rs`. v1 accepts ESP32-C6 boards only. Steps: give a vendor URL or datasheet, say "Activate esp32-board-spec-generator", add the board to the example FunctionalSpec checklist, regenerate docs so Supported Boards tables pick the file up. Link `creating-new-board-specs.md`. A board spec does not add a chip feature. |
| `docs/guides/creating-new-board-specs.md` | Long form of guide 05, using the ESP32-C6-DevKitC-1-N8 board spec as the walkthrough. Section list matches `templates/BoardSpec.template.md`. Do not publish a second template. |
| `docs/guides/06-deep-sleep-design.md` | Open with the task, not the skill name: edit the example FunctionalSpec, ask the agent to follow `esp32-deep-sleep-engineer`, then regenerate the crate with `esp32-sdd-full-project-generator`. Link `using-with-ai-agents.md` once. Then the C6 rules: RTC GPIO 0–7 only, tear down `esp-radio` before sleep, load the LP-core image before the HP core sleeps, persist state with the persistent-RAM attribute, UART-bridge vs native USB monitor behavior. Call `unstable` out. Point at `shared-specs/AIGenLessonsLearned.md` and `shared-specs/power-budget-template.md`. |
| `docs/guides/07-secure-your-project.md` | Same opening pattern for `esp32-hardware-crypto-configurer`. Does: hardware AES via `esp-hal`, keys via `esp-storage`, aligned with `shared-specs/security-baseline.md`. Does not: Secure Boot, eFuse burn, flash-encryption enrollment. OTA paragraph per the implementation-status rules above. |
| `docs/guides/using-with-ai-agents.md` | **Readers** items 1–4, then setup, the skills table from the master spec, the incremental workflow, the "open only the example folder" note, and troubleshooting: wrong output means edit the spec and regenerate. |
| `docs/guides/why-esp-idf-is-not-required.md` | **Guide: Why ESP-IDF is not required**, including the `no_std` sentence. |

### Workflows and reference

| File | Required content |
|---|---|
| `docs/workflows/README.md` | **Readers** rules for this file. Workflow 2 defines board spec in one sentence, then the steps from guide 05. Workflow 5: say "Activate esp32-sdd-project-validator"; fix a failure in the spec and regenerate; do not hand-edit the crate. Skill map uses the master-spec one-liners. The commit section matches `specs/contributing-spec.md`, including `spec+gen` and the `Generated-by` trailer. A short "why this method" restates glossary item **spec** before any benefit list. |
| `docs/reference/README.md` | Index of the three reference pages below, plus links to the human `shared-specs/` files. Chip-variant table per the implementation-status rules. No sdkconfig. |
| `docs/reference/spec-format.md` | **Readers** opening, then the FunctionalSpec / CodingSpec / TestSpec section tables. Target Hardware names SoC separately from the board checklist. Link `templates/FunctionalSpec.template.md`. |
| `docs/reference/skill-invocation.md` | **Readers** opening. Columns: skill, what the agent writes (master-spec purpose), phrase to type. New-example order: functional-spec-creator, full-project-generator, a domain skill only when the spec needs it, `cargo run --release`, documentation-generator, project-validator. |
| `docs/reference/pin-mapping-tables.md` | The two v1 C6 boards only. Facts come from `board-specs/`, not from a hand-typed second source. Include the WS2812 footnote when a board-spec says WS2812 or SK6812. Link each board-spec file. |

### Other generated indexes

| File | Required content |
|---|---|
| `docs/README.md` | Plain-language purpose (Readers), chip ≠ board paragraph, links to guides / reference / workflows, examples catalog with status, skills table from the master spec, repository layout with each top-level directory labeled human-written or agent-written, then **What a spec is** and the three-step workflow. |
| `examples/README.md` | Status rules in **Example implementation status**. Chip ≠ board one-liner. Build snippet on `blinky`. |
| `board-specs/README.md` | The two C6 boards, links to their spec files, and a link to `docs/guides/creating-new-board-specs.md`. One sentence on what a board spec is, for a reader who has not seen guide 05. |
| `skills/README.md` | The master-spec skills table and a pointer to `docs/reference/skill-invocation.md`. One glossary sentence for skill, plus a link to `docs/guides/using-with-ai-agents.md`. |
| Each `examples/<name>/README.md` | **Example README required order**. |

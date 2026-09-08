# Documentation Generation Specification (AI-First Rule)

This file is the single source of truth for ALL Markdown documentation.

## Core Rules

- Human input strictly limited to `/specs/`, `/shared-specs/`, and `examples/*/specs/{Functional,Coding,Test}Spec.md`.
- Generated files:
  - Root: README.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
  - GitHub templates: `.github/ISSUE_TEMPLATE/spec-improvement.md`, `.github/ISSUE_TEMPLATE/bug-report.md`, `.github/PULL_REQUEST_TEMPLATE.md`
  - Extended docs: `/docs/`, example READMEs, `board-specs/README.md`, `skills/README.md`, etc.
- Every generated `.md` file must carry the standard agent-generated header.
- Generated Rust files carry `// AGENT-GENERATED — DO NOT EDIT BY HAND` on line 1.
- Tone: professional, precise, enthusiastic, community-focused.
- Every generated file that describes or depicts example directory layout must match `specs/example-project-structure-spec.md` exactly.
- **NEVER generate files inside `.github/workflows/`** — GitHub restricts access to this directory without workflow scope.
- Primary build / flash / monitor tool is **Cargo + espflash** (`cargo run --release`). Do **not** present `idf.py` or ESP-IDF as the build system.
- Purpose of this repository: Spec-Driven Development for **rust-on-metal ESP32** (`no_std` `esp-hal` on ESP32-C6).

---

## Root README.md Required Sections

The root `README.md` must contain the following 13 sections in this exact order:

| # | Section |
|---|---|
| 0 | Agent-generated header |
| 1 | Badge row |
| 2 | Title + tagline |
| 3 | 2–3 sentence project description (what + why + who) |
| 4 | Features: 4–6 bullets covering unique value props |
| 5 | Table of contents with jump links |
| 6 | Quick Start |
| 7 | Prerequisites (rustup stable, rust-src, riscv32imac-unknown-none-elf, espflash/cargo-espflash, ESP32-C6-DevKitC-1-N8 or XIAO ESP32-C6) |
| 8 | Examples table (all examples: name + one-line description + link) |
| 9 | Skills table (all skills: skill name + purpose) |
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
```

Do not add an ESP-IDF badge. Do not add a CI/build-status badge — `.github/workflows/` is off-limits and a broken badge is worse than no badge.

### Quick Start (root README)

From `examples/<name>`:

```sh
cargo run --release
```

Optional XIAO board:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

### Prerequisites (root README)

Must list:

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: ESP32-C6-DevKitC-1-N8 (required default) or XIAO ESP32-C6

Must **not** list ESP-IDF, `idf.py`, or `espup` as prerequisites.

---

## New Generated Files

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
- Sections: Which spec file, Current behaviour/content, Proposed change, Motivation/rationale,
  Checklist (human-authored spec only, no generated files edited, documentation-generator run
  after change).

### .github/ISSUE_TEMPLATE/bug-report.md

Guided form for reporting generated-code defects. Must include:
- YAML front matter: `name`, `about`, `labels: ["bug"]`, `assignees: ""`
- Sections: Which example, rustc / `esp-hal` version, Board, Describe the bug, Steps to reproduce,
  Expected behaviour, Actual behaviour, `cargo build --release` log (code block), Checklist
  (confirmed not a hand-edit of generated files).
- Do **not** ask for an ESP-IDF version.

### .github/PULL_REQUEST_TEMPLATE.md

PR checklist enforcing SDD rules. Must include:
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

## Testing Section Generation

Every example README.md must include a **Testing** section generated from `specs/TestSpec.md`.

### Testing Philosophy (enforce in every generated Testing section)

- Automated testing using native Cargo / rustc tools is strongly preferred
  (`cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi).
- Additional tools are only included when TestSpec.md provides explicit justification and
  documented setup steps.
- Manual testing steps are only included when no practical automated alternative exists.
- Do not mention Unity, `idf.py`, or ESP-IDF test components.

### Generated Testing Section Structure

The Testing section must appear after "Key Concepts" and before "Spec Files". It must:

1. Open with the testing philosophy as a one-sentence note.
2. List **Automated Tests** first (subsection), with numbered steps. If none exist, omit the
   subsection rather than showing an empty list.
3. List **Manual Tests** second (subsection), clearly labelled "Manual (hardware required)".
   Only include if TestSpec.md specifies manual steps. Explain in a note why automation is not
   practical for each step.
4. Never mix automated and manual steps in the same numbered list.
5. Provide beginner-friendly wording — assume the reader has rustup + espflash installed but is
   not an embedded-Rust expert.

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
Also supported: **Seeed XIAO ESP32-C6**. Other C-showcase chips are not in v1.

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
6. **Add exclusion callout** for any board-spec in the catalog that fails a required
   check, naming the specific reason (e.g. "ESP32-S3 boards: not in v1 — C6 only").

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
| WiFi scan coverage | Dual-band WiFi 6 (e.g. ESP32-C5) → "**2.4 GHz + 5 GHz** (dual-band WiFi 6)"; Single-band WiFi 6 → "2.4 GHz (WiFi 6 ax)"; WiFi 4 (802.11n) → "2.4 GHz only". v1 C6 boards are 2.4 GHz WiFi 6. |
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

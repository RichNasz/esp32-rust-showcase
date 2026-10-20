<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 05 — Add a Board Spec

A **board spec** is the Markdown description of one board's pins, LED, USB, and power; the agent turns those facts into `src/board.rs`.

A **spec** is Markdown a person writes. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You say the activate phrase to the agent, not in the shell. **Regenerate** means generate again: the agent overwrites previous Markdown and leaves the spec files in place. The longer lesson is [Using With AI Agents](using-with-ai-agents.md).

v1 accepts ESP32-C6 boards only. A board spec does not add a chip feature. Chip feature `esp32c6` stays the SoC. Do not add other SoCs to `board-specs/`.

## Steps

1. Give the agent a vendor product-page URL or a datasheet. Prefer the vendor wiki over a third-party pinout.
2. Say: Activate esp32-board-spec-generator — the agent writes `board-specs/<vendor>/<board>.md`.
3. Add the board to the example FunctionalSpec board checklist (`board-devkitc-c6` is the default; `board-xiao-c6` is the other v1 board). Pins stay in the board spec, not in `main.rs`.
4. Regenerate docs so Supported Boards tables pick the file up. Say: Activate esp32-sdd-documentation-generator and rebuild all documentation.

Changing SoC is a new chip feature, a `rust-toolchain.md` row, and new example specs. It is not a new board file.

Long form, using the ESP32-C6-DevKitC-1-N8 as the walkthrough: [Creating New Board Specs](creating-new-board-specs.md).

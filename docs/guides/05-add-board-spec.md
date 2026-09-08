<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 05 — Add a Board Spec

Board specs are the source of truth for GPIO, power, and USB console. Never hard-code pins in `main.rs` — generated `src/board.rs` reads the board-spec.

## Workflow

```
1. Provide the vendor product page URL or datasheet
2. Say: "Activate esp32-board-spec-generator skill"
        → Creates board-specs/<vendor>/<board-name>.md
3. Reference the new board in the example FunctionalSpec Board Compatibility Checklist
4. For v1 firmware: only ESP32-C6 boards are compiled (`esp32c6` + a board Cargo feature)
5. Regenerate documentation so Supported Boards tables auto-discover the new file
```

v1 **firmware** still filters to ESP32-C6. You may add non-C6 board-specs to the catalog for hardware reference; example READMEs will list them under an exclusion callout until the hardware bar expands.

Full walkthrough: [Creating New Board Specs](creating-new-board-specs.md).

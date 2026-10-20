---
name: Bug Report
about: Report a defect in agent-generated code or documentation
labels: ["bug"]
assignees: ""
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

If the bug is in generated code, the fix is a spec change and a regeneration, not a hand-edit of `src/`.

## Which example?

<!-- e.g. blinky, deep-sleep-timer-node -->

## rustc / esp-hal version

<!-- rustc --version ; pin is esp-hal 1.2.1 (see shared-specs/rust-toolchain.md) -->

## Board

<!-- ESP32-C6-DevKitC-1-N8 or Seeed XIAO ESP32-C6 -->

## Describe the bug

<!-- What went wrong -->

## Steps to reproduce

1.
2.
3.

## Expected behaviour

<!-- What the spec says should happen -->

## Actual behaviour

<!-- What you observed. Firmware generated is not a claim that a board run passed. -->

## `cargo build --release` log

```
paste the log here
```

## Checklist

- [ ] I confirmed this is not a hand-edit of generated files
- [ ] I did not include an ESP-IDF version (this repo does not use ESP-IDF)

---
name: Bug Report
about: Report a defect in agent-generated code or documentation
labels: ["bug"]
assignees: ""
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

## Which example?

<!-- e.g. blinky, deep-sleep-timer-node -->

## rustc / esp-hal version

<!-- rustc --version ; pin is esp-hal 1.2.1 (see shared-specs/rust-toolchain.md) -->

## Board

<!-- e.g. ESP32-C6-DevKitC-1-N8, Seeed XIAO ESP32-C6 -->

## Describe the bug

<!-- A clear and concise description of what the bug is. -->

## Steps to reproduce

1.
2.
3.

## Expected behaviour

<!-- What did you expect to happen? -->

## Actual behaviour

<!-- What actually happened? -->

## `cargo build --release` log

```
<!-- Paste the relevant output from cargo build --release or cargo run --release here -->
```

## Checklist

- [ ] I confirmed that the file exhibiting the bug is agent-generated (has the agent-generated header comment)
- [ ] I have **not** manually edited the generated file before filing this report
- [ ] I have tried with a clean build (`cargo clean` then `cargo build --release`)

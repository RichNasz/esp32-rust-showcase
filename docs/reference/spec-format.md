<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Spec Format

Human-authored example specs live in `examples/<name>/specs/`. They must **not** start with an agent-generated header.

Template: [`templates/FunctionalSpec.template.md`](../../templates/FunctionalSpec.template.md). Layout lock: [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md).

## FunctionalSpec.md

Describes **what** and **why**. Typical sections:

| Section | Purpose |
|---|---|
| Overview | 2–4 sentences: what the example demonstrates |
| Target Hardware | v1 chip ESP32-C6 only; Board Compatibility Checklist |
| Features | Observable behaviour |
| Wakeup Strategy | timer / RTC GPIO 0–7 / LP-core / none |
| Connectivity | none / BLE / Wi-Fi / ESP-NOW / MQTT |
| Security Requirements | none, or AES / esp-storage / TLS / OTA |
| Success Criteria | Must include `cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` |

If the spec includes a heading containing **Limitations**, the generated example README must reproduce it.

Board Compatibility Checklist (v1):

- [x] Espressif ESP32-C6-DevKitC-1-N8 *(default)* — feature `board-devkitc-c6`
- [ ] Seeed XIAO ESP32-C6 — feature `board-xiao-c6`

Do not add non-C6 boards. RTC / LP GPIOs on C6 are **0–7 only**; GPIO 9 (XIAO boot button) is **not** RTC-capable.

## CodingSpec.md

High-level guidance — architecture, constraints, preferred crates, non-functional requirements, and gotchas.

**Never include:** code snippets, `const` item definitions, specific function signatures, API call sequences, exact log strings, or numbered step-by-step implementation instructions.

Philosophy: "The model we use today is the worst model we will ever use." A high-level spec benefits from better future models.

## TestSpec.md

Tiers in priority order:

1. **Automated — native Cargo / rustc**: `cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi
2. **Automated — additional tools**: only with explicit justification and documented setup
3. **Manual — hardware-only**: visual or physical observation that cannot be simulated

Do not specify ESP-IDF test components or CMake-based harnesses.

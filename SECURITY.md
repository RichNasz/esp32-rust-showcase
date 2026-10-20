<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Security policy

## Scope

This project generates firmware source and build configuration for rust-on-metal ESP32-C6. Reports are about that generation (for example, generated code that introduces a common vulnerability).

Chip-level hardware security is out of scope. That includes Secure Boot enrollment, eFuse programming, and flash-encryption enablement on the silicon.

No coordinated disclosure timeline is guaranteed at this stage of the project.

## Reporting

Do not open a public GitHub Issue that describes vulnerability details.

1. Open a GitHub Issue and apply the `security` label.
2. In the issue body, state only: "I have found a security issue and would like to discuss it privately." Do not include vulnerability details.
3. A maintainer will acknowledge the issue within 48 hours and arrange private follow-up.

## In scope

- The generation process and the firmware or configuration it writes.
- A generated crate that mishandles secrets, skips a check the spec required, or exposes a known class of flaw.

## Out of scope

- ESP32 chip errata, ROM boot ROM, and eFuse behavior.
- Vulnerabilities in ESP-IDF itself. This repository does not build with ESP-IDF.
- A missing hardware test result. **Firmware generated** means the crate is in the tree, not that a board run passed.

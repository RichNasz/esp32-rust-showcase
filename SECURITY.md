<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/contributing-spec.md using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Security Policy

## Scope

This project generates rust-on-metal firmware source code and Cargo build configurations for
ESP32-C6. Security reports are relevant to the **generation process** itself — for example, if
a generated file introduces a common vulnerability pattern (unchecked buffer use, weak random
seed, hard-coded credentials). Chip-level hardware security of the ESP32 (eFuse, flash
encryption at the silicon level, secure boot chain) is **out of scope** for this security policy.

## Reporting a Vulnerability

**Do not open a public GitHub Issue that describes vulnerability details.**

Follow these steps instead:

1. Open a GitHub Issue and apply the **`security`** label.
2. In the issue body, state only: _"I have found a security issue and would like to discuss it
   privately."_ Do not include any vulnerability details, proof-of-concept code, or reproduction
   steps in the public issue.
3. A maintainer will **acknowledge the issue within 48 hours** and arrange private follow-up
   (typically via GitHub's private vulnerability reporting or direct message).

## What Is In Scope

- Generated Rust that introduces known vulnerability patterns (e.g. `unwrap()` on fallible
  crypto or radio paths, predictable seeds for cryptographic use, hard-coded default credentials).
- Skill workflows that systematically produce insecure configurations (e.g. disabled TLS
  verification, world-readable storage of long-term keys).
- Spec templates that guide users toward insecure design decisions without warning.

## What Is Out of Scope

- ESP32 chip-level hardware vulnerabilities (report to Espressif directly).
- Vulnerabilities in `esp-hal`, `esp-radio`, or other crates in the `esp-rs` ecosystem
  (report upstream).
- Issues in third-party libraries referenced by generated code.
- Theoretical vulnerabilities with no practical impact on generated output.

## Disclosure Timeline

This project does not guarantee a coordinated disclosure timeline at this stage. The maintainer
will work in good faith to acknowledge, investigate, and address valid reports as quickly as
possible given project resources.

<!-- ================================================
     HUMAN-WRITTEN — SINGLE SOURCE OF TRUTH — PERMANENT
     ================================================ -->

# Example Project Structure Specification (PERMANENT — NEVER CHANGE)

This file defines the official, immutable structure for every example in the repository.

## The Locked Structure

```
examples/<kebab-name>/
├── specs/                          ← human-only
│   ├── FunctionalSpec.md
│   ├── CodingSpec.md
│   └── TestSpec.md
├── src/                            ← 100% agent-generated
│   └── main.rs                     ← #![no_std] #![no_main]; extra modules as needed
├── Cargo.toml
├── build.rs
├── .cargo/config.toml              ← target = riscv32imac-unknown-none-elf, runner = espflash
├── README.md
├── .gitignore
└── target/                         ← gitignored
```

Examples are **standalone Cargo packages**, not members of a repository-level workspace. There is no root `Cargo.toml` workspace, no `sdkconfig`, no `main.c`, and no `CMakeLists.txt`.

Board selection is via mutually exclusive Cargo features: default `board-devkitc-c6`, optional `board-xiao-c6`. Chip feature `esp32c6` only in v1. Generated `src/board.rs` holds GPIO/LED/button constants derived from `board-specs/` — never hardcode pins in `main.rs`.

`src/main.rs` is always present and is `#![no_std]` `#![no_main]`. Additional modules (`src/board.rs`, `src/<subsystem>.rs`) are generated as needed.

## Allowed extras

The tree above is the locked layout. These paths are allowed only when the FunctionalSpec requires them:

- `src/board.rs` and other `src/<subsystem>.rs` modules
- `partitions.csv` and `espflash.toml` for a custom or OTA partition table (`shared-specs/partition-table-profiles.md`)
- `certs/` for an HTTPS OTA CA PEM. No private key in that directory
- nested `lp-core/` when the spec requires LP-core firmware
- `Cargo.lock`

Generated pages that draw the locked tree copy it as written. Pages that teach the layout also list these extras and say they appear only when the spec requires them.

## CodingSpec.md — Abstraction Level

CodingSpec.md is high-level technical guidance for the AI model generating the project. It must describe **what** the system should do and **why** — not **how** to implement it.

**Include in CodingSpec.md:**
- Architecture and module breakdown
- Key timing, memory, power, and security constraints
- Preferred crates and APIs (and the reason for each choice)
- Non-functional requirements (error recovery philosophy, logging expectations, concurrency)
- Trade-off decisions (e.g., QoS 0 vs 1, polling vs interrupt-driven, heap vs static buffers)
- Known gotchas specific to the hardware, `esp-hal`, or the pinned crate versions

**Never include in CodingSpec.md:**
- Code snippets, `const` item definitions, or pseudo-code
- Specific function signatures or API call sequences
- Exact log message strings
- Numbered step-by-step implementation instructions

**Philosophy:** "The model we use today is the worst model we will ever use." A high-level spec will benefit from better future models. A low-level spec just replicates code in prose.

## TestSpec.md — Testing Philosophy

TestSpec.md is the third human-written spec file. It defines testing requirements for the example.

**Philosophy (enforced in every TestSpec.md):**
> Automated testing using native Cargo / rustc tools is strongly preferred. Additional tools are only allowed with explicit justification and documented benefits. Manual testing is used only when no practical automated alternative exists.

**Tiers (in priority order):**
1. **Automated — native Cargo / rustc**: `cargo build --release` (zero `rustc` warnings on the C6 target), `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi. No extra tooling required beyond rustup + the RISC-V target + espflash (or Wokwi).
2. **Automated — additional tools**: Wokwi CI integration, host-side tests that cannot run on the device. Allowed only with explicit justification and documented setup steps in TestSpec.md.
3. **Manual — hardware-only**: Visual or physical observation that cannot be simulated (LED polarity, haptics, RF behaviour). Required steps must be numbered and beginner-friendly.

Do **not** specify `idf.py`, ESP-IDF Unity test components, or any CMake-based test harness. Those tools are not part of this repository.

This structure is permanent and must be strictly enforced by the full-project-generator skill in every example, now and forever.

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 04 — Create a New Example

This is task B. Read [Using With AI Agents](using-with-ai-agents.md) first if you have not used specs as the source of truth.

To set up a new example, you write three spec files. A **spec** is Markdown a person writes. **Spec-driven development (SDD)** means those files are the source of truth and the agent writes the Rust. **FunctionalSpec** is observable behavior. **CodingSpec** is architecture and constraints with no code. **TestSpec** is how to check the example. They live in `examples/<name>/specs/`. A new example starts with a row in `specs/showcase-master-spec.md`, then those three files. Section list: [`templates/FunctionalSpec.template.md`](../../templates/FunctionalSpec.template.md). Directory lock: [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md).

To modify an example, edit those files. Behavior goes in FunctionalSpec, architecture in CodingSpec, and checks in TestSpec. Do not hand-edit `src/` or the example README. **Generate** means the agent writes the crate or the Markdown from the specs. **Regenerate** means generate again: previous agent output is overwritten and `specs/` stays. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. The phrases below are said to the agent, not run in the shell.

## Workflow

1. Edit `specs/showcase-master-spec.md` — the catalog row is how the repository lists the example before a crate exists.
2. Say to the agent: Activate esp32-sdd-functional-spec-creator — the agent drafts `FunctionalSpec.md` with you.
3. Review `FunctionalSpec.md` — you check observable behavior before any Rust is written.
4. Add `CodingSpec.md` — architecture and constraints, with no code, so the generator is not locked to one implementation.
5. Add `TestSpec.md` — the checks the example will be built and observed against.
6. Say: Activate esp32-sdd-full-project-generator — the agent writes the crate and leaves `specs/` in place.
7. From `examples/<name>/`: `cargo run --release` — this builds and flashes the crate the agent just wrote.
8. Say: Activate esp32-sdd-documentation-generator and rebuild all documentation. — the agent rewrites Markdown so the docs match the spec.

Use a domain skill only when the spec needs it (WS2812, BLE, deep sleep, hardware AES, or LP-core). ESP-NOW unicast, raw IEEE 802.15.4, and an HTTPS CRC slot flip are steps inside `esp32-sdd-full-project-generator`, not separate skills. Do not send CRC HTTPS OTA to `esp32-hardware-crypto-configurer`. Phrases are in [skill invocation](../reference/skill-invocation.md).

## Locked layout

Every example must match [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md):

```
examples/<kebab-name>/
├── specs/                          ← human-written
│   ├── FunctionalSpec.md           ← human-written
│   ├── CodingSpec.md               ← human-written
│   └── TestSpec.md                 ← human-written
├── src/                            ← agent-written
│   └── main.rs                     ← agent-written (#![no_std] #![no_main]; extra modules as needed)
├── Cargo.toml                      ← agent-written
├── build.rs                        ← agent-written
├── .cargo/config.toml              ← agent-written (target = riscv32imac-unknown-none-elf, runner = espflash)
├── README.md                       ← agent-written
├── .gitignore                      ← agent-written
└── target/                         ← gitignored
```

Examples are standalone Cargo packages. There is no root workspace, no `sdkconfig`, and no ESP-IDF project files. `src/board.rs` is generated when the example needs board pins. Never hardcode pins in `main.rs`.

These paths are allowed only when the FunctionalSpec requires them:

- `src/board.rs` and other `src/<subsystem>.rs` modules
- `partitions.csv` and `espflash.toml` for a custom or OTA partition table
- `certs/` for an HTTPS OTA CA PEM. No private key in that directory
- nested `lp-core/` when the spec requires LP-core firmware
- `Cargo.lock`

## Hardware bar

**Chip ≠ board:** Cargo feature `esp32c6` is the SoC; `board-devkitc-c6` (default) and `board-xiao-c6` are boards. rust-on-metal is not `idf.py set-target`. v1 is ESP32-C6 only. RTC / LP GPIOs on the C6 are 0–7 only.

Next: [05 — Add a Board Spec](05-add-board-spec.md).

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 04 — Create a New Example

Use this when you want to add a crate to the catalog. Humans write specs only.

## Workflow

```
1. Edit specs/showcase-master-spec.md   (add your example to the catalog)
2. Say: "Activate esp32-sdd-functional-spec-creator skill"
        → Creates examples/<name>/specs/FunctionalSpec.md
3. Review and approve FunctionalSpec.md
4. Add examples/<name>/specs/CodingSpec.md   (architecture, constraints, crates, gotchas — never code)
5. Add examples/<name>/specs/TestSpec.md     (optional: automated + hardware-only tests)
6. Say: "Activate esp32-sdd-full-project-generator skill"
        → Generates the complete no_std Cargo crate
7. From examples/<name>/: cargo run --release
8. Say: "Activate esp32-sdd-documentation-generator and rebuild all documentation."
```

## Locked layout

Every example **must** match [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md):

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

Examples are standalone Cargo packages. There is no root workspace, no `sdkconfig`, and no ESP-IDF project files.

## Hardware bar

v1 is ESP32-C6 only. Default board: ESP32-C6-DevKitC-1-N8 (`board-devkitc-c6`). Optional: XIAO ESP32-C6 (`board-xiao-c6`). RTC / LP GPIOs on C6 are **0–7 only**.

Next: [05 — Add a Board Spec](05-add-board-spec.md).

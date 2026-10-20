<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# SDD Workflows

The complete Spec-Driven Development workflows used in this project. Every workflow starts with a human-written spec and ends with agent-generated, buildable output — no manual firmware authoring.

## Core Principle

```
Human writes specs  →  AI reads specs  →  AI generates everything else
```

Humans are responsible for **what** and **why**. AI is responsible for **how**.

---

## Workflow 1 — Generate a New Example

Use this when you want to add a new ESP32-C6 example to the catalog.

```
1. Edit specs/showcase-master-spec.md   (add your example to the catalog)
2. Say: "Activate esp32-sdd-functional-spec-creator skill"
        → Creates examples/<name>/specs/FunctionalSpec.md
3. Review and approve the FunctionalSpec.md
4. Add examples/<name>/specs/CodingSpec.md   (high-level technical guidance — architecture,
                                               constraints, preferred crates, gotchas)
5. Add examples/<name>/specs/TestSpec.md     (optional: testing requirements — automated and
                                               manual test cases for the README Testing section)
6. Say: "Activate esp32-sdd-full-project-generator skill"
        → Generates the complete no_std esp-hal Cargo crate
7. cargo run --release
```

---

## Workflow 2 — Add a Board Spec

Use this when adding support for a new development board or ESP32 module.

```
1. Provide the vendor product page URL or datasheet
2. Say: "Activate esp32-board-spec-generator skill"
        → Creates board-specs/<vendor>/<board-name>.md
3. Reference the new board spec in your example's FunctionalSpec.md
   (v1 firmware still filters to ESP32-C6)
```

---

## Workflow 3 — Regenerate Documentation

Use this after any change to `specs/` or any `shared-specs/` file.

```
1. Edit the relevant file in specs/ or shared-specs/
2. Say: "Activate esp32-sdd-documentation-generator and rebuild all documentation."
        → Regenerates README.md, docs/, skills/README.md, example READMEs, etc.
3. Commit: specs/ changes + all regenerated files in the same commit
```

Never generate anything inside `.github/workflows/`.

---

## Workflow 4 — Update an Existing Example

Use this when a FunctionalSpec or CodingSpec changes and the generated crate needs to be rebuilt.

```
1. Edit examples/<name>/specs/FunctionalSpec.md or CodingSpec.md
2. Say: "Activate esp32-sdd-full-project-generator skill"
        → Overwrites all generated files in examples/<name>/
        → Preserves specs/ unchanged
3. Review the diff, then cargo run --release
```

---

## Workflow 5 — Validate an Example

Use this after generating or modifying an example to confirm it meets every structural and spec-compliance requirement.

```
1. Say: "Activate esp32-sdd-project-validator skill"
        → Checks locked crate layout, agent headers, Cargo board-feature guards,
          board-spec pin accuracy, README quality, and overall build readiness
        → Hard FAIL if main.c, CMakeLists.txt, or sdkconfig.defaults appear
        → Reports PASS/FAIL for each check
2. Fix any FAIL items by updating the relevant spec and regenerating
3. Rerun the validator until all checks pass
```

---

## Skill Dependency Map

```
esp32-sdd-functional-spec-creator
  │
  └─► esp32-sdd-full-project-generator
        │
        ├─► esp32-deep-sleep-engineer        (auto-invoked if deep sleep required)
        ├─► esp32-ble-beacon-engineer        (auto-invoked if BLE advertising required)
        ├─► esp32-ws2812-led-engineer        (auto-invoked if WS2812 RGB LED required)
        ├─► esp32-hardware-crypto-configurer (auto-invoked if security required)
        └─► esp32-lp-core-engineer           (auto-invoked if LP-core firmware / wakeup required)

esp32-board-spec-generator                   (independent, run on demand)

esp32-sdd-documentation-generator            (independent, run after spec changes)

esp32-sdd-project-validator                  (independent, run after generation to verify correctness)
```

---

## Git Commit Convention

Every commit should be one of:

| Type | Contents |
|---|---|
| `spec:` | Only changes to `specs/` or `shared-specs/` (human-authored) |
| `gen:` | Only changes to agent-generated files (triggered by a spec change) |
| `spec+gen:` | Both spec and generated files in one atomic update |

For `gen:` and `spec+gen:` commits, append `Generated-by: <invoking agent name>`.

Never commit generated files without the driving spec change in the same or prior commit.

---

## Why SDD?

- **Reproducibility** — specs are the single source of truth; regeneration produces consistent output
- **Reviewability** — PRs contain spec changes that are human-readable and auditable
- **Scalability** — adding a new example or board is a spec edit + skill invocation, not a coding session
- **Quality** — every generated file is consistent with every other; no drift between docs and code

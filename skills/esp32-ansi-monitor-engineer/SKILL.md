---
name: esp32-ansi-monitor-engineer
description: Generates ANSI TUI dashboard code for ESP32-C6 UART serial output via esp-println. Supports Pattern A (continuous dashboard with DECSTBM scroll region), Pattern B (per-cycle snapshot for duty-cycled / deep-sleep firmware), and Pattern B-Lifecycle (sequential pipeline with lifecycle-driven redraws). Not graphical. Emits escape definitions, drawing functions, refresh wiring, and log suppression.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, ansi, tui, terminal, dashboard, monitor, serial, esp-println, rust, uart
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 ANSI Monitor Engineer

## When to use

Any **ESP32-C6 rust-on-metal** example where the UART serial terminal output — viewed via
`cargo run --release` / `espflash flash --monitor` — should be a structured terminal dashboard
rather than plain `log::info!` lines. Invoke standalone to add a dashboard to an existing
example, or as a sub-workflow of `esp32-sdd-full-project-generator` when the CodingSpec
references this skill.

### Scope

This skill is intentionally narrow. It exists solely to improve the **UART serial monitor
experience** for developers using `espflash` monitor or a compatible serial terminal. Every
design decision — escape sequence selection, `esp-println` discipline, log suppression timing,
pattern selection — is made in the context of `no_std` firmware on ESP32-C6 outputting to a
UART-backed terminal.

**This skill does NOT cover and must not drift into:**
- Graphical displays: OLED, LCD, TFT, e-ink
- Network or cloud monitoring: MQTT dashboards, HTTP status pages, Grafana, etc.
- Bluetooth or BLE-based monitoring UIs
- Web serial or browser-based terminal interfaces
- General-purpose embedded TUI frameworks
- Production telemetry or logging pipelines (this is a developer-facing debug interface only)

If a requirement falls outside this scope, it belongs in a different skill or module.

## Pattern Selection

Choose one pattern based on the firmware's execution model:

**Pattern A — Continuous Dashboard (scroll region)**
- Firmware runs indefinitely with a real-time incoming event stream (ESP-NOW gateway, Wi-Fi
  server, MQTT broker).
- New events must appear as scrolling lines below a pinned header.
- Use DECSTBM scroll region (`ESC[top;botr`) to pin header rows; new event lines scroll below.

**Pattern B — Per-Cycle Snapshot (cursor-home + overwrite, timer-driven)**
- Firmware is duty-cycled: discrete active windows separated by deep sleep or idle periods.
- Each wake cycle produces a fixed set of data fields; screen freezes during sleep.
- A periodic `embassy_time` ticker (default 250 ms) drives redraws within each active window.

**Pattern B-Lifecycle — Sequential Pipeline (cursor-home + overwrite, lifecycle-driven)**
- Firmware runs continuously but progresses through discrete sequential phases
  (e.g. `BOOTING → CONNECTING → READY → DOWNLOADING → COMPLETE`).
- No continuous event stream; no deep sleep. Phase transitions naturally drive redraws.
- No periodic ticker — each phase calls `draw_dashboard(false)` at appropriate points.
- For blocking operations within a phase, use polling-loop substitution or threshold-based redraws.

**Decision rule:**

| Firmware model | Pattern |
|---|---|
| Always-on, real-time event stream (events arrive continuously) | **A** |
| Duty-cycled with deep sleep between active windows | **B** |
| Always-on, sequential discrete phases, fixed field set, no streaming events | **B-Lifecycle** |

## Workflow (follow exactly)

MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. **Read the CodingSpec** for the target example. Confirm which pattern is specified. If the
   CodingSpec specifies a dashboard layout (fields, rows, refresh rate), use it exactly. If it
   does not, choose the pattern from the decision rule above and document the choice.

2. **Emit the ANSI escape block** near the top of the dashboard module. Use these definitions
   verbatim — do not abbreviate or reorder:

   ```rust
   const ESC: &str = "\x1b";
   const ANSI_HOME: &str = "\x1b[H";
   const ANSI_CLEAR: &str = "\x1b[2J\x1b[H";
   const ANSI_ERASE_EOL: &str = "\x1b[K";
   const ANSI_CURSOR_OFF: &str = "\x1b[?25l";
   // Pattern A only — DEC cursor save/restore (NOT SCO \x1b[s / \x1b[u; macOS Terminal.app
   // does not support SCO).
   const ANSI_SAVE: &str = "\x1b7";
   const ANSI_RESTORE: &str = "\x1b8";
   // Pattern A only — DECSTBM. Adjust the top row to match the header height.
   const ANSI_SCROLL_LOCK: &str = "\x1b[6;500r";
   const C_RESET: &str = "\x1b[0m";
   const C_BOLD: &str = "\x1b[1m";
   const C_DIM: &str = "\x1b[2m";
   const C_GREEN: &str = "\x1b[32m";
   const C_YELLOW: &str = "\x1b[33m";
   const C_RED: &str = "\x1b[31m";
   const C_CYAN: &str = "\x1b[36m";
   const C_GRAY: &str = "\x1b[90m";
   const C_WHITE: &str = "\x1b[97m";
   const C_BGBLU: &str = "\x1b[44m";
   ```

3. **Module-scope dashboard state** — values written by tasks/callbacks, read only by the
   drawing function. Typical fields: MAC buffer, status enum, cycle start `Instant`, error
   reason buffer. Prefer `embassy_sync` atomics / mutexes over FreeRTOS primitives.

4. **Implement the drawing function(s)** according to the chosen pattern. Adhere strictly to
   the output discipline in step 6.

5. **Wire up the refresh mechanism:**

   **Pattern A and Pattern B (timer-driven):**
   - Use `embassy_time::Ticker` (or `Timer::after`) at the CodingSpec interval (default 250 ms).
   - The ticker task (or the screen-owner task) is the only writer to the terminal.
   - Other tasks only update state and signal (Embassy channel / signal / atomic flag).

   **Pattern B-Lifecycle (lifecycle-driven):**
   - No ticker. Each phase transition calls `draw_dashboard(false)` directly.
   - For blocking waits, split into short `Timer` / `select` iterations and redraw on timeout
     (polling-loop substitution), or gate redraws on a progress threshold.

6. **Mandatory output discipline** (add as comments where the rules apply):

   - All post-init terminal output via `esp-println` (or a `core::fmt::Write` wrapper over it).
     Do not use `std::print`.
   - Format into a stack buffer first (`heapless::String<256>` or `[u8; 256]` + `write!`).
     ANSI escapes add invisible bytes; 128 bytes overflows when a row has several colour resets.
   - After the first frame, suppress the `log` crate (`log::set_max_level(log::LevelFilter::Off)`).
     Suppression is permanent for the rest of the wake cycle. Errors go in a dashboard `Status`
     field, not `log::error!`.
   - Only the task that owns the screen writes to the terminal. Other tasks set flags / buffers
     *before* signalling (the signal is the memory barrier).
   - Event handlers and ISRs after the first draw must not call `log::*` or `esp_println`.

7. **Dependencies:** `esp-println` with `log-04` (already in the crate template). Add `heapless`
   if the CodingSpec does not already justify a heap for formatting. Do not add CMake REQUIRES.

8. Verify by inspecting `cargo run --release` / `espflash` monitor. Confirm:
   - All rows appear on first boot without corruption.
   - Colours and bold/dim styling render in macOS Terminal.app.
   - Refresh rate matches the CodingSpec (no perceptible flicker).
   - Radio / HAL `log::*` lines do not appear after log suppression.

---

## Pattern A Workflow — Continuous Dashboard (scroll region)

Use for always-on firmware with a continuous incoming event stream.

### draw_header() — called once after peripheral init

```
ANSI_CLEAR                     — clear screen
Draw rows 1..N as fixed header:
  Row 1: title bar (C_BGBLU C_BOLD C_WHITE)
  Row 2: status row (left empty; draw_status() populates it)
  Row 3: separator line (C_GRAY dashes)
  Row 4: column header labels (C_DIM)
  Row 5: separator line (C_GRAY dashes)
ANSI_SCROLL_LOCK               — pin rows 1-5; scroll region = row 6+
Position cursor at row 6
ANSI_CURSOR_OFF
draw_status()
```

After `draw_header()`, call `log::set_max_level(log::LevelFilter::Off)`.

### draw_status() — overwrite the status row without disturbing the scroll cursor

```
ANSI_SAVE
Position at status row (e.g. ESC[2;1H)
Write status content
ANSI_ERASE_EOL
ANSI_RESTORE
```

Overwrite content *before* ERASE_EOL to avoid a visible blank flash at 115200 baud.

### print_event_line() — per incoming event

Pre-format the entire line into a stack buffer, then `esp_println` with `\r\n`.
The scroll region keeps the header fixed.

### Screen-owner task (Pattern A)

Wait on an Embassy signal / channel. If status changed: `draw_status()`. If new event:
`print_event_line()`.

---

## Pattern B Workflow — Per-Cycle Snapshot (cursor-home + overwrite)

Use for duty-cycled firmware (deep sleep between active windows).

### draw_dashboard(initial: bool)

On `initial = true` (first call only): `ANSI_CLEAR`, `ANSI_CURSOR_OFF`, then
`log::set_max_level(log::LevelFilter::Off)`.

On every call: `ANSI_HOME` (not `ANSI_CLEAR` — avoids flicker). For each row: write content,
then `ANSI_ERASE_EOL`. Never blank the line first.

### Progress bar (time-based, duty-cycled windows)

`elapsed / window` clamped to `[0, 1]`, fill width from CodingSpec (default 20). Fill
character `=` unless the spec says otherwise. Use `embassy_time::Instant`.

### Active-window loop

Replace a single blocking wait with a loop that redraws on each ticker tick until the window
completes (advertising done flag, etc.). Draw a final SLEEPING frame before deep sleep.
USB-native boards drop the monitor on sleep — that is expected (`AIGenLessonsLearned.md`).

---

## Pattern B-Lifecycle Workflow — Sequential Pipeline

Use for always-on firmware with discrete sequential phases and a fixed field set. No deep
sleep. No continuous event stream. No periodic ticker.

`draw_dashboard` is the same as Pattern B; *when* it is called differs.

### Sub-pattern 1: Polling-loop substitution

Replace one long wait with short `Timer` / `select` iterations. On timeout, update elapsed
time and redraw. The event that completes the wait only sets state + signals — it does not
draw.

### Sub-pattern 2: Threshold-based redraws

Inside a tight inner loop (download, decode), gate `draw_dashboard(false)` on a minimum
progress increment (default: 1 KiB) so the UART is not saturated.

### Progress bar (byte-count-based)

If total size is known, show `[====    ]  nn%  received / total`. If unknown, show bytes
received only.

Populate state that the first frame must display *before* `draw_dashboard(true)`.

---

## Terminal Compatibility Notes

- **macOS Terminal.app**: ANSI colours, DECTCEM, DECSTBM, DEC save/restore (`\x1b7`/`\x1b8`).
  Does NOT support SCO save/restore (`ESC[s`/`ESC[u`).
- **iTerm2**: all of the above plus SCO. Always prefer DEC over SCO.
- **`espflash` monitor / `cargo run --release`**: passes ANSI sequences through to the host
  terminal. This skill targets that path as the reference terminal.

# Testing Specification — Raw IEEE 802.15.4 Frames (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

On-air frame delivery needs a second C6 board. That check stays manual.

---

## Automated Tests

### T-A1 — Zero-warning build (DevKitC-C6 node, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: Default features (`board-devkitc-c6`, `role-node`) compile cleanly
with crates.io `esp-radio` `=1.0.0-beta.1` (IEEE 802.15.4) and placeholder channel / PAN.

```sh
cd examples/ieee802154-raw-frames
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (DevKitC-C6 peer)

**Tool**: `cargo` (native)
**What it verifies**: The receive role compiles. `--no-default-features` is required so
`role-node` is not also enabled.

```sh
cd examples/ieee802154-raw-frames
cargo build --release --no-default-features --features board-devkitc-c6,role-peer
cargo clippy --release --no-default-features --features board-devkitc-c6,role-peer -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Zero-warning builds (XIAO ESP32-C6, both roles)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED path compiles for each role.

```sh
cd examples/ieee802154-raw-frames
cargo build --release --no-default-features --features board-xiao-c6,role-node
cargo clippy --release --no-default-features --features board-xiao-c6,role-node -- -D warnings
cargo build --release --no-default-features --features board-xiao-c6,role-peer
cargo clippy --release --no-default-features --features board-xiao-c6,role-peer -- -D warnings
```

Pass criterion: all four commands exit 0 with no warnings.

---

### T-A4 — Role features are mutually exclusive

**Tool**: `cargo` (native)
**What it verifies**: Both roles together, and a board with no role, fail at compile time.

```sh
cd examples/ieee802154-raw-frames
cargo build --release --no-default-features --features board-devkitc-c6,role-node,role-peer
cargo build --release --no-default-features --features board-devkitc-c6
```

Pass criterion: both commands fail to compile. The default build (T-A1) still succeeds.

---

## Manual Tests

### T-M1 — Two-board exchange

**Why manual**: ACK and echo require a second radio. No host tool speaks 802.15.4 to the board.

1. Flash one C6 board as the node (`cargo run --release` on DevKitC, or the XIAO node feature set).
2. Flash a second C6 board as the peer (DevKitC or XIAO peer feature set). Both images
   must use the same `IEEE802154_CHANNEL` and `IEEE802154_PAN_ID`.
3. Watch both monitors for about 20 seconds.

Pass criterion: the node sequence increases by one each period. The peer logs that
sequence. The node shows the success LED pattern when an ACK arrives. The peer blinks
on each accepted frame. The node log may also show an echoed payload. Neither side panics.

### T-M2 — Node alone

**Why manual**: The missing-ACK path is a live radio timeout.

1. Flash only the node. Leave the peer unpowered.
2. Watch at least three periods.

Pass criterion: each period logs a missing ACK, shows the failure indication, and the
firmware keeps looping.

### T-M3 — Channel mismatch

**Why manual**: The radios drop the frame with no error code on the sender.

1. Flash the peer on channel 15 and the node on a different channel in 11 through 26.
2. Watch one node period.

Pass criterion: the node shows the failure indication. The peer does not log the magic.
Putting both back on the same channel restores T-M1.

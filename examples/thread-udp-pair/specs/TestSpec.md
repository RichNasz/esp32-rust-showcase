# Testing Specification — Thread UDP Pair (ESP32-C6)

Automated host builds are the required check. Two-board Thread traffic cannot be
judged from the host.

## Automated — native Cargo / rustc

From `examples/thread-udp-pair/`:

1. `cargo build --release` for the default child role (DevKitC). Zero `rustc`
   warnings. This role uses the prebuilt OpenThread archive and does not need
   CMake.
2. `cargo clippy -- -D warnings` for that same feature set.
3. `cargo build --release --no-default-features --features board-xiao-c6,role-child`
   and the matching clippy invocation.
4. `cargo build --release --no-default-features --features board-devkitc-c6,role-leader`
   and the matching clippy invocation, on a host with Clang, CMake, and Ninja.
   Skip this step when those tools are absent, and say so. Do not treat a skipped
   leader build as a pass of the leader role.

`cargo run --release` is the on-device command. A host build does not flash.

## Manual — hardware required

1. **T-T1 — Two boards, one dataset.** Why automation is impractical: Thread
   attach and UDP delivery need a second 802.15.4 radio. Flash the leader with
   `role-leader` and the child with `role-child`. Both use the placeholder
   dataset unless you override `THREAD_DATASET` for both.
2. Confirm the leader log reaches role leader.
3. Confirm the child log reaches role child and then logs a send.
4. Confirm the leader logs the child payload and the leader LED flashes three
   times (green on DevKitC).
5. Confirm the child LED flashes three times after a send (blue on DevKitC).
6. Power-cycle either board. RAM settings are discarded. The board rejoins from
   the compiled dataset. There is no border router in this test.

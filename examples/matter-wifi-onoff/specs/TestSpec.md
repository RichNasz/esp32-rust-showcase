# Testing Specification — Matter Wi-Fi On/Off (ESP32-C6)

Automated host builds are the required check. Commissioning needs a Matter
controller and a Wi-Fi network, which the host build does not provide.

## Automated — native Cargo / rustc

From `examples/matter-wifi-onoff/`:

1. `cargo build --release` for the default DevKitC features. Zero `rustc`
   warnings.
2. `cargo clippy -- -D warnings` for that feature set.
3. `cargo build --release --no-default-features --features board-xiao-c6` and
   the matching clippy invocation.
4. After a successful link, record the application size. If it exceeds 1 MB,
   the crate must ship a custom `partitions.csv` that fits in 4 MB, and the
   runner must pass it to espflash.

`cargo run --release` is the on-device command. A host build does not flash
and does not commission.

## Manual — hardware required

1. **T-M1 — Commissioner.** Why automation is impractical: commissioning needs
   a Matter controller (Home Assistant or a phone) and a Wi-Fi access point.
   Flash with `cargo run --release`. Read the manual pairing code from the log.
2. Commission the board. Remove any previous instance of this test device
   first, because the RAM store does not survive reset.
3. Turn the light on from the controller. The status LED lights (white on
   DevKitC, active-LOW on XIAO).
4. Turn the light off. The LED goes dark.
5. Reset the board. The log offers the pairing code again, and the controller
   must commission it as a new device.

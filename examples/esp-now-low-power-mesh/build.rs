// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker / esp-hal build-time config. Do not shell out to CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ESPNOW_CHANNEL");
    println!("cargo:rerun-if-env-changed=ESPNOW_GATEWAY_MAC");
    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

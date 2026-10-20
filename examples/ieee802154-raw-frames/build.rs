// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker / esp-hal build-time config. Do not shell out to CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=IEEE802154_CHANNEL");
    println!("cargo:rerun-if-env-changed=IEEE802154_PAN_ID");
    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker script for a crate that does not build LP-core firmware.
//! Emit linkall.x last. Do not shell out to CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

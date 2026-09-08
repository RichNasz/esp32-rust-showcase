// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker / esp-hal build-time config. Embed an LP-core image here when the
//! FunctionalSpec requires lp-core firmware. Do not shell out to CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker script only. This file does not shell out to CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

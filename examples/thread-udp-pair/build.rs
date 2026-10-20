// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Linker script only. OpenThread's C build, when the leader enables `ftd`,
//! is `openthread-sys`'s build script. This file does not invoke CMake.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=THREAD_DATASET");
    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

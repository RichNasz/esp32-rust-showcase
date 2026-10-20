// AGENT-GENERATED — DO NOT EDIT BY HAND
//! LP memory map. `esp-lp-hal` publishes `link.x` on the link search path.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Last, so the LP script supplies the 0x5000_0000 memory map.
    println!("cargo:rustc-link-arg=-Tlink.x");
}

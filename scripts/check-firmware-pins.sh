#!/usr/bin/env bash
# Check every example image for the application descriptor and the exact
# crates.io pins in shared-specs/rust-toolchain.md.
# The nested LP-core image (examples/lp-core-wakeup/lp-core/) is not an HP app
# and is outside examples/*/src and examples/*/Cargo.toml.
set -euo pipefail

cd "$(dirname "$0")/.."

fail=0
say() {
    printf 'check-firmware-pins: %s\n' "$1" >&2
    fail=1
}

# $1 file, $2 crate name, $3 version without the leading '='
require_dep() {
    local file=$1 crate=$2 version=$3
    local line
    line=$(grep -E "^${crate}[[:space:]]*=" "$file" || true)
    if [[ -z "$line" ]]; then
        say "$file is missing ${crate} =${version}"
        return
    fi
    if [[ "$line" != *"version = \"=${version}\""* && "$line" != *"version=\"=${version}\""* ]]; then
        say "$file pins ${crate} incorrectly: ${line}"
    fi
}

optional_dep() {
    local file=$1 crate=$2 version=$3
    local line
    line=$(grep -E "^${crate}[[:space:]]*=" "$file" || true)
    if [[ -z "$line" ]]; then
        return
    fi
    if [[ "$line" != *"version = \"=${version}\""* && "$line" != *"version=\"=${version}\""* ]]; then
        say "$file pins ${crate} incorrectly: ${line}"
    fi
}

for main in examples/*/src/main.rs; do
    if ! grep -q 'esp_bootloader_esp_idf::esp_app_desc!' "$main"; then
        say "$main is missing esp_app_desc!"
    fi
done

for cargo in examples/*/Cargo.toml; do
    require_dep "$cargo" esp-hal 1.2.1
    require_dep "$cargo" esp-rtos 0.4.0
    require_dep "$cargo" esp-bootloader-esp-idf 0.6.0
    optional_dep "$cargo" esp-radio 1.0.0-beta.1
    optional_dep "$cargo" esp-alloc 0.11.0
    optional_dep "$cargo" esp-storage 0.10.0
done

if [[ "$fail" -ne 0 ]]; then
    exit 1
fi

echo "check-firmware-pins: ok"

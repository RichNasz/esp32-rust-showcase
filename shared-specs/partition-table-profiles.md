# Partition Table Profiles

**Classification:** Human-authored shared spec
**Applies to:** All examples that customize the partition table beyond the espflash default

There is no `sdkconfig` and no `CONFIG_PARTITION_TABLE_*`. Custom tables are IDF-format **CSV** files flashed by **espflash**.

---

## How to Use

Reference a profile from this file in the example's `FunctionalSpec.md`:

```markdown
## Partition Table
Profile: `dual-ota` from shared-specs/partition-table-profiles.md
```

The full-project-generator will emit the corresponding `partitions.csv` next to `Cargo.toml` and point **espflash** at it. Required mechanism (pick one; both may be used together):

1. Runner in `.cargo/config.toml`:

   ```toml
   [target.riscv32imac-unknown-none-elf]
   runner = "espflash flash --monitor --partition-table partitions.csv"
   ```

2. Project `espflash.toml`:

   ```toml
   [idf]
   partition_table = "partitions.csv"
   ```

CSV is the source of truth. `espflash flash --partition-table` accepts the CSV (or a binary produced by `espflash partition-table --to-binary`). Do **not** set partition-table Kconfig symbols. Do **not** generate sdkconfig.

If none of the standard profiles fit, define a custom `partitions.csv` in the FunctionalSpec.md and mark it clearly with the rationale.

Flash size is an espflash setting (`--flash-size` or `espflash.toml` `[flash] size = "8MB"`), not `CONFIG_ESPTOOLPY_FLASHSIZE`.

---

## Profile 1: `default-single-app` — espflash bundled default

Use when no customization is required. Do not emit `partitions.csv`. espflash flashes its bundled single-app table.

Layout (4 MB flash, typical):

| Name | Type | SubType | Offset | Size |
|---|---|---|---|---|
| nvs | data | nvs | 0x9000 | 0x6000 |
| phy_init | data | phy | 0xF000 | 0x1000 |
| factory | app | factory | 0x10000 | 1 MB |

**When to use:** Prototypes, demos, and examples that do not require OTA or an extra data partition. The `nvs` and `phy_init` rows are the bootloader's partitions. Application code does not open an NVS namespace.

---

## Profile 2: `dual-ota` — Dual OTA Partitions

OTA layout used by `secure-ota-https`. Required when an example writes an inactive app slot.

```csv
# Name,   Type, SubType,  Offset,   Size,   Flags
nvs,      data, nvs,      0x9000,   0x6000,
otadata,  data, ota,      0xF000,   0x2000,
phy_init, data, phy,      0x11000,  0x1000,
ota_0,    app,  ota_0,    0x20000,  1536K,
ota_1,    app,  ota_1,    0x1A0000, 1536K,
```

- Requires 4 MB flash minimum. The table ends at `0x320000`.
- `otadata` tracks the active OTA slot (IDF-compatible bootloader via `esp-bootloader-esp-idf`).
- Both `ota_0` and `ota_1` are 1.5 MB — size according to the application binary size.
- `nvs` and `phy_init` stay because the bootloader expects them. This profile has no application data partition. Do not add an NVS `storage` row. The HTTPS OTA example writes the inactive app slot. It does not use NVS.

**When to use:** `secure-ota-https`. The stock `espflash` bootloader does not auto-rollback.

If neither profile fits, define a custom `partitions.csv` in the FunctionalSpec and say why. `hardware-aes-secure-storage` does that for its storage partition.

---

## Flash Size Notes

| Flash Size | What fits |
|---|---|
| 4 MB (XIAO ESP32-C6) | `default-single-app` and `dual-ota` (the `dual-ota` table ends at `0x320000`) |
| 8 MB (DevKitC-C6-N8) | Both profiles |

Set espflash flash size to the chip (`espflash flash --flash-size 8MB` or `espflash.toml` `[flash] size = "8MB"`) when auto-detect is wrong. DevKitC-C6-N8 is 8 MB. XIAO ESP32-C6 is 4 MB.

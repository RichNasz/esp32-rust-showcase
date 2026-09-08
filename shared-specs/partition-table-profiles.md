# Partition Table Profiles

**Classification:** Human-authored shared spec
**Applies to:** All examples that customize the partition table beyond the espflash default

There is no `sdkconfig` and no `CONFIG_PARTITION_TABLE_*`. Custom tables are IDF-format **CSV** files flashed by **espflash**.

---

## How to Use

Reference a profile from this file in the example's `FunctionalSpec.md`:

```markdown
## Partition Table
Profile: `single-app-nvs` from shared-specs/partition-table-profiles.md
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

**When to use:** Prototypes, demos, and examples that do not require OTA or custom NVS sizing.

---

## Profile 2: `single-app-nvs` — Single App + Expanded NVS

Use when an example stores a meaningful amount of data in NVS (credentials, calibration data,
event logs) but does not need OTA.

```csv
# Name,   Type, SubType,  Offset,   Size,   Flags
nvs,      data, nvs,      0x9000,   0xF000,
phy_init, data, phy,      0x18000,  0x1000,
factory,  app,  factory,  0x20000,  1M,
storage,  data, nvs,      0x120000, 0x60000,
```

- `nvs` (60 KB): system NVS (Wi-Fi credentials, netif state).
- `storage` (384 KB): application NVS namespace for user data.
- Remaining flash: unused (available for future factory reset to expand app).

**When to use:** Connected examples with significant NVS data, no OTA requirement.

---

## Profile 3: `dual-ota` — Dual OTA Partitions

Standard OTA layout for production-targeted examples. Required whenever the example performs OTA.

```csv
# Name,   Type, SubType,  Offset,   Size,   Flags
nvs,      data, nvs,      0x9000,   0x6000,
otadata,  data, ota,      0xF000,   0x2000,
phy_init, data, phy,      0x11000,  0x1000,
ota_0,    app,  ota_0,    0x20000,  1536K,
ota_1,    app,  ota_1,    0x1A0000, 1536K,
storage,  data, nvs,      0x320000, 0x40000,
```

- Requires 4 MB flash minimum.
- `otadata` tracks the active OTA slot (IDF-compatible bootloader via `esp-bootloader-esp-idf`).
- Both `ota_0` and `ota_1` are 1.5 MB — size according to the application binary size.
- `storage` (256 KB): application NVS.

**When to use:** Any example with OTA update capability. Required for all production-targeted examples.

---

## Profile 4: `dual-ota-encrypted-nvs` — Dual OTA + Encrypted NVS

Extends `dual-ota` with a dedicated NVS key partition required for NVS encryption.

```csv
# Name,    Type, SubType,  Offset,   Size,   Flags
nvs,       data, nvs,      0x9000,   0x6000,
otadata,   data, ota,      0xF000,   0x2000,
phy_init,  data, phy,      0x11000,  0x1000,
nvs_key,   data, nvs_keys, 0x12000,  0x1000, encrypted
ota_0,     app,  ota_0,    0x20000,  1536K,
ota_1,     app,  ota_1,    0x1A0000, 1536K,
storage,   data, nvs,      0x320000, 0x3E000, encrypted
```

- `nvs_key` (4 KB, `encrypted` flag): stores the NVS encryption key. Requires flash encryption (eFuse, not Kconfig — see `security-baseline.md`).
- `storage` (252 KB, `encrypted` flag): application NVS, encrypted with the key from `nvs_key`.

**When to use:** Production-targeted examples with NVS encryption requirement.

---

## Profile 5: `factory-plus-ota` — Factory Reset + Single OTA

Three-partition app layout for examples that need a factory-reset fallback image.

```csv
# Name,    Type, SubType,  Offset,   Size,   Flags
nvs,       data, nvs,      0x9000,   0x6000,
otadata,   data, ota,      0xF000,   0x2000,
phy_init,  data, phy,      0x11000,  0x1000,
factory,   app,  factory,  0x20000,  1M,
ota_0,     app,  ota_0,    0x120000, 1M,
storage,   data, nvs,      0x220000, 0x60000,
```

- `factory`: permanent factory image written at manufacturing; never overwritten by OTA.
- `ota_0`: single OTA slot; rollback goes to `factory`.
- Requires 4 MB flash.

**When to use:** Examples demonstrating factory reset or provisioning workflows.

---

## Profile 6: `minimal-4mb` — Minimal 4 MB Layout

Compact layout for firmware-only examples with no OTA and minimal NVS.

```csv
# Name,   Type, SubType,  Offset,   Size,
nvs,      data, nvs,      0x9000,   0x4000,
phy_init, data, phy,      0xD000,   0x1000,
factory,  app,  factory,  0x10000,  3M,
```

- App partition is maximized (3 MB) at the expense of NVS space.
- 4 KB NVS is sufficient for Wi-Fi credentials and minimal app state.

**When to use:** Large firmware images (TFLite, LVGL) on 4 MB flash boards.

---

## Flash Size Notes

| Flash Size | Maximum app partition | Profiles supported |
|---|---|---|
| 2 MB | ~1.5 MB (with NVS) | `default-single-app`, `minimal-4mb` (with adjustment) |
| 4 MB | ~3 MB single / 1.5 MB OTA | All profiles |
| 8 MB | ~6 MB single / 3 MB OTA | All profiles; increase sizes proportionally |
| 16 MB | ~14 MB single / 7 MB OTA | All profiles; set espflash flash size to 16MB |

Always set espflash flash size to match the actual flash chip (`espflash flash --flash-size 8MB` or `espflash.toml` `[flash] size = "8MB"`). Mismatches cause silent failures during OTA or NVS initialization.

DevKitC-C6-N8 is 8 MB; confirm the board-spec before picking a 4 MB-only layout.

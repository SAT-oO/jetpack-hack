# Human Verification Results

- Device: `3006d789-0d8f-7433-40c0-e3421c918002`
- Plan: `verify_plan_m_modes.json`

| id | label | sent | expect | verdict |
| --- | --- | --- | --- | --- |
| m2_t5 | Preset mode M2 travel 5 | `55 08 00 03 02 05 F7 (4s burst)` | Preset pattern 2 — distinct rhythm from M1. | **success** |
| m4_t5 | Preset mode M4 travel 5 | `55 08 00 03 04 05 F9 (4s burst)` | Preset pattern 4 — distinct rhythm from M2/M3. | **success** |
| m6_t5 | Preset mode M6 travel 5 | `55 08 00 03 06 05 F8 (4s burst)` | Preset pattern 6 — distinct rhythm from M5. | **success** |
| m7_t5 | Preset mode M7 travel 5 | `55 08 00 03 07 05 FA (4s burst)` | Preset pattern 7 — distinct from M6. | **success** |
| m8_t5 | Preset mode M8 travel 5 | `55 08 00 03 08 05 FE (4s burst)` | Preset pattern 8 — distinct from M7. | **success** |
| stretch_stop | Stretch/M stop | `55 08 00 01 00 00 F9` | All stretch/M motion stops. | **success** |

## FINDINGS.md gate

Copy only **success** rows into FINDINGS.md → Verified commands.

# Human Verification Results

- Device: `3006d789-0d8f-7433-40c0-e3421c918002`
- Plan: `verify_plan.json`

| id | label | sent | expect | verdict |
| --- | --- | --- | --- | --- |
| battery_query | Battery query | `55 02 00 00 00 00 FC` | No movement. Battery/status may update if visible. | **success** |
| status_query | Status sync query (0xA0) | `55 A0 00 00 00 00 FB` | No movement. Device returns status payload on notify. | **success** |
| boost_stop | Boost stop | `55 04 00 00 00 00 AA` | No thrust; holds position. | **success** |
| boost_20 | Boost scale 0x20 | `55 04 00 00 00 20 AA (4s burst)` | Stable rhythmic thrust, minimum effective depth | **success** |
| boost_40 | Boost scale 0x40 | `55 04 00 00 00 40 AA (4s burst)` | Rhythmic thrust, moderate depth | **success** |
| boost_80 | Boost scale 0x80 | `55 04 00 00 00 80 AA (4s burst)` | Medium stroke depth | **success** |
| boost_cc | Boost scale 0xCC | `55 04 00 00 00 CC AA (4s burst)` | Koosync Boost capture common depth | **success** |
| boost_ff | Boost scale 0xFF | `55 04 00 00 00 FF AA (4s burst)` | Maximum stroke depth / speed | **success** |
| stretch_l1 | Direct stretch level 1 | `55 08 00 00 01 01 FC (4s burst)` | Shallow stroke position (level 1). | **success** |
| stretch_l5 | Direct stretch level 5 | `55 08 00 00 04 03 F9 (4s burst)` | Mid/deep stroke — distinct from level 1. | **success** |
| stretch_l10 | Direct stretch level 10 | `55 08 00 00 08 0A F3 (4s burst)` | Deepest direct-stretch position. | **success** |
| m1_t5 | M1 / Fast travel 5 | `55 08 00 03 01 05 F4 (4s burst)` | Preset pattern M1 / Fast — distinct rhythmic thrust. | **success** |
| m2_t5 | M2 / Intense travel 5 | `55 08 00 03 02 05 F7 (4s burst)` | Preset pattern M2 / Intense — distinct rhythmic thrust. | **success** |
| m3_t5 | M3 / Teaser travel 5 | `55 08 00 03 03 05 F5 (4s burst)` | Preset pattern M3 / Teaser — distinct rhythmic thrust. | **success** |
| m4_t5 | M4 / Training travel 5 | `55 08 00 03 04 05 F9 (4s burst)` | Preset pattern M4 / Training — distinct rhythmic thrust. | **success** |
| m5_t5 | M5 / Thrill Seeking travel 5 | `55 08 00 03 05 05 FB (4s burst)` | Preset pattern M5 / Thrill Seeking — distinct rhythmic thrust. | **success** |
| m6_t5 | M6 / Charming travel 5 | `55 08 00 03 06 05 F8 (4s burst)` | Preset pattern M6 / Charming — distinct rhythmic thrust. | **success** |
| m7_t5 | M7 / Seduction travel 5 | `55 08 00 03 07 05 FA (4s burst)` | Preset pattern M7 / Seduction — distinct rhythmic thrust. | **success** |
| m8_t5 | M8 / Playful travel 5 | `55 08 00 03 08 05 FE (4s burst)` | Preset pattern M8 / Playful — distinct rhythmic thrust. | **success** |
| boost_latch | Boost latch (single frame) | `55 04 00 00 00 40 AA` | Single 0x40 frame sustains motion without 50 ms repeat; stop frame halts. | **success** |
| stretch_stop | Stretch/M stop | `55 08 00 01 00 00 F9` | All stretch/M motion stops. | **success** |
| stretch_l2 | Direct stretch level 2 | `55 08 00 00 01 02 FB (4s burst)` | Level 2 stroke | **success** |
| stretch_l3 | Direct stretch level 3 | `55 08 00 00 02 03 F7 (4s burst)` | Level 3 stroke | **success** |
| stretch_l4 | Direct stretch level 4 | `55 08 00 00 03 04 F6 (4s burst)` | Level 4 stroke | **success** |
| stretch_l6 | Direct stretch level 6 | `55 08 00 00 05 06 F0 (4s burst)` | Level 6 stroke | **success** |
| stretch_l7 | Direct stretch level 7 | `55 08 00 00 06 07 FC (4s burst)` | Level 7 stroke | **success** |
| stretch_l8 | Direct stretch level 8 | `55 08 00 00 07 08 FF (4s burst)` | Level 8 stroke | **success** |
| stretch_l9 | Direct stretch level 9 | `55 08 00 00 08 09 F4 (4s burst)` | Level 9 stroke | **success** |
| m1_t1 | M1 / Fast travel 1 | `55 08 00 03 01 01 F0 (4s burst)` | M1 / Fast at travel 0x01. | **success** |
| m1_ta | M1 / Fast travel 10 | `55 08 00 03 01 0A F5 (4s burst)` | M1 / Fast at travel 0x0a. | **success** |
| m2_t1 | M2 / Intense travel 1 | `55 08 00 03 02 01 F3 (4s burst)` | M2 / Intense at travel 0x01. | **success** |
| m2_ta | M2 / Intense travel 10 | `55 08 00 03 02 0A F6 (4s burst)` | M2 / Intense at travel 0x0a. | **success** |
| m3_t1 | M3 / Teaser travel 1 | `55 08 00 03 03 01 F1 (4s burst)` | M3 / Teaser at travel 0x01. | **success** |
| m3_ta | M3 / Teaser travel 10 | `55 08 00 03 03 0A F4 (4s burst)` | M3 / Teaser at travel 0x0a. | **success** |
| m4_t1 | M4 / Training travel 1 | `55 08 00 03 04 01 FD (4s burst)` | M4 / Training at travel 0x01. | **success** |
| m4_ta | M4 / Training travel 10 | `55 08 00 03 04 0A F8 (4s burst)` | M4 / Training at travel 0x0a. | **success** |
| m5_t1 | M5 / Thrill Seeking travel 1 | `55 08 00 03 05 01 FF (4s burst)` | M5 / Thrill Seeking at travel 0x01. | **success** |
| m5_ta | M5 / Thrill Seeking travel 10 | `55 08 00 03 05 0A FA (4s burst)` | M5 / Thrill Seeking at travel 0x0a. | **success** |
| m6_t1 | M6 / Charming travel 1 | `55 08 00 03 06 01 FC (4s burst)` | M6 / Charming at travel 0x01. | **success** |
| m6_ta | M6 / Charming travel 10 | `55 08 00 03 06 0A F9 (4s burst)` | M6 / Charming at travel 0x0a. | **success** |
| m7_t1 | M7 / Seduction travel 1 | `55 08 00 03 07 01 FE (4s burst)` | M7 / Seduction at travel 0x01. | **success** |
| m7_ta | M7 / Seduction travel 10 | `55 08 00 03 07 0A FB (4s burst)` | M7 / Seduction at travel 0x0a. | **success** |
| m8_t1 | M8 / Playful travel 1 | `55 08 00 03 08 01 FA (4s burst)` | M8 / Playful at travel 0x01. | **success** |
| m8_ta | M8 / Playful travel 10 | `55 08 00 03 08 0A FF (4s burst)` | M8 / Playful at travel 0x0a. | **success** |

## FINDINGS.md gate

Copy only **success** rows into FINDINGS.md → Verified commands.

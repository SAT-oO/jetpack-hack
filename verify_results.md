# Human Verification Results

- Device: `3006d789-0d8f-7433-40c0-e3421c918002`
- Plan: `verify_plan.json`

| id | label | sent | expect | verdict |
| --- | --- | --- | --- | --- |
| battery_query | Battery query | `55 02 00 00 00 00 FC` | No movement. Battery/status may update if visible. | **success** |
| boost_stop | Boost stop | `55 04 00 00 00 00 AA` | No thrust; holds position. | **success** |
| boost_40 | Boost scale 0x40 | `55 04 00 00 00 40 AA (4s burst)` | Rhythmic up/down thrust, moderate depth (video-sync style). | **success** |
| boost_ff | Boost scale 0xFF | `55 04 00 00 00 FF AA (4s burst)` | Stronger/faster thrust than 0x40. | **success** |
| stretch_l1 | Direct stretch level 1 | `55 08 00 00 01 01 FC (4s burst)` | Shallow stroke position (level 1 of 5). | **success** |
| stretch_l5 | Direct stretch level 5 | `55 08 00 00 05 05 F7 (4s burst)` | Deeper stroke position than level 1. | **success** |
| m1_t5 | Preset mode M1 travel 5 | `55 08 00 03 01 05 F4 (4s burst)` | Preset pattern 1 (e.g. Fast) at max travel. | **success** |
| m3_t3 | Preset mode M3 travel 3 | `55 08 00 03 03 03 F9 (4s burst)` | Preset pattern 3 — distinct rhythm from M1. | **success** |
| m5_t5 | Preset mode M5 travel 5 | `55 08 00 03 05 05 FB (4s burst)` | Preset pattern 5 — distinct from M1/M3. | **success** |
| stretch_stop | Stretch/M stop | `55 08 00 01 00 00 F9` | All stretch/M motion stops. | **success** |

## FINDINGS.md gate

Copy only **success** rows into FINDINGS.md → Verified commands.

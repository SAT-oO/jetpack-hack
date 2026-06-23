# Kaotik Lab The Jetpack — Verified BLE Commands

Commands listed only from `verify_results.md` **success** rows. Candidates came from `ble_sweep` echo/non-standard hits; hex was human-confirmed with `ble_verify`.

## Device Info

| Item | Value |
| --- | --- |
| Brand | Kaotik Lab |
| Product | The Jetpack |

## Frame Format

```text
55 <cmd> <p0> <p1> <p2> <p3> <tail>
```

| Opcode | Tail (from sweep) | Verified count |
| --- | --- | --- |
| `0x02` | CRC-8 C2 | 1 |
| `0x04` | fixed AA | 7 |
| `0x08` | CRC-8 C2 | 35 |
| `0xA0` | CRC-8 C2 | 1 |

---

## Battery query

### Query

```text
55 02 00 00 00 00 FC
```

### Response (sweep capture)

```text
55 02 10 01 00 00 00
```

No movement. Battery/status may update if visible.

---

## Boost (video-sync thrust)

### Command format

```text
55 04 00 00 00 <scale> AA
```

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| boost_stop | `55 04 00 00 00 00 AA` | No thrust; holds position. |
| boost_20 | `55 04 00 00 00 20 AA` | Stable rhythmic thrust, minimum effective depth |
| boost_40 | `55 04 00 00 00 40 AA` | Rhythmic thrust, moderate depth |
| boost_80 | `55 04 00 00 00 80 AA` | Medium stroke depth |
| boost_cc | `55 04 00 00 00 CC AA` | Koosync Boost capture common depth |
| boost_ff | `55 04 00 00 00 FF AA` | Maximum stroke depth / speed |

### Confirmed behavior

- Single non-zero frame may sustain motion without 50 ms repeat; stop frame halts.

---

## Direct stretch (stroke position)

### Command format

```text
55 08 00 00 <A> <B> <CRC>
```

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| stretch_l1 | `55 08 00 00 01 01 FC` | Shallow stroke position (level 1). |
| stretch_l5 | `55 08 00 00 04 03 F9` | Mid/deep stroke — distinct from level 1. |
| stretch_l10 | `55 08 00 00 08 0A F3` | Deepest direct-stretch position. |
| stretch_l2 | `55 08 00 00 01 02 FB` | Level 2 stroke |
| stretch_l3 | `55 08 00 00 02 03 F7` | Level 3 stroke |
| stretch_l4 | `55 08 00 00 03 04 F6` | Level 4 stroke |
| stretch_l6 | `55 08 00 00 05 06 F0` | Level 6 stroke |
| stretch_l7 | `55 08 00 00 06 07 FC` | Level 7 stroke |
| stretch_l8 | `55 08 00 00 07 08 FF` | Level 8 stroke |
| stretch_l9 | `55 08 00 00 08 09 F4` | Level 9 stroke |

---

## M-mode presets

### Command format

```text
55 08 00 03 <mode> <travel> <CRC>
```

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| m1_t5 | `55 08 00 03 01 05 F4` | Preset pattern M1 / Fast — distinct rhythmic thrust. |
| m2_t5 | `55 08 00 03 02 05 F7` | Preset pattern M2 / Intense — distinct rhythmic thrust. |
| m3_t5 | `55 08 00 03 03 05 F5` | Preset pattern M3 / Teaser — distinct rhythmic thrust. |
| m4_t5 | `55 08 00 03 04 05 F9` | Preset pattern M4 / Training — distinct rhythmic thrust. |
| m5_t5 | `55 08 00 03 05 05 FB` | Preset pattern M5 / Thrill Seeking — distinct rhythmic thrust. |
| m6_t5 | `55 08 00 03 06 05 F8` | Preset pattern M6 / Charming — distinct rhythmic thrust. |
| m7_t5 | `55 08 00 03 07 05 FA` | Preset pattern M7 / Seduction — distinct rhythmic thrust. |
| m8_t5 | `55 08 00 03 08 05 FE` | Preset pattern M8 / Playful — distinct rhythmic thrust. |
| m1_t1 | `55 08 00 03 01 01 F0` | M1 / Fast at travel 0x01. |
| m1_ta | `55 08 00 03 01 0A F5` | M1 / Fast at travel 0x0a. |
| m2_t1 | `55 08 00 03 02 01 F3` | M2 / Intense at travel 0x01. |
| m2_ta | `55 08 00 03 02 0A F6` | M2 / Intense at travel 0x0a. |
| m3_t1 | `55 08 00 03 03 01 F1` | M3 / Teaser at travel 0x01. |
| m3_ta | `55 08 00 03 03 0A F4` | M3 / Teaser at travel 0x0a. |
| m4_t1 | `55 08 00 03 04 01 FD` | M4 / Training at travel 0x01. |
| m4_ta | `55 08 00 03 04 0A F8` | M4 / Training at travel 0x0a. |
| m5_t1 | `55 08 00 03 05 01 FF` | M5 / Thrill Seeking at travel 0x01. |
| m5_ta | `55 08 00 03 05 0A FA` | M5 / Thrill Seeking at travel 0x0a. |
| m6_t1 | `55 08 00 03 06 01 FC` | M6 / Charming at travel 0x01. |
| m6_ta | `55 08 00 03 06 0A F9` | M6 / Charming at travel 0x0a. |
| m7_t1 | `55 08 00 03 07 01 FE` | M7 / Seduction at travel 0x01. |
| m7_ta | `55 08 00 03 07 0A FB` | M7 / Seduction at travel 0x0a. |
| m8_t1 | `55 08 00 03 08 01 FA` | M8 / Playful at travel 0x01. |
| m8_ta | `55 08 00 03 08 0A FF` | M8 / Playful at travel 0x0a. |

---

## Status sync / query

### Query

```text
55 A0 00 00 00 00 FB
```

### Response (sweep capture)

```text
55 A0 01 0A 00 00 00
```

No movement. Device returns status payload on notify.

---

## Stop command

```text
55 08 00 01 00 00 F9
```

All stretch/M motion stops.

---

## Boost latch behavior

- Single 0x40 frame sustains motion without 50 ms repeat; stop frame halts.

---

## Implementation Notes

| Use case | Format |
| --- | --- |
| Video sync | `55 04 00 00 00 00 AA` |
| Direct stretch | `55 08 00 00 01 01 FC` |
| M-mode presets | `55 08 00 03 01 05 F4` |
| Stop | `55 08 00 01 00 00 F9` |

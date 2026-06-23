# Kaotik The Jetpack — Verified BLE Commands

Document only commands confirmed by human verification (`verify_results.md`). Modes M2, M4, M6, M7, M8 are probed (echo) but not yet verified — run `verify_plan_m_modes.json`.

## Device Info

| Item | Value |
| --- | --- |
| Brand | Kaotik Lab |
| Product | The Jetpack |
| Internal model | HF470 |
| Product code | HF470A |
| Official app | KooSync |

## BLE UUID

| Role | UUID |
| --- | --- |
| Service | `0000ffe0-0000-1000-8000-00805f9b34fb` |
| Write | `0000ffe1-0000-1000-8000-00805f9b34fb` |
| Notify | `0000ffe2-0000-1000-8000-00805f9b34fb` |

## Frame Format

Most commands are 7 bytes:

```text
55 <cmd> <p0> <p1> <p2> <p3> <tail>
```

| Command family | Tail rule |
| --- | --- |
| `0x08` stretch / M-modes | CRC-8 C2 over bytes 0–5 |
| `0x04` boost (video-sync thrust) | Fixed `AA` |
| `0x02` battery query | CRC-8 C2 |

CRC-8 C2 parameters:

```text
poly   = 0xF0
init   = 0xFF
xorout = 0xFF
refin  = false
refout = true
```

---

## Battery query

### Query

```text
55 02 00 00 00 00 FC
```

### Response example

```text
55 02 17 01 00 00 00
```

Byte 2 (`0x17` in sample) is state-of-charge indicator; no motor movement.

---

## Boost (video-sync thrust)

Continuous rhythmic thrust; scale byte sets depth/speed. Sustain ~50 ms between frames.

### Command format

```text
55 04 00 00 00 <scale> AA
```

| Field | Meaning |
| --- | --- |
| `<scale>` | `0x00` = stop; `0x01`–`0xFF` = thrust intensity |

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| stop | `55 04 00 00 00 00 AA` | No thrust; holds position |
| moderate | `55 04 00 00 00 40 AA` | Rhythmic thrust, moderate depth |
| max | `55 04 00 00 00 FF AA` | Stronger/faster than `0x40` |

### Confirmed behavior

- Frames must be repeated ~50 ms to sustain motion
- Stop frame halts boost thrust immediately

---

## Direct stretch (stroke position)

Sets stroke length/position without a preset rhythm pattern (`p1 = 0x00`).

### Command format

```text
55 08 00 00 <level> <travel> <CRC>
```

| Field | Meaning |
| --- | --- |
| `<level>` | Stroke position level `0x01`–`0x05` |
| `<travel>` | Travel depth `0x01`–`0x05` (often matches level) |

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| level 1 | `55 08 00 00 01 01 FC` | Shallow stroke (level 1 of 5) |
| level 5 | `55 08 00 00 05 05 F7` | Deeper stroke than level 1 |

### Confirmed behavior

- Distinct physical positions between level 1 and level 5
- Uses CRC tail, not `AA`

---

## M-mode presets (thrust patterns)

Eight preset thrust rhythms (`p1 = 0x03`, mode byte `p2 = 0x01`–`0x08`, travel `p3 = 0x01`–`0x05`). **M2, M4, M6, M7, M8 probed only — pending verify.**

### Command format

```text
55 08 00 03 <mode> <travel> <CRC>
```

| Field | Meaning |
| --- | --- |
| `<mode>` | Preset pattern M1–M8 (`0x01`–`0x08`) |
| `<travel>` | Stroke travel `0x01`–`0x05` |

### Verified commands

| key | Command | Effect |
| --- | --- | --- |
| M1 travel 5 | `55 08 00 03 01 05 F4` | Preset pattern 1 at max travel |
| M3 travel 3 | `55 08 00 03 03 03 F9` | Preset pattern 3, distinct from M1 |
| M5 travel 5 | `55 08 00 03 05 05 FB` | Preset pattern 5, distinct from M1/M3 |
| stop | `55 08 00 01 00 00 F9` | All stretch/M motion stops |

### Probe candidates (echo — not verified)

| key | Command | Notes |
| --- | --- | --- |
| M2 travel 5 | `55 08 00 03 02 05 F7` | Echo on FFE1 |
| M4 travel 5 | `55 08 00 03 04 05 F9` | Echo on FFE1 |
| M6 travel 5 | `55 08 00 03 06 05 F8` | Echo on FFE1 |
| M7 travel 5 | `55 08 00 03 07 05 FA` | Echo on FFE1 |
| M8 travel 5 | `55 08 00 03 08 05 FE` | Echo on FFE1 |

### Confirmed behavior

- M1, M3, M5 produce distinct rhythmic patterns
- Stop frame `55 08 00 01 00 00 F9` halts M-mode motion
- Sustain ~50 ms between frames during bursts

---

## Implementation Notes

| Use case | Family | Format |
| --- | --- | --- |
| Video sync | Boost | `55 04 00 00 00 <scale> AA` |
| Stroke length | Direct stretch | `55 08 00 00 <level> <travel> CRC` |
| Preset M1–M8 | M-mode | `55 08 00 03 <mode> <travel> CRC` |
| Stop M/stretch | Stop | `55 08 00 01 00 00 F9` |
| Battery | Query | `55 02 00 00 00 00 FC` |

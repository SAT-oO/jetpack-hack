# jetpack-hack

BLE protocol reverse-engineering for **Kaotik Lab The Jetpack** (HF470A), using the reusable tooling in [`ble-hack-skill/`](ble-hack-skill/).

## Layout

| Path | Purpose |
|------|---------|
| `ble-hack-skill/` | **Generic** BLE RE skill — copy to other projects unchanged |
| `src/jetpack_protocol.rs` | Jetpack/KooSync UART frames, handshake, scan hints |
| `src/bin/jetpack_probe.rs` | Product-specific burst probes (after generic `ble_probe`) |
| `src/bin/jetpack_sweep.rs` | Product-specific boost + M-mode parameter grid |
| `verify_plan.json` | Human verification checkpoints for The Jetpack |
| `verify_plan_m_modes.json` | M2–M8 preset verification only |
| `FINDINGS.md` | Verified commands (from `verify_results.md`) |
| `scan_results.md`, `test_results.md`, … | Session artifacts |

## Commands (from repo root)

**Scan + generic probe (skill):**

```bash
cargo run -p ble-hack-skill --bin ble_scan -- \
  --brand Kaotik --product Jetpack --discover --output scan_results.md

cargo run -p ble-hack-skill --bin ble_probe -- \
  --device <UUID> --auto --output test_results.md
```

**Jetpack-specific protocol work (this repo):**

```bash
cargo run --bin jetpack_probe -- --device <UUID> --output test_results.md
cargo run --bin jetpack_sweep -- --device <UUID> --output sweep_results.md
```

**Human verification:**

```bash
cargo run -p ble-hack-skill --bin ble_verify -- \
  --device <UUID> --plan verify_plan.json --output verify_results.md
```

**One-go (generic skill only — does not run `jetpack_probe`):**

```bash
cargo run -p ble-hack-skill --bin ble_run -- --brand Kaotik --product Jetpack --workdir .
```

After `ble_run` passes the automation gate, run `jetpack_probe` before drafting or updating `verify_plan.json`.

## Jetpack protocol notes

- Local BLE name: **The Jetpack** — use `--product Jetpack` with `ble_scan`
- Motor path: **FFE1** write / **FFE2** notify
- Optional Kaotik GATT bonus UUIDs: `daf55d01-…`, `015df5da-…` (see `jetpack_protocol::kaotik_scan_bonus`)
- Anti-pattern learned on this device: do not target anonymous high-RSSI peripherals with `77777777` GATT — pick the named device with `FFE0`/`FFE1`/`FFE2`

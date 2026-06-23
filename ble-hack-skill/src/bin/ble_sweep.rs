//! Single-connection generic parameter grid — stays connected between probes.
//!
//!   cargo run --bin ble_sweep -- --device UUID [--handshake]

use anyhow::{Context, Result};
use ble_hack_skill::session::{
    adapter, connect, send_and_wait, send_burst, spaced_hex, ChannelPair,
};
use btleplug::api::{bleuuid::uuid_from_u16, Peripheral};
use std::fs;
use std::time::Duration;

fn default_channel() -> ChannelPair {
    ChannelPair {
        label: "FFE1/FFE2".into(),
        rx: uuid_from_u16(0xFFE1),
        tx: uuid_from_u16(0xFFE2),
    }
}

struct Row {
    label: String,
    sent: String,
    response: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let device = arg_value(&args, "--device").context("--device required")?;
    let output = arg_value(&args, "--output").unwrap_or_else(|| "sweep_results.md".into());

    let adpt = adapter().await?;
    let session = connect(&adpt, &device, &default_channel()).await?;
    let mut notifications = session.peripheral.notifications().await?;

    let mut rows = Vec::new();

    // Phase 1: legacy 7-byte zero-tail grid (Klitty-style and similar UART OEMs)
    let opcodes = [0x03u8, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    for opcode in opcodes {
        for mode in 1u8..=5 {
            for intensity in 1u8..=5 {
                let frame = vec![0x55, opcode, 0x00, 0x00, mode, intensity, 0x00];
                let response = send_and_wait(&session, &mut notifications, &frame).await?;
                let resp_str = response
                    .as_ref()
                    .map(|r| spaced_hex(r))
                    .unwrap_or_else(|| "(silent)".into());
                let is_echo = response.as_ref().is_some_and(|r| r.as_slice() == frame.as_slice());
                let is_nack = resp_str.contains("55 FF 01");
                if !is_nack && !is_echo && resp_str != "(silent)" {
                    println!(
                        "HIT op={opcode:02X} m={mode} i={intensity}: {resp_str}"
                    );
                }
                rows.push(Row {
                    label: format!("op_{opcode:02X}_m{mode}_i{intensity}"),
                    sent: spaced_hex(&frame),
                    response: resp_str,
                });
            }
        }
    }

    // Phase 2: 7-byte fixed-AA tail grid (common on Svakom/KooSync-class devices)
    for opcode in 0x03u8..=0x10 {
        for p4 in [0x01u8, 0x05] {
            for p5 in [0x01u8, 0x05, 0xFF] {
                let frame = vec![0x55, opcode, 0x00, 0x00, p4, p5, 0xAA];
                let response = send_and_wait(&session, &mut notifications, &frame).await?;
                let resp_str = response
                    .as_ref()
                    .map(|r| spaced_hex(r))
                    .unwrap_or_else(|| "(silent)".into());
                if !resp_str.contains("55 FF 01") && resp_str != "(silent)" {
                    println!("AA-frame op={opcode:02X} p4={p4:02X} p5={p5:02X}: {resp_str}");
                }
                rows.push(Row {
                    label: format!("aa_op_{opcode:02X}_{p4:02X}_{p5:02X}"),
                    sent: spaced_hex(&frame),
                    response: resp_str,
                });
            }
        }
    }

    // Phase 3: burst top non-silent candidates
    let candidates: Vec<_> = rows
        .iter()
        .filter(|r| r.response != "(silent)" && r.response != r.sent)
        .take(10)
        .collect();

    println!("\n=== Burst candidates ({}) ===", candidates.len());
    for c in candidates {
        let bytes: Vec<u8> = c
            .sent
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect();
        println!("Bursting {} for 3s...", c.label);
        let _ = send_burst(
            &session,
            &mut notifications,
            &[bytes],
            Duration::from_secs(3),
        )
        .await?;
    }

    session.peripheral.disconnect().await?;

    let md = format_sweep(&device, &rows);
    fs::write(&output, md)?;
    println!("\nWrote {output}");
    Ok(())
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

fn format_sweep(device: &str, rows: &[Row]) -> String {
    let mut out = format!("# BLE Sweep Results\n\n- Device: `{device}`\n\n");
    out.push_str("| label | sent | response |\n");
    out.push_str("| ----- | ---- | -------- |\n");
    for r in rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` |\n",
            r.label, r.sent, r.response
        ));
    }
    out
}

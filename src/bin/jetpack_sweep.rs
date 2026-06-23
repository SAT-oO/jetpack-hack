//! Jetpack-specific parameter sweep (KooSync UART on FFE1).
//!
//!   cargo run --bin jetpack_sweep -- --device UUID --output sweep_results.md

use anyhow::{Context, Result};
use ble_hack_skill::crc::{frame_with_aa, frame_with_crc};
use ble_hack_skill::session::{
    adapter, connect, send_and_wait, send_handshake, spaced_hex, ChannelPair,
};
use btleplug::api::{bleuuid::uuid_from_u16, Peripheral};
use futures::StreamExt;
use jetpack_hack::jetpack_protocol::{BATTERY_QUERY, HANDSHAKE};
use std::fs;

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
    status_before: String,
    status_after: String,
    status_delta: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let device = arg_value(&args, "--device").context("--device required")?;
    let with_handshake = args.iter().any(|a| a == "--handshake");
    let output = arg_value(&args, "--output").unwrap_or_else(|| "sweep_results.md".into());

    let adpt = adapter().await?;
    let session = connect(&adpt, &device, &default_channel()).await?;
    let mut notifications = session.peripheral.notifications().await?;

    if with_handshake {
        send_handshake(&session, &mut notifications, &HANDSHAKE).await?;
        println!("Handshake sent.\n");
    }

    let mut rows = Vec::new();

    let resp = send_and_wait(&session, &mut notifications, &BATTERY_QUERY).await?;
    println!(
        "battery: {} -> {}",
        spaced_hex(&BATTERY_QUERY),
        resp.as_ref().map(|r| spaced_hex(r)).unwrap_or_default()
    );

    for scale in [0x00u8, 0x20, 0x40, 0x80, 0xCC, 0xFF] {
        let frame = frame_with_aa([0x55, 0x04, 0x00, 0x00, 0x00, scale]).to_vec();
        rows.push(probe_row(
            &session,
            &mut notifications,
            &format!("boost_{scale:02X}"),
            &frame,
        )
        .await?);
    }

    for p1 in [0x00u8, 0x03] {
        for mode in 1u8..=8 {
            for travel in 1u8..=5 {
                let frame = frame_with_crc([0x55, 0x08, 0x00, p1, mode, travel]).to_vec();
                let row = probe_row(
                    &session,
                    &mut notifications,
                    &format!("crc_p1_{p1:02X}_m{mode}_t{travel}"),
                    &frame,
                )
                .await?;
                if !row.response.contains("55 FF") && row.response != "(silent)" {
                    println!("HIT {}: {}", row.label, row.response);
                }
                rows.push(row);
            }
        }
    }

    session.peripheral.disconnect().await?;

    let md = format_sweep(&device, &rows);
    fs::write(&output, md)?;
    println!("\nWrote {output}");
    Ok(())
}

async fn probe_row(
    session: &ble_hack_skill::session::Session,
    notifications: &mut (impl StreamExt<Item = btleplug::api::ValueNotification> + Unpin),
    label: &str,
    frame: &[u8],
) -> Result<Row> {
    let status_before = read_status(session, notifications).await?;
    let response = send_and_wait(session, notifications, frame).await?;
    let status_after = read_status(session, notifications).await?;
    let resp_str = response
        .as_ref()
        .map(|r| spaced_hex(r))
        .unwrap_or_else(|| "(silent)".into());
    Ok(Row {
        label: label.into(),
        sent: spaced_hex(frame),
        response: resp_str,
        status_before: status_before.clone(),
        status_after: status_after.clone(),
        status_delta: status_before != status_after,
    })
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

async fn read_status(
    session: &ble_hack_skill::session::Session,
    notifications: &mut (impl StreamExt<Item = btleplug::api::ValueNotification> + Unpin),
) -> Result<String> {
    let resp = send_and_wait(session, notifications, &BATTERY_QUERY).await?;
    Ok(resp
        .as_ref()
        .map(|r| spaced_hex(r))
        .unwrap_or_else(|| "(none)".into()))
}

fn format_sweep(device: &str, rows: &[Row]) -> String {
    let mut out = format!("# Jetpack Sweep Results\n\n- Device: `{device}`\n\n");
    out.push_str("| label | sent | response | status_before | status_after | delta |\n");
    out.push_str("| ----- | ---- | -------- | ------------- | ------------ | ----- |\n");
    for r in rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
            r.label, r.sent, r.response, r.status_before, r.status_after, r.status_delta
        ));
    }
    out
}

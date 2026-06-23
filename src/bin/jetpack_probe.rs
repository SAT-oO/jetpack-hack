//! Jetpack-specific BLE probes (HF470 / KooSync UART on FFE1).
//!
//! Run after generic `ble_probe --auto`:
//!   cargo run --bin jetpack_probe -- --device UUID --output test_results.md

use anyhow::{Context, Result};
use ble_hack_skill::session::{
    adapter, classify_response, connect, discover_channels_on_device, read_readable_chars,
    send_burst, spaced_hex, listen_notifications, ChannelPair, Session,
};
use btleplug::api::Peripheral;
use futures::StreamExt;
use jetpack_hack::jetpack_protocol::uart_burst_candidates;
use std::fs;
use std::time::Duration;

struct ProbeRow {
    label: String,
    channel: String,
    sent: String,
    response: String,
    class: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let device = arg_value(&args, "--device").context("--device UUID required")?;
    let output = arg_value(&args, "--output").unwrap_or_else(|| "test_results.md".into());

    let adpt = adapter().await?;
    let channels = discover_channels_on_device(&adpt, &device).await?;
    let channel = channels
        .iter()
        .find(|c| c.label.contains("FFE1"))
        .cloned()
        .or_else(|| channels.into_iter().next())
        .context("no write/notify channels on device")?;

    let rows = run_jetpack_probes(&device, &channel).await?;
    let md = format_results(&device, &rows);
    fs::write(&output, &md)?;
    print_summary(&rows);
    println!("\nWrote {output}");
    Ok(())
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

async fn run_jetpack_probes(device: &str, channel: &ChannelPair) -> Result<Vec<ProbeRow>> {
    let mut rows = Vec::new();
    let adpt = adapter().await?;
    let session = connect(&adpt, device, channel).await?;
    let mut notifications = session.peripheral.notifications().await?;

    for (uuid, data) in listen_notifications(&session, &mut notifications, Duration::from_secs(2)).await
    {
        rows.push(ProbeRow {
            label: "idle_notify".into(),
            channel: channel.label.clone(),
            sent: "(listen)".into(),
            response: spaced_hex(&data),
            class: "non-standard".into(),
        });
        println!("  idle notify {uuid}: {}", spaced_hex(&data));
    }

    for (uuid, data) in read_readable_chars(&session).await? {
        rows.push(ProbeRow {
            label: format!("read_{uuid}"),
            channel: channel.label.clone(),
            sent: "(read)".into(),
            response: spaced_hex(&data),
            class: "non-standard".into(),
        });
        println!("  read {uuid}: {}", spaced_hex(&data));
    }

    println!("\n=== KooSync UART burst candidates on {} ===\n", channel.label);
    for (label, frame) in uart_burst_candidates() {
        let row = burst_frame_session(&session, &mut notifications, channel, label, &frame, 2).await?;
        println!("  {label}: {} -> {}", row.sent, row.response);
        rows.push(row);
    }

    session.peripheral.disconnect().await?;
    Ok(rows)
}

async fn burst_frame_session(
    session: &Session,
    notifications: &mut (impl StreamExt<Item = btleplug::api::ValueNotification> + Unpin),
    channel: &ChannelPair,
    label: &str,
    frame: &[u8],
    seconds: u64,
) -> Result<ProbeRow> {
    let response = send_burst(
        session,
        notifications,
        &[frame.to_vec()],
        Duration::from_secs(seconds),
    )
    .await?;
    let class = classify_response(frame, &response);
    Ok(ProbeRow {
        label: label.into(),
        channel: channel.label.clone(),
        sent: spaced_hex(frame),
        response: response
            .as_ref()
            .map(|r| spaced_hex(r))
            .unwrap_or_else(|| "(no response)".into()),
        class: class.into(),
    })
}

fn format_results(device: &str, rows: &[ProbeRow]) -> String {
    let mut out = format!("# Jetpack BLE Probe Results\n\n- Device: `{device}`\n\n");
    out.push_str("| label | channel | sent | response | class |\n");
    out.push_str("| ----- | ------- | ---- | -------- | ----- |\n");
    for r in rows {
        out.push_str(&format!(
            "| {} | {} | `{}` | `{}` | {} |\n",
            r.label, r.channel, r.sent, r.response, r.class
        ));
    }
    out
}

fn print_summary(rows: &[ProbeRow]) {
    let interesting: Vec<_> = rows
        .iter()
        .filter(|r| r.class == "non-standard" || r.class == "echo")
        .collect();
    println!("\n=== Interesting responses ({}) ===", interesting.len());
    for r in interesting {
        println!(
            "  [{}] {} | {} -> {} ({})",
            r.channel, r.label, r.sent, r.response, r.class
        );
    }
}

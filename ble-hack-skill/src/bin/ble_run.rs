//! One-go BLE hack orchestrator — scan → probe → (sweep) → verify plan → ble_verify → FINDINGS.
//!
//! Run from the project root that vendors `ble-hack-skill/`:
//!   cargo run -p ble-hack-skill --bin ble_run -- --brand BRAND --product PRODUCT --workdir .

use anyhow::{bail, Context, Result};
use ble_hack_skill::pipeline::{self, pick_target, probe_passes_automation_gate};
use ble_hack_skill::workdir as wd;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let brand = arg_value(&args, "--brand");
    let product = arg_value(&args, "--product");
    let workdir = arg_value(&args, "--workdir")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let max_iters: u32 = arg_value(&args, "--max-iter")
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let scan_seconds: u64 = arg_value(&args, "--seconds")
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let skip_verify = args.iter().any(|a| a == "--skip-verify");

    let require_name = product.is_some() || brand.is_some();
    let mut device_id: Option<String> = None;

    for attempt in 1..=max_iters {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║  BLE hack iteration {attempt}/{max_iters}");
        println!("╚══════════════════════════════════════════════════╝\n");

        // STEP 0 — scan
        let adpt = pipeline::adapter().await?;
        let devices = pipeline::scan(
            &adpt,
            scan_seconds,
            brand.as_deref(),
            product.as_deref(),
        )
        .await?;

        let scan_path = workdir.join("scan_results.md");
        fs::write(&scan_path, format_scan_md(&devices, brand.as_deref(), product.as_deref()))?;
        println!("Wrote {}", scan_path.display());

        let target = pick_target(&devices, require_name).with_context(|| {
            if require_name {
                "no device with matching local name — power on target, disconnect official app"
            } else {
                "no viable BLE candidate found"
            }
        })?;

        println!(
            "Target: {} ({}) tier={} score={}",
            target.id,
            target.local_name.as_deref().unwrap_or("—"),
            target.tier,
            target.score
        );
        device_id = Some(target.id.clone());
        wd::save_session(&workdir, &target.id, target.local_name.as_deref())?;

        if args.iter().any(|a| a == "--discover") {
            let seconds_s = scan_seconds.to_string();
            let mut scan_args: Vec<&str> = vec![
                "--discover",
                "--seconds",
                &seconds_s,
                "--output",
                scan_path.to_str().unwrap(),
            ];
            let brand_s;
            let product_s;
            if let Some(ref b) = brand {
                brand_s = b.clone();
                scan_args.push("--brand");
                scan_args.push(&brand_s);
            }
            if let Some(ref p) = product {
                product_s = p.clone();
                scan_args.push("--product");
                scan_args.push(&product_s);
            }
            run_subcommand(&workdir, "ble_scan", &scan_args).await?;
        }

        // STEP 2 — probe (auto)
        let probe_path = workdir.join("test_results.md");
        run_subcommand(
            &workdir,
            "ble_probe",
            &[
                "--device",
                &target.id,
                "--auto",
                "--output",
                probe_path.to_str().unwrap(),
            ],
        )
        .await?;

        let probe_md = fs::read_to_string(&probe_path)?;
        if probe_passes_automation_gate(&probe_md) {
            println!("\n✓ Automation gate passed (FFE1 motor-channel responses).");
            break;
        }

        println!("\n✗ Automation gate failed — no FFE1 boost/stretch echo. Retrying in 5s…");
        if attempt == max_iters {
            bail!(
                "probe did not surface motor candidates after {max_iters} iterations; see {}",
                probe_path.display()
            );
        }
        time::sleep(Duration::from_secs(5)).await;
    }

    let _device_id = device_id.context("no device selected")?;
    let plan_path = workdir.join("verify_plan.json");
    if !plan_path.exists() {
        let example = Path::new("ble-hack-skill/verify_plan.example.json");
        let src = if example.exists() {
            example
        } else {
            Path::new("verify_plan.example.json")
        };
        fs::copy(src, &plan_path).with_context(|| format!("copy {}", src.display()))?;
        println!("Created {} from example — edit if probe suggests different frames.", plan_path.display());
    }

    if skip_verify {
        println!("\n--skip-verify set. Next:");
        println!("  cargo run -p ble-hack-skill --bin ble_verify -- --workdir {}", workdir.display());
        return Ok(());
    }

    // STEP 5 — human verify (interactive; must run in a real terminal)
    println!("\n═══ STEP 5: Human verification (watch the device) ═══\n");
    let verify_out = workdir.join(wd::DEFAULT_VERIFY_OUTPUT);
    let workdir_s = workdir.to_str().unwrap();
    let status = run_subcommand_interactive(
        &workdir,
        "ble_verify",
        &["--workdir", workdir_s],
    )
    .await?;

    if !status.success() {
        bail!("ble_verify exited with {}", status);
    }

    // STEP 6 — FINDINGS if successes exist
    if verify_out.exists() {
        let vr = fs::read_to_string(&verify_out)?;
        if vr.contains("| success |") || vr.contains("SUCCESS") {
            let findings = workdir.join("FINDINGS.md");
            if !findings.exists() {
                let template = Path::new("ble-hack-skill/FINDINGS.template.md");
                if template.exists() {
                    fs::copy(template, &findings)?;
                    println!(
                        "\nCreated {} — fill from SUCCESS rows in {}",
                        findings.display(),
                        verify_out.display()
                    );
                }
            }
            println!("\n✓ Pipeline complete. Update FINDINGS.md from verify success rows.");
        } else {
            println!("\nVerify finished with no SUCCESS rows — revise verify_plan.json and re-run ble_verify.");
        }
    }

    Ok(())
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

async fn run_subcommand(workdir: &Path, bin: &str, extra_args: &[&str]) -> Result<()> {
    let mut args: Vec<&str> = vec!["run", "-p", "ble-hack-skill", "--bin", bin, "--"];
    args.extend(extra_args);
    let status = Command::new("cargo")
        .args(&args)
        .current_dir(workdir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("cargo run --bin {bin}"))?;
    if !status.success() {
        bail!("cargo run --bin {bin} failed");
    }
    Ok(())
}

async fn run_subcommand_interactive(workdir: &Path, bin: &str, extra_args: &[&str]) -> Result<std::process::ExitStatus> {
    let mut args: Vec<&str> = vec!["run", "-p", "ble-hack-skill", "--bin", bin, "--"];
    args.extend(extra_args);
    Command::new("cargo")
        .args(&args)
        .current_dir(workdir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("interactive subcommand")
}

fn format_scan_md(
    devices: &[pipeline::ScannedDevice],
    brand: Option<&str>,
    product: Option<&str>,
) -> String {
    let mut out = String::from("# BLE Scan Results\n\n");
    if let Some(b) = brand {
        out.push_str(&format!("- Brand filter: `{b}`\n"));
    }
    if let Some(p) = product {
        out.push_str(&format!("- Product filter: `{p}`\n"));
    }
    out.push_str(&format!("- Devices found: {}\n\n", devices.len()));
    out.push_str("| tier | device_id | name | brand_match | rssi | score |\n");
    out.push_str("| ---- | --------- | ---- | ----------- | ---- | ----- |\n");
    for d in devices {
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} |\n",
            d.tier,
            d.id,
            d.local_name.as_deref().unwrap_or("—"),
            d.brand_match,
            d.rssi.map(|r| r.to_string()).unwrap_or_else(|| "—".into()),
            d.score
        ));
    }
    out
}

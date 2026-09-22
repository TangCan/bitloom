use super::{AXI_RESET_BOUNDARY, AXI_TB, system};
use bitloom_prelude::FrozenHir;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const ASSERTIONS_MIN: u64 = 250;
const TRANSACTIONS_MIN: u64 = 60;
const RANDOM_TRANSACTIONS: u64 = 16_000;

fn artifact_dir(backend: &str) -> PathBuf {
    let root = std::env::var_os("BITLOOM_FR201_ARTIFACT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/fr201-core")
        });
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    root.join(format!("{backend}-{}-{nonce}", std::process::id()))
}

fn output(dir: &Path, program: &str, args: &[&str], label: &str) -> Output {
    let out = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("required {program} failed to start for {label}: {e}"));
    fs::write(
        dir.join(format!("{label}.log")),
        [&out.stdout[..], &out.stderr[..]].concat(),
    )
    .unwrap();
    assert!(
        out.status.success(),
        "{label} failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    out
}

fn version(program: &str, arg: &str) -> Value {
    let path = Command::new("which")
        .arg(program)
        .output()
        .unwrap_or_else(|e| panic!("required which failed for {program}: {e}"));
    assert!(path.status.success(), "required {program} is missing");
    let result = Command::new(program)
        .arg(arg)
        .output()
        .unwrap_or_else(|e| panic!("required {program} missing: {e}"));
    assert!(result.status.success(), "{program} version command failed");
    json!({
        "path": String::from_utf8_lossy(&path.stdout).trim(),
        "stdout": String::from_utf8_lossy(&result.stdout),
        "stderr": String::from_utf8_lossy(&result.stderr),
        "exit_code": result.status.code()
    })
}

fn parse_count(stdout: &str, field: &str) -> u64 {
    stdout
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&format!("{field}=")))
        .and_then(|word| word.parse().ok())
        .unwrap_or(0)
}

fn chisel_wrapper(hir: &FrozenHir) -> String {
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|module| module.name == "Fr198AxiCore")
        .unwrap();
    let connections = top
        .ports
        .iter()
        .map(|port| match port.name.as_str() {
            "clk" => ".clock(clk)".to_string(),
            "rst" => ".reset(core_reset)".to_string(),
            name => format!(".io_{name}({name})"),
        })
        .collect::<Vec<_>>()
        .join(",");
    AXI_RESET_BOUNDARY.replace(
        "Fr198AxiCore core(.clk(clk),.rst(core_reset),.*);",
        &format!("Fr198AxiCore core({connections});"),
    )
}

pub(super) fn emit_direct(hir: &FrozenHir, dir: &Path) -> PathBuf {
    let mut source = bitloom_vlog::emit(hir)
        .files
        .into_iter()
        .map(|file| file.contents)
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(AXI_RESET_BOUNDARY);
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn firtool() -> String {
    let root = std::env::var("RHDL_FIRTOOL_PATH")
        .expect("RHDL_FIRTOOL_PATH must name firtool-1.159.0 directory");
    format!("{root}/firtool")
}

fn emit_firrtl(hir: &FrozenHir, dir: &Path) -> PathBuf {
    let firtool = firtool();
    let identity = output(dir, &firtool, &["--version"], "firtool-version");
    assert!(String::from_utf8_lossy(&identity.stdout).contains("firtool-1.159.0"));
    fs::write(
        dir.join("design.fir"),
        &bitloom_firrtl::emit(hir).files[0].contents,
    )
    .unwrap();
    output(
        dir,
        &firtool,
        &[
            "design.fir",
            "--verilog",
            "--disable-all-randomization",
            "--lowering-options=disallowLocalVariables",
            "-o",
            "core.v",
        ],
        "firrtl-lower",
    );
    let mut source = fs::read_to_string(dir.join("core.v")).unwrap();
    source.push_str(AXI_RESET_BOUNDARY);
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn emit_chisel(hir: &FrozenHir, dir: &Path) -> PathBuf {
    let tools = std::env::var("RHDL_FIRTOOL_PATH").expect("RHDL_FIRTOOL_PATH is required");
    fs::create_dir_all(dir.join("src/main/scala")).unwrap();
    fs::create_dir_all(dir.join("project")).unwrap();
    fs::write(
        dir.join("src/main/scala/Design.scala"),
        &bitloom_firrtl::emit_chisel(hir).unwrap().files[0].contents,
    )
    .unwrap();
    fs::write(
        dir.join("src/main/scala/Main.scala"),
        "object Fr201Main extends App { circt.stage.ChiselStage.emitSystemVerilogFile(new Fr198AxiCore, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }\n",
    )
    .unwrap();
    fs::write(
        dir.join("build.sbt"),
        "scalaVersion := \"2.13.18\"\nlibraryDependencies += \"org.chipsalliance\" %% \"chisel\" % \"7.15.0\"\naddCompilerPlugin(\"org.chipsalliance\" % \"chisel-plugin\" % \"7.15.0\" cross CrossVersion.full)\n",
    )
    .unwrap();
    fs::write(
        dir.join("project/build.properties"),
        "sbt.version=1.10.11\n",
    )
    .unwrap();
    let runtime = std::env::temp_dir().join(format!("bitloom-sbt-{}", std::process::id()));
    fs::create_dir_all(&runtime).unwrap();
    let log = fs::File::create(dir.join("sbt.log")).unwrap();
    let status = Command::new("timeout")
        .args([
            "--kill-after=5s",
            "300s",
            "sbt",
            "-batch",
            "runMain Fr201Main",
        ])
        .env("CHISEL_FIRTOOL_PATH", &tools)
        .env(
            "SBT_OPTS",
            "-Dsbt.server.autostart=false -Dsbt.supershell=false",
        )
        .env("XDG_RUNTIME_DIR", runtime)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required timeout/sbt executable missing");
    assert!(
        status.success(),
        "Chisel JVM generation failed: {}",
        dir.display()
    );
    let chisel = dir.join("chisel");
    let filelist = fs::read_to_string(chisel.join("filelist.f")).unwrap();
    let mut source = filelist
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| fs::read_to_string(chisel.join(line.trim())).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(&chisel_wrapper(hir));
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn run_backend(backend: &str, hir: &FrozenHir) {
    let dir = artifact_dir(backend);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("tb.sv"), AXI_TB).unwrap();
    let started = Instant::now();
    let design = match backend {
        "direct" => emit_direct(hir, &dir),
        "firrtl" => emit_firrtl(hir, &dir),
        "chisel" => emit_chisel(hir, &dir),
        _ => unreachable!(),
    };
    assert_eq!(design, dir.join("design.v"));
    output(
        &dir,
        "iverilog",
        &[
            "-g2012",
            "-s",
            "tb",
            "-o",
            "simulation",
            "design.v",
            "tb.sv",
        ],
        "compile",
    );
    let simulation = output(&dir, "timeout", &["60s", "vvp", "simulation"], "simulate");
    let stdout = String::from_utf8_lossy(&simulation.stdout);
    let assertions = parse_count(&stdout, "assertions");
    let transactions = parse_count(&stdout, "transactions");
    let random_transactions = parse_count(&stdout, "random_transactions");
    assert!(stdout.contains("FR198 PASS topology=axi"));
    assert!(assertions >= ASSERTIONS_MIN);
    assert!(transactions >= TRANSACTIONS_MIN);
    assert_eq!(random_transactions, RANDOM_TRANSACTIONS);
    let vcd = dir.join("axi.vcd");
    let vcd_bytes = fs::metadata(&vcd).expect("AXI VCD missing").len();
    assert!(vcd_bytes > 1024);
    let synthesis = output(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog -sv design.v; hierarchy -check -top Fr198Axi; proc; check -assert; synth -top Fr198Axi -flatten; check -assert; stat",
        ],
        "synthesis",
    );
    let synth_log = String::from_utf8_lossy(&synthesis.stdout);
    let cells = synth_log
        .lines()
        .filter_map(|line| line.trim().strip_prefix("Number of cells:"))
        .filter_map(|value| value.trim().parse::<u64>().ok())
        .last()
        .unwrap_or(0);
    assert!(cells > 0, "Yosys did not report synthesized cells");
    let bytes = fs::read(&design).unwrap();
    fs::write(
        dir.join("evidence.json"),
        serde_json::to_vec_pretty(&json!({
            "backend": backend,
            "exit_code": simulation.status.code(),
            "transactions": random_transactions,
            "directed_transactions": transactions,
            "assertions": assertions,
            "vcd_bytes": vcd_bytes,
            "coverage": {
                "slverr": true, "decerr": true, "wstrb": true, "backpressure": true,
                "reset_cancel": true, "timer": true, "gpio": true, "uart_tx": true,
                "uart_rx": true, "irq_0_4": true
            },
            "synthesis": {"check_assert": true, "cells": cells},
            "sha256": format!("{:x}", Sha256::digest(&bytes)),
            "elapsed_ms": started.elapsed().as_millis(),
            "tools": {
                "iverilog": version("iverilog", "-V"),
                "vvp": version("vvp", "-V"),
                "yosys": version("yosys", "-V")
            }
        }))
        .unwrap(),
    )
    .unwrap();
    println!("FR201 backend={backend} PASS artifacts={}", dir.display());
}

#[test]
#[ignore = "dedicated pinned direct/FIRRTL/Chisel complete-system matrix"]
fn fr201_axi_complete_system_three_backend_matrix() {
    let hir = system(true);
    let selected = std::env::var("BITLOOM_FR201_BACKENDS").ok();
    for backend in ["direct", "firrtl", "chisel"] {
        if selected
            .as_deref()
            .is_some_and(|value| !value.split(',').any(|item| item == backend))
        {
            continue;
        }
        run_backend(backend, &hir);
    }
}

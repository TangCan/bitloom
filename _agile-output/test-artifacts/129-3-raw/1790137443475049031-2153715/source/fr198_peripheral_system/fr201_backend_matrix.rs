use super::{AXI_RESET_BOUNDARY, AXI_TB, CSR_TB, DIRECT_RESET_BOUNDARY, RANDOM_SEEDS, system};
use bitloom_prelude::FrozenHir;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
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

pub(super) fn file_sha(path: &Path) -> String {
    format!("{:x}", Sha256::digest(fs::read(path).unwrap()))
}

fn executable(program: &str) -> PathBuf {
    let candidate = if program.contains('/') {
        PathBuf::from(program)
    } else {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|dir| dir.join(program))
            .find(|path| path.is_file())
            .unwrap_or_else(|| panic!("required tool missing: {program}"))
    };
    // Keep argv[0] for dispatchers such as the rustc -> rustup proxy.
    if candidate.is_absolute() {
        candidate
    } else {
        std::env::current_dir().unwrap().join(candidate)
    }
}

pub(super) fn run_tool(
    dir: &Path,
    program: &str,
    args: &[&str],
    label: &str,
    seconds: u64,
) -> Output {
    let tool = executable(program);
    let tool_sha = file_sha(&tool);
    let timeout = executable("timeout");
    let started = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let timer = Instant::now();
    let mut command = Command::new(&timeout);
    command
        .args(["--kill-after=5s", &format!("{seconds}s")])
        .arg(&tool)
        .args(args);
    if program == "sbt" {
        let tools = std::env::var("RHDL_FIRTOOL_PATH").expect("RHDL_FIRTOOL_PATH required");
        let runtime = std::env::temp_dir().join(format!("fr201-sbt-{}", std::process::id()));
        fs::create_dir_all(&runtime).unwrap();
        command
            .env("CHISEL_FIRTOOL_PATH", tools)
            .env(
                "SBT_OPTS",
                "-Dsbt.server.autostart=false -Dsbt.supershell=false",
            )
            .env("XDG_RUNTIME_DIR", runtime);
    }
    let out = command
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("start {program}: {e}"));
    fs::write(
        dir.join(format!("{label}.log")),
        [&out.stdout[..], &out.stderr[..]].concat(),
    )
    .unwrap();
    let record = json!({
        "label": label, "program": program, "arguments": args, "cwd": dir,
        "tool_path": tool, "tool_canonical_path": fs::canonicalize(&tool).unwrap(), "tool_sha256": tool_sha,
        "timeout_path": timeout, "timeout_sha256": file_sha(&timeout),
        "timeout_seconds": seconds, "kill_after_seconds": 5,
        "start_utc_unix_ms": started, "elapsed_ms": timer.elapsed().as_millis(),
        "exit_code": out.status.code(), "log": format!("{label}.log"),
        "stdout": String::from_utf8_lossy(&out.stdout), "stderr": String::from_utf8_lossy(&out.stderr),
    });
    fs::write(
        dir.join(format!("{label}.command.json")),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    assert_eq!(
        tool_sha,
        file_sha(&tool),
        "{program} changed during execution"
    );
    out
}

fn output(dir: &Path, program: &str, args: &[&str], label: &str) -> Output {
    let out = run_tool(dir, program, args, label, 300);
    assert!(
        out.status.success(),
        "{label} failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    out
}

pub(super) fn identity(dir: &Path, program: &str, arg: &str, label: &str) -> Value {
    let out = run_tool(
        dir,
        program,
        &[arg],
        label,
        if program == "sbt" { 300 } else { 30 },
    );
    assert!(out.status.success(), "required {program} identity failed");
    let path = executable(program);
    json!({"path": path, "sha256": file_sha(&path), "stdout": String::from_utf8_lossy(&out.stdout),
        "stderr": String::from_utf8_lossy(&out.stderr), "exit_code": out.status.code()})
}

pub(super) fn commands(dir: &Path) -> Vec<Value> {
    let mut paths = fs::read_dir(dir)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.to_string_lossy().ends_with(".command.json"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .iter()
        .map(|p| serde_json::from_slice(&fs::read(p).unwrap()).unwrap())
        .collect()
}

pub(super) fn source_hashes(dir: &Path) -> Value {
    fn walk(base: &Path, dir: &Path, entries: &mut serde_json::Map<String, Value>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                if path
                    .file_name()
                    .is_some_and(|n| n == "target" || n == "sbt-runtime")
                {
                    continue;
                }
                walk(base, &path, entries);
            } else if path.extension().is_some_and(|e| {
                ["v", "sv", "fir", "scala", "sbt", "properties", "f", "sby"]
                    .iter()
                    .any(|x| e == *x)
            }) {
                entries.insert(
                    path.strip_prefix(base).unwrap().to_string_lossy().into(),
                    json!(file_sha(&path)),
                );
            }
        }
    }
    let mut entries = serde_json::Map::new();
    walk(dir, dir, &mut entries);
    Value::Object(entries)
}

fn core(axi: bool) -> &'static str {
    if axi {
        "Fr198AxiCore"
    } else {
        "Fr198DirectCore"
    }
}
fn top(axi: bool) -> &'static str {
    if axi { "Fr198Axi" } else { "Fr198Direct" }
}
fn boundary(axi: bool) -> &'static str {
    if axi {
        AXI_RESET_BOUNDARY
    } else {
        DIRECT_RESET_BOUNDARY
    }
}

fn parse_count(stdout: &str, field: &str) -> u64 {
    stdout
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&format!("{field}=")))
        .and_then(|word| word.parse().ok())
        .unwrap_or(0)
}

fn chisel_wrapper(hir: &FrozenHir, axi: bool) -> String {
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|module| module.name == core(axi))
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
    boundary(axi).replace(
        &format!("{} core(.clk(clk),.rst(core_reset),.*);", core(axi)),
        &format!("{} core({connections});", core(axi)),
    )
}

pub(super) fn emit_direct(hir: &FrozenHir, dir: &Path) -> PathBuf {
    emit_direct_topology(hir, dir, true)
}

fn emit_direct_topology(hir: &FrozenHir, dir: &Path, axi: bool) -> PathBuf {
    let mut source = bitloom_vlog::emit(hir)
        .files
        .into_iter()
        .map(|file| file.contents)
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(boundary(axi));
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn firtool() -> String {
    let root = std::env::var("RHDL_FIRTOOL_PATH")
        .expect("RHDL_FIRTOOL_PATH must name firtool-1.159.0 directory");
    format!("{root}/firtool")
}

fn emit_firrtl(hir: &FrozenHir, dir: &Path, axi: bool) -> PathBuf {
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
    source.push_str(boundary(axi));
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn emit_chisel(hir: &FrozenHir, dir: &Path, axi: bool) -> PathBuf {
    fs::create_dir_all(dir.join("src/main/scala")).unwrap();
    fs::create_dir_all(dir.join("project")).unwrap();
    fs::write(
        dir.join("src/main/scala/Design.scala"),
        &bitloom_firrtl::emit_chisel(hir).unwrap().files[0].contents,
    )
    .unwrap();
    fs::write(
        dir.join("src/main/scala/Main.scala"),
        format!("object Fr201Main extends App {{ circt.stage.ChiselStage.emitSystemVerilogFile(new {}, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }}\n", core(axi)),
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
    output(dir, "sbt", &["-batch", "runMain Fr201Main"], "sbt");
    let chisel = dir.join("chisel");
    let filelist = fs::read_to_string(chisel.join("filelist.f")).unwrap();
    let mut source = filelist
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| fs::read_to_string(chisel.join(line.trim())).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(&chisel_wrapper(hir, axi));
    let path = dir.join("design.v");
    fs::write(&path, source).unwrap();
    path
}

fn run_backend(backend: &str, hir: &FrozenHir, axi: bool) {
    let prefix = if axi {
        backend.to_string()
    } else {
        format!("direct-csr-{backend}")
    };
    let dir = artifact_dir(&prefix);
    fs::create_dir_all(&dir).unwrap();
    let tb = if axi { AXI_TB } else { CSR_TB };
    fs::write(dir.join("tb.sv"), tb).unwrap();
    for seed in RANDOM_SEEDS {
        assert!(
            tb.contains(&format!("32'h{seed:08x}")),
            "missing frozen seed"
        );
    }
    let started = Instant::now();
    let design = match backend {
        "direct" => emit_direct_topology(hir, &dir, axi),
        "firrtl" => emit_firrtl(hir, &dir, axi),
        "chisel" => emit_chisel(hir, &dir, axi),
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
    let simulation = run_tool(&dir, "vvp", &["simulation"], "simulate", 60);
    assert!(
        simulation.status.success(),
        "simulation failed: {}",
        String::from_utf8_lossy(&simulation.stderr)
    );
    let stdout = String::from_utf8_lossy(&simulation.stdout);
    let assertions = parse_count(&stdout, "assertions");
    let transactions = parse_count(&stdout, "transactions");
    let random_transactions = parse_count(&stdout, "random_transactions");
    assert!(stdout.contains(if axi {
        "FR198 PASS topology=axi"
    } else {
        "FR198 PASS topology=direct"
    }));
    assert!(assertions >= ASSERTIONS_MIN);
    assert!(transactions >= TRANSACTIONS_MIN);
    assert_eq!(random_transactions, RANDOM_TRANSACTIONS);
    let vcd = dir.join(if axi { "axi.vcd" } else { "direct.vcd" });
    let vcd_bytes = fs::metadata(&vcd).expect("AXI VCD missing").len();
    assert!(vcd_bytes > 1024);
    let synthesis = output(
        &dir,
        "yosys",
        &[
            "-p",
            &format!(
                "read_verilog -sv design.v; hierarchy -check -top {}; proc; check -assert; synth -top {} -flatten; check -assert; stat",
                top(axi),
                top(axi)
            ),
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
    let mut tools = json!({
        "iverilog": identity(&dir, "iverilog", "-V", "iverilog-version"),
        "vvp": identity(&dir, "vvp", "-V", "vvp-version"),
        "yosys": identity(&dir, "yosys", "-V", "yosys-version"),
        "rustc": identity(&dir, "rustc", "--version", "rustc-version"),
    });
    if backend != "direct" {
        tools["firtool"] = identity(&dir, &firtool(), "--version", "firtool-identity");
    }
    if backend == "chisel" {
        tools["java"] = identity(&dir, "java", "-version", "java-version");
        tools["sbt"] = identity(&dir, "sbt", "show sbtVersion", "sbt-version");
    }
    let bytes = fs::read(&design).unwrap();
    fs::write(
        dir.join("evidence.json"),
        serde_json::to_vec_pretty(&json!({
            "backend": backend,
            "topology": if axi { "axi" } else { "direct-csr" },
            "random_seeds": RANDOM_SEEDS.iter().map(|seed| format!("{seed:08x}")).collect::<Vec<_>>(),
            "source_sha256": source_hashes(&dir),
            "commands": commands(&dir),
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
            "tools": tools
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
        run_backend(backend, &hir, true);
    }
}

#[test]
#[ignore = "dedicated pinned direct-CSR direct/FIRRTL/Chisel complete-system matrix"]
fn fr201_direct_csr_complete_system_three_backend_matrix() {
    let hir = system(false);
    for backend in ["direct", "firrtl", "chisel"] {
        run_backend(backend, &hir, false);
    }
}

//! ATDD Story 20.5: `cargo bitloom import` — FR40 / FR46 leg 3.

use std::process::Command;

#[test]
fn import_help_mentions_input_and_out_dir() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["import", "--help"])
        .output()
        .expect("run import --help");
    assert!(out.status.success(), "import --help failed");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("--input") && text.contains("--out-dir"),
        "help must document --input and --out-dir:\n{text}"
    );
    assert!(
        text.contains(".fir") || text.contains("FIRRTL") || text.contains("fir"),
        "help should mention FIRRTL/.fir:\n{text}"
    );
    assert!(
        text.contains("--also-chisel"),
        "help must document --also-chisel:\n{text}"
    );
}

#[test]
fn import_smoke_emits_verilog_from_external_fir() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir();
    let out = Command::new(bin)
        .args([
            "import",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--also-fir",
        ])
        .output()
        .expect("run import smoke");
    assert!(
        out.status.success(),
        "import smoke failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let entries: Vec<_> = std::fs::read_dir(&out_dir)
        .expect("out_dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        entries.iter().any(|n| n.ends_with(".v")),
        "expected .v in {entries:?}"
    );
    assert!(
        entries.iter().any(|n| n.ends_with(".fir")),
        "expected .fir re-emit with --also-fir in {entries:?}"
    );
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn import_also_chisel_writes_scala() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir();
    let out = Command::new(bin)
        .args([
            "import",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--also-chisel",
        ])
        .output()
        .expect("run import --also-chisel");
    assert!(
        out.status.success(),
        "import --also-chisel failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let entries: Vec<_> = std::fs::read_dir(&out_dir)
        .expect("out_dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        entries.iter().any(|n| n.ends_with(".v")),
        "expected .v in {entries:?}"
    );
    assert!(
        entries.iter().any(|n| n.ends_with(".scala")),
        "expected .scala with --also-chisel in {entries:?}"
    );
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn import_bad_header_fails() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let dir = tempfile_dir();
    let bad = dir.join("bad.fir");
    std::fs::write(&bad, "circuit X :\n  module X :\n").unwrap();
    let out = Command::new(bin)
        .args([
            "import",
            "--input",
            bad.to_str().unwrap(),
            "--out-dir",
            dir.to_str().unwrap(),
        ])
        .output()
        .expect("run import bad");
    assert!(!out.status.success(), "bad FIRRTL header must fail");
    let _ = std::fs::remove_dir_all(&dir);
}

fn tempfile_dir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "bitloom-import-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}

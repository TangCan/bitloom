//! ATDD Story 23.3: `cargo bitloom wave` — FR38 / FR49 / FR40 timing product entry.

use std::fs;
use std::process::Command;

#[test]
fn wave_help_mentions_input_out_dir_ticks() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = Command::new(bin)
        .args(["wave", "--help"])
        .output()
        .expect("run wave --help");
    assert!(out.status.success(), "wave --help failed");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("--input") && text.contains("--out-dir") && text.contains("--ticks"),
        "help must document --input/--out-dir/--ticks:\n{text}"
    );
}

#[test]
fn wave_smoke_emits_vcd_and_browsable_timing_html() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("wave");
    let out = Command::new(bin)
        .args([
            "wave",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--ticks",
            "4",
        ])
        .output()
        .expect("run wave smoke");
    assert!(
        out.status.success(),
        "wave smoke failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let vcd = fs::read_to_string(out_dir.join("wave.vcd")).expect("wave.vcd");
    assert!(!vcd.trim().is_empty(), "wave.vcd must be non-empty");
    assert!(
        vcd.contains("$var") || vcd.contains("#"),
        "VCD looks malformed:\n{vcd}"
    );

    let html = fs::read_to_string(out_dir.join("timing.html")).expect("timing.html");
    assert!(!html.trim().is_empty(), "timing.html must be non-empty");
    assert!(
        html.contains("Bitloom")
            && html.contains("Value table")
            && (html.contains("not") && html.contains("sole")),
        "timing HTML must be a product view (not GTKWave-only):\n{html}"
    );
    // Must not be a stub that only tells users to open GTKWave.
    assert!(
        !html.to_lowercase().contains("please open gtkwave")
            || html.contains("not</strong> the sole"),
        "must not be GTKWave-only instructions"
    );
    let _ = fs::remove_dir_all(&out_dir);
}

#[test]
fn wave_without_fst_still_writes_vcd_path() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("wave-nofst");
    let out = Command::new(bin)
        .args([
            "wave",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--ticks",
            "2",
        ])
        .output()
        .expect("run wave no-fst");
    assert!(out.status.success());
    assert!(out_dir.join("wave.vcd").is_file());
    assert!(out_dir.join("timing.html").is_file());
    assert!(
        !out_dir.join("wave.fst").is_file(),
        "FST must remain optional when --fst not set"
    );
    let _ = fs::remove_dir_all(&out_dir);
}

fn tempfile_dir(tag: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "bitloom-{tag}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&p).unwrap();
    p
}

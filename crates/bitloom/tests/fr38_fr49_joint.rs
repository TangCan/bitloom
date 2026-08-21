//! ATDD Story 23.5: joint FR38/FR49 acceptance — same fixture → hierarchy + timing.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn tempfile_dir(tag: &str) -> PathBuf {
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

#[test]
fn fr38_fr49_joint_acceptance_same_fixture() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = workspace_root().join("crates/rhdl-firrtl/fixtures/external_hierarchy.fir");
    assert!(fir.is_file(), "fixture missing: {}", fir.display());

    let out = tempfile_dir("fr38-fr49-joint");
    let fir_s = fir.to_str().unwrap();
    let out_s = out.to_str().unwrap();

    let viz = Command::new(bin)
        .args(["visualize", "--input", fir_s, "--out-dir", out_s])
        .output()
        .expect("visualize");
    assert!(
        viz.status.success(),
        "visualize failed:\n{}",
        String::from_utf8_lossy(&viz.stderr)
    );

    let wave = Command::new(bin)
        .args([
            "wave",
            "--input",
            fir_s,
            "--out-dir",
            out_s,
            "--ticks",
            "4",
        ])
        .output()
        .expect("wave");
    assert!(
        wave.status.success(),
        "wave failed:\n{}",
        String::from_utf8_lossy(&wave.stderr)
    );

    let hierarchy = fs::read_to_string(out.join("hierarchy.html")).expect("hierarchy.html");
    let timing = fs::read_to_string(out.join("timing.html")).expect("timing.html");
    let vcd = fs::read_to_string(out.join("wave.vcd")).expect("wave.vcd");

    assert!(!hierarchy.trim().is_empty(), "hierarchy.html empty");
    assert!(!timing.trim().is_empty(), "timing.html empty");
    assert!(!vcd.trim().is_empty(), "wave.vcd empty");

    // Minimal content checks (docs / FR38 / FR49)
    assert!(
        hierarchy.contains("Bitloom")
            && hierarchy.contains("Modules and ports")
            && hierarchy.contains("Instance hierarchy")
            && hierarchy.contains("u0")
            && hierarchy.contains("Child"),
        "hierarchy missing required content:\n{hierarchy}"
    );
    assert!(
        timing.contains("Bitloom")
            && timing.contains("Value table")
            && timing.contains("sole"),
        "timing missing product-view markers:\n{timing}"
    );

    // Docs still describe the joint path (UJ-6)
    let uj6 = fs::read_to_string(workspace_root().join("docs/tutorials/uj6-visualization.md"))
        .expect("uj6");
    assert!(
        uj6.contains("visualize") && uj6.contains("wave") && uj6.contains("external_hierarchy"),
        "UJ-6 must document the same fixture joint path"
    );

    let _ = fs::remove_dir_all(&out);
}

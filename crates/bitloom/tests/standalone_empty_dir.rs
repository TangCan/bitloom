//! ATDD Story 13.5: empty-dir install→new→build→.v without cloning the monorepo.
//!
//! Wall-clock: keep under CI `timeout-minutes: 25` (Epic 13 retro action). Prefer
//! absolute `--out-dir` and avoid extra registry round-trips when possible.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn empty_dir_new_then_build_emits_verilog() {
    let root = unique_tmpdir("bitloom-13-5");
    let name = "standalone_blink";
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");

    let new_out = Command::new(bin)
        .arg("new")
        .arg(name)
        .arg("--path")
        .arg(&root)
        .current_dir(&root)
        .output()
        .expect("new");
    assert!(
        new_out.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_out.stderr)
    );

    let crate_dir = root.join(name);
    assert!(crate_dir.join("Cargo.toml").is_file());

    // Absolute out_dir avoids relative join under --manifest-dir.
    let out_dir = root.join("verilog_out");
    let build_out = Command::new(bin)
        .arg("build")
        .arg("--package")
        .arg(name)
        .arg("--manifest-dir")
        .arg(&crate_dir)
        .arg("--out-dir")
        .arg(&out_dir)
        .current_dir(&root)
        .output()
        .expect("build");
    assert!(
        build_out.status.success(),
        "build failed (true-standalone contract):\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&build_out.stdout),
        String::from_utf8_lossy(&build_out.stderr)
    );

    let host_toml = fs::read_to_string(
        crate_dir
            .join("target/rhdl-host")
            .join(name)
            .join("Cargo.toml"),
    )
    .expect("host Cargo.toml");
    assert!(
        !host_toml.contains("crates/rhdl-vlog") && !host_toml.contains("crates/bitloom-vlog"),
        "standalone host must not path-depend monorepo:\n{host_toml}"
    );
    assert!(
        host_toml.contains("bitloom-vlog = \""),
        "expected registry-pinned bitloom-vlog:\n{host_toml}"
    );

    let v_files: Vec<PathBuf> = fs::read_dir(&out_dir)
        .expect("out_dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("v"))
        .collect();
    assert!(
        !v_files.is_empty(),
        "expected at least one .v under {}",
        out_dir.display()
    );
    let contents = fs::read_to_string(&v_files[0]).expect("read .v");
    assert!(
        !contents.trim().is_empty(),
        "Verilog must be non-empty: {}",
        v_files[0].display()
    );
    assert!(
        contents.contains("module") || contents.contains("endmodule"),
        "expected Yosys-friendly module text:\n{contents}"
    );

    let _ = fs::remove_dir_all(&root);
}

fn unique_tmpdir(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("tmpdir");
    dir
}

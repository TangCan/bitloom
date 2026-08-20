//! Story 15.3: `cargo bitloom build --package rv32_core` emits non-empty `.v`.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn cargo_bitloom_build_emits_episode_i_v() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace");
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out_dir = workspace.join("target/rv32_core_v_out");
    let _ = fs::remove_dir_all(&out_dir);
    fs::create_dir_all(&out_dir).unwrap();

    let out = Command::new(bin)
        .arg("build")
        .arg("--package")
        .arg("rv32_core")
        .arg("--manifest-dir")
        .arg(&workspace)
        .arg("--out-dir")
        .arg(&out_dir)
        .current_dir(&workspace)
        .output()
        .expect("build");
    assert!(
        out.status.success(),
        "build failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let v_files: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("v"))
        .collect();
    assert!(
        !v_files.is_empty(),
        "expected .v under {}",
        out_dir.display()
    );
    let contents = fs::read_to_string(&v_files[0]).unwrap();
    assert!(contents.contains("module EpisodeICore"));
    assert!(!contents.trim().is_empty());
}

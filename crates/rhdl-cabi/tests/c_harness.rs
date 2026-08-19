//! Compile and run the C harness against the cdylib (FR33).

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn c_harness_matches_rust_golden() {
    assert_eq!(rhdl_cabi::rust_golden_data_out(), 3);

    let root = workspace_root();
    let so = std::env::var_os("CARGO_CDYLIB_FILE_RHDL_CABI")
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .or_else(|| {
            let p = root.join("target/debug/deps/librhdl_cabi.so");
            p.is_file().then_some(p)
        })
        .or_else(|| {
            let p = root.join("target/debug/librhdl_cabi.so");
            p.is_file().then_some(p)
        })
        .expect("cdylib librhdl_cabi.so not found");
    let libdir = so.parent().unwrap();

    let harness_c = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/harness.c");
    let include = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("include");
    let out = std::env::temp_dir().join("rhdl_cabi_harness");
    let status = Command::new("cc")
        .args(["-o"])
        .arg(&out)
        .arg(&harness_c)
        .arg("-I")
        .arg(&include)
        .arg("-L")
        .arg(libdir)
        .arg("-lrhdl_cabi")
        .arg(format!("-Wl,-rpath,{}", libdir.display()))
        .status()
        .expect("spawn cc");
    assert!(status.success(), "cc failed to link C harness");

    let run = Command::new(&out).output().expect("run harness");
    assert!(
        run.status.success(),
        "harness failed: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(stdout.contains("ok rtl=3"), "{stdout}");
}

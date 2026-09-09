//! Compile and run C harnesses against the cdylib (FR33 / FR83).

use std::ffi::CString;
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

fn find_cdylib() -> PathBuf {
    let root = workspace_root();
    std::env::var_os("CARGO_CDYLIB_FILE_RHDL_CABI")
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
        .expect("cdylib librhdl_cabi.so not found")
}

fn compile_and_run(harness_c: &str, out_name: &str, expect_stdout: &str) {
    let so = find_cdylib();
    let libdir = so.parent().unwrap();
    let harness = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(harness_c);
    let include = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("include");
    let out = std::env::temp_dir().join(out_name);
    let status = Command::new("cc")
        .args(["-o"])
        .arg(&out)
        .arg(&harness)
        .arg("-I")
        .arg(&include)
        .arg("-L")
        .arg(libdir)
        .arg("-lrhdl_cabi")
        .arg(format!("-Wl,-rpath,{}", libdir.display()))
        .status()
        .expect("spawn cc");
    assert!(status.success(), "cc failed to link {harness_c}");

    let run = Command::new(&out).output().expect("run harness");
    assert!(
        run.status.success(),
        "{harness_c} failed: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.contains(expect_stdout),
        "{harness_c} stdout missing {expect_stdout:?}: {stdout}"
    );
}

#[test]
fn c_harness_matches_rust_golden() {
    assert_eq!(rhdl_cabi::rust_golden_data_out(), 3);
    compile_and_run("harness.c", "rhdl_cabi_harness", "ok rtl=3");
}

#[test]
fn c_harness_adder_proves_not_counter_only() {
    assert_eq!(rhdl_cabi::rust_golden_adder_sum(), 12);
    compile_and_run(
        "harness_adder.c",
        "rhdl_cabi_harness_adder",
        "ok adder rtl=12",
    );
}

#[test]
fn unknown_dut_fails_with_diagnosis() {
    let name = CString::new("NotADut").unwrap();
    let h = unsafe { rhdl_cabi::rhdl_sim_new_dut(name.as_ptr()) };
    assert!(h.is_null(), "unknown DUT must not silently succeed");
    let err = rhdl_cabi::rhdl_last_error();
    assert!(!err.is_null());
    let msg = unsafe { std::ffi::CStr::from_ptr(err) }.to_str().unwrap();
    assert!(msg.contains("unknown DUT"), "{msg}");
    assert!(msg.contains("NotADut"), "{msg}");
}

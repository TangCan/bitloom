//! Story 130.3 verified lock-to-binding acceptance tests.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf, process::Command};

fn paths() -> (PathBuf, PathBuf, PathBuf) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cache = std::env::var_os("BITLOOM_EXTERNAL_IP_CACHE")
        .map(PathBuf::from)
        .expect("dedicated gate requires BITLOOM_EXTERNAL_IP_CACHE");
    (
        root.clone(),
        std::env::var_os("BITLOOM_EXTERNAL_IP_MANIFEST")
            .map(PathBuf::from)
            .expect("dedicated gate requires BITLOOM_EXTERNAL_IP_MANIFEST"),
        cache,
    )
}

fn run_binding(
    manifest: &std::path::Path,
    lock: &std::path::Path,
    cache: &std::path::Path,
    out: &std::path::Path,
) -> std::process::Output {
    Command::new("timeout")
        .arg("180")
        .arg(env!("CARGO_BIN_EXE_cargo-bitloom"))
        .args(["external-ip", "binding", "--manifest"])
        .arg(manifest)
        .arg("--lock")
        .arg(lock)
        .arg("--cache")
        .arg(cache)
        .arg("--out")
        .arg(out)
        .output()
        .expect("run binding command")
}

#[test]
#[ignore = "explicit provisioned real-tool gate: just fr200-external-ip-pilot-check"]
fn binding_matches_locked_fifo_module_and_shape() {
    let (_root, manifest, cache) = paths();
    let lock = PathBuf::from(
        std::env::var_os("BITLOOM_EXTERNAL_IP_LOCK")
            .expect("dedicated gate requires BITLOOM_EXTERNAL_IP_LOCK"),
    );
    let out = std::env::temp_dir().join(format!("bitloom-binding-{}.json", std::process::id()));
    let result = run_binding(&manifest, &lock, &cache, &out);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let binding: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    let locked: Value = serde_json::from_slice(&fs::read(&lock).unwrap()).unwrap();
    let intent: Value = serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    let source = locked["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "common_cells")
        .unwrap();
    assert_eq!(binding["schemaVersion"], 1);
    assert_eq!(binding["module"], locked["compile"]["top"]);
    assert_eq!(binding["parameters"], locked["compile"]["parameters"]);
    assert_eq!(binding["ports"], locked["compile"]["ports"]);
    assert_eq!(binding["clockReset"], locked["compile"]["clockReset"]);
    assert_eq!(binding["upstream"]["name"], intent["name"]);
    for field in ["url", "ref", "commit"] {
        assert_eq!(binding["upstream"][field], source[field]);
    }
    assert_eq!(
        binding["upstream"]["sourcePath"],
        locked["compile"]["files"][0]
    );
    let rtl = source["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "src/fifo_v3.sv")
        .unwrap();
    assert_eq!(binding["upstream"]["sourceSha256"], rtl["sha256"]);
    assert_eq!(binding["adapter"]["name"], "bitloom-yosys-sv-compat");
    assert_eq!(binding["adapter"]["version"], 1);
    assert_eq!(binding["wrapper"]["name"], "BitloomExternalFifo");
    let verilog = binding["wrapper"]["verilog"].as_str().unwrap();
    assert!(!verilog.is_empty());
    assert_eq!(
        binding["wrapper"]["sha256"],
        format!("{:x}", Sha256::digest(verilog.as_bytes()))
    );
    assert_eq!(
        binding["wrapper"]["parameterBinding"],
        "fixed upstream defaults; no backend parameter overrides"
    );
    assert_eq!(binding["wrapper"]["version"], 1);
    assert_eq!(binding["supportLevel"], "locked");
    fs::remove_file(out).unwrap();
}

#[test]
#[ignore = "explicit provisioned real-tool gate: just fr200-external-ip-pilot-check"]
fn binding_rejects_source_parameter_port_and_reset_drift() {
    let (_root, manifest, cache) = paths();
    let lock = PathBuf::from(
        std::env::var_os("BITLOOM_EXTERNAL_IP_LOCK")
            .expect("dedicated gate requires BITLOOM_EXTERNAL_IP_LOCK"),
    );
    let original = fs::read(&manifest).unwrap();
    let dir = std::env::temp_dir().join(format!("bitloom-binding-mut-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let changed = dir.join("manifest.json");
    let out = dir.join("binding.json");
    for (from, to) in [
        ("\"DEPTH\": \"8\"", "\"DEPTH\": \"9\""),
        ("\"name\": \"pop_i\"", "\"name\": \"drift_i\""),
        (
            "\"resetPolarity\": \"active-low\"",
            "\"resetPolarity\": \"active-high\"",
        ),
    ] {
        let text = String::from_utf8(original.clone()).unwrap();
        fs::write(&changed, text.replacen(from, to, 1)).unwrap();
        let result = run_binding(&changed, &lock, &cache, &out);
        assert!(String::from_utf8_lossy(&result.stderr).contains("bitloom.external-ip."));
        assert!(
            !result.status.success(),
            "mutation unexpectedly passed: {from}"
        );
        assert!(!out.exists(), "failed validation wrote a binding");
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn binding_rejects_invalid_manifest_without_external_tools_or_cache() {
    let dir = std::env::temp_dir().join(format!("fr200-invalid-manifest-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let manifest = dir.join("manifest.json");
    fs::write(&manifest, b"{}").unwrap();
    let out = dir.join("binding.json");
    let result = run_binding(
        &manifest,
        &dir.join("absent-lock"),
        &dir.join("absent-cache"),
        &out,
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("bitloom.external-ip.schema"));
    assert!(!out.exists());
    fs::remove_dir_all(dir).unwrap();
}

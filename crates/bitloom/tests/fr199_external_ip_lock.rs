//! Story130.2 / FR199 source manifest, immutable lock, and fail-closed validation.
//! The online closure gate is explicit: `cargo test -p bitloom --test
//! fr199_external_ip_lock p0_online -- --ignored --nocapture`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bitloom-fr199-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create isolated fixture");
    dir
}

fn manifest(path: &Path) {
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../ip/external/pulp-common-cells-fifo-v3.source.json"),
        path,
    )
    .expect("copy canonical manifest");
}

fn run(args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-bitloom"));
    command
        .args(args)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default());
    for name in [
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "no_proxy",
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "NO_PROXY",
    ] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    command.output().expect("run cargo-bitloom")
}

fn text(output: &Output) -> String {
    format!(
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
#[ignore = "dedicated FR199 gate performs real upstream fetch; ordinary workspace tests must remain offline"]
fn p0_online_lock_verify_is_deterministic_and_mutations_fail_closed() {
    let dir = temp("lock");
    let manifest_path = dir.join("source.json");
    let lock = dir.join("source.lock.json");
    let cache = dir.join("cache");
    manifest(&manifest_path);

    let first = run(&[
        "external-ip",
        "lock",
        "--manifest",
        manifest_path.to_str().unwrap(),
        "--lock",
        lock.to_str().unwrap(),
        "--cache",
        cache.to_str().unwrap(),
    ]);
    assert!(first.status.success(), "{}", text(&first));

    let bytes = fs::read(&lock).expect("lock emitted");
    let body = String::from_utf8(bytes.clone()).expect("lock is UTF-8 JSON");
    assert!(body.contains("1281545696eb3fcba50ec5b4275993476a3c710e"));
    assert!(body.contains("lightweight") && body.contains("src/fifo_v3.sv"));
    assert!(body.contains("common_verification") && body.contains("tech_cells_generic"));
    assert_eq!(
        bytes,
        fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../ip/external/pulp-common-cells-fifo-v3.source.lock.json")
        )
        .unwrap(),
        "fresh resolution must reproduce the canonical deterministic lock"
    );

    let verify = |lock: &Path, cache: &Path| {
        run(&[
            "external-ip",
            "verify",
            "--manifest",
            manifest_path.to_str().unwrap(),
            "--lock",
            lock.to_str().unwrap(),
            "--cache",
            cache.to_str().unwrap(),
            "--offline",
        ])
    };
    let good = verify(&lock, &cache);
    assert!(good.status.success(), "{}", text(&good));

    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let source = &value["sources"][0];
    let source_root = cache.join(source["cachePath"].as_str().unwrap());
    let fifo = source_root.join("src/fifo_v3.sv");
    let fifo_bytes = fs::read(&fifo).unwrap();
    fs::write(&fifo, b"drift").unwrap();
    assert_failure(&verify(&lock, &cache), "content-drift");
    fs::write(&fifo, fifo_bytes.clone()).unwrap();

    fs::remove_file(&fifo).unwrap();
    assert_failure(&verify(&lock, &cache), "undeclared-file");
    fs::write(&fifo, fifo_bytes).unwrap();

    let license = dir.join("licenses/common_cells-LICENSE");
    let license_bytes = fs::read(&license).unwrap();
    fs::write(&license, b"license drift").unwrap();
    assert_failure(&verify(&lock, &cache), "license-drift");
    fs::write(&license, license_bytes).unwrap();

    let extra = source_root.join("undeclared.sv");
    fs::write(&extra, b"module undeclared; endmodule\n").unwrap();
    assert_failure(&verify(&lock, &cache), "undeclared-file");
    fs::remove_file(&extra).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let escape = source_root.join("escape");
        symlink("../../../../outside", &escape).unwrap();
        assert_failure(&verify(&lock, &cache), "symlink");
        fs::remove_file(&escape).unwrap();
        let hardlink = source_root.join("hardlink");
        fs::hard_link(&fifo, &hardlink).unwrap();
        assert_failure(&verify(&lock, &cache), "hardlink");
        fs::remove_file(&hardlink).unwrap();
    }

    for (pointer, replacement, expected) in [
        (
            "/sources/0/commit",
            serde_json::json!("0000000000000000000000000000000000000000"),
            "identity-drift",
        ),
        (
            "/tools/yosys",
            serde_json::json!("Yosys drift"),
            "tool-drift",
        ),
        (
            "/tools/adapterVersion",
            serde_json::json!(999),
            "tool-drift",
        ),
        (
            "/tools/bitloomSha256",
            serde_json::json!("00".repeat(32)),
            "tool-drift",
        ),
        (
            "/tools/bitloomPath",
            serde_json::json!("/tmp/not-the-locked-bitloom"),
            "tool-drift",
        ),
        (
            "/generator/kind",
            serde_json::json!("unexpected generator"),
            "identity-drift",
        ),
        (
            "/generator/version",
            serde_json::json!("1"),
            "identity-drift",
        ),
        (
            "/generator/inputs",
            serde_json::json!(["generated/input.json"]),
            "identity-drift",
        ),
        ("/dependencies", serde_json::json!([]), "identity-drift"),
        (
            "/compile/includeDirs",
            serde_json::json!([]),
            "identity-drift",
        ),
        ("/compile/defines", serde_json::json!([]), "identity-drift"),
        (
            "/sources/0/license/noticePaths",
            serde_json::json!(["NOTICE"]),
            "license-drift",
        ),
    ] {
        let mut mutation = value.clone();
        *mutation.pointer_mut(pointer).unwrap() = replacement;
        fs::write(&lock, serde_json::to_vec_pretty(&mutation).unwrap()).unwrap();
        assert_failure(&verify(&lock, &cache), expected);
        fs::write(&lock, &bytes).unwrap();
    }
    let mut reordered = value.clone();
    reordered["sources"][0]["files"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    fs::write(&lock, serde_json::to_vec_pretty(&reordered).unwrap()).unwrap();
    assert_failure(&verify(&lock, &cache), "order");
    fs::write(&lock, &bytes).unwrap();
    assert_eq!(bytes, fs::read(&lock).unwrap(), "verify must not relock");
}

fn assert_failure(output: &Output, expected: &str) {
    assert!(!output.status.success(), "mutation must fail");
    assert!(text(output).contains(expected), "{}", text(output));
}

#[test]
fn p0_floating_ref_fails_closed() {
    let dir = temp("floating");
    let manifest_path = dir.join("source.json");
    let lock = dir.join("source.lock.json");
    manifest(&manifest_path);
    let body = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace("v1.40.0", "master");
    fs::write(&manifest_path, body).unwrap();

    let floating = run(&[
        "external-ip",
        "lock",
        "--manifest",
        manifest_path.to_str().unwrap(),
        "--lock",
        lock.to_str().unwrap(),
        "--cache",
        dir.join("cache").to_str().unwrap(),
    ]);
    assert!(!floating.status.success(), "floating ref must fail");
    let diagnostic = text(&floating);
    assert!(
        diagnostic.contains("floating") || diagnostic.contains("immutable"),
        "{diagnostic}"
    );
}

#[test]
fn p0_verify_requires_explicit_offline_flag() {
    let dir = temp("explicit-offline");
    let manifest_path = dir.join("source.json");
    manifest(&manifest_path);
    let output = run(&[
        "external-ip",
        "verify",
        "--manifest",
        manifest_path.to_str().unwrap(),
        "--lock",
        dir.join("source.lock.json").to_str().unwrap(),
        "--cache",
        dir.join("cache").to_str().unwrap(),
    ]);
    assert_failure(&output, "requires explicit --offline");
}

#[test]
fn p1_cli_exposes_separate_read_only_verify_command() {
    let help = run(&["external-ip", "--help"]);
    assert!(help.status.success(), "{}", text(&help));
    let stdout = String::from_utf8_lossy(&help.stdout);
    assert!(stdout.contains("lock") && stdout.contains("verify") && stdout.contains("replay"));
}

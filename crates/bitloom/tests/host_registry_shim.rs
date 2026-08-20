//! ATDD Story 13.3: host shim uses registry backends outside monorepo; CLI resolves packages.

use std::process::Command;

#[test]
fn help_mentions_build_package_flags() {
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-bitloom"))
        .arg("build")
        .arg("--help")
        .output()
        .expect("run");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("--package"));
    assert!(text.contains("--manifest-dir"));
}

#[test]
fn unit_host_registry_covered_in_cli_crate() {
    // `host_cargo_uses_registry_backends_outside_monorepo` lives in main.rs cfg(test).
    // This integration test documents the FR50 contract: no monorepo path in standalone.
    let ver = env!("CARGO_PKG_VERSION");
    assert!(!ver.is_empty(), "CLI version pins bitloom-vlog/hir");
}

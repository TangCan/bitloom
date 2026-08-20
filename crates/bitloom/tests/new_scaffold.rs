//! ATDD Story 13.4: `cargo bitloom new` scaffolds prelude-only design crate.

use std::fs;
use std::process::Command;

#[test]
fn help_mentions_new_subcommand() {
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-bitloom"))
        .arg("new")
        .arg("--help")
        .output()
        .expect("run");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.to_lowercase().contains("bitloom-prelude")
            || text.contains("Scaffold")
            || text.contains("scaffold"),
        "help should describe scaffolding: {text}"
    );
}

#[test]
fn new_scaffolds_prelude_only_crate() {
    let tmp = tempfile_dir();
    let name = "blink_demo";
    let out = Command::new(env!("CARGO_BIN_EXE_cargo-bitloom"))
        .arg("new")
        .arg(name)
        .arg("--path")
        .arg(&tmp)
        .output()
        .expect("run new");
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let crate_dir = tmp.join(name);
    let cargo = fs::read_to_string(crate_dir.join("Cargo.toml")).expect("Cargo.toml");
    assert!(
        cargo.contains("bitloom-prelude"),
        "must depend on bitloom-prelude:\n{cargo}"
    );
    assert!(
        !cargo.contains("bitloom =") && !cargo.contains("name = \"bitloom\""),
        "must not depend on CLI package bitloom:\n{cargo}"
    );
    let lib = fs::read_to_string(crate_dir.join("src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("#[module]") || lib.contains("module"),
        "need module macro"
    );
    assert!(
        lib.contains("rhdl_elaborate"),
        "need elaborate entry:\n{lib}"
    );
    let _ = fs::remove_dir_all(&tmp);
}

fn tempfile_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "bitloom-13-4-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("tmpdir");
    dir
}

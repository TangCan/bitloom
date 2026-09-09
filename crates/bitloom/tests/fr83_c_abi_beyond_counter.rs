//! ATDD — Story 35.2 / FR83: C ABI beyond Counter-only.
//! Red until `rhdl_sim_new_dut`, documented Adder DUT, last_error, and docs land.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr83_docs_cover_symbols_lifetime_errors_and_second_dut() {
    let path = workspace_root().join("docs/fr33-c-abi.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("rhdl_sim_new_dut") && text.contains("rhdl_last_error"),
        "docs must document rhdl_sim_new_dut and rhdl_last_error"
    );
    assert!(
        text.contains("Adder")
            && (text.contains("FR83") || text.contains("second DUT") || text.contains("第二")),
        "docs must document Adder as second DUT / FR83"
    );
    assert!(
        text.contains("rhdl_sim_free")
            || text.contains("lifetime")
            || text.contains("生命周期")
            || text.contains("Lifetime"),
        "docs must describe handle lifetime (new/free)"
    );
    assert!(
        text.contains("error") || text.contains("错误") || text.contains("Error"),
        "docs must describe error modes"
    );
}

#[test]
fn fr83_header_exports_new_dut_and_last_error() {
    let path = workspace_root().join("crates/rhdl-cabi/include/rhdl_cabi.h");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(
        text.contains("rhdl_sim_new_dut"),
        "header must declare rhdl_sim_new_dut"
    );
    assert!(
        text.contains("rhdl_last_error"),
        "header must declare rhdl_last_error"
    );
}

#[test]
fn fr83_source_is_not_counter_only() {
    let path = workspace_root().join("crates/rhdl-cabi/src/lib.rs");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(
        text.contains("fn counter_hir") && text.contains("fn adder_hir"),
        "lib.rs must define both counter_hir and adder_hir"
    );
    assert!(
        text.contains("rhdl_sim_new_dut") && text.contains("rhdl_last_error"),
        "lib.rs must export rhdl_sim_new_dut and rhdl_last_error"
    );
    assert!(
        text.contains("\"Adder\"") || text.contains("DutKind::Adder") || text.contains("Adder"),
        "lib.rs must name Adder as a selectable DUT"
    );
}

#[test]
fn fr83_adder_harness_fixture_exists() {
    let path = workspace_root().join("crates/rhdl-cabi/tests/harness_adder.c");
    assert!(
        path.is_file(),
        "missing non-Counter C harness at {}",
        path.display()
    );
    let text = fs::read_to_string(&path).unwrap();
    assert!(
        text.contains("Adder") || text.contains("rhdl_sim_new_dut"),
        "adder harness must select Adder via rhdl_sim_new_dut"
    );
    assert!(
        !text.contains("expected=3") && !text.contains("rtl=3"),
        "adder harness must not use Counter golden 3"
    );
}

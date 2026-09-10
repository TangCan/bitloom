//! ATDD — Story 49.2 / FR107: SystemC TLM-2.0 AT product path.
//!
//! ```text
//! cargo test -p bitloom --test fr107_systemc_tlm_at
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::FrozenHir;
use bitloom_sim::{
    SYSTEMC_PIN_VERSION, build_and_run_tlm_at_smoke, emit_systemc_tlm_at, resolve_systemc,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn counter_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("Fr107Counter");
    s.begin_module("Fr107Counter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("Fr107Counter elaborate")
}

#[test]
fn fr107_docs_at_ad5_not_lt_not_rust_fl() {
    let text = read("docs/fr107-systemc-tlm-at.md");
    assert!(text.contains("FR107"));
    assert!(text.contains("Bitloom"));
    assert!(text.contains("nb_transport_fw") || text.contains("nb_transport"));
    assert!(text.contains("AD-5"));
    assert!(text.contains("FR101") && (text.contains("LT") || text.contains("并行")));
    assert!(
        text.contains("FR47")
            && (text.contains("≠") || text.contains("Not") || text.contains("not"))
    );
    assert!(text.contains("2.3.3") || text.contains(SYSTEMC_PIN_VERSION));
    assert!(text.contains("gen-tlm-at") || text.contains("emit_systemc_tlm_at"));
}

#[test]
fn fr107_ad5_allows_at() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(spine.contains("FR107") && spine.contains("nb_transport"));
    assert!(spine.contains("2026-09-10") || spine.contains("Phase 13"));
    assert!(spine.contains("NFR46"));
}

#[test]
fn fr107_emit_contains_nb_transport_fw() {
    let hir = counter_hir();
    let dir = std::env::temp_dir().join(format!("fr107-emit-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let out = emit_systemc_tlm_at(&hir, &dir).expect("emit");
    let hpp = fs::read_to_string(out.join("bitloom_tlm_at.hpp")).expect("hpp");
    let cpp = fs::read_to_string(out.join("bitloom_tlm_at.cpp")).expect("cpp");
    assert!(hpp.contains("nb_transport_fw"));
    assert!(cpp.contains("register_nb_transport_fw"));
    assert!(cpp.contains("nb_transport_fw"));
    assert!(!hpp.contains("register_b_transport"));
    assert!(!cpp.contains("register_b_transport"));
    assert!(
        fs::read_to_string(out.join("Makefile"))
            .unwrap()
            .contains("systemc")
    );
}

#[test]
fn fr107_smoke_when_systemc_present() {
    if resolve_systemc().is_err() {
        eprintln!("skip: SystemC not installed");
        return;
    }
    let hir = counter_hir();
    let dir = std::env::temp_dir().join(format!("fr107-smoke-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    emit_systemc_tlm_at(&hir, &dir).expect("emit");
    let out = build_and_run_tlm_at_smoke(&dir).expect("AT smoke");
    assert!(out.contains("BITLOOM_TLM_AT_OK"));
}

#[test]
fn fr107_cli_mentions_gen_tlm_at() {
    let main_rs = read("crates/bitloom/src/main.rs");
    assert!(main_rs.contains("GenTlmAt") || main_rs.contains("gen-tlm-at"));
    assert!(main_rs.contains("emit_systemc_tlm_at"));
}

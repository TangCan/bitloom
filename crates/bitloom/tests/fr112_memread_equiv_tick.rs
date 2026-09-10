//! ATDD: FR112 branch B — GeneratedFunctional MemRead ≡ Sim::tick (Story 54.2).
//!
//! cargo test -p bitloom --test fr112_memread_equiv_tick

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{
    AbstractionView, EquivStatus, FormalEquivProduct, check_generated_bridge,
    check_generated_bridge_with,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn sync_read_mem_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("fr112");
    s.begin_module("Srm", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("addr", GroundType::UInt { width: 4 }, Span::default());
    s.add_input("wdata", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("we", GroundType::Bool, Span::default());
    s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());
    s.declare_sync_read_mem("ram", 16, 8, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("rdata", "q", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_mem_write("ram", "addr", "wdata", Span::default());
    s.assign_reg_d_mem_read("q", "ram", "addr", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("SyncReadMem fixture")
}

fn write_then_read_stimuli() -> Vec<PortValues> {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("addr", 3);
    pv.set("wdata", 0xAB);
    pv.set("we", 1);
    vec![pv.clone(), pv]
}

#[test]
fn fr112_docs_name_branch_b_and_bans() {
    let text = read("docs/fr112-generated-functional-memread-equiv.md");
    assert!(text.contains("FR112") && text.contains("Bitloom"));
    assert!(
        text.contains("GeneratedFunctional") && text.contains("MemRead"),
        "must name GeneratedFunctional MemRead face"
    );
    assert!(
        text.contains("deferred")
            && (text.contains("SymbiYosys") || text.contains("F1-(ii)"))
            && (text.contains("handwritten") || text.contains("(C)")),
        "must keep A/C deferred"
    );
    assert!(
        text.contains("FR92") || text.contains("F1-(i)") || text.contains("alone"),
        "must record forbidden alone closes"
    );
    assert!(
        text.contains("fr100") || text.contains("FR100"),
        "must cross-link FR100"
    );
    assert!(
        text.contains("fr103") || text.contains("FR103"),
        "must cross-link FR103"
    );
}

#[test]
fn fr112_generated_functional_bridge_pass() {
    let status = check_generated_bridge(sync_read_mem_hir(), write_then_read_stimuli());
    assert!(
        status.is_pass(),
        "GeneratedFunctional must match tick on SyncReadMem fixture: {status:?}"
    );
}

/// Stub that never delivers latency-1 MemRead (always rdata=0) — must Fail, not silent Ok.
struct ZeroRdataFl;

impl AbstractionView for ZeroRdataFl {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let mut out = inputs.clone();
        out.set("rdata", 0);
        out
    }
}

#[test]
fn fr112_memread_stub_fails_readable() {
    let status = check_generated_bridge_with(
        sync_read_mem_hir(),
        &mut ZeroRdataFl,
        write_then_read_stimuli(),
    );
    match status {
        EquivStatus::Fail { mismatches, .. } => {
            assert!(
                !mismatches.is_empty(),
                "Fail must list PortMismatch diagnostics"
            );
            assert!(
                mismatches.iter().any(|m| m.name == "rdata"),
                "mismatch should mention rdata: {mismatches:?}"
            );
        }
        other => panic!("MemRead stub must Fail, got {other:?}"),
    }
}

#[test]
fn fr112_fr100_bounded_exhaustive_still_green() {
    // Non-mem counter — F1-(i) MVP must remain valid (NFR44).
    let mut s = ElaborateSession::new("fr112_fr100");
    s.begin_module("Cnt", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    let hir = s.finish().expect("counter");
    let product = FormalEquivProduct::new(0, 0)
        .with_boolean_ports(&["rst"])
        .with_exhaustive_depth(2);
    assert!(
        product.check_bounded_exhaustive(hir).is_pass(),
        "FR100 F1-(i) must stay green"
    );
}

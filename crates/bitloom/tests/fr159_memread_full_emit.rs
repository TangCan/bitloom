//! ATDD: FR159 — emitted functional-sim crate MemRead full generate (Story 91.2).
//!
//! cargo test -p bitloom --test fr159_memread_full_emit

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::FrozenHir;
use bitloom_sim::generate_functional_sim;

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
    let mut s = ElaborateSession::new("fr159");
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

#[test]
fn fr159_docs_name_emit_and_forbid_stub_alone() {
    let text = read("docs/fr159-memread-full-emit.md");
    assert!(text.contains("FR159") && text.contains("Bitloom"));
    assert!(
        text.contains("generate_functional_sim")
            && (text.contains("MemRead") || text.contains("SyncReadMem")),
        "must name emit / MemRead face"
    );
    assert!(
        text.contains("FR112")
            && (text.contains("alone") || text.contains("≠") || text.contains("in-process")),
        "must keep FR112 distinct / alone ban"
    );
    assert!(
        text.contains("stub") || text.contains("0"),
        "must forbid stub-alone close"
    );
    let fr112 = read("docs/fr112-generated-functional-memread-equiv.md");
    assert!(
        fr112.contains("fr159") || fr112.contains("FR159"),
        "FR112 must cross-link FR159 (emitted stub superseded)"
    );
    assert!(
        !fr112.contains("may still stub `MemRead` as `0`"),
        "FR112 must not still claim emitted stub after FR159"
    );
}

#[test]
fn fr159_emitted_crate_sync_read_mem_matches_latency1() {
    let hir = sync_read_mem_hir();
    let out = workspace_root().join("target/bitloom-fr159-func-mem");
    let _ = fs::remove_dir_all(&out);
    generate_functional_sim(&hir, &out).expect("generate_functional_sim");
    let lib = fs::read_to_string(out.join("src/lib.rs")).expect("lib.rs");
    assert!(
        lib.contains("eval_mem_read") && lib.contains("pending_mem_reads"),
        "emitted crate must implement MemRead / SyncReadMem latency-1"
    );
    assert!(
        !lib.contains("AssignExpr::MemRead"),
        "emit is source text, not HIR match arms"
    );
    // Pre-FR159 stub was literal 0 for MemRead; sync path must queue pending.
    assert!(
        lib.contains("next_pending"),
        "SyncReadMem must use pending queue, not constant 0 assign"
    );

    let status = Command::new("cargo")
        .arg("+1.97.1")
        .arg("test")
        .arg("--manifest-path")
        .arg(out.join("Cargo.toml"))
        .arg("--quiet")
        .status()
        .expect("spawn cargo test on emitted crate");
    assert!(
        status.success(),
        "FR159 emitted SyncReadMem functional-sim crate cargo test failed"
    );
}

#[test]
fn fr159_fr112_in_process_still_green() {
    use bitloom_hir::PortValues;
    use bitloom_sim::check_generated_bridge;

    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("addr", 3);
    pv.set("wdata", 0xAB);
    pv.set("we", 1);
    let status = check_generated_bridge(sync_read_mem_hir(), vec![pv.clone(), pv]);
    assert!(
        status.is_pass(),
        "FR112 in-process MemRead≡tick must remain green: {status:?}"
    );
}

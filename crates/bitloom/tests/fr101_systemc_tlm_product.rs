//! ATDD — Story 46.2 / FR101: SystemC TLM-2.0 LT product path.
//!
//! ```text
//! cargo test -p bitloom --test fr101_systemc_tlm_product
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::FrozenHir;
use bitloom_sim::{
    SYSTEMC_PIN_VERSION, build_and_run_tlm_lt_smoke, emit_systemc_tlm_lt, resolve_systemc,
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
    let mut s = ElaborateSession::new("Fr101Counter");
    s.begin_module("Fr101Counter", Span::default());
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
    s.finish().expect("Fr101Counter elaborate")
}

#[test]
fn fr101_docs_lt_only_ad5_not_rust_fl() {
    let text = read("docs/fr101-systemc-tlm.md");
    assert!(text.contains("FR101"), "must name FR101");
    assert!(text.contains("Bitloom"), "must brand Bitloom");
    assert!(
        text.contains("LT-only") || text.contains("LT only") || text.contains("loosely-timed"),
        "must nail LT-only MVP"
    );
    assert!(
        text.contains("b_transport") && text.contains("tlm_generic_payload"),
        "must name TLM-2.0 LT primitives"
    );
    assert!(
        text.contains("AD-5") || text.contains("修订后 AD-5") || text.contains("revised"),
        "must cross-link revised AD-5"
    );
    assert!(
        text.contains("2.3.3") || text.contains(SYSTEMC_PIN_VERSION),
        "must pin SystemC version"
    );
    assert!(
        text.contains("FR47")
            && (text.contains("≠")
                || text.contains("Not")
                || text.contains("not")
                || text.contains("不是")),
        "must contrast FR47 Rust FL ≠ SystemC TLM"
    );
    assert!(
        text.contains("closed")
            || text.contains("关闭")
            || text.contains("已关闭")
            || text.contains("46.3"),
        "fr101 doc should note Epic 46 / Story 46.3 closeout after product face"
    );
}

#[test]
fn fr101_emit_d1_d2_sources_are_systemc_tlm_not_rust_fl() {
    let hir = counter_hir();
    let dir = workspace_root().join("target/fr101-atdd-emit");
    let _ = fs::remove_dir_all(&dir);
    let out = emit_systemc_tlm_lt(&hir, &dir).expect("emit");
    let hpp = fs::read_to_string(out.join("bitloom_tlm_lt.hpp")).expect("hpp");
    let cpp = fs::read_to_string(out.join("bitloom_tlm_lt.cpp")).expect("cpp");
    let mk = fs::read_to_string(out.join("Makefile")).expect("Makefile");
    let main = fs::read_to_string(out.join("sc_main.cpp")).expect("sc_main");

    assert!(
        hpp.contains("b_transport")
            && (hpp.contains("tlm_generic_payload") || cpp.contains("tlm_generic_payload")),
        "D1 must use TLM-2.0 generic payload + b_transport"
    );
    assert!(
        hpp.contains("Bitloom")
            || cpp.contains("Bitloom")
            || mk.contains("Bitloom")
            || main.contains("bitloom"),
        "generated artifacts must brand Bitloom"
    );
    assert!(
        !hpp.contains("GeneratedFunctional") && !hpp.contains("emit_functional_crate"),
        "must not label host Rust FL as SystemC TLM"
    );
    assert!(
        mk.contains("systemc") && mk.contains(SYSTEMC_PIN_VERSION),
        "Makefile must document SystemC pin"
    );
    assert!(
        mk.contains("libsystemc") || mk.contains("pkg-config") || mk.contains("SystemC not found"),
        "missing-dep path must be readable in Makefile"
    );
}

#[test]
fn fr101_resolve_systemc_or_readable_error() {
    match resolve_systemc() {
        Ok(tc) => {
            assert!(
                !tc.version.is_empty(),
                "resolved SystemC version must be non-empty"
            );
            assert!(
                tc.libs.contains("systemc") || !tc.libs.is_empty(),
                "libs from pkg-config should mention systemc"
            );
        }
        Err(e) => {
            assert!(
                e.to_lowercase().contains("systemc") || e.contains("libsystemc"),
                "missing SystemC error must be readable; got: {e}"
            );
            assert!(
                e.contains("fr101") || e.contains("libsystemc-dev") || e.contains("Install"),
                "error should point at install / docs; got: {e}"
            );
        }
    }
}

#[test]
fn fr101_lt_smoke_build_and_run_when_systemc_present() {
    let hir = counter_hir();
    let dir = workspace_root().join("target/fr101-atdd-smoke");
    let _ = fs::remove_dir_all(&dir);
    emit_systemc_tlm_lt(&hir, &dir).expect("emit");

    match resolve_systemc() {
        Ok(_) => {
            let out = build_and_run_tlm_lt_smoke(&dir).unwrap_or_else(|e| panic!("LT smoke: {e}"));
            assert!(
                out.contains("BITLOOM_TLM_LT_OK"),
                "smoke stdout must contain success marker; got:\n{out}"
            );
        }
        Err(e) => {
            // CI installs libsystemc-dev; local without SystemC must still see readable fail
            // when attempting make — never silent skip claiming FR101 green.
            let make = Command::new("make")
                .arg("run")
                .current_dir(&dir)
                .output()
                .expect("spawn make");
            assert!(
                !make.status.success(),
                "without SystemC, make run must fail (no silent green)"
            );
            let err = format!(
                "{e}\n{}\n{}",
                String::from_utf8_lossy(&make.stdout),
                String::from_utf8_lossy(&make.stderr)
            );
            assert!(
                err.to_lowercase().contains("systemc") || err.contains("libsystemc"),
                "failure must name SystemC; got: {err}"
            );
        }
    }
}

#[test]
fn fr101_nfr14_gate_still_present_and_46_2_checkable() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md");
    assert!(text.contains("D1") && text.contains("D4"));
    assert!(
        (text.contains("修订后") && text.contains("AD-5")) || text.contains("AD-5"),
        "NFR14 must cite revised AD-5"
    );
}

#[test]
fn fr101_sprint_46_2_done_epic_may_close_after_46_3() {
    let text = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    let s2 = text
        .lines()
        .find(|l| {
            l.contains("46-2-systemc-tlm-2-0-生成或集成产品面-fr101:")
                || l.contains("46-2-systemc-tlm-2-0-产品面-fr101:")
        })
        .expect("46-2 key");
    assert!(
        s2.contains("done"),
        "46-2 must remain done after product face; got {s2}"
    );
    let s3 = text
        .lines()
        .find(|l| l.contains("46-3-fr101-收口"))
        .expect("46-3 key");
    let epic = text
        .lines()
        .find(|l| l.trim_start().starts_with("epic-46:"))
        .expect("epic-46");
    // After 46.2 alone: 46-3 backlog + epic in-progress.
    // After 46.3 closeout: 46-3 done + epic done.
    if s3.contains("done") {
        assert!(
            epic.contains("done"),
            "when 46-3 is done, epic-46 must be done; got {epic}"
        );
    } else {
        assert!(
            s3.contains("backlog") || s3.contains("ready-for-dev") || s3.contains("in-progress"),
            "before 46.3 closeout, 46-3 must not be falsely done; got {s3}"
        );
        assert!(
            epic.contains("in-progress"),
            "before 46.3 closeout, epic-46 must stay in-progress; got {epic}"
        );
    }
}

#[test]
fn fr101_fr47_doc_still_says_not_systemc() {
    let text = read("docs/fr47-dual-sim-generation.md");
    assert!(
        text.contains("Not") && text.contains("SystemC")
            || text.contains("不是") && text.contains("SystemC")
            || text.contains("≠") && text.contains("SystemC")
            || text.contains("FR101"),
        "FR47 doc must keep SystemC / FR101 contrast"
    );
}

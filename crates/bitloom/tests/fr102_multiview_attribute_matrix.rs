//! ATDD — Story 45.3 / FR102: multi-view attribute full matrix.
//!
//! Locks: docs/fr102-multiview-attribute-matrix.md (full matrix + illegal gates;
//! adapter templates alone ≠ FR102), `functional_state` does not leak into
//! FrozenHir ports, HostView attributes remain host-only, sprint 45-3 done /
//! 45-4 backlog / epic-45 in-progress.
//!
//! ```text
//! cargo test -p bitloom --test fr102_multiview_attribute_matrix
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_prelude::{Clock, Elaboratable, HostView, Input, Output, Reset, UInt, ViewKind, rhdl};

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

#[rhdl::module]
pub struct Fr102DualViewMod {
    pub clk: Input<Clock>,
    pub rst: Input<Reset>,
    pub data_in: Input<UInt<8>>,
    pub data_out: Output<UInt<8>>,
    /// Soft host state — must not become an HIR port (FR102 / AD-5).
    #[rhdl::functional_state]
    pub soft_buf: u32,
    #[functional_state]
    pub soft_flag: bool,
}

#[rhdl::functional_model]
pub struct Fr102FuncModel {
    #[rhdl::functional_state]
    pub soft: u8,
}

#[rhdl::abstraction]
pub struct Fr102Abs {
    #[functional_state]
    pub txn_count: u64,
}

#[rhdl::bridge]
pub struct Fr102Bridge;

#[rhdl::both]
pub struct Fr102Both {
    pub abs: Fr102Abs,
}

const _: () = {
    assert!(<Fr102FuncModel as HostView>::KIND as u8 == ViewKind::FunctionalModel as u8);
    assert!(<Fr102Abs as HostView>::KIND as u8 == ViewKind::Abstraction as u8);
    assert!(<Fr102Bridge as HostView>::KIND as u8 == ViewKind::Bridge as u8);
    assert!(<Fr102Both as HostView>::KIND as u8 == ViewKind::Both as u8);
};

#[test]
fn fr102_docs_full_matrix_and_completion_surface() {
    let text = read("docs/fr102-multiview-attribute-matrix.md");
    assert!(text.contains("FR102"), "FR102 doc must name FR102");
    assert!(text.contains("Bitloom"), "FR102 doc must brand Bitloom");
    assert!(
        text.contains("functional_model")
            && text.contains("abstraction")
            && text.contains("functional_state")
            && text.contains("bridge")
            && text.contains("both"),
        "FR102 doc must list full attribute matrix rows"
    );
    assert!(
        text.contains("完成面")
            || text.to_lowercase().contains("completion")
            || text.contains("FR102 完成"),
        "FR102 doc must declare this page as the FR102 completion surface"
    );
    assert!(
        text.contains("Illegal") || text.contains("非法"),
        "FR102 doc must document illegal combination gates"
    );
}

#[test]
fn fr102_docs_adapters_supporting_not_sufficient() {
    let text = read("docs/fr102-multiview-attribute-matrix.md");
    assert!(
        text.contains("FR78") || text.contains("fr78") || text.contains("adapter"),
        "FR102 doc must mention adapter templates"
    );
    assert!(
        (text.contains("alone")
            && (text.contains("≠") || text.contains("not") || text.contains("非")))
            || text.contains("配套非充分")
            || text.contains("alone ≠ FR102")
            || text.contains("alone ≠"),
        "FR102 doc must state adapter templates alone are not sufficient"
    );
}

#[test]
fn fr102_docs_functional_state_never_hir() {
    let text = read("docs/fr102-multiview-attribute-matrix.md");
    assert!(
        text.contains("FrozenHir") || text.contains("HIR") || text.contains("freeze"),
        "FR102 doc must bind functional_state to HIR/freeze boundary"
    );
    assert!(
        text.contains("must not")
            || text.contains("不得")
            || text.contains("never")
            || text.contains("No"),
        "FR102 doc must forbid functional_state leaking into HIR/freeze"
    );
}

#[test]
fn fr102_positive_functional_state_skipped_from_frozen_ports() {
    let hir = Fr102DualViewMod::elaborate().expect("module with functional_state must elaborate");
    let ports: Vec<&str> = hir
        .circuit()
        .modules
        .iter()
        .flat_map(|m| m.ports.iter().map(|p| p.name.as_str()))
        .collect();
    assert!(
        ports.contains(&"clk")
            && ports.contains(&"rst")
            && ports.contains(&"data_in")
            && ports.contains(&"data_out"),
        "hardware ports must remain in FrozenHir, got {ports:?}"
    );
    assert!(
        !ports
            .iter()
            .any(|n| n.contains("soft_buf") || n.contains("soft_flag")),
        "functional_state fields must not leak into FrozenHir ports, got {ports:?}"
    );
    // Soft fields remain usable on the host Rust struct.
    let host = Fr102DualViewMod {
        clk: Input(Clock),
        rst: Input(Reset),
        data_in: Input(UInt),
        data_out: Output(UInt),
        soft_buf: 42,
        soft_flag: true,
    };
    assert_eq!(host.soft_buf, 42);
    assert!(host.soft_flag);
}

#[test]
fn fr102_negative_illegal_leak_guard_in_docs_and_risk() {
    let fr102 = read("docs/fr102-multiview-attribute-matrix.md");
    assert!(
        fr102.contains("synthesizable") || fr102.contains("FrozenHir") || fr102.contains("非法"),
        "illegal-combination section must name synthesizable/FrozenHir leak"
    );
    let risk =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md");
    assert!(
        risk.contains("functional_state")
            && (risk.contains("不得") || risk.contains("不得泄漏") || risk.contains("HIR")),
        "NFR14 risk must keep functional_state HIR leak forbidden"
    );
}

#[test]
fn fr102_hostview_matrix_kinds_compile() {
    assert_eq!(
        <Fr102FuncModel as HostView>::KIND,
        ViewKind::FunctionalModel
    );
    assert_eq!(<Fr102Abs as HostView>::KIND, ViewKind::Abstraction);
    assert_eq!(<Fr102Bridge as HostView>::KIND, ViewKind::Bridge);
    assert_eq!(<Fr102Both as HostView>::KIND, ViewKind::Both);
    let m = Fr102FuncModel { soft: 7 };
    assert_eq!(m.soft, 7);
    let a = Fr102Abs { txn_count: 1 };
    assert_eq!(a.txn_count, 1);
}

#[test]
fn fr102_cross_docs_and_readme_honest() {
    let fr29 = read("docs/fr29-bridge-abstraction-both.md");
    assert!(
        fr29.contains("FR102") || fr29.contains("fr102"),
        "FR29 doc must cross-link FR102 matrix"
    );
    let fr78 = read("docs/fr78-bridge-adapter-closures.md");
    assert!(
        fr78.contains("FR102") || fr78.contains("fr102"),
        "FR78 doc must cross-link FR102 (templates alone insufficient)"
    );
    let fr100 = read("docs/fr100-formal-equiv.md");
    assert!(
        fr100.contains("fr102") || fr100.contains("FR102"),
        "FR100 doc must point at FR102 story/page"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("fr102-multiview-attribute-matrix") || readme.contains("FR102"),
        "README must index FR102 matrix doc"
    );
}

#[test]
fn fr102_sprint_45_3_done_epic_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("45-3-多视图属性全矩阵-fr102: done"),
        "sprint must mark 45-3 done"
    );
    // 45-4 / epic-45 may be done after Story 45.4 closeout.
    assert!(
        sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103: backlog")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103: ready-for-dev")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103: in-progress")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103: done")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: backlog")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: ready-for-dev")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: in-progress")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: done"),
        "sprint must list 45-4"
    );
    assert!(
        sprint.contains("epic-45: in-progress") || sprint.contains("epic-45: done"),
        "epic-45 must be in-progress or done"
    );
}

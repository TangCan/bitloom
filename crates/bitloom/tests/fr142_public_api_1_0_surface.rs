//! ATDD / guardrail: Story 80.2 / FR142 — public API 1.0 surface document.
//!
//! ```text
//! cargo test -p bitloom --test fr142_public_api_1_0_surface
//! ```

use std::fs;
use std::path::PathBuf;

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

#[test]
fn fr142_surface_doc_exists_with_required_sections() {
    let text = read("docs/public-api-1-0-surface.md");
    assert!(
        text.contains("In-surface") || text.contains("in-surface"),
        "must have in-surface section"
    );
    assert!(
        text.contains("Out-of-promise")
            || text.contains("out-of-promise")
            || text.contains("非承诺"),
        "must have out-of-promise section"
    );
    assert!(
        text.contains("Out-of-surface")
            || text.contains("out-of-surface")
            || text.contains("out of surface"),
        "must have out-of-surface section"
    );
    assert!(text.contains("FR142"), "must cite FR142");
}

#[test]
fn fr142_in_surface_crates() {
    let text = read("docs/public-api-1-0-surface.md");
    for crate_name in ["bitloom", "bitloom-prelude", "bitloom-macro", "bitloom-sim"] {
        assert!(
            text.contains(crate_name),
            "in-surface must mention {crate_name}"
        );
    }
    assert!(
        text.contains("build") && text.contains("new") && text.contains("wave"),
        "CLI in-surface must list key documented verbs"
    );
    assert!(
        text.contains("tick") || text.contains("VCD") || text.contains("dual"),
        "bitloom-sim in-surface must cite tick/VCD/dual-model"
    );
}

#[test]
fn fr142_q2_out_of_promise_and_lsp_out() {
    let text = read("docs/public-api-1-0-surface.md");
    assert!(
        text.contains("bitloom-hir")
            && text.contains("bitloom-builder")
            && text.contains("bitloom-vlog"),
        "must list hir/builder/vlog"
    );
    assert!(
        text.contains("out-of-promise")
            || text.contains("Out-of-promise")
            || text.contains("不") && text.contains("承诺"),
        "hir/builder/vlog must be out-of-promise"
    );
    assert!(
        text.contains("bitloom-lsp") || text.contains("LSP"),
        "LSP must be called out as out-of-surface / not promised"
    );
}

#[test]
fn fr142_prelude_design_boundary() {
    let text = read("docs/public-api-1-0-surface.md");
    assert!(
        text.contains("bitloom-prelude")
            && (text.contains("AD-6") || text.contains("only") || text.contains("仅")),
        "must state design crates depend only on bitloom-prelude"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "must keep Bitloom brand"
    );
}

#[test]
fn fr142_nfr14_gate_and_nfr59() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("80-1-epic-80-nfr14-风险记录: done")
            || sprint.contains("80-1-epic-80-nfr14-风险记录:done"),
        "Story 80.1 must be done"
    );
    let text = read("docs/public-api-1-0-surface.md");
    assert!(
        text.contains("NFR59"),
        "surface doc must keep NFR59 honesty"
    );
}

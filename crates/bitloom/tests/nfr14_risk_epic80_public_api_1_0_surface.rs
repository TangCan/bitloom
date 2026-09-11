//! ATDD / guardrail: Epic 80 NFR14 risk record for FR142 public API surface.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic80_public_api_1_0_surface
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic80_public_api_1_0_surface_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic80-public-api-1-0-surface.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "must include (a)"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "must include (b)"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "must include (c)"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "must include (d)"
    );

    assert!(
        text.contains("bitloom-sim") && (text.contains("Q1") || text.contains("in-surface")),
        "must document Q1 bitloom-sim IN"
    );
    assert!(
        (text.contains("bitloom-hir") || text.contains("hir"))
            && (text.contains("builder") || text.contains("bitloom-builder"))
            && (text.contains("vlog") || text.contains("bitloom-vlog"))
            && (text.contains("out-of-promise")
                || text.contains("非承诺")
                || text.contains("不进")),
        "must document Q2 hir/builder/vlog out-of-promise"
    );
    assert!(
        text.contains("bitloom-prelude")
            && (text.contains("设计") || text.contains("AD-6") || text.contains("仅依赖")),
        "must document design crate → prelude boundary"
    );
    assert!(
        text.contains("ATDD") || text.contains("atdd") || text.contains("测试"),
        "must document ATDD strategy"
    );
    assert!(
        text.contains("禁止")
            && (text.contains("未文档化")
                || text.contains("内部 API")
                || text.contains("in-surface")),
        "must forbid undocumented internal API in in-surface"
    );
    assert!(
        text.contains("FR142") && text.contains("Epic 80"),
        "must name FR142 / Epic 80"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-79: done") || sprint.contains("epic-79:done"),
        "Epic 79 must be done before Epic 80 NFR14"
    );
}

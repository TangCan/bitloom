//! ATDD / guardrail: Epic 102 NFR14 risk record for FR169
//! broader CIRCT/MLIR allocation and/or firtool bump (Story 102.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn yaml_value_for_key(sprint: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    for line in sprint.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&needle) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

#[test]
fn nfr14_risk_epic102_circt_mlir_firtool_fr169_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic102-circt-mlir-firtool-fr169.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")));
    assert!(text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")));
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）"))
    );
    assert!(text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")));

    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 102.3");
    let status_closed = text.contains("closed — Story 102.3")
        || (text.contains("closed") && text.contains("Epic 102") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-102: done") || sprint.contains("epic-102:done")) {
        assert!(
            status_open,
            "before epic-102 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR73")
            && text.contains("FR164")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR73 / FR164 isolation"
    );
    assert!(
        text.contains("FR169")
            && (text.contains("allocation")
                || text.contains("Allocation")
                || text.contains("MLIR"))
            && (text.contains("firtool") || text.contains("升钉")),
        "must cover FR169 allocation and/or firtool bump"
    );
    assert!(
        text.contains("AD-9")
            && (text.contains("配对") || text.contains("NFR12") || text.contains("NFR75")),
        "must require AD-9 / pairing for bumps"
    );
    assert!(
        (text.contains("PATH") || text.contains("随机") || text.contains("HEAD"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid PATH-random / unpaired HEAD fake bumps"
    );
    assert!(
        (text.contains("FR164") || text.contains("FR137"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR164/FR137-alone closing FR169"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        text.contains("102.2")
            && text.contains("102.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 102.2–102.3"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "must name owner"
    );
    assert!(
        text.contains("NFR14-crates"),
        "must disambiguate NFR14-crates"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude"),
        "must cite Bitloom / prelude"
    );

    let s102_1 = yaml_value_for_key(&sprint, "102-1-epic-102-nfr14-风险记录").expect("102-1 key");
    if s102_1 != "done" {
        for key in [
            "102-2-circt-mlir-allocation-与或-firtool-升钉实现与验收-fr169",
            "102-3-fr169-收口与文档指针",
        ] {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 102-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-99: done") || sprint.contains("epic-99:done"),
        "Epic 99 must be done before Epic 102 NFR14"
    );
}

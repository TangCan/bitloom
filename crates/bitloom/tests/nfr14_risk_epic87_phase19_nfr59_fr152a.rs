//! ATDD / guardrail: Epic 87 NFR14 risk record for Phase 19
//! NFR59 + FR152(a) / FR154 (Story 87.1 / AD-28 / NFR68–72).
//! Red if file missing, epic already closed, or 87.2+ ready before 87.1 done.

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
fn nfr14_risk_epic87_phase19_nfr59_fr152a_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic87-phase19-nfr59-fr152a.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "risk record must include labeled field (a) 上游约束"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "risk record must include labeled field (b) 粗工期带"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "risk record must include labeled field (c) 禁止的静默降级清单"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "risk record must include labeled field (d) 负责人"
    );

    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 87.4")
        && !text.contains("closed — Epic 87");
    let status_closed = text.contains("closed — Story 87.4")
        || (text.contains("closed") && text.contains("Epic 87") && text.contains("闸门已开"));
    assert!(
        status_open || status_closed,
        "Story 87.1 risk record status must be open/in-progress or closed after Story 87.4"
    );

    let sprint_early = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic87_done =
        sprint_early.contains("epic-87: done") || sprint_early.contains("epic-87:done");
    if !epic87_done {
        assert!(
            status_open,
            "before epic-87 done, risk record must remain open/in-progress"
        );
    }

    assert!(
        text.contains("NFR68")
            && (text.contains("Phase 12") || text.contains("Phase 18") || text.contains("FR153"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR68 Phase 12–18 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("FR94") || text.contains("FR153") || text.contains("Phase 18"))
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–153 closed as failed"
    );

    for fr in [
        "FR155", "FR156", "FR157", "FR158", "FR159", "FR160", "FR161", "FR162", "FR163", "FR164",
        "FR165",
    ] {
        assert!(text.contains(fr), "risk record must summarize {fr}");
    }

    assert!(
        text.contains("Q1") && text.contains("FR157") && text.contains("FR165"),
        "risk record must document Q1 NFR59 full FR157–165"
    );
    assert!(
        text.contains("Q2")
            && (text.contains("FR152") || text.contains("lsp"))
            && (text.contains("(a)") || text.contains("live") || text.contains("publish")),
        "risk record must document Q2 FR152(a) live publish"
    );
    assert!(
        text.contains("Q3") && text.contains("NFR68"),
        "risk record must document Q3 NFR68 no rewrite"
    );
    assert!(
        text.contains("Q4") && (text.contains("FR142") || text.contains("FR156")),
        "risk record must document Q4 no silent FR142 expand / FR156 claims"
    );
    assert!(
        text.contains("Q5")
            && (text.contains("MSRV") || text.contains("1.0.0") || text.contains("FR154")),
        "risk record must document Q5 MSRV / FR154 gate"
    );

    assert!(
        (text.contains("88") || text.contains("88–98") || text.contains("88-98"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR154") || text.contains("Epic 87") || text.contains("ready")),
        "risk record must forbid opening Epic 88–98 before FR154 / Epic 87 gate"
    );

    assert!(
        text.contains("FR142")
            && (text.contains("静默") || text.contains("扩大") || text.contains("不得")),
        "risk record must forbid silent FR142 expansion"
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "risk record must state git push is not an FR"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "risk record must cite NFR71 subset / no silent expand"
    );

    assert!(
        text.contains("87.2")
            && text.contains("87.3")
            && text.contains("87.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 87.2–87.4 from ready without this record"
    );

    assert!(text.contains("FR154"), "risk record must cite FR154");
    assert!(
        text.contains("Epic 87") || text.contains("Epic87"),
        "risk record must name Epic 87"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR68") && text.contains("负责人"),
        "risk record must assign NFR68 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR69") || text.contains("NFR71") || text.contains("NFR72"))
            && text.contains("负责人"),
        "risk record must assign NFR68–72 ownership"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "risk record must cite bitloom-prelude design dependency boundary"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("87") && text.contains("98") && (text.contains("软序") || text.contains("→")),
        "risk record must document soft order involving 87…98"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let s87_1 = yaml_value_for_key(&sprint, "87-1-epic-87-nfr14-风险记录")
        .expect("87-1 key in sprint-status");
    let later = [
        "87-2-correct-course-prd-批准-phase-19-fr154",
        "87-3-同步-readme-deferred-路线图指针-fr154",
        "87-4-ad-指针与-epic-87-收口-fr154",
    ];
    if s87_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 87-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-86: done") || sprint.contains("epic-86:done"),
        "Phase 18 Epic 86 must be done before Epic 87 NFR14"
    );
    let epic87_done_gate = sprint.contains("epic-87: done") || sprint.contains("epic-87:done");
    if !epic87_done_gate {
        for line in sprint.lines() {
            let t = line.trim();
            if t.starts_with("88-")
                || t.starts_with("89-")
                || t.starts_with("90-")
                || t.starts_with("91-")
                || t.starts_with("92-")
                || t.starts_with("93-")
                || t.starts_with("94-")
                || t.starts_with("95-")
                || t.starts_with("96-")
                || t.starts_with("97-")
                || t.starts_with("98-")
            {
                if t.contains("ready-for-dev") {
                    panic!("Epic 88–98 stories must not be ready-for-dev before epic-87 done: {t}");
                }
            }
        }
        for epic in 88..=98 {
            let key = format!("epic-{epic}");
            let st = yaml_value_for_key(&sprint, &key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until epic-87 done (got {st})"
            );
        }
    }
}

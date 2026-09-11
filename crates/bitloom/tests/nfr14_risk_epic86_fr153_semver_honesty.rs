//! ATDD / guardrail: Epic 86 NFR14 risk record for FR153 SemVer honesty
//! (Story 86.1 / AD-28 / NFR64–67).
//! Red if file missing, or 86.2+ ready before 86.1 done.

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
fn nfr14_risk_epic86_fr153_semver_honesty_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic86-fr153-semver-honesty.md");
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
        && !text.contains("closed — Story 86.3")
        && !text.contains("closed — Epic 86");
    let status_closed = text.contains("closed — Story 86.3")
        || (text.contains("closed") && text.contains("Epic 86") && text.contains("FR153"));
    assert!(
        status_open || status_closed,
        "Story 86.1 risk record status must be open/in-progress or closed after Story 86.3"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic86_done = sprint.contains("epic-86: done") || sprint.contains("epic-86:done");
    if !epic86_done {
        assert!(
            status_open,
            "before epic-86 done, risk record must remain open/in-progress"
        );
    }

    assert!(text.contains("FR153"), "risk record must summarize FR153");

    assert!(
        text.contains("NFR59")
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("清") || text.contains("吞并") || text.contains("deferred")),
        "risk record must forbid implying NFR59 is cleared"
    );
    assert!(
        text.contains("FR142")
            && (text.contains("不得") || text.contains("禁止") || text.contains("扩大")),
        "risk record must forbid silently expanding FR142"
    );
    assert!(
        (text.contains("CLI") || text.contains("cargo install") || text.contains("上架"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("证据")),
        "risk record must forbid claiming CLI installable when not published / require evidence when published"
    );

    assert!(
        text.contains("86.2")
            && text.contains("86.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 86.2–86.3 from ready without this record"
    );

    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR64") && text.contains("NFR67"),
        "risk record must cite NFR64/NFR67"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("assume-published")
            || text.contains("ASSUME_PUBLISHED")
            || text.contains("1.0.0 特例")
            || text.contains("semver-check"),
        "risk record must mention SemVer assume-published / 1.0.0 special-case"
    );

    let s86_1 = yaml_value_for_key(&sprint, "86-1-epic-86-nfr14-风险记录")
        .expect("86-1 key in sprint-status");
    let later = [
        "86-2-semver-assume-published-与发版诚实更新-fr153",
        "86-3-fr153-收口与-phase-18-故事清单指针",
    ];
    if s86_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 86-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-85: done") || sprint.contains("epic-85:done"),
        "Epic 85 must be done before Epic 86 NFR14"
    );
    let epic86 = yaml_value_for_key(&sprint, "epic-86").unwrap_or_default();
    assert!(
        epic86 == "backlog" || epic86 == "in-progress" || epic86 == "done",
        "epic-86 status must be backlog|in-progress|done (got {epic86})"
    );
}

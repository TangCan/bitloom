//! ATDD / guardrail: Story 37.3 / FR88 — HLS Path B (explicit stub default).
//!
//! Red until `docs/fr35-hls.md` records Epic 37 Path B disposition, deferred-work
//! closes optional nightly as「本阶段选 B」, and NFR14 Epic 37 close conditions
//! are checked. Tree-in HLS scheduler remains a non-goal.
//!
//! ```text
//! cargo test -p bitloom --test fr88_hls_stub_path_b_honesty
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
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr88_fr35_documents_path_b_stub_default() {
    let text = read("docs/fr35-hls.md");
    assert!(
        (text.contains("Path B") || text.contains("路径 B") || text.contains("选 B"))
            && (text.contains("Epic 37") || text.contains("FR88")),
        "fr35 must record Epic 37 / FR88 Path B disposition"
    );
    assert!(
        text.contains("bambu-ci-stub")
            && (text.contains("非真实")
                || text.contains("≠")
                || text.contains("不是")
                || text.contains("质量")),
        "fr35 must keep stub ≠ real HLS quality honesty"
    );
    assert!(
        text.contains("BITLOOM_HLS_USE_REAL"),
        "fr35 must document explicit real-Bambu entry BITLOOM_HLS_USE_REAL"
    );
    assert!(
        (text.contains("夜间") || text.contains("nightly"))
            && (text.contains("不")
                || text.contains("未")
                || text.contains("选 B")
                || text.contains("Path B")),
        "fr35 must state that optional/nightly real Bambu is not the chosen default path"
    );
}

#[test]
fn fr88_fr35_tree_in_hls_scheduler_remains_non_goal() {
    let text = read("docs/fr35-hls.md");
    assert!(
        (text.contains("AD-25") || text.contains("FR86") || text.contains("FR93"))
            && (text.contains("调度") || text.contains("scheduler") || text.contains("scheduling"))
            && (text.contains("永不")
                || text.contains("不实现")
                || text.contains("非目标")
                || text.contains("never")),
        "fr35 must keep in-tree HLS scheduler as non-goal (AD-25)"
    );
}

#[test]
fn fr88_deferred_closes_optional_nightly_as_path_b() {
    let text = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        text.contains("epic-24-retro-item-54")
            && text.contains("本阶段选 B")
            && (text.contains("status: closed") || text.contains("closed — 本阶段选 B")),
        "deferred-work must close epic-24-retro-item-54 as Phase Path B (本阶段选 B)"
    );
    assert!(
        !text.contains("status: deferred — 可选夜间真机 job"),
        "deferred-work must not leave epic-24-retro-item-54 as open 'optional nightly'"
    );
}

#[test]
fn fr88_nfr14_epic37_close_conditions_checked_path_b() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md");
    assert!(
        text.contains("- [x] **FR88（HLS）")
            && (text.contains("选 Path B") || text.contains("Path B")),
        "NFR14 must check FR88 HLS close condition with Path B"
    );
    assert!(
        text.contains("- [x] **FR88（firtool/Chisel）"),
        "NFR14 must check FR88 firtool/Chisel close condition (37.2)"
    );
    assert!(
        text.contains("- [x] **NFR12 / NFR39**") || text.contains("- [x] **NFR12 / NFR39"),
        "NFR14 must check NFR12 / NFR39 close condition"
    );
    assert!(
        text.contains("- [x] **禁止事项未触发**") || text.contains("- [x] **禁止事项未触发"),
        "NFR14 must check forbidden-actions close condition"
    );
    assert!(
        text.contains("- [x] **品牌**") || text.contains("- [x] **品牌"),
        "NFR14 must check brand close condition"
    );
    assert!(
        text.contains("closed — Story 37.3"),
        "NFR14 Epic 37 record status must be closed after 37.3"
    );
}

#[test]
fn fr88_readme_cross_links_path_b_honesty() {
    let text = read("README.md");
    assert!(
        text.contains("docs/fr35-hls.md"),
        "README must link FR35 HLS chapter"
    );
    assert!(
        text.contains("FR88")
            && (text.contains("Path B") || text.contains("选 B"))
            && (text.contains("stub") || text.contains("Stub")),
        "README HLS section must cross-link FR88 Path B stub-default honesty"
    );
}

#[test]
fn fr88_ci_hls_smoke_has_no_continue_on_error() {
    let text = read(".github/workflows/ci.yml");
    assert!(text.contains("hls-smoke"), "ci.yml must keep hls-smoke job");
    let hls_idx = text.find("hls-smoke:").expect("hls-smoke job");
    let after = &text[hls_idx..];
    // Bound to this job only (next top-level job key at column 2).
    let end = after[1..]
        .find("\n  ")
        .map(|i| i + 1)
        .and_then(|i| {
            after[i..]
                .find('\n')
                .map(|j| i + j)
                .or(Some(after.len()))
        })
        .unwrap_or_else(|| after.len().min(600));
    // Prefer cutting at next `\n  [a-z].*:` job header
    let mut job_end = after.len();
    for (i, line) in after.lines().enumerate().skip(1) {
        if line.starts_with("  ") && line.ends_with(':') && !line.starts_with("    ") {
            job_end = after
                .lines()
                .take(i)
                .map(|l| l.len() + 1)
                .sum::<usize>()
                .min(after.len());
            break;
        }
    }
    let slice = &after[..job_end.min(after.len())];
    let _ = end;
    assert!(
        !slice.contains("continue-on-error"),
        "hls-smoke job must not use continue-on-error"
    );
}

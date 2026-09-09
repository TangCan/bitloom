//! ATDD — Story 35.4 / FR85: Formal fixture beyond toy + LSP deferred close.
//! Chosen close path: **Option A** (real design → emit_sva + documented
//! external checker script). Red if FR85 is closed via `check_sva_text` alone,
//! missing-tool path is silent success, LSP is claimed done for Epic 35, or
//! NFR14 Epic 35 close checklist is incomplete.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
fn fr85_user_docs_document_external_checker_not_toy_only() {
    let text = read("docs/fr39-formal-sva.md");

    assert!(
        text.contains("FR85")
            && (text.contains("Option A")
                || text.contains("选项 A")
                || text.contains("Path A")
                || text.contains("非玩具")
                || text.contains("non-toy")
                || text.contains("beyond toy")),
        "fr39 docs must document FR85 Option A / non-toy formal path"
    );
    assert!(
        text.contains("formal-sva-check")
            || text.contains("just formal-sva-check")
            || text.contains("scripts/formal-sva-check"),
        "fr39 docs must document just/script formal-sva-check entry"
    );
    assert!(
        text.contains("verilator") || text.contains("sby") || text.contains("SymbiYosys"),
        "fr39 docs must name an external checker (verilator and/or sby)"
    );
    assert!(
        text.contains("check_sva_text")
            && (text.contains("toy") || text.contains("玩具") || text.contains("heuristic")),
        "fr39 docs must situate check_sva_text as toy / not FR85 close"
    );
    assert!(
        (text.contains("不得") || text.contains("must not") || text.contains("MUST NOT"))
            && (text.contains("check_sva_text") || text.contains("toy")),
        "fr39 docs must forbid closing FR85 with toy check_sva_text alone"
    );
}

#[test]
fn fr85_lsp_docs_state_not_epic35_completion() {
    let text = read("docs/fr38-viz-lsp.md");

    assert!(
        text.contains("LSP") && (text.contains("deferred") || text.contains("Deferred")),
        "fr38 docs must keep LSP deferred"
    );
    assert!(
        (text.contains("Epic 35") || text.contains("epic 35") || text.contains("本 epic"))
            && (text.contains("非") || text.contains("not") || text.contains("NOT"))
            && (text.contains("完成")
                || text.contains("completion")
                || text.contains("done criterion")
                || text.contains("完成条件")),
        "fr38 docs must state LSP hover/goto is not an Epic 35 completion criterion"
    );
    assert!(
        text.contains("hover") || text.contains("goto"),
        "fr38 docs must name hover/goto as the deferred LSP slice"
    );
}

#[test]
fn fr85_prd_addendum_option_a_contract() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");

    assert!(
        text.contains("FR85")
            && (text.contains("Option A")
                || text.contains("选项 A")
                || text.contains("Path A")
                || text.contains("非玩具")
                || text.contains("non-toy")),
        "PRD addendum must contain FR85 Option A / non-toy contract"
    );
    assert!(
        (text.contains("不得") || text.contains("must not") || text.contains("MUST NOT"))
            && (text.contains("check_sva_text") || text.contains("toy") || text.contains("玩具")),
        "addendum must forbid closing FR85 with toy check"
    );
}

#[test]
fn fr85_nfr14_marks_option_a_and_epic35_close_checklist() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md");

    assert!(
        text.contains("FR85")
            && (text.contains("已选 A")
                || text.contains("已选 **A**")
                || (text.contains("选项 A") && text.contains("已选"))
                || (text.contains("Option A")
                    && (text.contains("已选") || text.contains("selected")))),
        "NFR14 record must mark FR85 as Option A selected"
    );

    // Epic 35 close checklist: FR83/84/85 + LSP + NFR37 + 禁止事项 all ticked
    for needle in ["FR83", "FR84", "FR85", "LSP", "NFR37", "禁止事项"] {
        assert!(
            text.contains(needle),
            "NFR14 close checklist must mention {needle}"
        );
    }
    let close_section = text
        .split("### Epic 35 关闭条件")
        .nth(1)
        .unwrap_or("")
        .split("\n## ")
        .next()
        .unwrap_or("");
    assert!(
        !close_section.is_empty(),
        "NFR14 must contain ### Epic 35 关闭条件 section"
    );
    assert!(
        close_section.matches("- [x]").count() >= 5 || close_section.matches("- [X]").count() >= 5,
        "Epic 35 close checklist must tick FR83/84/85/LSP/NFR37/禁止事项 (at least 5 [x]); section={close_section}"
    );
    assert!(
        !close_section.contains("- [ ] **FR85:**") && !close_section.contains("- [ ] **LSP:**"),
        "FR85 and LSP close checkboxes must not remain unticked"
    );
}

#[test]
fn fr85_formal_check_script_exists_and_mentions_external_tool() {
    let script = workspace_root().join("scripts/formal-sva-check.sh");
    assert!(
        script.is_file(),
        "missing scripts/formal-sva-check.sh at {}",
        script.display()
    );
    let text = fs::read_to_string(&script).unwrap_or_else(|e| panic!("read script: {e}"));
    assert!(
        text.contains("verilator") || text.contains("sby"),
        "formal-sva-check.sh must invoke verilator and/or sby"
    );
    assert!(
        text.contains("emit_sva")
            || text.contains("rhdl-formal")
            || text.contains("export")
            || text.contains("FR85"),
        "script must tie to rhdl-formal export / FR85 path"
    );
    // Missing tool must not be silent success
    assert!(
        text.contains("exit 1")
            || text.contains("exit 2")
            || text.contains("not found")
            || text.contains("missing"),
        "script must fail clearly when checker is missing"
    );
    assert!(
        !text.contains("check_sva_text"),
        "FR85 script must not close via check_sva_text"
    );

    let justfile = read("Justfile");
    assert!(
        justfile.contains("formal-sva-check"),
        "Justfile must expose formal-sva-check recipe"
    );
}

#[test]
fn fr85_real_design_export_fixture_exceeds_string_heuristics() {
    // Fixture must exist and look like emitted SVA from a named design module.
    let candidates = [
        "crates/rhdl-formal/fixtures/fr85_counter_sva.sv",
        "crates/rhdl-formal/fixtures/Counter_sva.sv",
    ];
    let fixture = candidates
        .iter()
        .map(|p| workspace_root().join(p))
        .find(|p| p.is_file())
        .unwrap_or_else(|| {
            panic!(
                "missing FR85 SVA fixture under crates/rhdl-formal/fixtures/ (expected one of {:?})",
                candidates
            )
        });
    let sv = fs::read_to_string(&fixture).unwrap_or_else(|e| panic!("read fixture: {e}"));
    assert!(
        sv.contains("assert property"),
        "fixture must contain concurrent assert property"
    );
    assert!(
        sv.contains("module") && (sv.contains("Counter") || sv.contains("counter")),
        "fixture must be from a real Counter (or named) design module"
    );
    assert!(
        sv.contains("generated by rhdl formal") || sv.contains("FR39") || sv.contains("FR85"),
        "fixture should identify rhdl formal export provenance"
    );
}

#[test]
fn fr85_script_fails_clearly_when_checker_unavailable() {
    let script = workspace_root().join("scripts/formal-sva-check.sh");
    if !script.is_file() {
        panic!("scripts/formal-sva-check.sh missing — implement T2 first");
    }
    // Force empty PATH so neither verilator nor sby resolve; script must not exit 0.
    let status = Command::new("bash")
        .arg(&script)
        .env("PATH", "/usr/bin:/bin") // keep bash basics; strip typical tool dirs via BITLOOM override
        .env("BITLOOM_FORMAL_CHECKER", "")
        .env("BITLOOM_FORMAL_FORCE_MISSING", "1")
        .current_dir(workspace_root())
        .output()
        .expect("spawn formal-sva-check.sh");
    assert!(
        !status.status.success(),
        "formal-sva-check.sh must fail when checker forced missing; stdout={:?} stderr={:?}",
        String::from_utf8_lossy(&status.stdout),
        String::from_utf8_lossy(&status.stderr)
    );
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&status.stdout),
        String::from_utf8_lossy(&status.stderr)
    );
    assert!(
        err.to_lowercase().contains("not found")
            || err.to_lowercase().contains("missing")
            || err.contains("error:")
            || err.contains("无")
            || err.contains("unavailable"),
        "missing-checker failure must be readable, got: {err}"
    );
}

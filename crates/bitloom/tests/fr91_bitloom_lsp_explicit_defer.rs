//! ATDD — Story 39.3 / FR91: shallow Bitloom LSP **or** explicit defer.
//! Chosen close path: **Path B (explicit defer)**. Red until README / fr38
//! contract Path B, NFR14 FR91 is ticked, and no half-built LSP binary exists.
//!
//! ```text
//! cargo test -p bitloom --test fr91_bitloom_lsp_explicit_defer
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

fn has_path_b(text: &str) -> bool {
    text.contains("Path B")
        || text.contains("分支 B")
        || text.contains("选项 B")
        || text.contains("Option B")
        || (text.contains("显式 defer") || text.contains("explicit defer"))
}

fn forbids_lsp_delivered_claim(text: &str) -> bool {
    let lower = text.to_lowercase();
    let forbid = text.contains("不得")
        || text.contains("must not")
        || text.contains("MUST NOT")
        || text.contains("不得声称")
        || text.contains("不声称")
        || text.contains("does **not** claim")
        || text.contains("do not claim")
        || text.contains("不计入");
    let lsp = text.contains("LSP")
        || lower.contains("language-server")
        || text.contains("language server");
    let delivered = text.contains("已交付")
        || lower.contains("delivered")
        || text.contains("完成")
        || lower.contains("done")
        || text.contains("冒充");
    forbid && lsp && (delivered || text.contains("deferred") || text.contains("defer"))
}

#[test]
fn fr91_fr38_contracts_path_b_explicit_defer() {
    let text = read("docs/fr38-viz-lsp.md");
    assert!(
        text.contains("FR91") && has_path_b(&text),
        "docs/fr38-viz-lsp.md must name FR91 and Path B / explicit defer"
    );
    assert!(
        text.contains("deferred") || text.contains("defer") || text.contains("Deferred"),
        "fr38 must state Bitloom LSP remains deferred"
    );
    assert!(
        forbids_lsp_delivered_claim(&text)
            || ((text.contains("不得") || text.contains("must not") || text.contains("MUST NOT"))
                && (text.contains("LSP") || text.contains("language-server"))),
        "fr38 must forbid claiming Bitloom LSP / language-server delivered"
    );
    assert!(text.contains("Bitloom"), "fr38 must brand Bitloom");
}

#[test]
fn fr91_readme_contracts_path_b_and_forbids_delivered_claim() {
    let text = read("README.md");
    assert!(
        text.contains("FR91") && has_path_b(&text),
        "README must name FR91 Path B / explicit defer"
    );
    assert!(
        (text.contains("不得")
            || text.contains("must not")
            || text.contains("MUST NOT")
            || text.contains("不得声称")
            || text.contains("未承诺"))
            && (text.contains("LSP") || text.contains("language-server")),
        "README must situate Bitloom LSP as deferred / not claimed delivered"
    );
    assert!(
        !text.contains("Bitloom LSP 已交付")
            && !text.to_lowercase().contains("bitloom lsp delivered")
            && !text.contains("自研 LSP 已交付"),
        "README must not claim Bitloom LSP delivered"
    );
}

#[test]
fn fr91_html_does_not_count_as_lsp() {
    let fr38 = read("docs/fr38-viz-lsp.md");
    let readme = read("README.md");
    let combined = format!("{fr38}\n{readme}");
    assert!(
        (combined.contains("HTML") || combined.contains("层次") || combined.contains("hierarchy"))
            && (combined.contains("≠")
                || combined.contains("不计入")
                || combined.contains("不声称")
                || combined.contains("does **not** claim")
                || combined.contains("not claim")
                || combined.contains("≠ LSP")),
        "docs must state hierarchy/timing HTML does not count as LSP completion"
    );
}

#[test]
fn fr91_nfr14_fr91_checkbox_ticked_fr92_open() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md");
    assert!(
        text.contains("- [x] **FR91：**")
            || text.contains("- [x] **FR91:**")
            || (text.contains("[x]")
                && text.contains("FR91")
                && (text.contains("Path B")
                    || text.contains("分支 B")
                    || text.contains("显式 defer")
                    || text.contains("explicit defer"))),
        "NFR14 must tick FR91 Path B close checkbox"
    );
    assert!(
        text.contains("- [ ] **FR92：**")
            || text.contains("- [ ] **FR92:**")
            || (text.contains("[ ]") && text.contains("FR92")),
        "NFR14 FR92 close checkbox must remain open (Story 39.4)"
    );
    // Must not mark entire epic record closed by this story alone
    let status_line = text
        .lines()
        .find(|l| l.contains("| 状态 |") || l.contains("| Status |"))
        .unwrap_or("");
    assert!(
        !status_line.to_lowercase().contains("closed")
            || status_line.contains("39.4")
            || !text.contains("status: closed"),
        "Epic 39 NFR14 must not be fully closed by Story 39.3 alone"
    );
}

#[test]
fn fr91_no_half_built_lsp_binary() {
    let root = workspace_root();
    for name in ["language-server", "bitloom-lsp", "bitloom_lsp"] {
        let p = root.join("crates").join(name);
        assert!(
            !p.exists(),
            "Path B forbids half-built Bitloom LSP crate/binary at {}",
            p.display()
        );
    }
}

#[test]
fn fr91_scope_guards_39_4_backlog() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("39-4-多视图同刺激与-adapter-模板-fr92: backlog")
            || sprint
                .lines()
                .any(|l| l.contains("39-4-") && l.contains("backlog")),
        "Story 39.4 must remain backlog (FR92 not started)"
    );
}

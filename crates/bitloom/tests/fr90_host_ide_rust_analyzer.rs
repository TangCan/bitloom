//! ATDD — Story 39.2 / FR90: host IDE / rust-analyzer workflow docs.
//!
//! Red until `docs/fr90-host-ide-rust-analyzer.md` delivers reproducible
//! steps + fixture explanation, and README/fr38 cross-link the host path
//! without claiming Bitloom LSP delivered.
//!
//! ```text
//! cargo test -p bitloom --test fr90_host_ide_rust_analyzer
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

fn fr90_doc() -> String {
    let primary = workspace_root().join("docs/fr90-host-ide-rust-analyzer.md");
    assert!(
        primary.is_file(),
        "docs/fr90-host-ide-rust-analyzer.md must exist for FR90 host IDE workflow"
    );
    fs::read_to_string(&primary).expect("fr90 doc")
}

#[test]
fn fr90_doc_identifies_bitloom_and_rust_analyzer_workflow() {
    let text = fr90_doc();
    assert!(text.contains("FR90"), "FR90 user doc must identify FR90");
    assert!(text.contains("Bitloom"), "FR90 user doc must brand Bitloom");
    assert!(
        text.contains("rust-analyzer"),
        "FR90 user doc must name rust-analyzer as host LSP"
    );
    assert!(
        text.contains("bitloom-prelude"),
        "FR90 user doc must state design crates depend on bitloom-prelude"
    );
}

#[test]
fn fr90_doc_has_reproducible_steps_for_completion_goto_diagnostics() {
    let text = fr90_doc();
    let has_completion = text.contains("补全")
        || text.to_lowercase().contains("completion")
        || text.contains("autocomplete");
    let has_goto = text.contains("跳转")
        || text.to_lowercase().contains("goto")
        || text.contains("Go to Definition")
        || text.contains("转到定义");
    let has_diagnostics = text.contains("诊断") || text.to_lowercase().contains("diagnostic");
    assert!(
        has_completion && has_goto && has_diagnostics,
        "FR90 doc must document completion, goto/jump, and diagnostics wording"
    );
    assert!(
        text.contains("1.97.1"),
        "FR90 doc must pin toolchain version 1.97.1"
    );
    assert!(
        text.contains("rust-toolchain") || text.contains("rust-toolchain.toml"),
        "FR90 doc must cite rust-toolchain"
    );
    assert!(
        text.contains("打开") || text.contains("open") || text.contains("Open"),
        "FR90 doc must include open-workspace / open-crate reproducible steps"
    );
    assert!(
        text.contains("rust-lang.rust-analyzer")
            || text.contains("Rust Analyzer")
            || text.contains("启用 rust-analyzer"),
        "FR90 doc must give a high-level rust-analyzer enable/install tip"
    );
}

#[test]
fn fr90_doc_explains_fixture_project() {
    let text = fr90_doc();
    assert!(
        text.contains("examples/counter_ports") || text.contains("counter_ports"),
        "FR90 doc must explain at least one fixture (examples/counter_ports)"
    );
    assert!(
        workspace_root()
            .join("examples/counter_ports/Cargo.toml")
            .is_file(),
        "fixture examples/counter_ports must exist in the workspace"
    );
    let toml = read("examples/counter_ports/Cargo.toml");
    assert!(
        toml.contains("bitloom-prelude"),
        "counter_ports fixture must depend on bitloom-prelude"
    );
    assert!(
        !toml.contains("[dependencies]\nbitloom ")
            && !toml.contains("bitloom = {")
            && !toml.contains("bitloom-macro"),
        "counter_ports must not depend on CLI bitloom or bitloom-macro"
    );
}

#[test]
fn fr90_doc_distinguishes_host_ide_from_hardware_lsp_and_html() {
    let text = fr90_doc();
    assert!(
        (text.contains("硬件语义") || text.contains("netlist"))
            && (text.contains("不")
                || text.contains("非")
                || text.contains("❌")
                || text.contains("not")),
        "FR90 doc must reject hardware-semantic / netlist LSP as in-scope"
    );
    assert!(
        (text.contains("Bitloom LSP")
            || (text.contains("自研") && text.contains("language-server"))
            || text.contains("自研 LSP"))
            && (text.contains("不交付")
                || text.contains("deferred")
                || text.contains("defer")
                || text.contains("Path B")
                || text.contains("分支 B")
                || text.contains("❌")),
        "FR90 doc must state Bitloom / self-hosted LSP is not delivered (Path B)"
    );
    assert!(
        (text.contains("fr38") || text.contains("层次") || text.contains("HTML"))
            && (text.contains("不计入")
                || text.contains("≠")
                || text.contains("不是")
                || text.contains("不声称")
                || text.contains("not")),
        "FR90 doc must situate hierarchy HTML / fr38 as not substituting for host IDE LSP"
    );
    let lower = text.to_lowercase();
    assert!(
        !text.contains("Bitloom LSP 已交付")
            && !text.contains("Bitloom LSP delivered")
            && !lower.contains("bitloom lsp is complete")
            && !lower.contains("bitloom language server delivered"),
        "FR90 doc must not claim Bitloom LSP delivered"
    );
}

#[test]
fn fr90_readme_and_fr38_cross_link_host_path() {
    let readme = read("README.md");
    assert!(
        readme.contains("docs/fr90-host-ide-rust-analyzer.md")
            || readme.contains("fr90-host-ide-rust-analyzer.md"),
        "README must cross-link docs/fr90-host-ide-rust-analyzer.md"
    );

    let fr38 = read("docs/fr38-viz-lsp.md");
    assert!(
        fr38.contains("FR90") || fr38.contains("fr90"),
        "docs/fr38-viz-lsp.md must name FR90 / fr90 host path"
    );
    assert!(
        fr38.contains("rust-analyzer"),
        "docs/fr38-viz-lsp.md must mention rust-analyzer host path"
    );
}

#[test]
fn fr90_scope_guards_no_lsp_binary_and_fr91_path_b() {
    let root = workspace_root();
    // No Bitloom language-server binary / crate delivered this story
    let forbidden_names = ["language-server", "bitloom-lsp", "bitloom_lsp"];
    for name in forbidden_names {
        let p = root.join("crates").join(name);
        assert!(
            !p.exists(),
            "must not ship Bitloom LSP crate/binary at {}",
            p.display()
        );
    }
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    // Story 39.3 Path B may be done; 39.4 / FR92 close Epic 39
    assert!(
        sprint.contains("39-3-浅层-bitloom-lsp-或显式-defer-fr91: done")
            || sprint
                .lines()
                .any(|l| l.contains("39-3-") && l.contains("done")),
        "Story 39.3 (FR91 Path B) should be done when FR90 regression runs post-39.4"
    );
    let nfr14 = read("_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md");
    assert!(
        nfr14.contains("- [x] **FR91：**")
            || nfr14.contains("- [x] **FR91:**")
            || (nfr14.contains("[x]") && nfr14.contains("FR91")),
        "Epic 39 NFR14 must keep FR91 Path B ticked"
    );
}

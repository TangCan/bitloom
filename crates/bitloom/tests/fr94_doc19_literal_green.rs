//! ATDD / guardrail: Story 40.3 / FR94 — doc-19 §19.7–19.9 literal-green
//! completion definitions (Path B) + README/deferred pointers.
//!
//! Red until `docs/requirements/19. 实施路线图.md` rewrites P5–P7 green
//! as FR95–105 checkbox conditions (NFR42) with Phase 11/12 dual framing.
//!
//! Does **not** revoke README FR93 "须新 PRD" lock (→ Story 40.4).
//!
//! ```text
//! cargo test -p bitloom --test fr94_doc19_literal_green
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read_doc19() -> String {
    let path = workspace_root().join("docs/requirements/19. 实施路线图.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr94_doc19_header_dual_framing() {
    let text = read_doc19();
    assert!(
        (text.contains("Phase 11") || text.contains("阶段十一"))
            && (text.contains("合同绿") || text.contains("FR87"))
            && (text.contains("历史") || text.contains("里程碑")),
        "doc-19 header must frame Phase 11 contract-green as historical milestone"
    );
    assert!(
        (text.contains("Phase 12") || text.contains("阶段十二"))
            && (text.contains("字面绿") || text.contains("字面"))
            && (text.contains("FR94") || text.contains("FR95") || text.contains("当前")),
        "doc-19 header must frame Phase 12 literal-green as current completion contract"
    );
}

#[test]
fn fr94_doc19_p5_literal_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.7") || text.contains("阶段五"),
        "doc-19 must retain stage-five section"
    );
    assert!(
        text.contains("字面绿完成定义") || (text.contains("字面绿") && text.contains("P5")),
        "P5 section must use literal-green completion framing"
    );
    assert!(
        text.contains("FR95")
            && (text.contains("树内") || text.contains("自研") || text.contains("#[hls]")),
        "P5 literal green must require in-tree / self HLS scheduling (FR95)"
    );
    assert!(
        text.contains("FR96")
            && (text.contains("闭包")
                && (text.contains("数据流") || text.contains("变换") || text.contains("调度前"))),
        "P5 literal green must require HLS closure dataflow transform (FR96)"
    );
}

#[test]
fn fr94_doc19_p6_literal_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.8") || text.contains("阶段六"),
        "doc-19 must retain stage-six section"
    );
    assert!(
        text.contains("字面绿完成定义") || (text.contains("字面绿") && text.contains("P6")),
        "P6 section must use literal-green completion framing"
    );
    assert!(
        text.contains("FR98") && (text.contains("VIP") || text.contains("全协议")),
        "P6 literal green must require VIP / full-protocol IP (FR98)"
    );
    assert!(
        text.contains("FR99") && (text.contains("elaborate") || text.contains("LSP")),
        "P6 literal green must require full-elaborate Bitloom LSP (FR99)"
    );
    assert!(
        text.contains("FR97")
            && (text.contains("idiomatic") || text.contains("可维护"))
            && (text.contains("Chisel") || text.contains("Scala")),
        "P6 (or cross-link) must cite idiomatic Chisel / FR97"
    );
}

#[test]
fn fr94_doc19_p7_literal_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.9") || text.contains("阶段七"),
        "doc-19 must retain stage-seven section"
    );
    assert!(
        text.contains("字面绿完成定义") || (text.contains("字面绿") && text.contains("P7")),
        "P7 section must use literal-green completion framing"
    );
    assert!(
        text.contains("FR100")
            && (text.contains("形式等价") || text.contains("自动等价") || text.contains("FL≡RTL")),
        "P7 literal green must include auto FL≡RTL / formal equivalence product (FR100)"
    );
    assert!(
        text.contains("FR101") && (text.contains("SystemC") || text.contains("TLM")),
        "P7 literal green must include SystemC TLM product path (FR101)"
    );
    assert!(
        text.contains("FR102")
            && (text.contains("#[abstraction]")
                || text.contains("functional_model")
                || text.contains("多视图")),
        "P7 literal green must include multi-view attribute matrix (FR102)"
    );
    assert!(
        text.contains("FR103") && text.contains("双模型"),
        "P7 literal green must include dual-model complete IP face (FR103)"
    );
    assert!(
        text.contains("FR104")
            && (text.contains("富波形") || (text.contains("交互") && text.contains("波形"))),
        "P7 (or cross) must include interactive rich waveform (FR104)"
    );
    assert!(
        text.contains("FR105") && text.contains("覆盖率"),
        "P7 (or cross) must include coverage recording extension (FR105)"
    );
}

#[test]
fn fr94_doc19_nfr42_checkbox_discipline() {
    let text = read_doc19();
    assert!(
        text.contains("NFR42")
            || ((text.contains("对应 FR") || text.contains("FR 关闭") || text.contains("FR关闭"))
                && (text.contains("勾选") || text.contains("方可"))),
        "doc-19 must require literal items checkbox only after corresponding FR closes (NFR42)"
    );
    // Current completion discipline must NOT be the Phase-11 forbid-literal rule alone
    let has_nfr42_style = text.contains("NFR42")
        || text.contains("对应 FR")
        || text.contains("FR 关闭后方可")
        || text.contains("关闭后方可勾选");
    assert!(
        has_nfr42_style,
        "doc-19 must replace Path-B-conflicting forbid-literal-as-current-discipline with NFR42"
    );
}

#[test]
fn fr94_readme_deferred_literal_green_pointer() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");
    assert!(
        readme.contains("状态与 deferred") || readme.contains("状态与 deferred（诚实声明）"),
        "README must keep 状态与 deferred section"
    );
    assert!(
        (readme.contains("字面绿") || readme.contains("FR94") || readme.contains("Phase 12"))
            && (readme.contains("19")
                || readme.contains("实施路线图")
                || readme.contains("doc-19")
                || readme.contains("§19.7")),
        "README must point at Phase 12 literal-green / doc-19 current completion"
    );

    let deferred =
        fs::read_to_string(root.join("_agile-output/implementation-artifacts/deferred-work.md"))
            .expect("deferred-work.md");
    assert!(
        (deferred.contains("字面绿") || deferred.contains("FR94") || deferred.contains("Phase 12"))
            && (deferred.contains("实施路线图")
                || deferred.contains("19.")
                || deferred.contains("doc-19")),
        "deferred-work must cross-link literal-green current contract / doc-19"
    );
}

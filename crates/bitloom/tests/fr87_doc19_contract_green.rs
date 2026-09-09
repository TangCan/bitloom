//! ATDD / guardrail: Story 36.2 / FR87 / NFR38 — doc-19 P5–P7 contract-green
//! completion definitions + README deferred pointer.
//!
//! Red until `docs/requirements/19. 实施路线图.md` and README/deferred
//! cross-links land the research Recommendations clauses.
//!
//! ```text
//! cargo test -p bitloom --test fr87_doc19_contract_green
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
fn fr87_doc19_p5_contract_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.7") || text.contains("阶段五"),
        "doc-19 must retain stage-five section"
    );
    assert!(
        (text.contains("合同绿") || text.contains("完成定义"))
            && (text.contains("外挂") || text.contains("Bambu") || text.contains("HLS"))
            && (text.contains("可复现") || text.contains("复现")),
        "P5 green must require external HLS reproducible artifact"
    );
    assert!(
        (text.contains("薄 IP") || text.contains("薄IP") || text.contains("IP 生成器"))
            && (text.contains("生成器") || text.contains("薄")),
        "P5 green must require thin IP generators"
    );
    assert!(
        (text.contains("VCD") || text.contains("波形"))
            && (text.contains("层次") || text.contains("可视化") || text.contains("hierarchy")),
        "P5 green must require VCD / hierarchy visualization"
    );
}

#[test]
fn fr87_doc19_p6_contract_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.8") || text.contains("阶段六"),
        "doc-19 must retain stage-six section"
    );
    assert!(
        (text.contains("精选 IP") || text.contains("深度合同") || text.contains("非 VIP"))
            && (text.contains("IP")),
        "P6 green must require selected IP depth contract (non-VIP)"
    );
    assert!(
        text.contains("文档站") || text.contains("文档网站") || text.contains("docs site"),
        "P6 green must require docs site"
    );
    assert!(
        text.contains("rust-analyzer")
            && (text.contains("宿主") || text.contains("LSP") || text.contains("IDE")),
        "P6 green must require host LSP path (rust-analyzer)"
    );
}

#[test]
fn fr87_doc19_p7_contract_green() {
    let text = read_doc19();
    assert!(
        text.contains("19.9") || text.contains("阶段七"),
        "doc-19 must retain stage-seven section"
    );
    assert!(
        (text.contains("同刺激") || text.contains("共享刺激"))
            && (text.contains("功能") || text.contains("周期")),
        "P7 green must require shared-stimulus functional/cycle paths"
    );
    assert!(
        (text.contains("adapter") || text.contains("适配器"))
            && (text.contains("模板") || text.contains("template") || text.contains("桥接")),
        "P7 green must require bridge adapter template"
    );
    assert!(
        (text.contains("不") || text.contains("禁止") || text.contains("不承诺"))
            && (text.contains("自动等价")
                || text.contains("形式化等价")
                || text.contains("等价证明")),
        "P7 green must refuse auto equivalence as completion"
    );
    assert!(
        (text.contains("不")
            || text.contains("禁止")
            || text.contains("不承诺")
            || text.contains("非合同"))
            && (text.contains("SystemC") || text.contains("TLM")),
        "P7 green must refuse SystemC TLM product as completion (AD-5)"
    );
}

#[test]
fn fr87_doc19_forbids_literal_full_green_checkbox() {
    let text = read_doc19();
    assert!(
        (text.contains("禁止") || text.contains("不得") || text.contains("不可"))
            && (text.contains("字面") || text.contains("未交付"))
            && (text.contains("全绿") || text.contains("勾选")),
        "doc-19 must forbid claiming literal full-green via undelivered items (FR87/NFR38)"
    );
    assert!(
        text.contains("NFR38") || text.contains("合同绿"),
        "doc-19 must cite NFR38 / 合同绿 framing"
    );
}

#[test]
fn fr87_readme_and_deferred_point_to_contract_green() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");
    assert!(
        readme.contains("状态与 deferred") || readme.contains("状态与 deferred（诚实声明）"),
        "README must keep 状态与 deferred section"
    );
    assert!(
        (readme.contains("合同绿") || readme.contains("FR87") || readme.contains("NFR38"))
            && (readme.contains("19")
                || readme.contains("实施路线图")
                || readme.contains("doc-19")),
        "README 状态与 deferred must point at contract-green / doc-19 definition"
    );

    let deferred =
        fs::read_to_string(root.join("_agile-output/implementation-artifacts/deferred-work.md"))
            .expect("deferred-work.md");
    assert!(
        deferred.contains("合同绿")
            || deferred.contains("FR87")
            || deferred.contains("实施路线图")
            || deferred.contains("NFR38"),
        "deferred-work must cross-link contract-green / FR87 / doc-19"
    );
}

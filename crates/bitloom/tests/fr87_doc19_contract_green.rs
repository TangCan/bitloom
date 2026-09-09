//! ATDD / guardrail: Story 36.2 / FR87 / NFR38 — Phase 11 contract-green
//! as **historical milestone** (still traceable in doc-19 / README / deferred).
//!
//! After Story 40.3 (Path B literal-green), current P5–P7 completion is
//! FR94–105; this suite only asserts the historical FR87 framing remains
//! documented — it does **not** require forbidding literal-green as the
//! current completion discipline (see `fr94_doc19_literal_green`).
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
fn fr87_doc19_p5_contract_green_historical() {
    let text = read_doc19();
    assert!(
        text.contains("19.7") || text.contains("阶段五"),
        "doc-19 must retain stage-five section"
    );
    assert!(
        (text.contains("合同绿") || text.contains("FR87") || text.contains("历史"))
            && (text.contains("外挂") || text.contains("Bambu") || text.contains("HLS"))
            && (text.contains("可复现") || text.contains("复现") || text.contains("薄 IP")),
        "P5 must still document Phase 11 contract-green historical baseline (external HLS / thin IP)"
    );
    assert!(
        (text.contains("VCD") || text.contains("波形"))
            && (text.contains("层次") || text.contains("可视化")),
        "P5 historical baseline must mention VCD / hierarchy visualization"
    );
}

#[test]
fn fr87_doc19_p6_contract_green_historical() {
    let text = read_doc19();
    assert!(
        text.contains("19.8") || text.contains("阶段六"),
        "doc-19 must retain stage-six section"
    );
    assert!(
        (text.contains("精选 IP") || text.contains("深度合同") || text.contains("非 VIP"))
            && text.contains("IP"),
        "P6 must still document Phase 11 selected IP depth (non-VIP) as historical"
    );
    assert!(
        text.contains("文档站") || text.contains("文档网站"),
        "P6 historical / overlapping baseline must mention docs site"
    );
    assert!(
        text.contains("rust-analyzer")
            && (text.contains("宿主") || text.contains("LSP") || text.contains("历史")),
        "P6 must still document host rust-analyzer path as historical contract-green"
    );
}

#[test]
fn fr87_doc19_p7_contract_green_historical() {
    let text = read_doc19();
    assert!(
        text.contains("19.9") || text.contains("阶段七"),
        "doc-19 must retain stage-seven section"
    );
    assert!(
        (text.contains("同刺激") || text.contains("共享刺激"))
            && (text.contains("功能") || text.contains("周期")),
        "P7 must still document shared-stimulus functional/cycle paths as historical baseline"
    );
    assert!(
        (text.contains("adapter") || text.contains("适配器"))
            && (text.contains("模板") || text.contains("桥接")),
        "P7 must still document bridge adapter template as historical baseline"
    );
    // Historical Phase 11 framing: contract-green did not treat auto-equiv / TLM as P7 green.
    // After Path B those are FR100/FR101 literal checkboxes — do not require "refuse as completion".
    assert!(
        text.contains("FR87")
            || text.contains("合同绿")
            || text.contains("历史里程碑")
            || text.contains("历史合同绿"),
        "P7 must retain Phase 11 contract-green historical framing"
    );
}

#[test]
fn fr87_doc19_contract_green_as_historical_milestone() {
    let text = read_doc19();
    assert!(
        (text.contains("合同绿") || text.contains("FR87") || text.contains("NFR38"))
            && (text.contains("历史") || text.contains("里程碑") || text.contains("Phase 11")),
        "doc-19 must frame Phase 11 contract-green as historical milestone"
    );
}

#[test]
fn fr87_readme_and_deferred_retain_contract_green_trace() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");
    assert!(
        readme.contains("状态与 deferred") || readme.contains("状态与 deferred（诚实声明）"),
        "README must keep 状态与 deferred section"
    );
    assert!(
        (readme.contains("合同绿") || readme.contains("FR87") || readme.contains("NFR38"))
            && (readme.contains("历史")
                || readme.contains("里程碑")
                || readme.contains("Phase 11")),
        "README must retain contract-green as historical milestone pointer"
    );

    let deferred =
        fs::read_to_string(root.join("_agile-output/implementation-artifacts/deferred-work.md"))
            .expect("deferred-work.md");
    assert!(
        deferred.contains("合同绿")
            || deferred.contains("FR87")
            || deferred.contains("NFR38")
            || deferred.contains("历史"),
        "deferred-work must retain contract-green / FR87 historical trace"
    );
}

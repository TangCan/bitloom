//! ATDD / guardrail: Epic 47 NFR14 risk record for interactive
//! waveform (FR104) + coverage extension (FR105) (Story 47.1 / AD-28).
//! Red if file missing or required sections absent.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic47_waveform_coverage
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic47_waveform_coverage_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic47-waveform-coverage.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    // (a)–(d) mandatory NFR14 fields
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

    // Interactive acceptance: browse / zoom / search or embedded viewer
    assert!(
        (text.contains("交互") || text.contains("interactive") || text.contains("Interactive"))
            && (text.contains("浏览") || text.contains("browse") || text.contains("I1"))
            && (text.contains("缩放") || text.contains("zoom") || text.contains("I2"))
            && (text.contains("检索")
                || text.contains("search")
                || text.contains("过滤")
                || text.contains("I3")),
        "risk record must nail interactive acceptance (browse/zoom/search)"
    );
    assert!(
        text.contains("嵌入")
            || text.contains("查看器")
            || text.contains("viewer")
            || text.contains("E1"),
        "risk record must allow embedded viewer contract as alternative path"
    );

    // Coverage metric types + report format
    assert!(
        (text.contains("覆盖率") || text.contains("coverage") || text.contains("Coverage"))
            && (text.contains("度量")
                || text.contains("metric")
                || text.contains("C1")
                || text.contains("C2"))
            && (text.contains("报告")
                || text.contains("report")
                || text.contains("格式")
                || text.contains("R1")),
        "risk record must nail coverage metric types and report format"
    );
    assert!(
        text.contains("记录器") || text.contains("recorder") || text.contains("R2"),
        "risk record must require a coverage recorder implementation"
    );

    // Forbidden: close FR104 on static HTML alone
    assert!(
        (text.contains("静态") || text.contains("static") || text.contains("timing.html"))
            && (text.contains("FR104") || text.contains("关闭"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("仅")
                || text.contains("alone")),
        "risk record must forbid closing FR104 on static HTML alone"
    );

    // Forbidden: close FR105 by docs-only without recorder
    assert!(
        (text.contains("文档") || text.contains("docs") || text.contains("README"))
            && (text.contains("FR105") || text.contains("覆盖率"))
            && (text.contains("记录器") || text.contains("recorder"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("仅")
                || text.contains("alone")),
        "risk record must forbid docs-only FR105 close without recorder"
    );

    // Must not confuse with FR38/49 VCD / hierarchy / timing HTML
    assert!(
        (text.contains("FR38") || text.contains("FR49"))
            && (text.contains("VCD")
                || text.contains("timing.html")
                || text.contains("层次")
                || text.contains("时序"))
            && (text.contains("≠")
                || text.contains("不是")
                || text.contains("不得")
                || text.contains("对照")
                || text.contains("混淆")),
        "risk record must contrast FR38/49 VCD/timing HTML with FR104"
    );

    // Must not confuse with FR34 baseline coverage
    assert!(
        text.contains("FR34")
            && (text.contains("≠")
                || text.contains("不是")
                || text.contains("不得")
                || text.contains("对照")
                || text.contains("基线")),
        "risk record must contrast FR34 baseline coverage with FR105"
    );

    // Gate: 47.2–47.3 must not be ready without this record
    assert!(
        text.contains("47.2")
            && text.contains("47.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 47.2–47.3 from ready without this record"
    );

    assert!(text.contains("FR104"), "risk record must name FR104");
    assert!(text.contains("FR105"), "risk record must name FR105");
    assert!(
        text.contains("Epic 47") || text.contains("Epic47"),
        "risk record must name Epic 47"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR40") && text.contains("负责人"),
        "risk record must assign NFR40 ownership alongside NFR14"
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
        text.contains("Epic 40") || text.contains("FR94"),
        "risk record must acknowledge Epic 40 / FR94 gate already closed"
    );
}

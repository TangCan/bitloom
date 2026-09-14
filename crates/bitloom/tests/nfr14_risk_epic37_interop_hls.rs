//! ATDD / guardrail: Epic 37 NFR14 risk record for interop hardening /
//! HLS honesty (Story 37.1 / AD-28 / FR88 / NFR12 / NFR39). Red if
//! file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic37_interop_hls_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md");
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

    // firtool / Chisel pin drift risk
    assert!(
        (text.contains("firtool") || text.contains("FIRTOOL"))
            && (text.contains("Chisel") || text.contains("chisel"))
            && (text.contains("漂移") || text.contains("钉死") || text.contains("升")),
        "risk record must cover firtool/Chisel pin drift risk"
    );
    assert!(
        (text.contains("7.14.0") || text.contains("7.15.0"))
            && (text.contains("1.155.0") || text.contains("1.158.0") || text.contains("1.159.0")),
        "risk record must cite current NFR12 pin pair Chisel 7.14.0 ↔ firtool 1.155.0"
    );

    // Mechanical Chisel mistaken for idiomatic
    assert!(
        (text.contains("机械") || text.contains("可编译") || text.contains("mechanical"))
            && (text.contains("idiomatic") || text.contains("惯用") || text.contains("可维护"))
            && (text.contains("≠")
                || text.contains("不是")
                || text.contains("误读")
                || text.contains("误")),
        "risk record must cover mechanical Chisel mistaken for idiomatic risk"
    );

    // Nightly real Bambu vs stub default
    assert!(
        (text.contains("Bambu") || text.contains("bambu"))
            && (text.contains("stub") || text.contains("Stub"))
            && (text.contains("夜间") || text.contains("真机") || text.contains("默认")),
        "risk record must cover nightly real Bambu vs stub default path"
    );

    // Forbidden: silent firtool bump
    assert!(
        (text.contains("firtool") || text.contains("FIRTOOL"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("私自") || text.contains("升") || text.contains("交差")),
        "risk record must forbid silent / unilateral firtool bumps as done"
    );

    // Forbidden: stub CI as HLS quality
    assert!(
        (text.contains("stub") || text.contains("Stub"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("HLS") || text.contains("质量") || text.contains("已验")),
        "risk record must forbid writing stub CI as HLS quality verified"
    );

    // Forbidden: continue-on-error masking real failures
    assert!(
        text.contains("continue-on-error")
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("真机") || text.contains("失败") || text.contains("掩盖")),
        "risk record must forbid continue-on-error masking real Bambu failures"
    );

    // Gate: 37.2–37.3 must not be ready without this record
    assert!(
        text.contains("37.2")
            && text.contains("37.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 37.2–37.3 from ready without this record"
    );

    assert!(text.contains("FR88"), "risk record must cite FR88");
    assert!(
        text.contains("NFR12") && text.contains("NFR39"),
        "risk record must cite NFR12 and NFR39"
    );
    assert!(
        text.contains("Epic 37") || text.contains("Epic37"),
        "risk record must name Epic 37"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR12") && text.contains("负责人") && text.contains("NFR39"),
        "risk record must assign NFR12 / NFR39 ownership alongside NFR14"
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
        text.contains("AD-9") || text.contains("AD-25") || text.contains("AD-27"),
        "risk record must cite AD-9 / AD-25 / AD-27 context"
    );
}

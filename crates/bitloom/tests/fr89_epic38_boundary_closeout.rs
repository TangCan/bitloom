//! ATDD (red→green): Story 38.3 / FR89 — Epic 38 boundary closeout.
//!
//! Locks NFR14 Epic 38 close-condition checkboxes, deferred-work cross-ref
//! (full protocol still needs new contract; Story 38.3 closed the pointer),
//! deepen-path ATDD stability, and no branch-B RX delivery claims.
//! Optional `examples/ip_box` baud_div demo is not required for close.
//!
//! ```text
//! cargo test -p bitloom --test fr89_epic38_boundary_closeout
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
fn fr89_nfr14_epic38_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md");
    assert!(
        text.contains("- [x] **FR89：**") || text.contains("- [x] **FR89:**"),
        "NFR14 must check FR89 deepen close condition"
    );
    assert!(
        text.contains("- [x] **文档边界：**") || text.contains("- [x] **文档边界:**"),
        "NFR14 must check docs-boundary close condition"
    );
    assert!(
        text.contains("- [x] **未选分支：**") || text.contains("- [x] **未选分支:**"),
        "NFR14 must check unselected branch B (RX) non-delivery"
    );
    assert!(
        text.contains("- [x] **ATDD / 收口：**") || text.contains("- [x] **ATDD / 收口:**"),
        "NFR14 must check ATDD / closeout condition"
    );
    assert!(
        text.contains("- [x] **禁止事项未触发：**") || text.contains("- [x] **禁止事项未触发:**"),
        "NFR14 must check forbidden-actions close condition"
    );
    assert!(
        text.contains("- [x] **品牌 / 依赖：**") || text.contains("- [x] **品牌 / 依赖:**"),
        "NFR14 must check brand / prelude-only close condition"
    );
    assert!(
        text.contains("closed — Story 38.3") || text.contains("closed — Story 38.3"),
        "NFR14 Epic 38 record status must be closed after 38.3"
    );
}

#[test]
fn fr89_deferred_full_protocol_cross_ref_closed_by_38_3() {
    let text = read("_agile-output/implementation-artifacts/deferred-work.md");
    // epic-22-retro-item-49 area: full protocol still needs new contract
    assert!(
        text.contains("全协议")
            && (text.contains("仍须新合同")
                || text.contains("须显式改合同")
                || text.contains("须新合同")),
        "deferred-work must keep 'full protocol still needs new contract' narrative"
    );
    assert!(
        text.contains("Story 38.3") || text.contains("38.3"),
        "deferred-work must cross-ref Story 38.3 closeout"
    );
    // Must not leave the open pointer "关闭交叉引用 → Story 38.3" without closure note
    assert!(
        !text.contains("关闭交叉引用 → Story 38.3"),
        "deferred-work must close the '→ Story 38.3' open pointer after closeout"
    );
    // FR89 subset ≠ 全家桶
    assert!(
        text.contains("38.2")
            && (text.contains("子集") || text.contains("baud_div") || text.contains("可编程")),
        "deferred-work must note FR89 programmable-baud subset (not full family)"
    );
}

#[test]
fn fr89_deepen_atdd_fixture_still_present() {
    let path = workspace_root().join("crates/bitloom/tests/fr89_uarttx_programmable_baud.rs");
    assert!(
        path.is_file(),
        "FR89 deepen ATDD fixture fr89_uarttx_programmable_baud.rs must remain"
    );
    let src = fs::read_to_string(&path).expect("read fr89 deepen ATDD");
    assert!(
        src.contains("fr89_uarttx_programmable_baud_elaborate_emit_tick")
            && src.contains("baud_div"),
        "deepen ATDD must keep baud_div elaborate→emit→tick coverage"
    );
    assert!(
        src.contains("fr89_docs_ip_readme_documents_baud_subset_and_non_goals"),
        "deepen ATDD must keep docs boundary assertions"
    );
}

#[test]
fn fr89_no_branch_b_rx_delivery_claims() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md");
    let readme = read("docs/ip/README.md");
    let ip = read("crates/bitloom-prelude/src/ip.rs");
    assert!(
        !readme.contains("UartRx") && !ip.contains("struct UartRx"),
        "must not deliver UartRx / claim branch B"
    );
    assert!(
        nfr.contains("分支 B") && (nfr.contains("不选用") || nfr.contains("不得")),
        "NFR14 must keep branch B as not selected / not delivered"
    );
    assert!(
        !nfr.contains("最小 RX 已交付") && !readme.contains("最小 RX 已交付"),
        "must not claim minimal RX delivered"
    );
}

#[test]
fn fr89_docs_ip_keeps_fr89_subset_and_non_goals() {
    let readme = read("docs/ip/README.md");
    assert!(
        (readme.contains("FR89") || readme.contains("Epic 38"))
            && (readme.contains("baud_div") || readme.contains("可编程")),
        "docs/ip must keep FR89 programmable baud subset"
    );
    assert!(
        readme.contains("RX")
            && (readme.contains("非目标") || readme.contains("非"))
            && (readme.contains("VIP") || readme.contains("全协议"))
            && readme.contains("全双工"),
        "docs/ip must keep RX / VIP|全协议 / 全双工 as non-goals"
    );
}

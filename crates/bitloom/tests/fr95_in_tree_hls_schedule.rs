//! ATDD Story 41.2 / FR95 — in-tree `#[hls]` schedule MVP (loop-unroll).
//!
//! Must prove a checkable schedule/IR artifact **without** invoking external Bambu.
//! External stub / `BITLOOM_HLS_USE_REAL` must not alone satisfy this FR.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn tmp_out(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bitloom_fr95_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

#[test]
fn fr95_in_tree_loop_unroll_emits_schedule_ir_without_bambu() {
    // In-tree path must succeed without consulting BITLOOM_BAMBU_PATH / spawning bambu.
    let out = tmp_out("unroll");
    let artifact = bitloom::hls::schedule_in_tree(
        "map_add1",
        bitloom::hls::HlsDataflowOp::AddConst(1),
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 4 },
    )
    .expect("in-tree schedule must succeed without Bambu");

    assert_eq!(
        artifact.stages.len(),
        4,
        "loop-unroll must produce trip_count stages"
    );
    let ir = &artifact.schedule_ir;
    assert!(
        ir.contains("fr95") || ir.contains("FR95"),
        "schedule IR must cite FR95: {ir}"
    );
    assert!(
        ir.to_lowercase().contains("loop-unroll") || ir.to_lowercase().contains("loop_unroll"),
        "schedule IR must name loop-unroll: {ir}"
    );
    assert!(
        ir.contains("trip_count") && ir.contains('4'),
        "schedule IR must record trip_count=4: {ir}"
    );
    assert!(
        ir.contains("stage") || ir.contains("Stage"),
        "schedule IR must list stages: {ir}"
    );
    assert!(
        !ir.to_lowercase().contains("bambu"),
        "in-tree schedule IR must not claim Bambu: {ir}"
    );

    let (sched_path, _) =
        bitloom::hls::emit_in_tree_schedule(&artifact, &out).expect("emit schedule artifacts");
    assert!(
        sched_path.is_file(),
        "schedule file missing: {}",
        sched_path.display()
    );
    let on_disk = fs::read_to_string(&sched_path).expect("read schedule");
    assert!(
        on_disk.contains("fr95") || on_disk.contains("FR95"),
        "emitted schedule must cite FR95"
    );
}

#[test]
fn fr95_in_tree_emits_rtl_stub_marked_mvp() {
    let out = tmp_out("rtl");
    let artifact = bitloom::hls::schedule_in_tree(
        "xor_pipe",
        bitloom::hls::HlsDataflowOp::XorConst(0xa5),
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 2 },
    )
    .unwrap();
    let (_, rtl_path) = bitloom::hls::emit_in_tree_schedule(&artifact, &out).unwrap();
    assert!(rtl_path.is_file(), "rtl stub missing");
    let v = fs::read_to_string(&rtl_path).unwrap();
    assert!(
        v.contains("in-tree-mvp") || v.contains("FR95"),
        "RTL stub must be honestly marked in-tree MVP / FR95: {v}"
    );
    assert!(
        v.contains("module") && v.contains("xor_pipe"),
        "RTL stub must be a named module: {v}"
    );
}

#[test]
fn fr95_in_tree_rejects_zero_unroll() {
    let err = bitloom::hls::schedule_in_tree(
        "bad",
        bitloom::hls::HlsDataflowOp::Identity,
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 0 },
    )
    .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("trip_count")
            || msg.contains("unroll")
            || msg.contains("≥1")
            || msg.contains(">= 1"),
        "zero unroll must fail readably: {msg}"
    );
}

#[test]
fn fr95_docs_distinguish_in_tree_vs_external() {
    let root = workspace_root();
    let fr35 = fs::read_to_string(root.join("docs/fr35-hls.md")).expect("fr35");
    assert!(
        fr35.contains("FR95")
            && (fr35.contains("树内") || fr35.contains("in-tree") || fr35.contains("自研")),
        "docs must name FR95 in-tree completion surface"
    );
    assert!(
        (fr35.contains("不得单独") || fr35.contains("不可单独") || fr35.contains("不得单独满足"))
            && (fr35.contains("外挂") || fr35.contains("Bambu") || fr35.contains("bambu")),
        "docs must state external path alone does not satisfy FR95"
    );
    assert!(
        fr35.contains("Bitloom") || fr35.contains("bitloom"),
        "docs must keep Bitloom brand"
    );
    // Must not keep absolute "never in-tree" as the only product rule.
    let lower = fr35.to_lowercase();
    let still_absolute_ban = fr35.contains("永不")
        && (fr35.contains("树内调度") || lower.contains("never") && lower.contains("schedul"));
    assert!(
        !still_absolute_ban || fr35.contains("FR95"),
        "docs must not leave absolute ban on in-tree scheduling without FR95 carve-out"
    );
}

#[test]
fn fr95_external_path_api_still_present() {
    // Regression: FR35 external dissolve/emit must remain callable.
    let dir = tmp_out("external_still");
    let p = bitloom::hls::emit_c_stub("add", &dir).expect("external emit_c_stub");
    assert!(p.is_file());
    let text = fs::read_to_string(p).unwrap();
    assert!(text.contains("unsigned add"));
}

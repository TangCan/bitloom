//! ATDD Story 41.3 / FR96 — HLS closure dataflow transform before in-tree schedule.
//!
//! Closure dissolve/inline must feed Story 41.2 `schedule_in_tree` path.
//! Capturing / illegal surfaces fail readably before schedule.
//! External FR76 dissolve alone does **not** satisfy FR96.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn tmp_out(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bitloom_fr96_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

#[test]
fn fr96_closure_transform_enters_in_tree_schedule() {
    let out = tmp_out("from_transform");
    let artifact = bitloom::hls::schedule_in_tree_from_transform(
        "map_xor",
        &[],
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 3 },
        || bitloom::hls::HlsDataflowOp::XorConst(0xa5),
    )
    .expect("FR96: non-capturing transform must dissolve then schedule in-tree");

    assert_eq!(artifact.stages.len(), 3);
    let ir = &artifact.schedule_ir;
    assert!(
        ir.contains("fr96") || ir.contains("FR96"),
        "schedule IR must cite FR96: {ir}"
    );
    assert!(
        ir.contains("fr95") || ir.contains("FR95"),
        "FR96 path must enter 41.2 FR95 schedule: {ir}"
    );
    assert!(
        ir.to_lowercase().contains("loop-unroll") || ir.to_lowercase().contains("loop_unroll"),
        "must retain 41.2 loop-unroll: {ir}"
    );
    assert!(
        !ir.to_lowercase().contains("bambu"),
        "in-tree FR96 must not claim Bambu: {ir}"
    );

    let (sched_path, _) =
        bitloom::hls::emit_in_tree_schedule(&artifact, &out).expect("emit schedule");
    assert!(sched_path.is_file());
    let on_disk = fs::read_to_string(&sched_path).expect("read");
    assert!(
        on_disk.contains("fr96") || on_disk.contains("FR96"),
        "emitted schedule must cite FR96"
    );
}

#[test]
fn fr96_capturing_closure_rejected_before_schedule() {
    let out = tmp_out("capturing");
    let err = bitloom::hls::schedule_in_tree_from_transform(
        "bad_capture",
        &[bitloom::hls::HlsDataflowClosureViolation::capturing(
            "captures mut counter",
        )],
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 2 },
        || bitloom::hls::HlsDataflowOp::AddConst(1),
    )
    .expect_err("capturing closure must fail before schedule");

    let msg = err.to_string().to_lowercase();
    assert!(
        msg.contains("captur") || msg.contains("reject") || msg.contains("hls"),
        "error must be readable about capture/reject: {err}"
    );
    assert!(
        !out.join("bad_capture.schedule.json").is_file(),
        "must not emit schedule after capture reject"
    );
}

#[test]
fn fr96_wrong_path_rejected_before_schedule() {
    let err = bitloom::hls::schedule_in_tree_from_transform(
        "wrong_path",
        &[
            bitloom::hls::HlsDataflowClosureViolation::wrong_path_synthesizable(
                "HlsFree on synthesizable comb path",
            ),
        ],
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 2 },
        || bitloom::hls::HlsDataflowOp::Identity,
    )
    .expect_err("wrong-path token must fail before schedule");

    let msg = err.to_string().to_lowercase();
    assert!(
        msg.contains("path") || msg.contains("reject") || msg.contains("synthesiz"),
        "error must mention wrong path / synthesizable: {err}"
    );
}

#[test]
fn fr96_schedule_has_no_closure_residue() {
    let artifact = bitloom::hls::schedule_in_tree_from_transform(
        "id_pipe",
        &[],
        bitloom::hls::InTreeScheduleKind::LoopUnroll { trip_count: 2 },
        || bitloom::hls::HlsDataflowOp::Identity,
    )
    .unwrap();
    let lower = artifact.schedule_ir.to_lowercase();
    assert!(
        !lower.contains("closure")
            && !lower.contains("callback")
            && !artifact.schedule_ir.contains("||"),
        "NFR36: schedule IR must not retain closure residue:\n{}",
        artifact.schedule_ir
    );
    let rtl_lower = artifact.rtl_stub.to_lowercase();
    assert!(
        !rtl_lower.contains("closure") && !rtl_lower.contains("callback"),
        "NFR36: RTL stub must not retain closure residue"
    );
}

#[test]
fn fr96_docs_crosslink_fr72_through_fr78() {
    let root = workspace_root();
    let fr35 = fs::read_to_string(root.join("docs/fr35-hls.md")).expect("fr35-hls.md");
    let lower = fr35.to_lowercase();

    assert!(
        fr35.contains("FR96") || lower.contains("fr96"),
        "docs must name FR96"
    );
    assert!(
        lower.contains("闭包") || lower.contains("closure") || lower.contains("transform"),
        "FR96 docs must describe closure/transform"
    );
    // Cross-links / citations to FR72–78 closure contract family
    for needle in ["FR72", "FR73", "FR74", "FR75", "FR76", "FR77", "FR78"] {
        assert!(
            fr35.contains(needle),
            "docs/fr35-hls.md must cross-link or cite {needle} in FR96/closure contract section"
        );
    }
    assert!(
        fr35.contains("fr22-construct-bar")
            || fr35.contains("fr78-bridge-adapter")
            || fr35.contains("docs/ip"),
        "FR96 docs should point at FR72–78 doc anchors"
    );
    assert!(
        fr35.contains("Bitloom") || fr35.contains("bitloom"),
        "public brand must remain Bitloom"
    );
    assert!(
        !lower.contains("fr96 done via bambu") && !lower.contains("only external satisfies fr96"),
        "must not claim external-only FR96"
    );
}

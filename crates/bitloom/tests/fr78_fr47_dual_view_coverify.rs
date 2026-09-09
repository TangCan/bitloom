//! ATDD Story 30.3: FR78 `start_wait_complete` × FR47 dual-view co-verification.
//!
//! Functional/bridge side uses the FR78 template (free closures) to emit
//! transaction-shaped PortValues stimuli; cycle-accurate side ticks via the
//! existing FR47 generated-path bridge (`check_functional_equiv_generated`).
//! Match → pass; deliberate mismatch → fail (FR30 spirit). FR16 capture ban
//! and NFR36 emit/HIR spot-checks stay green.
//!
//! Recipe:
//! ```text
//! cargo test -p bitloom --test fr78_fr47_dual_view_coverify
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_prelude::{StartWaitComplete, start_wait_complete};
use bitloom_sim::{
    AbstractionView, check_functional_equiv_generated, check_generated_bridge_with,
    generate_cycle_accurate_sim, generate_functional_sim,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

/// Flat Counter — FR47 MVP subset (no Mem / hierarchy).
fn counter_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("CoverifyCounter");
    s.begin_module("CoverifyCounter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("Counter elaborate")
}

/// Host that records PortValues each tick while running FR78 start→wait→complete.
/// Closures stay on the host stack; recorded frames are ordinary signal maps.
struct TemplateStimulusHost {
    frames: Vec<PortValues>,
    data_in: u8,
    busy_remaining: u32,
}

impl StartWaitComplete for TemplateStimulusHost {
    fn is_busy(&self) -> bool {
        self.busy_remaining > 0
    }

    fn tick(&mut self) {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("data_in", u64::from(self.data_in));
        self.frames.push(pv);
        if self.busy_remaining > 0 {
            self.busy_remaining -= 1;
        }
    }
}

/// Reset-high + FR78 template-driven transaction frames (Cap-R-66/67 free closure).
fn template_driven_stimuli() -> Vec<PortValues> {
    let mut host = TemplateStimulusHost {
        frames: Vec::new(),
        data_in: 0,
        busy_remaining: 0,
    };

    let mut rst = PortValues::default();
    rst.set("rst", 1);
    rst.set("data_in", 0);
    host.frames.push(rst);

    // Functional-side free closure: start a 4-cycle "transaction" payload.
    host.start_wait_complete(|h| {
        h.data_in = 0x5A;
        h.busy_remaining = 4;
    });

    // Free-fn form: second transaction (documented equivalent).
    start_wait_complete(
        &mut host,
        |h| h.tick(),
        |h| h.is_busy(),
        |h| {
            h.data_in = 0xA5;
            h.busy_remaining = 2;
        },
    );

    host.frames
}

/// NFR36: FrozenHir / Verilog / FIRRTL must not encode closure IR tokens.
/// (Generated *Rust* functional crates may use ordinary host `||` — that is not HIR.)
fn assert_no_closure_ir(label: &str, text: &str) {
    let lower = text.to_lowercase();
    let has_fn_ir = ["Fn(", "Fn (", "FnOnce", "FnMut", "dyn Fn"]
        .iter()
        .any(|needle| {
            let mut start = 0;
            while let Some(rel) = text[start..].find(needle) {
                let abs = start + rel;
                let prev_ok = abs == 0 || !text.as_bytes()[abs - 1].is_ascii_alphanumeric();
                if prev_ok {
                    return true;
                }
                start = abs + 1;
            }
            false
        });
    let bad =
        lower.contains("closure") || lower.contains("callback") || has_fn_ir || text.contains("||");
    assert!(
        !bad,
        "{label}: must not contain closure/callback IR (NFR36):\n{text}"
    );
}

/// Softer NFR36 for generated Rust crates: no closure *IR types*, host `||` OK.
fn assert_no_closure_ir_types(label: &str, text: &str) {
    let lower = text.to_lowercase();
    let bad = lower.contains("stmt::closure")
        || lower.contains("enum closure")
        || lower.contains("callback")
        || text.contains("FnOnce")
        || text.contains("FnMut")
        || text.contains("dyn Fn");
    assert!(
        !bad,
        "{label}: generated Rust must not encode closure IR types (NFR36):\n{text}"
    );
}

#[test]
fn fr78_fr47_template_stimuli_match_generated_bridge() {
    let hir = counter_hir();
    let stimuli = template_driven_stimuli();
    assert!(
        stimuli.len() >= 5,
        "template must record reset + transaction ticks"
    );

    assert!(
        check_functional_equiv_generated(hir.clone(), stimuli.clone()).is_pass(),
        "FR78 template stimuli must pass FR47 generated-path bridge (FR30 spirit)"
    );

    // Deliberate breakage must fail.
    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 0xFF);
            o
        }
    }
    let mut wrong = Wrong;
    assert!(
        !check_generated_bridge_with(hir, &mut wrong, stimuli).is_pass(),
        "deliberate functional mismatch must fail (FR30 / NFR14)"
    );
}

#[test]
fn fr78_fr47_generate_paths_smoke_and_nfr36() {
    let hir = counter_hir();
    let root = workspace_root();
    let func_out = root.join("target/bitloom-fr78-fr47-func-coverify");
    let cycle_out = root.join("target/bitloom-fr78-fr47-cycle-coverify");
    let _ = fs::remove_dir_all(&func_out);
    let _ = fs::remove_dir_all(&cycle_out);

    generate_functional_sim(&hir, &func_out).expect("generate_functional_sim");
    generate_cycle_accurate_sim(&hir, &cycle_out).expect("generate_cycle_accurate_sim");

    let func_lib = fs::read_to_string(func_out.join("src/lib.rs")).expect("func lib.rs");
    let cycle_lib = fs::read_to_string(cycle_out.join("src/lib.rs")).expect("cycle lib.rs");
    assert_no_closure_ir_types("generate_functional_sim lib.rs", &func_lib);
    assert_no_closure_ir_types("generate_cycle_accurate_sim lib.rs", &cycle_lib);

    let hir_dbg = format!("{hir:?}");
    // FrozenHir Debug may contain `|` in formatting; ban explicit closure IR names only.
    assert_no_closure_ir_types("FrozenHir Debug", &hir_dbg);
    assert!(
        !hir_dbg.to_lowercase().contains("closure"),
        "FrozenHir Debug must not mention closure IR"
    );

    let v = bitloom_vlog::emit(&hir).files[0].contents.clone();
    let fir = rhdl_firrtl::emit(&hir).files[0].contents.clone();
    assert_no_closure_ir("verilog emit", &v);
    assert_no_closure_ir("firrtl emit", &fir);
}

#[test]
fn fr16_capturing_closure_still_rejected_on_cycle_path() {
    // FR16 / AD-18 regression sample: capturing closure remains E0141.
    let mut s = ElaborateSession::new("Fr16Coverify");
    s.begin_module("Fr16Coverify", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.reject_unsynthesizable("capturing closure", Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s
        .finish()
        .expect_err("FR16 capturing closure must stay red");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0141"),
        "FR16 regression expects E0141, got: {err}"
    );
    assert!(
        !err.0.iter().any(|d| d.code == "rhdl::E0142"),
        "FR16 must not be reclassified as E0142"
    );
}

#[test]
fn fr78_fr47_docs_cross_link_coverify() {
    let root = workspace_root();
    let fr78 =
        fs::read_to_string(root.join("docs/fr78-bridge-adapter-closures.md")).expect("fr78 doc");
    assert!(
        fr78.contains("fr78_fr47_dual_view_coverify")
            || (fr78.contains("FR47") && fr78.contains("30.3")),
        "fr78 doc must name Story 30.3 / coverify ATDD"
    );
    assert!(
        fr78.contains("check_functional_equiv_generated")
            || fr78.contains("check_generated_bridge"),
        "fr78 doc must cite FR47 bridge entry points"
    );

    let surface =
        fs::read_to_string(root.join("_agile-output/specs/spec-rhdl/language-surface.md"))
            .expect("language-surface");
    assert!(
        surface.contains("FR78")
            && (surface.contains("30.3") || surface.contains("fr78_fr47_dual_view_coverify")),
        "language-surface must mark FR78×FR47 coverify"
    );
}

//! ATDD: FR73 elaborate-time module factory closures (Story 27.2).
//!
//! Design surface is `bitloom-prelude` / `ElaborateSession`; factory `Fn`
//! runs at elaborate time and dissolves to ordinary Instance/Connect HIR
//! (NFR36 / Cap-R-53).

use bitloom_hir::Stmt;
use bitloom_prelude::{ElaborateSession, GeneratedInstance, GroundType, Span};

fn declare_lane(s: &mut ElaborateSession) {
    s.begin_module("Lane", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "x", Span::default());
    s.end_process();
    s.end_module();
}

#[test]
fn fr73_factory_batches_instances_emit_hierarchy_and_connects() {
    const N: usize = 4;
    let mut s = ElaborateSession::new("BusTop");
    declare_lane(&mut s);

    s.begin_module("BusTop", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    for i in 0..N {
        s.add_input(
            format!("x{i}"),
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_output(
            format!("y{i}"),
            GroundType::UInt { width: 8 },
            Span::default(),
        );
    }
    // Elaborate-time factory Fn — less boilerplate than N hand-written add_instance (FR73).
    s.generate_instances(N, |i, sess| {
        sess.add_instance(
            format!("u{i}"),
            "Lane",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), format!("x{i}")),
                ("y".into(), format!("y{i}")),
            ],
            vec![],
            Span::default(),
        );
    });
    s.end_module();
    let frozen = s.finish().expect("elaborate");

    let top = frozen
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "BusTop")
        .expect("BusTop");
    let instances: Vec<_> = top
        .body
        .iter()
        .filter_map(|st| match st {
            Stmt::Instance(inst) => Some(inst),
            _ => None,
        })
        .collect();
    assert_eq!(instances.len(), N, "factory must emit N instances");
    for (i, inst) in instances.iter().enumerate() {
        assert_eq!(inst.name, format!("u{i}"));
        assert_eq!(inst.module, "Lane");
        let x = inst
            .connects
            .iter()
            .find(|c| c.child_port == "x")
            .expect("x connect");
        assert_eq!(x.parent_net, format!("x{i}"));
        let y = inst
            .connects
            .iter()
            .find(|c| c.child_port == "y")
            .expect("y connect");
        assert_eq!(y.parent_net, format!("y{i}"));
    }

    // NFR36: FrozenHir holds plain Instance stmts — no closure residue.
    for st in &top.body {
        let debug = format!("{st:?}");
        assert!(
            !debug.to_lowercase().contains("closure")
                && !debug.contains("FnOnce")
                && !debug.contains("FnMut")
                && !debug.contains("GeneratedInstance"),
            "FrozenHir must not retain factory/closure residue: {debug}"
        );
    }

    let v = bitloom_vlog::emit(&frozen);
    let vlog = v
        .files
        .iter()
        .find(|f| f.path.ends_with("BusTop.v") || f.contents.contains("module BusTop"))
        .map(|f| f.contents.as_str())
        .unwrap_or(&v.files[0].contents);
    for i in 0..N {
        assert!(
            vlog.contains(&format!("Lane u{i}"))
                && vlog.contains(&format!(".x(x{i})"))
                && vlog.contains(&format!(".y(y{i})")),
            "instance u{i} / connects missing in .v:\n{vlog}"
        );
    }

    let fir = bitloom_firrtl::emit(&frozen);
    let fir = fir
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for i in 0..N {
        assert!(
            fir.contains(&format!("inst u{i} of Lane"))
                || (fir.contains(&format!("u{i}")) && fir.contains("Lane")),
            "FIRRTL hierarchy missing u{i}:\n{fir}"
        );
    }
}

#[test]
fn fr73_generate_instances_from_prelude_surface() {
    let mut s = ElaborateSession::new("Parent");
    declare_lane(&mut s);
    s.begin_module("Parent", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x0", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y0", GroundType::UInt { width: 8 }, Span::default());
    s.generate_instances_from(
        1,
        |i| {
            GeneratedInstance::new(
                format!("u{i}"),
                "Lane",
                vec![
                    ("clk".into(), "clk".into()),
                    ("rst".into(), "rst".into()),
                    ("x".into(), format!("x{i}")),
                    ("y".into(), format!("y{i}")),
                ],
                vec![],
            )
        },
        Span::default(),
    );
    s.end_module();
    let frozen = s.finish().unwrap();
    assert!(frozen.circuit().modules.iter().any(|m| {
        m.body.iter().any(
            |st| matches!(st, Stmt::Instance(inst) if inst.name == "u0" && inst.module == "Lane"),
        )
    }));
}

#[test]
fn fr73_factory_width_mismatch_fails_before_emit() {
    let mut s = ElaborateSession::new("Parent");
    declare_lane(&mut s); // child x/y are 8-bit
    s.begin_module("Parent", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    // Parent net is 16-bit — FR8 width check must fail at finish.
    s.add_input("x0", GroundType::UInt { width: 16 }, Span::default());
    s.add_output("y0", GroundType::UInt { width: 8 }, Span::default());
    s.generate_instances(1, |i, sess| {
        sess.add_instance(
            format!("u{i}"),
            "Lane",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), format!("x{i}")),
                ("y".into(), format!("y{i}")),
            ],
            vec![],
            Span::default(),
        );
    });
    s.end_module();
    let err = s
        .finish()
        .expect_err("width mismatch must fail before emit");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0203"),
        "expected E0203 width mismatch, got: {err}"
    );
}

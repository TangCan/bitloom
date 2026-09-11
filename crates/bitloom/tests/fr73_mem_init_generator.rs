//! ATDD: FR73 elaborate-time Mem/ROM init generator closures (Story 27.1).
//!
//! Design surface is `bitloom-prelude` / `ElaborateSession`; closures run at
//! elaborate time and dissolve to plain `MemDecl.init` words (NFR36).

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::Stmt;

#[test]
fn fr73_closure_built_lut_visible_in_verilog_and_firrtl() {
    let mut s = ElaborateSession::new("LutRom");
    s.begin_module("LutRom", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    // Elaborate-time generator Fn — not hand-written table (FR73).
    s.declare_mem_with_init_fn(
        "rom",
        8,
        8,
        |i| ((i.wrapping_mul(3).wrapping_add(1)) & 0xff) as u64,
        Span::default(),
    );
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();
    let frozen = s.finish().expect("elaborate");

    // NFR36: FrozenHir holds plain init words — no closure residue in Stmt/Expr.
    let init = frozen.circuit().modules[0]
        .body
        .iter()
        .find_map(|st| match st {
            Stmt::MemDecl {
                name,
                init: Some(words),
                ..
            } if name == "rom" => Some(words.clone()),
            _ => None,
        })
        .expect("MemDecl.init present after freeze");
    assert_eq!(
        init,
        vec![1, 4, 7, 10, 13, 16, 19, 22],
        "closure results must be dissolved to u64 words"
    );
    for st in &frozen.circuit().modules[0].body {
        let debug = format!("{st:?}");
        assert!(
            !debug.to_lowercase().contains("closure")
                && !debug.contains("FnOnce")
                && !debug.contains("FnMut"),
            "FrozenHir must not retain closure residue: {debug}"
        );
    }

    let v = bitloom_vlog::emit(&frozen);
    let vlog = &v.files[0].contents;
    assert!(
        vlog.contains("reg [7:0] rom [0:7]"),
        "mem decl missing in .v:\n{vlog}"
    );
    assert!(
        vlog.contains("initial begin"),
        "expected initial block for mem init:\n{vlog}"
    );
    assert!(
        vlog.contains("rom[0] = 1;") && vlog.contains("rom[7] = 22;"),
        "closure-built table not visible in .v:\n{vlog}"
    );

    let fir = bitloom_firrtl::emit(&frozen);
    let fir = &fir.files[0].contents;
    assert!(
        fir.contains("mem rom") || fir.contains("mem rom :"),
        "FIRRTL mem missing:\n{fir}"
    );
    assert!(
        fir.contains("mem-init rom") && fir.contains("1, 4, 7, 10, 13, 16, 19, 22"),
        "FIRRTL should comment dissolved init words:\n{fir}"
    );
}

#[test]
fn fr73_prelude_generate_mem_init_matches_session_fn() {
    // Design crates depend on bitloom-prelude; helper is re-exported there.
    let via_helper = bitloom_prelude::generate_mem_init(4, 8, |i| (i * i) as u64);
    let mut s = ElaborateSession::new("t");
    s.begin_module("M", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.declare_sync_read_mem_with_init_fn("lut", 4, 8, |i| (i * i) as u64, Span::default());
    s.end_module();
    let frozen = s.finish().unwrap();
    let via_session = frozen.circuit().modules[0]
        .body
        .iter()
        .find_map(|st| match st {
            Stmt::MemDecl {
                init: Some(words), ..
            } => Some(words.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!(via_helper, via_session);
    assert_eq!(via_helper, vec![0, 1, 4, 9]);
}

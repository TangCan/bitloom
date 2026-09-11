//! ATDD: FR81 Path A — `emit_chisel` lowers documented Mem subset (Story 33.3).
//!
//! Red if SyncReadMem/Mem (+ optional init) fail to emit Path A Scala, NFR12 pins
//! drift, or E0901 is deleted for out-of-subset MemDecl.

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{CHISEL_TARGET, FIRTOOL_TARGET, emit_chisel};
use bitloom_hir::{BuilderOwnedHir, Module, Port, PortDirection, Stmt, seal_from_builder};

fn pins_locked(scala: &str) {
    assert_eq!(CHISEL_TARGET, "7.14.0");
    assert_eq!(FIRTOOL_TARGET, "1.155.0");
    assert!(
        scala.contains("Chisel 7.14.0") && scala.contains("firtool-1.155.0"),
        "NFR12 pin strings missing:\n{scala}"
    );
}

#[test]
fn fr81_path_a_sync_read_mem_emits_chisel() {
    let mut s = ElaborateSession::new("Fr81Sync");
    s.begin_module("Fr81Sync", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("addr", GroundType::UInt { width: 2 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_sync_read_mem("ram", 4, 8, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.assign_reg_d_mem_read("q", "ram", "addr", Span::default());
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let scala = emit_chisel(&s.finish().unwrap())
        .expect("Path A SyncReadMem must emit")
        .files[0]
        .contents
        .clone();
    assert!(
        scala.contains("SyncReadMem(4, UInt(8.W))"),
        "expected SyncReadMem ctor:\n{scala}"
    );
    assert!(scala.contains("ram.read("), "expected mem read:\n{scala}");
    pins_locked(&scala);
}

#[test]
fn fr81_path_a_async_mem_with_init_emits_chisel() {
    let mut s = ElaborateSession::new("Fr81Mem");
    s.begin_module("Fr81Mem", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("addr", GroundType::UInt { width: 2 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_mem_with_init("rom", 4, 8, vec![10, 20, 30, 40], Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.assign_reg_d_mem_read("q", "rom", "addr", Span::default());
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let scala = emit_chisel(&s.finish().unwrap())
        .expect("Path A Mem+init must emit")
        .files[0]
        .contents
        .clone();
    assert!(
        scala.contains("Mem(4, UInt(8.W))"),
        "expected Mem:\n{scala}"
    );
    assert!(
        !scala.contains("SyncReadMem("),
        "async Mem must not use SyncReadMem:\n{scala}"
    );
    assert!(
        scala.contains("rom_init = VecInit(10.U(8.W), 20.U(8.W), 30.U(8.W), 40.U(8.W))"),
        "expected constant init:\n{scala}"
    );
    pins_locked(&scala);
}

#[test]
fn fr81_path_a_out_of_subset_still_e0901() {
    let mut owned = BuilderOwnedHir::new("OosMem");
    owned.add_module(Module {
        name: "OosMem".into(),
        ports: vec![
            Port {
                name: "clk".into(),
                direction: PortDirection::Input,
                ty: GroundType::Clock,
                span: Span::default(),
            },
            Port {
                name: "rst".into(),
                direction: PortDirection::Input,
                ty: GroundType::Reset,
                span: Span::default(),
            },
        ],
        body: vec![Stmt::MemDecl {
            name: "bad".into(),
            depth: 8,
            width: 8,
            sync_read: false,
            init: Some(vec![0, 1, 2]), // len ≠ depth
            span: Span::default(),
        }],
        span: Span::default(),
    });
    let frozen = seal_from_builder(owned).expect("seal");
    let err = emit_chisel(&frozen).expect_err("out-of-subset must keep E0901");
    assert_eq!(err.code, "rhdl::E0901");
}

#[test]
fn fr81_path_a_no_mem_counter_still_emits() {
    let mut s = ElaborateSession::new("Fr81Ctr");
    s.begin_module("Fr81Ctr", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("r", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("r", Span::default());
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("out", "r", Span::default());
    s.end_process();
    s.end_module();
    let scala = emit_chisel(&s.finish().unwrap())
        .expect("FR71 no-Mem path must still emit")
        .files[0]
        .contents
        .clone();
    assert!(!scala.contains("Mem(") && !scala.contains("SyncReadMem("));
    pins_locked(&scala);
}

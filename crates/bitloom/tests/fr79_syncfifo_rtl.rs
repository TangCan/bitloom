//! FR79 / AD-29 — SyncFIFO synthesizable cross-domain FIFO (Story 31.3).
//!
//! ATDD: language-level SyncFIFO ≠ ip::SyncFifo; elaborate→emit FIFO+sync
//! structure; cross-domain write/read under documented latency/full/empty;
//! illegal unsync CDC still E0220.

use bitloom_hir::{PortValues, Stmt};
use bitloom_prelude::ip::SyncFifo;
use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, GroundType, Span, SyncFIFO};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn assert_e0220(err: &Diagnostics) {
    let hit = err.0.iter().find(|d| d.code == "rhdl::E0220");
    assert!(hit.is_some(), "expected rhdl::E0220, got {err}");
    let d = hit.unwrap();
    assert!(
        d.en.contains("DoubleFlop") || d.en.contains("SyncFIFO"),
        "EN diagnostic should name DoubleFlop/SyncFIFO: {}",
        d.en
    );
}

#[test]
fn fr79_syncfifo_params_documented() {
    assert_eq!(SyncFIFO::<4, 8>::DEPTH, 4);
    assert_eq!(SyncFIFO::<4, 8>::WIDTH, 8);
    assert_eq!(SyncFIFO::<4, 8>::PTR_WIDTH, 3);
    assert_eq!(SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS, 2);
    assert_eq!(SyncFIFO::<4, 8>::LATENCY_REG_READ_TICKS, 1);
}

#[test]
fn fr79_syncfifo_distinct_from_ip_syncfifo() {
    let cdc = SyncFIFO::<4, 8>::elaborate().expect("CDC SyncFIFO");
    let ip = SyncFifo::elaborate().expect("IP SyncFifo");
    assert_eq!(cdc.abi_name, "SyncFIFO");
    assert_eq!(ip.abi_name, "SyncFifo");
    assert_ne!(
        cdc.abi_name, ip.abi_name,
        "FR82 IP must stay distinct from CDC SyncFIFO"
    );
}

#[test]
fn fr79_syncfifo_elaborate_emits_fifo_and_sync_structure() {
    let hir = SyncFIFO::<4, 8>::elaborate().expect("SyncFIFO must elaborate");
    let body = &hir.circuit().modules[0].body;
    let has_mem = body.iter().any(|st| matches!(st, Stmt::MemDecl { .. }));
    let reg_names: Vec<&str> = body
        .iter()
        .filter_map(|st| match st {
            Stmt::RegDecl { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert!(has_mem, "NFR37: must declare FIFO mem, got no MemDecl");
    assert!(
        reg_names.iter().any(|n| n.contains("ff0")) && reg_names.iter().any(|n| n.contains("ff1")),
        "must declare DoubleFlop sync stages for gray ptr CDC, got {reg_names:?}"
    );
    assert!(
        reg_names
            .iter()
            .any(|n| n.contains("wr_ptr") || n.contains("rd_ptr")),
        "must declare FIFO pointers, got {reg_names:?}"
    );

    let art = emit(&hir);
    assert!(
        art.filelist.iter().any(|f| f.contains("SyncFIFO")),
        "emit filelist should name SyncFIFO: {:?}",
        art.filelist
    );
    let v = &art.files[0].contents;
    assert!(v.contains("module SyncFIFO"), "{v}");
    assert!(
        v.contains("full") && v.contains("empty") && v.contains("wr_en") && v.contains("rd_en"),
        "emit must expose FIFO handshake: {v}"
    );
    assert!(
        (v.contains("ff0") || v.contains("sync")) && v.contains("always @(posedge"),
        "emit must show sync/FIFO sequential structure: {v}"
    );
}

#[test]
fn fr79_syncfifo_cross_domain_write_read_golden() {
    let hir = SyncFIFO::<4, 8>::elaborate().expect("elaborate");
    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();

    // Reset
    pv.set("rst", 1);
    pv.set("wr_en", 0);
    pv.set("rd_en", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("empty"), Some(1));
    assert_eq!(sim.ports().get("full"), Some(0));

    // Write one word on src domain.
    pv.set("rst", 0);
    pv.set("wr_en", 1);
    pv.set("data_in", 0xA5);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();

    // Pointer sync latency: empty stays 1 for LATENCY_PTR_SYNC_TICKS-1 more observations
    // after the write tick (write tick samples ff0; need 2 dst ticks total for ff1).
    pv.set("wr_en", 0);
    pv.set("data_in", 0);
    for i in 0..SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS {
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();
        if i + 1 < SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS {
            assert_eq!(
                sim.ports().get("empty"),
                Some(1),
                "empty must stay 1 until ptr sync completes (tick {i})"
            );
        }
    }
    assert_eq!(
        sim.ports().get("empty"),
        Some(0),
        "after ptr sync empty must clear"
    );

    // Registered read: rd_en then one tick → data_out.
    pv.set("rd_en", 1);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("data_out"), Some(0xA5));
}

#[test]
fn fr79_syncfifo_full_empty_semantics() {
    let hir = SyncFIFO::<4, 8>::elaborate().expect("elaborate");
    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("wr_en", 0);
    pv.set("rd_en", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    pv.set("rst", 0);

    // Fill DEPTH words; allow ptr sync between so empty tracks, but full is write-local.
    for word in 1u64..=u64::from(SyncFIFO::<4, 8>::DEPTH) {
        pv.set("wr_en", 1);
        pv.set("rd_en", 0);
        pv.set("data_in", word * 0x11);
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();
        // Drain sync latency so subsequent writes see updated rd_gray_sync for full.
        pv.set("wr_en", 0);
        for _ in 0..SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS {
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
        }
    }
    assert_eq!(
        sim.ports().get("full"),
        Some(1),
        "FIFO must be full after DEPTH writes"
    );

    // Extra write ignored: still full; head intact after later reads.
    pv.set("wr_en", 1);
    pv.set("data_in", 0xFF);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("full"), Some(1));

    // Read all DEPTH words in order (0x11, 0x22, 0x33, 0x44) — not 0xFF.
    pv.set("wr_en", 0);
    let mut expected = Vec::new();
    for word in 1u64..=u64::from(SyncFIFO::<4, 8>::DEPTH) {
        expected.push(word * 0x11);
    }
    let mut got = Vec::new();
    for _ in 0..SyncFIFO::<4, 8>::DEPTH {
        // Ensure empty cleared for this occupancy (already synced from fills).
        pv.set("rd_en", 1);
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();
        got.push(sim.ports().get("data_out").expect("data_out"));
        pv.set("rd_en", 0);
        for _ in 0..SyncFIFO::<4, 8>::LATENCY_PTR_SYNC_TICKS {
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
        }
    }
    assert_eq!(
        got, expected,
        "reads must preserve order; overflow write ignored"
    );
}

#[test]
fn fr79_illegal_unsync_cross_domain_still_e0220() {
    let mut s = ElaborateSession::new("CdcIllegal");
    s.begin_module("CdcIllegal", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 1 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 1 }, Span::default());
    s.bind_domain("a", 0);
    s.bind_domain("y", 1);
    s.begin_combinational(Span::default());
    s.assign_net("y", "a", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("illegal CDC must fail freeze");
    assert_e0220(&err);
}

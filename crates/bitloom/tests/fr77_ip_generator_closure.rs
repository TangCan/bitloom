//! ATDD: Story 29.3 / FR77 — IP generator closure customization (Cap-R-63).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr77_ip_generator_closure
//! ```
//!
//! Overlay on Epic 34 IP baselines + Epic 27 Mem init. Design surface:
//! `bitloom_prelude::ip::Crc8Lut`. Closures dissolve before freeze (NFR36);
//! synthesizable leg uses `SynthesizableClosure` violation tokens (D1).

use bitloom_hir::Stmt;
use bitloom_prelude::ip::{Crc8Lut, crc8_table_byte};
use bitloom_prelude::{Elaboratable, PortValues, SynthesizableClosureViolation};
use bitloom_sim::Sim;

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
        "{label}: emit must not contain closure/callback IR (NFR36):\n{text}"
    );
}

fn mem_init_words(hir: &bitloom_hir::FrozenHir) -> Vec<u64> {
    for m in &hir.circuit().modules {
        for stmt in &m.body {
            if let Stmt::MemDecl {
                name,
                init: Some(words),
                ..
            } = stmt
            {
                if name == "lut" {
                    return words.clone();
                }
            }
        }
    }
    panic!("missing lut MemDecl.init");
}

/// SyncReadMem: one tick schedules, next delivers into `q` / `rdata`.
fn sync_read_at(sim: &mut Sim, addr: u64) -> u64 {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("addr", addr);
    sim.set_inputs(pv);
    sim.tick();
    sim.tick();
    sim.ports().get("rdata").expect("rdata")
}

#[test]
fn fr77_default_poly_no_user_closure() {
    let hir = Crc8Lut::elaborate().expect("default");
    assert_eq!(hir.abi_name, "Crc8Lut");
    let words = mem_init_words(&hir);
    assert_eq!(words.len(), Crc8Lut::DEPTH as usize);
    assert_eq!(words[0], 0);
    assert_eq!(words[1], crc8_table_byte(1, Crc8Lut::DEFAULT_POLY) as u64);
    assert_eq!(
        words[0xA5],
        crc8_table_byte(0xA5, Crc8Lut::DEFAULT_POLY) as u64
    );

    let v = bitloom_vlog::emit(&hir).files[0].contents.clone();
    assert!(v.contains("module Crc8Lut"));
    assert!(v.contains(&format!(
        "lut[1] = {};",
        crc8_table_byte(1, Crc8Lut::DEFAULT_POLY)
    )));
    assert_no_closure_ir("default/.v", &v);
}

#[test]
fn fr77_custom_table_fn_proves_customization_emit_and_tick() {
    let poly = 0x1du8;
    let expected: Vec<u64> = (0..Crc8Lut::DEPTH as usize)
        .map(|i| crc8_table_byte(i as u8, poly) as u64)
        .collect();

    let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| crc8_table_byte(i as u8, poly) as u64)
        .expect("custom table");
    assert_eq!(mem_init_words(&hir), expected);

    let v = bitloom_vlog::emit(&hir).files[0].contents.clone();
    assert!(v.contains(&format!("lut[1] = {};", expected[1])));
    assert_no_closure_ir("custom/.v", &v);

    let fir = rhdl_firrtl::emit(&hir).files[0].contents.clone();
    assert!(fir.contains("mem-init lut"), "fir:\n{fir}");
    assert_no_closure_ir("custom/.fir", &fir);

    // Tick proves customization (poly 0x1D ≠ default 0x07 at 0xA5).
    assert_ne!(
        crc8_table_byte(0xA5, poly),
        crc8_table_byte(0xA5, Crc8Lut::DEFAULT_POLY)
    );
    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    sim.set_inputs(pv);
    sim.tick();
    for &addr in &[0u64, 1, 0xA5, 0xFF] {
        assert_eq!(
            sync_read_at(&mut sim, addr),
            expected[addr as usize],
            "addr={addr:#x}"
        );
    }
}

#[test]
fn fr77_without_legal_closure_violation_is_clear_error() {
    let err = Crc8Lut::elaborate_with_table_fn(
        &[SynthesizableClosureViolation::runtime_capture_state(
            "captures Wire in table generator",
        )],
        |_| 0,
    )
    .expect_err("must fail");
    assert!(
        err.0.iter().any(|d| {
            d.code == "rhdl::E0144"
                && (d.en.contains("synthesizable-closure") || d.en.contains("runtime capture"))
        }),
        "expected clear SynthesizableClosure diagnostic, got {err:?}"
    );
}

#[test]
fn fr77_freeze_has_no_closure_residue_nfr36() {
    let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| (i & 0xff) as u64).unwrap();
    let words = mem_init_words(&hir);
    assert_eq!(words[7], 7);
    // FrozenHir dump / emit must not mention Fn/closure.
    let debug = format!("{hir:?}");
    assert!(
        !debug.to_lowercase().contains("closure")
            && !debug.contains("FnOnce")
            && !debug.contains("FnMut"),
        "FrozenHir debug must not retain closure types:\n{debug}"
    );
    let v = bitloom_vlog::emit(&hir).files[0].contents.clone();
    assert_no_closure_ir("identity-table/.v", &v);
}

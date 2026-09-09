//! ATDD: FR73 CRC/LUT golden — closure-generated tables vs handwritten (Story 27.4).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr73_crc_lut_golden
//! ```
//!
//! Design surface remains `bitloom-prelude` / `ElaborateSession` (Bitloom).
//! Closures dissolve before freeze; backends see only plain mem init (NFR36).

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{PortValues, Stmt};
use bitloom_sim::Sim;

/// CRC-8 table entry: poly `0x07` (CRC-8/SMBUS-style), one byte in → residue.
fn crc8_poly07(byte: u8) -> u8 {
    let mut b = byte;
    for _ in 0..8 {
        if b & 0x80 != 0 {
            b = (b << 1) ^ 0x07;
        } else {
            b <<= 1;
        }
    }
    b
}

/// Handwritten golden — literal words, not the generator closure (FR73 ATDD).
const CRC8_GOLDEN_16: [u64; 16] = [0, 7, 14, 9, 28, 27, 18, 21, 56, 63, 54, 49, 36, 35, 42, 45];

/// NFR36 spot-check: backends must not emit closure/callback IR tokens.
/// Avoid false positives on identifiers like `…Fn (` in module headers.
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

fn rom_async_hir(name: &str, init: Option<Vec<u64>>, use_fn: bool) -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new(name);
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    if use_fn {
        s.declare_mem_with_init_fn(
            "crc",
            16,
            8,
            |i| crc8_poly07(i as u8) as u64,
            Span::default(),
        );
    } else {
        s.declare_mem_with_init(
            "crc",
            16,
            8,
            init.expect("handwritten init"),
            Span::default(),
        );
    }
    s.begin_combinational(Span::default());
    s.assign_net("y", "crc", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

fn sync_rom_hir(name: &str, use_fn: bool) -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new(name);
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("addr", GroundType::UInt { width: 4 }, Span::default());
    s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    if use_fn {
        s.declare_sync_read_mem_with_init_fn(
            "crc",
            16,
            8,
            |i| crc8_poly07(i as u8) as u64,
            Span::default(),
        );
    } else {
        s.declare_sync_read_mem_with_init("crc", 16, 8, CRC8_GOLDEN_16.to_vec(), Span::default());
    }
    s.begin_combinational(Span::default());
    s.assign_net("rdata", "q", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_mem_read("q", "crc", "addr", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

fn mem_init_words(hir: &bitloom_hir::FrozenHir) -> Vec<u64> {
    hir.circuit().modules[0]
        .body
        .iter()
        .find_map(|st| match st {
            Stmt::MemDecl {
                name,
                init: Some(words),
                ..
            } if name == "crc" => Some(words.clone()),
            _ => None,
        })
        .expect("MemDecl.init present")
}

fn sync_read_at(sim: &mut Sim, addr: u64) -> u64 {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("addr", addr);
    sim.set_inputs(pv);
    sim.tick(); // schedule SyncReadMem
    sim.tick(); // deliver into q / rdata
    sim.ports().get("rdata").expect("rdata")
}

#[test]
fn fr73_crc_closure_matches_handwritten_emit_and_tick() {
    // Sanity: handwritten literal equals offline algorithm (table is the golden).
    let recomputed: Vec<u64> = (0..16).map(|i| crc8_poly07(i as u8) as u64).collect();
    assert_eq!(recomputed, CRC8_GOLDEN_16.to_vec());

    let via_fn = rom_async_hir("CrcRomGen", None, true);
    let via_vec = rom_async_hir("CrcRomHand", Some(CRC8_GOLDEN_16.to_vec()), false);

    assert_eq!(mem_init_words(&via_fn), CRC8_GOLDEN_16.to_vec());
    assert_eq!(mem_init_words(&via_vec), CRC8_GOLDEN_16.to_vec());
    assert_eq!(mem_init_words(&via_fn), mem_init_words(&via_vec));

    let v_fn = bitloom_vlog::emit(&via_fn).files[0].contents.clone();
    let v_vec = bitloom_vlog::emit(&via_vec).files[0].contents.clone();
    for i in 0..16 {
        let line = format!("crc[{i}] = {};", CRC8_GOLDEN_16[i]);
        assert!(
            v_fn.contains(&line) && v_vec.contains(&line),
            "missing .v init {line}\nfn:\n{v_fn}\nvec:\n{v_vec}"
        );
    }
    // Init bodies must match (module name may differ).
    let extract_init = |v: &str| -> String {
        let start = v.find("initial begin").expect("initial");
        let end = v[start..].find("end\n").expect("end") + start + 4;
        v[start..end].to_string()
    };
    assert_eq!(
        extract_init(&v_fn),
        extract_init(&v_vec),
        "Verilog initial blocks must match (FR73 golden)"
    );

    let fir_fn = rhdl_firrtl::emit(&via_fn).files[0].contents.clone();
    let fir_vec = rhdl_firrtl::emit(&via_vec).files[0].contents.clone();
    let words = "0, 7, 14, 9, 28, 27, 18, 21, 56, 63, 54, 49, 36, 35, 42, 45";
    assert!(
        fir_fn.contains("mem-init crc") && fir_fn.contains(words),
        "FIRRTL closure path:\n{fir_fn}"
    );
    assert!(
        fir_vec.contains("mem-init crc") && fir_vec.contains(words),
        "FIRRTL handwritten path:\n{fir_vec}"
    );

    assert_no_closure_ir("verilog/fn", &v_fn);
    assert_no_closure_ir("verilog/vec", &v_vec);
    assert_no_closure_ir("firrtl/fn", &fir_fn);
    assert_no_closure_ir("firrtl/vec", &fir_vec);

    // Optional Chisel (FR81 Path A): async Mem+init lowers; must not leave closure IR in Scala.
    let scala = rhdl_firrtl::emit_chisel(&via_fn)
        .expect("Path A Mem init must emit")
        .files[0]
        .contents
        .clone();
    assert!(scala.contains("Mem(16, UInt(8.W))"), "{scala}");
    assert!(scala.contains("crc_init = VecInit("), "{scala}");
    assert_no_closure_ir("chisel/fn", &scala);

    // Tick equivalence on SyncReadMem (latency-1 reads of init).
    let mut sim_fn = Sim::new(sync_rom_hir("CrcSyncGen", true));
    let mut sim_vec = Sim::new(sync_rom_hir("CrcSyncHand", false));
    for addr in [0u64, 1, 7, 8, 15] {
        let a = sync_read_at(&mut sim_fn, addr);
        let b = sync_read_at(&mut sim_vec, addr);
        assert_eq!(a, b, "tick rdata mismatch at addr {addr}");
        assert_eq!(
            a, CRC8_GOLDEN_16[addr as usize],
            "golden mismatch at {addr}"
        );
    }
}

#[test]
fn fr73_prelude_generate_mem_init_matches_crc_golden() {
    let via_prelude = bitloom_prelude::generate_mem_init(16, 8, |i| crc8_poly07(i as u8) as u64);
    assert_eq!(via_prelude, CRC8_GOLDEN_16.to_vec());
}

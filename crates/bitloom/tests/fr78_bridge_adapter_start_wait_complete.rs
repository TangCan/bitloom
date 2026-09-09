//! ATDD: FR78 bridge adapter `start_wait_complete` template (Story 30.2 / Cap-R-65).
//!
//! Host/bridge free closures map start→wait→complete onto ordinary signals;
//! cycle-accurate path never sees `Fn` objects (NFR36 / Cap-R-66/67).

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::UartTx;
use bitloom_prelude::{StartWaitComplete, start_wait_complete};
use bitloom_sim::Sim;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

/// Minimal host bridge: start latches a busy countdown; tick decrements.
struct HandshakeHost {
    tx_data: u8,
    tx_start: bool,
    busy_remaining: u32,
    ticks: u32,
}

impl StartWaitComplete for HandshakeHost {
    fn is_busy(&self) -> bool {
        self.busy_remaining > 0
    }

    fn tick(&mut self) {
        self.ticks += 1;
        if self.busy_remaining > 0 {
            self.busy_remaining -= 1;
        }
        // After start pulse, clear start (ordinary signal hygiene).
        self.tx_start = false;
    }
}

#[test]
fn fr78_trait_start_wait_complete_binds_start_to_signal_timing() {
    let mut host = HandshakeHost {
        tx_data: 0,
        tx_start: false,
        busy_remaining: 0,
        ticks: 0,
    };

    host.start_wait_complete(|s| {
        s.tx_data = 0xA5;
        s.tx_start = true;
        s.busy_remaining = 3; // start implies 3 busy cycles after commit
    });

    assert_eq!(host.tx_data, 0xA5);
    assert!(!host.tx_start, "start pulse cleared by ticks");
    assert!(!host.is_busy());
    // 1 commit tick + 2 more while busy (3→2→1→0): total 3
    assert_eq!(host.ticks, 3, "waited exactly busy cycles");
}

#[test]
fn fr78_free_fn_equivalent_same_handshake() {
    let mut host = HandshakeHost {
        tx_data: 0,
        tx_start: false,
        busy_remaining: 0,
        ticks: 0,
    };

    start_wait_complete(
        &mut host,
        |h| h.tick(),
        |h| h.is_busy(),
        |h| {
            h.tx_data = 0x3C;
            h.tx_start = true;
            h.busy_remaining = 2;
        },
    );

    assert_eq!(host.tx_data, 0x3C);
    assert!(!host.is_busy());
    assert_eq!(host.ticks, 2);
}

/// Adapter that drives `UartTx` with ordinary PortValues only (no Fn into tick).
struct UartBridge<'a> {
    sim: &'a mut Sim,
    wr_en: bool,
    wr_data: u8,
}

impl StartWaitComplete for UartBridge<'_> {
    fn is_busy(&self) -> bool {
        self.sim.ports().get("tx_busy").unwrap_or(0) != 0
    }

    fn tick(&mut self) {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("wr_en", if self.wr_en { 1 } else { 0 });
        pv.set("wr_data", u64::from(self.wr_data));
        self.sim.set_inputs(pv);
        self.sim.settle();
        self.sim.tick();
        // One-cycle write strobe.
        self.wr_en = false;
    }
}

#[test]
fn fr78_template_drives_cycle_accurate_via_ordinary_signals_only() {
    let hir = UartTx::elaborate().expect("elaborate UartTx");
    let mut sim = Sim::new(hir);

    // Reset
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("wr_en", 0);
    pv.set("wr_data", 0);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();

    let mut bridge = UartBridge {
        sim: &mut sim,
        wr_en: false,
        wr_data: 0,
    };

    // Free functional-style closure on the host (Cap-R-66/67).
    bridge.start_wait_complete(|b| {
        b.wr_data = 0x01;
        b.wr_en = true;
    });

    assert!(
        !bridge.is_busy(),
        "UART start_wait_complete must finish when tx_busy clears"
    );
    // Cycle-accurate ports are plain ints — no closure residue in PortValues.
    assert_eq!(
        bridge.sim.ports().get("tx_busy"),
        Some(0),
        "tx_busy cleared after template"
    );
}

#[test]
fn fr78_docs_mark_view_boundary_and_nfr36() {
    let root = workspace_root();
    let doc = fs::read_to_string(root.join("docs/fr78-bridge-adapter-closures.md"))
        .expect("docs/fr78-bridge-adapter-closures.md");
    assert!(
        doc.contains("start_wait_complete") && doc.contains("StartWaitComplete"),
        "doc must name template API"
    );
    assert!(
        doc.contains("Cap-R-66") || doc.contains("自由"),
        "doc must mark functional free-closure boundary"
    );
    assert!(
        doc.contains("NFR36") && (doc.contains("PortValues") || doc.contains("普通信号")),
        "doc must state cycle-accurate sees ordinary signals only"
    );
    assert!(
        doc.contains("not") && doc.contains("TLM"),
        "doc must reject SystemC TLM as contract"
    );

    let surface =
        fs::read_to_string(root.join("_agile-output/specs/spec-rhdl/language-surface.md"))
            .expect("language-surface");
    assert!(
        surface.contains("FR78") && surface.contains("start_wait_complete"),
        "language-surface must document FR78 template"
    );

    let prelude =
        fs::read_to_string(root.join("crates/bitloom-prelude/src/lib.rs")).expect("prelude");
    assert!(
        prelude.contains("trait StartWaitComplete") && prelude.contains("fn start_wait_complete"),
        "prelude must export StartWaitComplete / start_wait_complete"
    );
    // NFR36: no HIR closure node types introduced for FR78.
    assert!(
        !prelude.contains("enum Closure") && !prelude.contains("Stmt::Closure"),
        "FR78 must not add closure IR to the design surface"
    );
}

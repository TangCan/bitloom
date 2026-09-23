//! FR164 — cycle-sim execution predicate for the external CIRCT sim gate.
//!
//! Invoked by `scripts/circt-external-sim-check.sh` after firtool compile succeeds.
//! Beyond FR137 compile-only: proves a representative Bitloom-importable `.fir` executes
//! under cycle-accurate tick (FIRRTL 6 dialect; firtool may use a paired v4 compile fixture).

use bitloom_hir::PortValues;
use bitloom_sim::Sim;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let fir_path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir")
    });
    let text = match fs::read_to_string(&fir_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: FR164 sim gate: read {}: {e}", fir_path.display());
            return ExitCode::FAILURE;
        }
    };
    let hir = match bitloom_firrtl::import(&text) {
        Ok(h) => h,
        Err(d) => {
            eprintln!("error: FR164 sim gate: import {}: {d}", fir_path.display());
            return ExitCode::FAILURE;
        }
    };

    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("x", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();

    pv.set("rst", 0);
    for x in [0, 0xa5, 0x5a, 0xff, 0] {
        pv.set("x", x);
        sim.set_inputs(pv.clone());
        sim.settle();
        sim.tick();
        let y = sim.ports().get("y");
        if y != Some(x) {
            eprintln!(
                "error: FR164 sim gate: expected y={x:#x} after tick, got {y:?} (≠ FR137 compile alone)"
            );
            return ExitCode::FAILURE;
        }
    }
    println!(
        "fr164_circt_sim_gate: OK cycle-sim sequence=00,a5,5a,ff,00 fixture={}",
        fir_path.display()
    );
    ExitCode::SUCCESS
}

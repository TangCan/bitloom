use bitloom_prelude::{Elaboratable, ElaborateSession, ip::GpioCsr};

fn main() {
    let mut session = ElaborateSession::new("Pins32");
    GpioCsr::define_module(&mut session, "Pins32").unwrap();
    GpioCsr::define_module(&mut session, "Pins32").unwrap(); // Reuse, not duplicate.
    let composed = session.finish().unwrap(); // Exactly one freeze.
    assert_eq!(composed.circuit().name, "Pins32");
    let _standalone = GpioCsr::elaborate().unwrap(); // Its own independent session.
    let registers = GpioCsr::registers();
    let output = std::env::args().nth(1).expect("software output directory");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(
        format!("{output}/gpio-csr-registers.h"),
        registers.emit_c_header().unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{output}/gpio-csr-registers.md"),
        registers.emit_markdown().unwrap(),
    )
    .unwrap();
}


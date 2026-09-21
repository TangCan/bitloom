use bitloom_prelude::{Elaboratable, ElaborateSession, ip::UartCsr};

fn main() {
    let mut session = ElaborateSession::new("BufferedSerial");
    UartCsr::define_module(&mut session, "BufferedSerial").unwrap();
    UartCsr::define_module(&mut session, "BufferedSerial").unwrap(); // Reuse, not duplicate.
    let composed = session.finish().unwrap(); // Exactly one freeze.
    assert_eq!(composed.circuit().name, "BufferedSerial");
    let _standalone = UartCsr::elaborate().unwrap(); // Its own independent session.
    let registers = UartCsr::registers();
    let output = std::env::args().nth(1).expect("software output directory");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(
        format!("{output}/uart-csr-registers.h"),
        registers.emit_c_header().unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{output}/uart-csr-registers.md"),
        registers.emit_markdown().unwrap(),
    )
    .unwrap();
}


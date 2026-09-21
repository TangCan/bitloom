use bitloom_prelude::{Elaboratable, ElaborateSession, ip::Timer};

fn main() {
    let mut session = ElaborateSession::new("Timer32");
    Timer::define_module(&mut session, "Timer32").unwrap();
    Timer::define_module(&mut session, "Timer32").unwrap(); // Reuse, not duplicate.
    let composed = session.finish().unwrap(); // Exactly one freeze.
    assert_eq!(composed.circuit().name, "Timer32");
    let _standalone = Timer::elaborate().unwrap(); // Its own independent session.
    let registers = Timer::registers();
    let output = std::env::args().nth(1).expect("software output directory");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(
        format!("{output}/timer-registers.h"),
        registers.emit_c_header().unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{output}/timer-registers.md"),
        registers.emit_markdown().unwrap(),
    )
    .unwrap();
}

use super::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};
use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Five synchronous event sources with sticky pending and software masking.
///
/// `raw_events` bits are Timer match, UART RX arrival, UART TX space,
/// UART error, and GPIO rising edge. They are event pulses, not sticky flags.
/// Synchronous high reset clears state and cancels outstanding CSR responses.
pub struct Irq;

const CSR_MODULE: &str = "BitloomIrqCsr";

impl Elaboratable for Irq {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut session = ElaborateSession::new("Irq");
        Self::define_module(&mut session, "Irq")?;
        session.finish()
    }
}

impl Irq {
    /// Local offsets 0/4/8/12; the system supplies its own base address.
    pub fn registers() -> CsrBlock {
        use CsrAccess::{Ro, Rw, W1c, Wo};
        use CsrOwner::{External, Leaf};
        CsrBlock {
            name: "Irq".into(),
            registers: [
                ("pending", 0, W1c, Leaf, Some("event_bits")),
                ("enable", 4, Rw, Leaf, None),
                ("test", 8, Wo, CsrOwner::None, None),
                ("raw", 12, Ro, External, None),
            ]
            .into_iter()
            .map(|(name, offset, access, owner, event)| CsrRegister {
                name: name.into(),
                offset,
                reset: 0,
                access,
                owner,
                event: event.map(str::to_owned),
                read_reject: false,
                write_reject: false,
                fields: ["timer", "uart_rx", "uart_tx", "uart_error", "gpio"]
                    .into_iter()
                    .enumerate()
                    .map(|(bit, name)| CsrField {
                        name: name.into(),
                        mask: 1 << bit,
                        reset: 0,
                        access,
                    })
                    .collect(),
            })
            .collect(),
        }
    }

    /// Define/reuse the IRQ and its shared CSR leaf without freezing the session.
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        Self::registers().define_module(session, CSR_MODULE)?;
        session.define_module(name, vec![], Self::define_body)
    }

    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        let mut connections = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (name, width) in [
            ("req_valid", 1),
            ("write", 1),
            ("addr", 16),
            ("wdata", 32),
            ("wstrb", 4),
            ("rsp_ready", 1),
        ] {
            s.add_input(name, GroundType::UInt { width }, sp);
            connections.push((name.into(), name.into()));
        }
        for (name, width) in [
            ("req_ready", 1),
            ("rsp_valid", 1),
            ("rdata", 32),
            ("error", 2),
        ] {
            s.add_output(name, GroundType::UInt { width }, sp);
            connections.push((name.into(), name.into()));
        }
        s.add_input("raw_events", GroundType::UInt { width: 5 }, sp);
        s.add_output("irq", GroundType::UInt { width: 1 }, sp);
        // Only CSR owns PENDING and ENABLE. WO has no value; RO has no
        // candidate or write_mask. Every actual leaf port is connected.
        for r in Self::registers().registers {
            let mut ports = vec![("read_commit", 1), ("write_commit", 1)];
            if r.access != CsrAccess::Wo {
                ports.push(("value", 32));
            }
            if r.access != CsrAccess::Ro {
                ports.extend([("candidate", 32), ("write_mask", 32)]);
            }
            for (suffix, width) in ports {
                let name = format!("{}_{suffix}", r.name);
                s.declare_wire(&name, GroundType::UInt { width }, sp);
                connections.push((name.clone(), name));
            }
        }
        for (name, width) in [
            ("event_bits", 32),
            ("test_bits", 32),
            ("masked", 32),
            ("zero32", 32),
            ("zero27", 27),
            ("empty", 1),
            ("one", 1),
        ] {
            s.declare_wire(name, GroundType::UInt { width }, sp);
        }
        connections.push(("event_bits".into(), "event_bits".into()));
        s.add_instance("csr", CSR_MODULE, connections, vec![], sp);
        s.begin_combinational(sp);
        s.assign_lit("zero32", 0, sp);
        s.assign_lit("zero27", 0, sp);
        s.assign_lit("one", 1, sp);
        // RAW is always hardware-only, including the TEST commit cycle.
        s.assign_concat("raw_value", "zero27", "raw_events", sp);
        s.assign_mux(
            "test_bits",
            "test_write_commit",
            "test_candidate",
            "zero32",
            sp,
        );
        s.assign_or("event_bits", "raw_value", "test_bits", sp);
        s.assign_and("masked", "pending_value", "enable_value", sp);
        s.assign_eq("empty", "masked", "zero32", sp);
        s.assign_xor("irq", "empty", "one", sp);
        s.end_process();
        Ok(())
    }
}

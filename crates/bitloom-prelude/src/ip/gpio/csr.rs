use crate::ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};
use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Thirty-two GPIO pins with CSR control, synchronized input and rising events.
///
/// `pad_out = OUT & DIR`, `pad_oe = DIR`; these are logical pad controls.
/// Inputs pass through two register stages, followed by edge history. Raw events
/// use the pre-edge direction and are suppressed during synchronous high reset.
/// This is not a debounce filter or an electrical/CDC reliability guarantee.
pub struct GpioCsr;

const CSR_MODULE: &str = "BitloomGpioCsrRegisters";

impl Elaboratable for GpioCsr {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut session = ElaborateSession::new("GpioCsr");
        Self::define_module(&mut session, "GpioCsr")?;
        session.finish()
    }
}

impl GpioCsr {
    /// Local offsets 0/4/8/12/16/20; the system supplies its own base address.
    pub fn registers() -> CsrBlock {
        use CsrAccess::{Ro, Rw, W1c, Wo};
        use CsrOwner::{External, Leaf};
        CsrBlock {
            name: "GpioCsr".into(),
            registers: [
                ("dir", 0, Rw, Leaf, None),
                ("out", 4, Rw, External, None),
                ("in", 8, Ro, External, None),
                ("set", 12, Wo, CsrOwner::None, None),
                ("clear", 16, Wo, CsrOwner::None, None),
                ("rise_event", 20, W1c, Leaf, Some("rise_bits")),
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
                fields: vec![CsrField {
                    name: "bits".into(),
                    mask: u32::MAX as u64,
                    reset: 0,
                    access,
                }],
            })
            .collect(),
        }
    }

    /// Define/reuse the GPIO and its shared CSR leaf without freezing the session.
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
        s.add_input("pad_in", GroundType::UInt { width: 32 }, sp);
        for (name, width) in [("pad_out", 32), ("pad_oe", 32), ("raw_event", 1)] {
            s.add_output(name, GroundType::UInt { width }, sp);
        }
        // Leaf owns DIR and RISE_EVENT; wrapper alone owns OUT. WO ports
        // carry candidates and commits but never create additional storage.
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
        for name in ["out_r", "sync1", "sync2", "history"] {
            s.declare_reg(name, GroundType::UInt { width: 32 }, sp);
        }
        for name in [
            "rise_bits",
            "all32",
            "zero32",
            "not_history",
            "not_dir",
            "rising",
            "set_value",
            "clear_value",
            "not_clear",
            "after_clear",
            "after_set",
            "out_next",
        ] {
            s.declare_wire(name, GroundType::UInt { width: 32 }, sp);
        }
        for name in ["empty", "has_rise", "one", "not_reset"] {
            s.declare_wire(name, GroundType::UInt { width: 1 }, sp);
        }
        connections.push(("rise_bits".into(), "rise_bits".into()));
        s.add_instance("csr", CSR_MODULE, connections, vec![], sp);
        s.begin_combinational(sp);
        s.assign_lit("all32", u32::MAX as u64, sp);
        s.assign_lit("zero32", 0, sp);
        s.assign_lit("one", 1, sp);
        s.assign_net("out_value", "out_r", sp);
        s.assign_net("in_value", "sync2", sp);
        s.assign_net("pad_oe", "dir_value", sp);
        s.assign_and("pad_out", "out_r", "dir_value", sp);
        s.assign_xor("not_history", "history", "all32", sp);
        s.assign_xor("not_dir", "dir_value", "all32", sp);
        s.assign_and("rising", "sync2", "not_history", sp);
        s.assign_and("rise_bits", "rising", "not_dir", sp);
        s.assign_eq("empty", "rise_bits", "zero32", sp);
        s.assign_xor("has_rise", "empty", "one", sp);
        s.assign_xor("not_reset", "rst", "one", sp);
        s.assign_and("raw_event", "has_rise", "not_reset", sp);
        // Single-request decoding makes these three write commits mutually
        // exclusive. Candidates depend only on old state and bus data.
        s.assign_or("set_value", "out_r", "set_candidate", sp);
        s.assign_xor("not_clear", "clear_candidate", "all32", sp);
        s.assign_and("clear_value", "out_r", "not_clear", sp);
        s.assign_mux(
            "after_clear",
            "clear_write_commit",
            "clear_value",
            "out_r",
            sp,
        );
        s.assign_mux(
            "after_set",
            "set_write_commit",
            "set_value",
            "after_clear",
            sp,
        );
        s.assign_mux(
            "out_next",
            "out_write_commit",
            "out_candidate",
            "after_set",
            sp,
        );
        s.end_process();
        s.begin_sequential(sp);
        s.assign_reg_d_from("out_r", "out_next", sp);
        s.assign_reg_d_from("sync1", "pad_in", sp);
        s.assign_reg_d_from("sync2", "sync1", sp);
        s.assign_reg_d_from("history", "sync2", sp);
        s.end_process();
        Ok(())
    }
}

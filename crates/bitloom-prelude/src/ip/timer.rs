use super::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};
use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Composable 32-bit timer with local CSR offsets 0, 4, 8, and 12.
///
/// `match_event` is a combinational event sampled at the upcoming rising edge,
/// not the sticky EVENT value. A synchronous high reset clears all state and
/// cancels outstanding responses. Reset this module with its upstream peers.
pub struct Timer;

// Fixed descriptor and body identity; callers may share this private definition.
const CSR_MODULE: &str = "BitloomTimerCsr";

impl Elaboratable for Timer {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut session = ElaborateSession::new("Timer");
        Self::define_module(&mut session, "Timer")?;
        session.finish()
    }
}

impl Timer {
    /// Static local register map, also used to generate software documentation.
    pub fn registers() -> CsrBlock {
        use CsrAccess::{Rw, W1c};
        use CsrOwner::{External, Leaf};
        CsrBlock {
            name: "Timer".into(),
            registers: [
                ("ctrl", 0, 3, Rw, External, None),
                ("count", 4, u32::MAX, Rw, External, None),
                ("compare", 8, u32::MAX, Rw, Leaf, None),
                ("EVENT", 12, 1, W1c, Leaf, Some("match_bits")),
            ]
            .into_iter()
            .map(|(name, offset, mask, access, owner, event)| CsrRegister {
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
                    mask: mask as u64,
                    reset: 0,
                    access,
                }],
            })
            .collect(),
        }
    }

    /// Define/reuse this timer and its CSR leaf without freezing the session.
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
        s.add_output("match_event", GroundType::UInt { width: 1 }, sp);
        // CTRL and COUNT have exactly one storage owner here. COMPARE and
        // EVENT are owned exclusively by the CSR leaf.
        for name in ["ctrl_value", "count_value"] {
            s.declare_reg(name, GroundType::UInt { width: 32 }, sp);
        }
        for r in Self::registers().registers {
            for (suffix, width) in [
                ("value", 32),
                ("candidate", 32),
                ("write_mask", 32),
                ("read_commit", 1),
                ("write_commit", 1),
            ] {
                let name = format!("{}_{suffix}", r.name);
                if suffix != "value" || r.owner != CsrOwner::External {
                    s.declare_wire(&name, GroundType::UInt { width }, sp);
                }
                connections.push((name.clone(), name));
            }
        }
        for (name, width) in [
            ("match_bits", 32),
            ("zero32", 32),
            ("one32", 32),
            ("ctrl_keep", 32),
            ("one", 1),
            ("active", 1),
            ("enabled", 1),
            ("periodic", 1),
            ("not_periodic", 1),
            ("write_ab", 1),
            ("config_write", 1),
            ("no_write", 1),
            ("running", 1),
            ("advance", 1),
            ("increment", 32),
            ("equal", 1),
            ("restart", 1),
            ("stop", 1),
            ("ctrl_stopped", 32),
            ("ctrl_auto", 32),
            ("ctrl_next", 32),
            ("count_auto", 32),
            ("count_running", 32),
            ("count_next", 32),
        ] {
            s.declare_wire(name, GroundType::UInt { width }, sp);
        }
        connections.push(("match_bits".into(), "match_bits".into()));
        s.add_instance("csr", CSR_MODULE, connections, vec![], sp);
        s.begin_combinational(sp);
        s.assign_lit("zero32", 0, sp);
        s.assign_lit("one32", 1, sp);
        s.assign_lit("ctrl_keep", 2, sp);
        s.assign_lit("one", 1, sp);
        s.assign_xor("active", "rst", "one", sp);
        s.assign_slice("enabled", "ctrl_value", 0, 1, sp);
        s.assign_slice("periodic", "ctrl_value", 1, 1, sp);
        s.assign_xor("not_periodic", "periodic", "one", sp);
        s.assign_or("write_ab", "ctrl_write_commit", "count_write_commit", sp);
        s.assign_or("config_write", "write_ab", "compare_write_commit", sp);
        s.assign_xor("no_write", "config_write", "one", sp);
        s.assign_and("running", "enabled", "active", sp);
        s.assign_and("advance", "running", "no_write", sp);
        // Same-width addition is modulo 2^32, including COMPARE=0 wrap.
        s.assign_add("increment", "count_value", "one32", sp);
        s.assign_eq("equal", "increment", "compare_value", sp);
        s.assign_and("match_event", "advance", "equal", sp);
        s.assign_mux("match_bits", "match_event", "one32", "zero32", sp);
        s.assign_and("restart", "match_event", "periodic", sp);
        s.assign_and("stop", "match_event", "not_periodic", sp);
        s.assign_and("ctrl_stopped", "ctrl_value", "ctrl_keep", sp);
        s.assign_mux("ctrl_auto", "stop", "ctrl_stopped", "ctrl_value", sp);
        s.assign_mux(
            "ctrl_next",
            "ctrl_write_commit",
            "ctrl_candidate",
            "ctrl_auto",
            sp,
        );
        s.assign_mux("count_auto", "restart", "zero32", "increment", sp);
        s.assign_mux("count_running", "advance", "count_auto", "count_value", sp);
        s.assign_mux(
            "count_next",
            "count_write_commit",
            "count_candidate",
            "count_running",
            sp,
        );
        s.end_process();
        s.begin_sequential(sp);
        s.assign_reg_d_from("ctrl_value", "ctrl_next", sp);
        s.assign_reg_d_from("count_value", "count_next", sp);
        s.end_process();
        Ok(())
    }
}

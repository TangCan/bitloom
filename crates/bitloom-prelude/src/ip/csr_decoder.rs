use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Fixed four-window, single-outstanding CSR decoder (FR196).
///
/// UART/GPIO/Timer/IRQ occupy 256-byte windows at 0x0000/0100/0200/0300.
/// Requests commit simultaneously upstream and at the selected leaf. The owner
/// is retained through response consumption; that edge cannot accept a new
/// request. Misses produce a registered DECERR. Apply a synchronous reset edge
/// before use and reset the bridge, decoder and all leaves together.
pub struct CsrDecoder;

impl Elaboratable for CsrDecoder {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut session = ElaborateSession::new("CsrDecoder");
        Self::define_module(&mut session, "CsrDecoder")?;
        session.finish()
    }
}

impl CsrDecoder {
    /// Register the fixed decoder definition in the caller's session.
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        session.define_module(name, vec![], Self::define_body)
    }

    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        let leaves = ["uart", "gpio", "timer", "irq"];
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        for (name, width) in [
            ("req_valid", 1),
            ("write", 1),
            ("addr", 16),
            ("wdata", 32),
            ("wstrb", 4),
            ("rsp_ready", 1),
        ] {
            s.add_input(name, GroundType::UInt { width }, sp);
        }
        for (name, width) in [
            ("req_ready", 1),
            ("rsp_valid", 1),
            ("rdata", 32),
            ("error", 2),
        ] {
            s.add_output(name, GroundType::UInt { width }, sp);
        }
        for leaf in leaves {
            for (name, width) in [
                ("req_ready", 1),
                ("rsp_valid", 1),
                ("rdata", 32),
                ("error", 2),
            ] {
                s.add_input(format!("{leaf}_{name}"), GroundType::UInt { width }, sp);
            }
            for (name, width) in [
                ("req_valid", 1),
                ("write", 1),
                ("addr", 16),
                ("wdata", 32),
                ("wstrb", 4),
                ("rsp_ready", 1),
            ] {
                s.add_output(format!("{leaf}_{name}"), GroundType::UInt { width }, sp);
            }
        }
        for (name, width) in [("busy", 1), ("miss", 1), ("owner", 2)] {
            s.declare_reg(name, GroundType::UInt { width }, sp);
        }
        for (name, width) in [
            ("one", 1),
            ("zero", 1),
            ("zero32", 32),
            ("zero2", 2),
            ("decerr", 2),
            ("page", 8),
            ("select_owner", 2),
            ("active", 1),
            ("idle", 1),
            ("available", 1),
            ("offering", 1),
            ("commit", 1),
            ("consume", 1),
            ("busy_started", 1),
            ("busy_next", 1),
            ("not_miss", 1),
            ("leaf_owned", 1),
            ("ready_active", 1),
            ("miss_valid", 1),
        ] {
            s.declare_wire(name, GroundType::UInt { width }, sp);
        }
        for j in 0..=4 {
            for (name, width) in [
                ("hit", 1),
                ("ready", 1),
                ("valid", 1),
                ("data", 32),
                ("error", 2),
            ] {
                s.declare_wire(format!("{name}_{j}"), GroundType::UInt { width }, sp);
            }
        }
        s.declare_wire("no_hit", GroundType::UInt { width: 1 }, sp);
        for leaf in leaves {
            for (name, width) in [
                ("page", 8),
                ("base", 16),
                ("id", 2),
                ("hit", 1),
                ("owner_eq", 1),
                ("owned", 1),
            ] {
                s.declare_wire(format!("{leaf}_{name}"), GroundType::UInt { width }, sp);
            }
        }
        s.begin_combinational(sp);
        s.assign_lit("one", 1, sp);
        s.assign_lit("zero", 0, sp);
        s.assign_lit("zero32", 0, sp);
        s.assign_lit("zero2", 0, sp);
        s.assign_lit("decerr", 3, sp);
        // Compare all eight high address bits before calculating the local
        // offset. In particular, 0x8104 must never alias GPIO offset 4.
        s.assign_slice("page", "addr", 8, 8, sp);
        s.assign_slice("select_owner", "addr", 8, 2, sp);
        s.assign_xor("active", "rst", "one", sp);
        s.assign_xor("idle", "busy", "one", sp);
        s.assign_and("available", "idle", "active", sp);
        s.assign_and("offering", "available", "req_valid", sp);
        s.assign_xor("not_miss", "miss", "one", sp);
        s.assign_and("leaf_owned", "busy", "not_miss", sp);
        s.assign_and("ready_active", "rsp_ready", "active", sp);
        s.assign_and("miss_valid", "busy", "miss", sp);
        s.assign_lit("hit_0", 0, sp);
        s.assign_lit("ready_0", 1, sp); // An idle miss always has local capacity.
        s.assign_net("valid_0", "miss_valid", sp);
        s.assign_net("data_0", "zero32", sp);
        s.assign_mux("error_0", "miss_valid", "decerr", "zero2", sp);
        for (j, leaf) in leaves.iter().enumerate() {
            let n = |suffix: &str| format!("{leaf}_{suffix}");
            s.assign_lit(n("page"), j as u64, sp);
            s.assign_lit(n("base"), (j * 256) as u64, sp);
            s.assign_lit(n("id"), j as u64, sp);
            s.assign_eq(n("hit"), "page", n("page"), sp);
            s.assign_or(format!("hit_{}", j + 1), format!("hit_{j}"), n("hit"), sp);
            s.assign_mux(
                format!("ready_{}", j + 1),
                n("hit"),
                n("req_ready"),
                format!("ready_{j}"),
                sp,
            );
            s.assign_and(n("req_valid"), "offering", n("hit"), sp);
            s.assign_sub(n("addr"), "addr", n("base"), sp);
            for field in ["write", "wdata", "wstrb"] {
                s.assign_net(n(field), field, sp);
            }
            s.assign_eq(n("owner_eq"), "owner", n("id"), sp);
            s.assign_and(n("owned"), "leaf_owned", n("owner_eq"), sp);
            s.assign_and(n("rsp_ready"), n("owned"), "ready_active", sp);
            for (dst, src) in [
                ("valid", "rsp_valid"),
                ("data", "rdata"),
                ("error", "error"),
            ] {
                s.assign_mux(
                    format!("{dst}_{}", j + 1),
                    n("owned"),
                    n(src),
                    format!("{dst}_{j}"),
                    sp,
                );
            }
        }
        s.assign_xor("no_hit", "hit_4", "one", sp);
        s.assign_and("req_ready", "available", "ready_4", sp);
        s.assign_net("rsp_valid", "valid_4", sp);
        s.assign_net("rdata", "data_4", sp);
        s.assign_net("error", "error_4", sp);
        s.assign_and("commit", "req_valid", "req_ready", sp);
        s.assign_and("consume", "rsp_valid", "ready_active", sp);
        s.assign_or("busy_started", "busy", "commit", sp);
        s.assign_mux("busy_next", "consume", "zero", "busy_started", sp);
        s.end_process();
        s.begin_sequential(sp);
        s.assign_reg_d_from("busy", "busy_next", sp);
        s.assign_reg_d_mux("miss", "commit", "no_hit", "miss", sp);
        s.assign_reg_d_mux("owner", "commit", "select_owner", "owner", sp);
        s.end_process();
        Ok(())
    }
}

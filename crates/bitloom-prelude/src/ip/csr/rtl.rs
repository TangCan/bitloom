use super::*;
use crate::GroundType;

/// Small expression emitter into the existing builder, never a second IR.
/// Private names cannot collide with the public identifier grammar.
struct Logic<'a> {
    s: &'a mut ElaborateSession,
    serial: usize,
}
impl Logic<'_> {
    fn wire(&mut self, width: u32) -> String {
        let name = format!("_csr_n{}", self.serial);
        self.serial += 1;
        self.s
            .declare_wire(&name, GroundType::UInt { width }, Span::default());
        name
    }
    fn lit(&mut self, width: u32, value: u64) -> String {
        let n = self.wire(width);
        self.s.assign_lit(&n, value, Span::default());
        n
    }
    fn and(&mut self, width: u32, a: &str, b: &str) -> String {
        let n = self.wire(width);
        self.s.assign_and(&n, a, b, Span::default());
        n
    }
    fn or(&mut self, width: u32, a: &str, b: &str) -> String {
        let n = self.wire(width);
        self.s.assign_or(&n, a, b, Span::default());
        n
    }
    fn not(&mut self, width: u32, a: &str) -> String {
        let ones = self.lit(width, (1u64 << width) - 1);
        let n = self.wire(width);
        self.s.assign_xor(&n, a, ones, Span::default());
        n
    }
    fn eq(&mut self, a: &str, b: &str) -> String {
        let n = self.wire(1);
        self.s.assign_eq(&n, a, b, Span::default());
        n
    }
    fn mux(&mut self, width: u32, condition: &str, yes: &str, no: &str) -> String {
        let n = self.wire(width);
        self.s.assign_mux(&n, condition, yes, no, Span::default());
        n
    }
    fn output(&mut self, port: &str, value: &str) {
        self.s.assign_net(port, value, Span::default());
    }
}

pub(super) fn define_body(
    s: &mut ElaborateSession,
    params: &[(String, u32)],
) -> Result<(), Diagnostics> {
    let block = codec::decode(params)?;
    let sp = Span::default();
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
    for (name, width) in [("_csr_pending", 1), ("_csr_data", 32), ("_csr_error", 2)] {
        s.declare_reg(name, GroundType::UInt { width }, sp);
    }
    for (i, r) in block.registers.iter().enumerate() {
        for suffix in ["read_commit", "write_commit"] {
            s.add_output(
                format!("{}_{suffix}", r.name),
                GroundType::UInt { width: 1 },
                sp,
            );
        }
        if r.access != CsrAccess::Wo {
            let value = format!("{}_value", r.name);
            if r.owner == CsrOwner::External {
                s.add_input(value, GroundType::UInt { width: 32 }, sp);
            } else {
                s.add_output(value, GroundType::UInt { width: 32 }, sp);
                s.declare_reg(format!("_csr_value{i}"), GroundType::UInt { width: 32 }, sp);
            }
        }
        if r.access.writable() {
            for suffix in ["candidate", "write_mask"] {
                s.add_output(
                    format!("{}_{suffix}", r.name),
                    GroundType::UInt { width: 32 },
                    sp,
                );
            }
        }
        for (enabled, suffix) in [
            (r.read_reject, "read_reject"),
            (r.write_reject, "write_reject"),
        ] {
            if enabled {
                s.add_input(
                    format!("{}_{suffix}", r.name),
                    GroundType::UInt { width: 1 },
                    sp,
                );
            }
        }
        if let Some(event) = &r.event {
            s.add_input(event, GroundType::UInt { width: 32 }, sp);
        }
    }
    s.begin_combinational(sp);
    let mut l = Logic { s, serial: 0 };
    let zero = l.lit(1, 0);
    let one = l.lit(1, 1);
    let zero32 = l.lit(32, 0);
    let okay = l.lit(2, 0);
    let slverr = l.lit(2, 2);
    let idle = l.not(1, "_csr_pending");
    let active = l.not(1, "rst");
    let ready = l.and(1, &idle, &active);
    l.output("req_ready", &ready);
    let submit = l.and(1, "req_valid", &ready);
    let reading = l.not(1, "write");
    let pending_after_consume = l.mux(1, "rsp_ready", &zero, "_csr_pending");
    let next_pending = l.mux(1, &submit, &one, &pending_after_consume);
    l.output("rsp_valid", "_csr_pending");
    l.output("rdata", "_csr_data");
    l.output("error", "_csr_error");
    // WSTRB -> little-endian byte mask; entirely independent of the handshake.
    let mut bytes = zero32.clone();
    for byte in 0..4 {
        let selected = l.wire(1);
        l.s.assign_slice(&selected, "wstrb", byte, 1, sp);
        let mask = l.lit(32, 0xff << (byte * 8));
        let selected_mask = l.mux(32, &selected, &mask, &zero32);
        bytes = l.or(32, &bytes, &selected_mask);
    }
    let mut response_data = zero32.clone();
    let mut response_error = slverr.clone();
    let mut updates = Vec::new();
    for (i, r) in block.registers.iter().enumerate() {
        let mask = l.lit(32, r.mask() as u64);
        let current = if r.access == CsrAccess::Wo {
            zero32.clone()
        } else {
            let raw = if r.owner == CsrOwner::Leaf {
                format!("_csr_value{i}")
            } else {
                format!("{}_value", r.name)
            };
            let value = l.and(32, &raw, &mask);
            if r.owner == CsrOwner::Leaf {
                l.output(&format!("{}_value", r.name), &value);
            }
            value
        };
        let offset = l.lit(16, r.offset as u64);
        let hit = l.eq("addr", &offset);
        let selected_submit = l.and(1, &submit, &hit);
        let read_allowed = if !r.access.readable() {
            zero.clone()
        } else if r.read_reject {
            l.not(1, &format!("{}_read_reject", r.name))
        } else {
            one.clone()
        };
        let good_read = l.and(1, &reading, &read_allowed);
        let read_commit = l.and(1, &selected_submit, &good_read);
        l.output(&format!("{}_read_commit", r.name), &read_commit);
        let (good_write, write_commit, candidate) = if r.access.writable() {
            let write_mask = l.and(32, &bytes, &mask);
            let empty = l.eq(&write_mask, &zero32);
            let effective = l.not(1, &empty);
            let write_data = l.and(32, "wdata", &write_mask);
            let candidate = if r.access == CsrAccess::Rw {
                let inverse = l.not(32, &write_mask);
                let retained = l.and(32, &current, &inverse);
                l.or(32, &retained, &write_data)
            } else {
                write_data
            };
            l.output(&format!("{}_write_mask", r.name), &write_mask);
            l.output(&format!("{}_candidate", r.name), &candidate);
            let allowed = if r.write_reject {
                let not_rejected = l.not(1, &format!("{}_write_reject", r.name));
                l.or(1, &empty, &not_rejected)
            } else {
                one.clone()
            };
            let good = l.and(1, "write", &allowed);
            let effective_good = l.and(1, &good, &effective);
            let commit = l.and(1, &selected_submit, &effective_good);
            (good, commit, candidate)
        } else {
            (zero.clone(), zero.clone(), zero32.clone())
        };
        l.output(&format!("{}_write_commit", r.name), &write_commit);
        let success = l.or(1, &good_read, &good_write);
        let reg_error = l.mux(2, &success, &okay, &slverr);
        response_error = l.mux(2, &hit, &reg_error, &response_error);
        let read_data = l.mux(32, &good_read, &current, &zero32);
        response_data = l.mux(32, &hit, &read_data, &response_data);
        if r.owner == CsrOwner::Leaf {
            let next = if r.access == CsrAccess::W1c {
                // Natural events are independent of submit, reject, and pending.
                let clear = l.mux(32, &write_commit, &candidate, &zero32);
                let inverse = l.not(32, &clear);
                let retained = l.and(32, &current, &inverse);
                let event = l.and(32, r.event.as_deref().expect("validated W1C event"), &mask);
                l.or(32, &retained, &event)
            } else {
                l.mux(32, &write_commit, &candidate, &current)
            };
            updates.push((format!("_csr_value{i}"), next));
        }
    }
    let next_data = l.mux(32, &submit, &response_data, "_csr_data");
    let next_error = l.mux(2, &submit, &response_error, "_csr_error");
    l.s.end_process();
    // Existing builder registers implement synchronous, active-high reset0 with
    // priority over all D inputs, including event accumulation.
    l.s.begin_sequential(sp);
    for (register, next) in [
        ("_csr_pending", &next_pending),
        ("_csr_data", &next_data),
        ("_csr_error", &next_error),
    ] {
        l.s.assign_reg_d_from(register, next, sp);
    }
    for (register, next) in updates {
        l.s.assign_reg_d_from(register, next, sp);
    }
    l.s.end_process();
    Ok(())
}

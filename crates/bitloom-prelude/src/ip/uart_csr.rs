use crate::ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister, ParamSyncFifo};
use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Buffered 8N1 UART with six local CSRs and two four-byte FIFOs.
///
/// Apply synchronous active-high reset before use. BAUD_DIV is a full-width
/// 32-bit divider; an enabled frame requires DIV >= 3 and lasts DIV+1 clocks
/// per bit. RX uses two synchronization stages and center sampling. This
/// logical synchronizer is not a board-level metastability or baud-tolerance guarantee.
/// `raw_events` bits are RX arrival, TX dequeue, overflow and framing error.
/// They are pre-edge pulses, suitable for a same-domain interrupt controller.
pub struct UartCsr;
const CSR_MODULE: &str = "BitloomUartCsrRegisters";
const FIFO_MODULE: &str = "BitloomUartCsrFifo8x4";

impl Elaboratable for UartCsr {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("UartCsr");
        Self::define_module(&mut s, "UartCsr")?;
        s.finish()
    }
}
impl UartCsr {
    /// Local byte offsets; the caller supplies the system base address.
    pub fn registers() -> CsrBlock {
        use CsrAccess::{Ro, Rw, W1c, Wo};
        use CsrOwner::{External, Leaf};
        CsrBlock {
            name: "UartCsr".into(),
            registers: [
                ("ctrl", 0, 1, Rw, Leaf, None, false, true),
                ("baud_div", 4, u32::MAX, Rw, Leaf, None, false, true),
                ("status", 8, 15, Ro, External, None, false, false),
                ("tx_data", 12, 255, Wo, CsrOwner::None, None, false, true),
                ("rx_data", 16, 255, Ro, External, None, true, false),
                ("EVENT", 20, 15, W1c, Leaf, Some("event_bits"), false, false),
            ]
            .into_iter()
            .map(
                |(name, offset, mask, access, owner, event, read_reject, write_reject)| {
                    CsrRegister {
                        name: name.into(),
                        offset,
                        reset: 0,
                        access,
                        owner,
                        event: event.map(str::to_owned),
                        read_reject,
                        write_reject,
                        fields: vec![CsrField {
                            name: "bits".into(),
                            mask: mask as u64,
                            reset: 0,
                            access,
                        }],
                    }
                },
            )
            .collect(),
        }
    }
    /// Define or reuse the UART, CSR leaf and FIFO in one unfrozen session.
    pub fn define_module(
        s: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        Self::registers().define_module(s, CSR_MODULE)?;
        ParamSyncFifo::<8, 4>::define_module(s, FIFO_MODULE)?;
        s.define_module(name, vec![], Self::define_body)
    }
    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        let mut c = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (n, w) in [
            ("req_valid", 1),
            ("write", 1),
            ("addr", 16),
            ("wdata", 32),
            ("wstrb", 4),
            ("rsp_ready", 1),
        ] {
            s.add_input(n, GroundType::UInt { width: w }, sp);
            c.push((n.into(), n.into()));
        }
        for (n, w) in [
            ("req_ready", 1),
            ("rsp_valid", 1),
            ("rdata", 32),
            ("error", 2),
        ] {
            s.add_output(n, GroundType::UInt { width: w }, sp);
            c.push((n.into(), n.into()));
        }
        s.add_input("rx", GroundType::UInt { width: 1 }, sp);
        s.add_output("tx", GroundType::UInt { width: 1 }, sp);
        s.add_output("raw_events", GroundType::UInt { width: 4 }, sp);
        for r in Self::registers().registers {
            let mut ports = vec![("read_commit", 1), ("write_commit", 1)];
            if r.access != CsrAccess::Wo {
                ports.push(("value", 32));
            }
            if r.access != CsrAccess::Ro {
                ports.extend([("candidate", 32), ("write_mask", 32)]);
            }
            if r.read_reject {
                ports.push(("read_reject", 1));
            }
            if r.write_reject {
                ports.push(("write_reject", 1));
            }
            for (suffix, width) in ports {
                let n = format!("{}_{suffix}", r.name);
                s.declare_wire(&n, GroundType::UInt { width }, sp);
                c.push((n.clone(), n));
            }
        }
        s.declare_wire("event_bits", GroundType::UInt { width: 32 }, sp);
        c.push(("event_bits".into(), "event_bits".into()));
        s.add_instance("csr", CSR_MODULE, c, vec![], sp);
        s.declare_wire("zero_bit", GroundType::UInt { width: 1 }, sp);
        for prefix in ["txq", "rxq"] {
            let mut c = vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("flush".into(), "zero_bit".into()),
            ];
            for (n, width) in [
                ("input_valid", 1),
                ("input_ready", 1),
                ("input_data", 8),
                ("output_valid", 1),
                ("output_ready", 1),
                ("output_data", 8),
            ] {
                let net = format!("{prefix}_{n}");
                s.declare_wire(&net, GroundType::UInt { width }, sp);
                c.push((n.into(), net));
            }
            s.add_instance(prefix, FIFO_MODULE, c, vec![], sp);
        }
        for (n, width) in [
            ("sync1", 1),
            ("sync2", 1),
            ("history", 1),
            ("tx_busy", 1),
            ("tx_line", 1),
            ("tx_index", 4),
            ("tx_shift", 8),
            ("tx_timer", 32),
            ("tx_div", 32),
            ("rx_state", 4),
            ("rx_shift", 8),
            ("rx_timer", 32),
            ("rx_div", 32),
        ] {
            s.declare_reg(n, GroundType::UInt { width }, sp);
        }
        s.begin_combinational(sp);
        let mut l = Logic { s, serial: 0 };
        let z = l.lit(1, 0);
        let one = l.lit(1, 1);
        let z4 = l.lit(4, 0);
        let one4 = l.lit(4, 1);
        let z32 = l.lit(32, 0);
        let one32 = l.lit(32, 1);
        l.out("zero_bit", &z);
        let active = l.not("rst");
        let rx_idle = l.eq("rx_state", &z4);
        let rx_busy = l.not(&rx_idle);
        let busy = l.or("tx_busy", &rx_busy);
        let enabled = l.slice("ctrl_value", 0, 1);
        let candidate_enable = l.slice("ctrl_candidate", 0, 1);
        let same = l.eq(&enabled, &candidate_enable);
        let changed = l.not(&same);
        let busy_change = l.and(&busy, &changed);
        let three = l.lit(32, 3);
        let invalid = l.ult("baud_div_value", &three);
        let invalid_enable = l.and(&candidate_enable, &invalid);
        let reject = l.or(&busy_change, &invalid_enable);
        l.out("ctrl_write_reject", &reject);
        let invalid_candidate = l.ult("baud_div_candidate", &three);
        let invalid_write = l.and(&enabled, &invalid_candidate);
        let reject = l.or(&busy, &invalid_write);
        l.out("baud_div_write_reject", &reject);
        let tx_full = l.not("txq_input_ready");
        l.out("tx_data_write_reject", &tx_full);
        let rx_empty = l.not("rxq_output_valid");
        l.out("rx_data_read_reject", &rx_empty);
        let effective_enable = l.mux(1, "ctrl_write_commit", &candidate_enable, &enabled);
        let effective_div = l.mux(
            32,
            "baud_div_write_commit",
            "baud_div_candidate",
            "baud_div_value",
        );
        let status_lo = l.concat(&tx_full, 1, "rxq_output_valid", 1);
        let status_hi = l.concat(&rx_busy, 1, "tx_busy", 1);
        let status4 = l.concat(&status_hi, 2, &status_lo, 2);
        let z28 = l.lit(28, 0);
        let status = l.concat(&z28, 28, &status4, 4);
        l.out("status_value", &status);
        let z24 = l.lit(24, 0);
        let data = l.concat(&z24, 24, "rxq_output_data", 8);
        l.out("rx_data_value", &data);
        l.out("txq_input_valid", "tx_data_write_commit");
        let byte = l.slice("tx_data_candidate", 0, 8);
        l.out("txq_input_data", &byte);
        l.out("rxq_output_ready", "rx_data_read_commit");
        l.out("rxq_input_data", "rx_shift");

        // Configuration commits flow only forward into idle launch decisions.
        let tx_idle = l.not("tx_busy");
        let launch = l.and(&tx_idle, &effective_enable);
        let launch = l.and(&launch, "txq_output_valid");
        let launch = l.and(&launch, &active);
        l.out("txq_output_ready", &launch);
        let tx_zero = l.eq("tx_timer", &z32);
        let tx_step = l.and("tx_busy", &tx_zero);
        let nine = l.lit(4, 9);
        let tx_last = l.eq("tx_index", &nine);
        let done = l.and(&tx_step, &tx_last);
        let still_busy = l.mux(1, &done, &z, "tx_busy");
        let tx_busy_next = l.mux(1, &launch, &one, &still_busy);
        let index_inc = l.add(4, "tx_index", &one4);
        let index_step = l.mux(4, &tx_step, &index_inc, "tx_index");
        let tx_index_next = l.mux(4, &launch, &z4, &index_step);
        let tx_dec = l.sub(32, "tx_timer", &one32);
        let tx_reload = l.mux(32, &tx_zero, "tx_div", &tx_dec);
        let tx_running = l.mux(32, "tx_busy", &tx_reload, "tx_timer");
        let tx_timer_next = l.mux(32, &launch, &effective_div, &tx_running);
        let tx_div_next = l.mux(32, &launch, &effective_div, "tx_div");
        let shift7 = l.slice("tx_shift", 1, 7);
        let shift = l.concat(&z, 1, &shift7, 7);
        let shifted = l.mux(8, &tx_step, &shift, "tx_shift");
        let tx_shift_next = l.mux(8, &launch, "txq_output_data", &shifted);
        let eight = l.lit(4, 8);
        let stop = l.eq("tx_index", &eight);
        let data0 = l.slice("tx_shift", 0, 1);
        let next_bit = l.mux(1, &stop, &one, &data0);
        let line_step = l.mux(1, &tx_step, &next_bit, "tx_line");
        let tx_line_next = l.mux(1, &launch, &z, &line_step);
        let tx = l.mux(1, "tx_busy", "tx_line", &one);
        l.out("tx", &tx);

        // e uses pre-edge sync2/history. H=DIV/2 + DIV%2 cannot overflow.
        let low = l.not("sync2");
        let falling = l.and(&low, "history");
        let start = l.and(&rx_idle, &falling);
        let start = l.and(&start, &effective_enable);
        let start = l.and(&start, &active);
        let half31 = l.slice(&effective_div, 1, 31);
        let half = l.concat(&z, 1, &half31, 31);
        let odd = l.slice(&effective_div, 0, 1);
        let z31 = l.lit(31, 0);
        let odd32 = l.concat(&z31, 31, &odd, 1);
        let half = l.add(32, &half, &odd32);
        let initial_timer = l.sub(32, &half, &one32);
        let rx_zero = l.eq("rx_timer", &z32);
        let sample = l.and(&rx_busy, &rx_zero);
        let confirm = l.eq("rx_state", &one4);
        let ten = l.lit(4, 10);
        let stop = l.eq("rx_state", &ten);
        let bad_start = l.and(&confirm, "sync2");
        let end = l.or(&bad_start, &stop);
        let finish = l.and(&sample, &end);
        let increment = l.add(4, "rx_state", &one4);
        let advance = l.mux(4, &sample, &increment, "rx_state");
        let finish_state = l.mux(4, &finish, &z4, &advance);
        let rx_state_next = l.mux(4, &start, &one4, &finish_state);
        let rx_dec = l.sub(32, "rx_timer", &one32);
        let reload = l.mux(32, &rx_zero, "rx_div", &rx_dec);
        let rx_running = l.mux(32, &rx_busy, &reload, "rx_timer");
        let rx_timer_next = l.mux(32, &start, &initial_timer, &rx_running);
        let rx_div_next = l.mux(32, &start, &effective_div, "rx_div");
        let not_confirm = l.not(&confirm);
        let not_stop = l.not(&stop);
        let data_phase = l.and(&not_confirm, &not_stop);
        let data_sample = l.and(&sample, &data_phase);
        let lower = l.slice("rx_shift", 1, 7);
        let captured = l.concat("sync2", 1, &lower, 7);
        let rx_shift_next = l.mux(8, &data_sample, &captured, "rx_shift");
        let arrived = l.and(&sample, &stop);
        let arrived = l.and(&arrived, &active);
        let good = l.and(&arrived, "sync2");
        let receive = l.and(&good, "rxq_input_ready");
        let full = l.not("rxq_input_ready");
        let overflow = l.and(&good, &full);
        let framing = l.and(&arrived, &low);
        l.out("rxq_input_valid", &receive);
        let events_lo = l.concat(&launch, 1, &receive, 1);
        let events_hi = l.concat(&framing, 1, &overflow, 1);
        let events = l.concat(&events_hi, 2, &events_lo, 2);
        l.out("raw_events", &events);
        let events32 = l.concat(&z28, 28, &events, 4);
        l.out("event_bits", &events32);
        l.s.end_process();
        l.s.begin_sequential(sp);
        for (reg, next) in [
            ("sync1", "rx"),
            ("sync2", "sync1"),
            ("history", "sync2"),
            ("tx_busy", &tx_busy_next),
            ("tx_index", &tx_index_next),
            ("tx_timer", &tx_timer_next),
            ("tx_div", &tx_div_next),
            ("tx_shift", &tx_shift_next),
            ("tx_line", &tx_line_next),
            ("rx_state", &rx_state_next),
            ("rx_timer", &rx_timer_next),
            ("rx_div", &rx_div_next),
            ("rx_shift", &rx_shift_next),
        ] {
            l.s.assign_reg_d_from(reg, next, sp);
        }
        l.s.end_process();
        Ok(())
    }
}

// Private expression conveniences emit straight into the existing builder.
struct Logic<'a> {
    s: &'a mut ElaborateSession,
    serial: usize,
}
impl Logic<'_> {
    fn wire(&mut self, width: u32) -> String {
        let n = format!("_uart_n{}", self.serial);
        self.serial += 1;
        self.s
            .declare_wire(&n, GroundType::UInt { width }, Span::default());
        n
    }
    fn lit(&mut self, w: u32, v: u64) -> String {
        let n = self.wire(w);
        self.s.assign_lit(&n, v, Span::default());
        n
    }
    fn out(&mut self, n: &str, v: &str) {
        self.s.assign_net(n, v, Span::default());
    }
    fn slice(&mut self, a: &str, lo: u32, w: u32) -> String {
        let n = self.wire(w);
        self.s.assign_slice(&n, a, lo, w, Span::default());
        n
    }
    fn concat(&mut self, a: &str, aw: u32, b: &str, bw: u32) -> String {
        let n = self.wire(aw + bw);
        self.s.assign_concat(&n, a, b, Span::default());
        n
    }
    fn mux(&mut self, w: u32, c: &str, a: &str, b: &str) -> String {
        let n = self.wire(w);
        self.s.assign_mux(&n, c, a, b, Span::default());
        n
    }
    fn eq(&mut self, a: &str, b: &str) -> String {
        let n = self.wire(1);
        self.s.assign_eq(&n, a, b, Span::default());
        n
    }
    fn ult(&mut self, a: &str, b: &str) -> String {
        let n = format!("_uart_n{}", self.serial);
        self.serial += 1;
        self.s.declare_wire(&n, GroundType::Bool, Span::default());
        self.s.assign_ult(&n, a, b, Span::default());
        n
    }
    fn not(&mut self, a: &str) -> String {
        let one = self.lit(1, 1);
        let n = self.wire(1);
        self.s.assign_xor(&n, a, &one, Span::default());
        n
    }
    fn and(&mut self, a: &str, b: &str) -> String {
        let n = self.wire(1);
        self.s.assign_and(&n, a, b, Span::default());
        n
    }
    fn or(&mut self, a: &str, b: &str) -> String {
        let n = self.wire(1);
        self.s.assign_or(&n, a, b, Span::default());
        n
    }
    fn add(&mut self, w: u32, a: &str, b: &str) -> String {
        let n = self.wire(w);
        self.s.assign_add(&n, a, b, Span::default());
        n
    }
    fn sub(&mut self, w: u32, a: &str, b: &str) -> String {
        let n = self.wire(w);
        self.s.assign_sub(&n, a, b, Span::default());
        n
    }
}

//! FR103 — first-class IP dual-model matrix (NFR14 FIFO/UART/SPI/I2C/AXI).
//!
//! Cycle path = FrozenHir [`crate::Sim::tick`]. Functional path =
//! [`crate::GeneratedFunctional`] (generation path) **or** handwritten
//! [`SyncFifoFunctional`] when FR103 nails architectural FL for Mem-based FIFO.
//! **FR126** adds handwritten [`GpioFunctional`]; **FR135** adds handwritten
//! [`UartTxFunctional`]; **FR163** adds handwritten [`UartRxFunctional`] beyond
//! UartTx / Gpio / GeneratedFunctional alone.
//!
//! Design crates stay on `bitloom-prelude`; this module lives in the toolchain.

use bitloom_hir::{FrozenHir, PortValues};

use crate::{
    AbstractionView, EquivStatus, FormalEquivProduct, PortMismatch, Sim, check_functional_equiv,
    compare_port_values,
};

/// Depth-4 SyncFifo functional model (architectural PortValues).
///
/// FR103 completion face for FIFO. **FR112** separately deepens
/// `GeneratedFunctional` MemRead≡tick; that path does not replace this
/// handwritten SyncFifo FL (see `docs/fr103-ip-dual-model.md`).
#[derive(Debug, Clone, Default)]
pub struct SyncFifoFunctional {
    ram: [u64; 4],
    wr_ptr: u8,
    rd_ptr: u8,
    count: u8,
    dout: u64,
}

impl SyncFifoFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for SyncFifoFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let wr_en = inputs.get("wr_en").unwrap_or(0) != 0;
        let rd_en = inputs.get("rd_en").unwrap_or(0) != 0;
        let data_in = inputs.get("data_in").unwrap_or(0);

        if rst {
            self.ram = [0; 4];
            self.wr_ptr = 0;
            self.rd_ptr = 0;
            self.count = 0;
            self.dout = 0;
        } else {
            // Match SyncFifo HIR order: mem write (if can_wr), then async
            // dout := ram[rd_ptr], then advance pointers/count.
            let full = self.count >= 4;
            let empty = self.count == 0;
            let can_wr = wr_en && !full;
            let can_rd = rd_en && !empty;

            if can_wr {
                self.ram[self.wr_ptr as usize % 4] = data_in & 0xff;
            }
            // Unconditional async-style head peek (assign_reg_d_mem_read on declare_mem).
            self.dout = self.ram[self.rd_ptr as usize % 4];

            if can_wr {
                self.wr_ptr = self.wr_ptr.wrapping_add(1) & 0b11;
            }
            if can_rd {
                self.rd_ptr = self.rd_ptr.wrapping_add(1) & 0b11;
            }
            match (can_wr, can_rd) {
                (true, true) => {}
                (true, false) => self.count = self.count.saturating_add(1).min(4),
                (false, true) => self.count = self.count.saturating_sub(1),
                (false, false) => {}
            }
        }

        let mut out = inputs.clone();
        out.set("full", u64::from(self.count >= 4));
        out.set("empty", u64::from(self.count == 0));
        out.set("data_out", self.dout);
        out
    }
}

/// Documented SyncFifo dual-model stimulus (reset, push, pop) — FR103 fixture.
pub fn sync_fifo_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64, wr_en: u64, rd_en: u64, data_in: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("wr_en", wr_en);
        pv.set("rd_en", rd_en);
        pv.set("data_in", data_in);
        out.push(pv);
    };
    frame(1, 0, 0, 0);
    frame(0, 1, 0, 0x11);
    frame(0, 1, 0, 0x22);
    frame(0, 0, 1, 0);
    frame(0, 0, 1, 0);
    frame(0, 0, 0, 0);
    out
}

/// Handwritten `Gpio` FL (FR126) — beyond FR103 SyncFifo / GeneratedFunctional UART–AXI.
///
/// Models architectural ports `pad_out` / `rd_data` ≡ `Sim::tick` on
/// [`gpio_dual_stimulus`]. Not GeneratedFunctional; not FR112 MemRead≡tick;
/// not FR119 sby alone.
#[derive(Debug, Clone, Default)]
pub struct GpioFunctional {
    out_r: u64,
}

impl GpioFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for GpioFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let dir = inputs.get("dir").unwrap_or(0) & 0xff;
        let wr_en = inputs.get("wr_en").unwrap_or(0) != 0;
        let wr_data = inputs.get("wr_data").unwrap_or(0) & 0xff;
        let wr_mask = inputs.get("wr_mask").unwrap_or(0) & 0xff;
        let pad_in = inputs.get("pad_in").unwrap_or(0) & 0xff;

        if rst {
            self.out_r = 0;
        } else if wr_en {
            let kept = self.out_r & (!wr_mask & 0xff);
            let newt = wr_data & wr_mask;
            self.out_r = (kept | newt) & 0xff;
        }

        let pad_out = self.out_r & dir;
        let rd_data = (self.out_r & dir) | (pad_in & (!dir & 0xff));

        let mut out = inputs.clone();
        out.set("pad_out", pad_out);
        out.set("rd_data", rd_data);
        out
    }
}

/// Documented Gpio dual-model stimulus (reset, masked write, pad read) — FR126.
pub fn gpio_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64, dir: u64, wr_en: u64, wr_data: u64, wr_mask: u64, pad_in: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("dir", dir);
        pv.set("wr_en", wr_en);
        pv.set("wr_data", wr_data);
        pv.set("wr_mask", wr_mask);
        pv.set("pad_in", pad_in);
        out.push(pv);
    };
    frame(1, 0xff, 0, 0, 0, 0);
    frame(0, 0xff, 1, 0xa5, 0xff, 0);
    frame(0, 0xff, 0, 0, 0, 0);
    frame(0, 0x0f, 0, 0, 0, 0xf0); // lower nybble out, upper from pad
    frame(0, 0x0f, 1, 0x03, 0x0f, 0xf0);
    frame(0, 0x0f, 0, 0, 0, 0xaa);
    out
}

/// Handwritten `UartTx` FL (FR135) — beyond FR126 Gpio / FR103 SyncFifo / GeneratedFunctional.
///
/// Models architectural ports `tx` / `tx_byte` / `tx_busy` ≡ `Sim::settle`+`tick` on
/// [`uart_tx_dual_stimulus`]. Not GeneratedFunctional; not Gpio alone; not SyncFifo alone.
#[derive(Debug, Clone, Default)]
pub struct UartTxFunctional {
    hold: u64,
    shift_reg: u64,
    busy: u64,
    bit_idx: u64,
    baud_cnt: u64,
}

impl UartTxFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for UartTxFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let wr_en = inputs.get("wr_en").unwrap_or(0) != 0;
        let wr_data = inputs.get("wr_data").unwrap_or(0) & 0xff;
        let baud_div = inputs.get("baud_div").unwrap_or(0) & 0xff;

        if rst {
            self.hold = 0;
            self.shift_reg = 0;
            self.busy = 0;
            self.bit_idx = 0;
            self.baud_cnt = 0;
        } else {
            let busy = self.busy != 0;
            let accept = wr_en && !busy;
            let is_start = self.bit_idx == 0;
            let is_stop = self.bit_idx == 9;
            let baud_eq = self.baud_cnt == baud_div;
            let baud_tick = busy && baud_eq;

            let baud_cnt_busy = if baud_eq {
                0
            } else {
                self.baud_cnt.wrapping_add(1) & 0xff
            };
            let baud_cnt_busy_or_idle = if busy { baud_cnt_busy } else { 0 };
            let next_baud_cnt = if accept { 0 } else { baud_cnt_busy_or_idle };

            let busy_after_tick = if is_stop { 0 } else { 1 };
            let busy_when_busy = if baud_tick { busy_after_tick } else { 1 };
            let busy_when_busy_or_idle = if busy { busy_when_busy } else { 0 };
            let next_busy = if accept { 1 } else { busy_when_busy_or_idle };

            let bit_idx_p1 = (self.bit_idx.wrapping_add(1)) & 0xf;
            let bit_after_tick = if is_stop { 0 } else { bit_idx_p1 };
            let bit_when_busy = if baud_tick {
                bit_after_tick
            } else {
                self.bit_idx
            };
            let bit_when_busy_or_idle = if busy { bit_when_busy } else { 0 };
            let next_bit_idx = if accept { 0 } else { bit_when_busy_or_idle };

            let do_shift = busy && !is_start && !is_stop;
            let do_shift_tick = do_shift && baud_eq;
            let shift_shr = (self.shift_reg >> 1) & 0xff;
            let shift_after_tick = if do_shift_tick {
                shift_shr
            } else {
                self.shift_reg
            };
            let next_shift_busy = if busy {
                shift_after_tick
            } else {
                self.shift_reg
            };
            let next_shift_final = if accept { wr_data } else { next_shift_busy };
            let next_hold = if accept { wr_data } else { self.hold };

            self.busy = next_busy;
            self.bit_idx = next_bit_idx;
            self.shift_reg = next_shift_final;
            self.hold = next_hold;
            self.baud_cnt = next_baud_cnt;
        }

        let busy = self.busy != 0;
        let is_start = self.bit_idx == 0;
        let is_stop = self.bit_idx == 9;
        let data_bit = (self.shift_reg & 1) != 0;
        let tx_data_or_stop = if is_stop { 1 } else { u64::from(data_bit) };
        let tx_active = if is_start { 0 } else { tx_data_or_stop };
        let tx = if busy { tx_active } else { 1 };

        let mut out = inputs.clone();
        out.set("tx", tx);
        out.set("tx_byte", self.hold & 0xff);
        out.set("tx_busy", self.busy & 1);
        out
    }
}

/// Documented UartTx dual-model stimulus (reset, 8N1 frame, baud_div hold) — FR135.
pub fn uart_tx_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64, wr_en: u64, wr_data: u64, baud_div: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("wr_en", wr_en);
        pv.set("wr_data", wr_data);
        pv.set("baud_div", baud_div);
        out.push(pv);
    };
    // baud_div=0: 1 clk/bit — full 0xA5 frame + idle clear
    frame(1, 0, 0, 0);
    frame(0, 0, 0, 0);
    frame(0, 1, 0xa5, 0); // accept → start
    for _ in 0..9 {
        frame(0, 0, 0, 0); // data…stop
    }
    frame(0, 0, 0, 0); // clear busy
    // baud_div=1: 2 clk/bit — start held then LSB
    frame(1, 0, 0, 1);
    frame(0, 1, 0x01, 1);
    frame(0, 0, 0, 1); // start held
    frame(0, 0, 0, 1); // LSB
    // busy ignore: latch 0x3C, wr_en with 0xFF must not replace
    frame(1, 0, 0, 0);
    frame(0, 1, 0x3c, 0);
    frame(0, 1, 0xff, 0);
    frame(0, 0, 0, 0);
    out
}

/// Handwritten `UartRx` FL (FR163) — beyond FR135 `UartTx` / FR126 Gpio / GeneratedFunctional.
///
/// Models architectural ports `rd_data` / `rd_valid` / `rx_busy` ≡ `Sim::settle`+`tick` on
/// [`uart_rx_dual_stimulus`]. Not GeneratedFunctional; not UartTx alone; not Gpio alone.
#[derive(Debug, Clone, Default)]
pub struct UartRxFunctional {
    rx_prev: u64,
    busy: u64,
    bit_idx: u64,
    baud_cnt: u64,
    shift_reg: u64,
    hold: u64,
    valid: u64,
}

impl UartRxFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for UartRxFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let rx = inputs.get("rx").unwrap_or(1) & 1;
        let baud_div = inputs.get("baud_div").unwrap_or(0) & 0xff;

        if rst {
            self.rx_prev = 0;
            self.busy = 0;
            self.bit_idx = 0;
            self.baud_cnt = 0;
            self.shift_reg = 0;
            self.hold = 0;
            self.valid = 0;
        } else {
            let busy = self.busy != 0;
            let fall = self.rx_prev == 1 && rx == 0;
            let start = !busy && fall;
            let is_stop = self.bit_idx == 9;
            let baud_eq = self.baud_cnt == baud_div;
            let baud_tick = busy && baud_eq;

            let baud_cnt_busy = if baud_eq {
                0
            } else {
                self.baud_cnt.wrapping_add(1) & 0xff
            };
            let baud_cnt_busy_or_idle = if busy { baud_cnt_busy } else { 0 };
            let next_baud_cnt = if start { 0 } else { baud_cnt_busy_or_idle };

            let do_sample = baud_tick && !is_stop;
            let do_finish = baud_tick && is_stop;

            let shift_shr = (self.shift_reg >> 1) & 0xff;
            let rx8 = if rx != 0 { 0x80 } else { 0 };
            let shift_in = shift_shr | rx8;

            let busy_after_tick = if is_stop { 0 } else { 1 };
            let busy_when_busy = if baud_tick { busy_after_tick } else { 1 };
            let busy_when_busy_or_idle = if busy { busy_when_busy } else { 0 };
            let next_busy = if start { 1 } else { busy_when_busy_or_idle };

            let bit_idx_p1 = (self.bit_idx.wrapping_add(1)) & 0xf;
            let bit_after_tick = if is_stop { 0 } else { bit_idx_p1 };
            let bit_when_busy = if baud_tick {
                bit_after_tick
            } else {
                self.bit_idx
            };
            let bit_when_busy_or_idle = if busy { bit_when_busy } else { 0 };
            let next_bit_idx = if start { 1 } else { bit_when_busy_or_idle };

            let shift_after_sample = if do_sample { shift_in } else { self.shift_reg };
            let next_shift_busy = if busy {
                shift_after_sample
            } else {
                self.shift_reg
            };
            let next_shift = if start { 0 } else { next_shift_busy };

            let next_hold = if do_finish { self.shift_reg } else { self.hold };
            let next_valid = if do_finish { 1 } else { 0 };

            self.rx_prev = rx;
            self.busy = next_busy;
            self.bit_idx = next_bit_idx;
            self.baud_cnt = next_baud_cnt;
            self.shift_reg = next_shift;
            self.hold = next_hold;
            self.valid = next_valid;
        }

        let mut out = inputs.clone();
        out.set("rd_data", self.hold & 0xff);
        out.set("rd_valid", self.valid & 1);
        out.set("rx_busy", self.busy & 1);
        out
    }
}

/// Documented UartRx dual-model stimulus (reset, 8N1 RX frame, baud_div hold) — FR163.
pub fn uart_rx_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64, rx: u64, baud_div: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("rx", rx);
        pv.set("baud_div", baud_div);
        out.push(pv);
    };
    // baud_div=0: 1 clk/bit — 0xA5 LSB-first (matches prelude uart_rx smoke)
    frame(1, 1, 0);
    frame(0, 1, 0);
    // start + data 0b1010_0101 LSB-first + stop
    for &b in &[0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1] {
        frame(0, b, 0);
    }
    frame(0, 1, 0); // idle after valid
    // baud_div=1: 2 clk/bit — start held then sample 0x01 (LSB=1)
    frame(1, 1, 1);
    frame(0, 1, 1);
    frame(0, 0, 1); // fall → start
    frame(0, 0, 1); // start held (baud_cnt 0→1)
    frame(0, 1, 1); // sample bit0=1 @ baud_tick; bit_idx 1→2
    frame(0, 1, 1); // hold
    for _ in 0..7 {
        // remaining data 0 + stop, 2 clk each
        frame(0, 0, 1);
        frame(0, 0, 1);
    }
    frame(0, 1, 1); // stop held
    frame(0, 1, 1); // finish
    out
}

/// Architectural ports compared for SyncFifo dual-model (avoid internal wires).
const SYNC_FIFO_ARCH_PORTS: &[&str] = &["full", "empty", "data_out"];

/// Architectural ports for Gpio handwritten FL (FR126).
const GPIO_ARCH_PORTS: &[&str] = &["pad_out", "rd_data"];

/// Architectural ports for UartTx handwritten FL (FR135).
const UART_TX_ARCH_PORTS: &[&str] = &["tx", "tx_byte", "tx_busy"];

/// Architectural ports for UartRx handwritten FL (FR163).
const UART_RX_ARCH_PORTS: &[&str] = &["rd_data", "rd_valid", "rx_busy"];
/// FR103 product entry: co-verify functional + cycle models for the NFR14 IP set.
#[derive(Debug, Clone, Default)]
pub struct IpDualModelMatrix;

impl IpDualModelMatrix {
    pub fn new() -> Self {
        Self
    }

    /// SyncFifo: handwritten FL vs `settle`+`tick` on pinned stimulus.
    pub fn verify_sync_fifo(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = SyncFifoFunctional::new();
        let mut cycles = 0usize;
        for inputs in sync_fifo_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, SYNC_FIFO_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// Deliberate mismatch ATDD for SyncFifo.
    pub fn verify_sync_fifo_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in sync_fifo_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, SYNC_FIFO_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// FR126: handwritten `Gpio` FL ≡ tick on [`gpio_dual_stimulus`].
    pub fn verify_gpio_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = GpioFunctional::new();
        let mut cycles = 0usize;
        for inputs in gpio_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) = compare_named_ports(sim.ports(), &abs_out, GPIO_ARCH_PORTS) {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// FR135: handwritten `UartTx` FL ≡ tick on [`uart_tx_dual_stimulus`].
    pub fn verify_uart_tx_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = UartTxFunctional::new();
        let mut cycles = 0usize;
        for inputs in uart_tx_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) = compare_named_ports(sim.ports(), &abs_out, UART_TX_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// Deliberate mismatch / alternate FL ATDD for UartTx (FR135).
    pub fn verify_uart_tx_handwritten_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in uart_tx_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) = compare_named_ports(sim.ports(), &abs_out, UART_TX_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// FR163: handwritten `UartRx` FL ≡ tick on [`uart_rx_dual_stimulus`].
    pub fn verify_uart_rx_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = UartRxFunctional::new();
        let mut cycles = 0usize;
        for inputs in uart_rx_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) = compare_named_ports(sim.ports(), &abs_out, UART_RX_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// Deliberate mismatch / alternate FL ATDD for UartRx (FR163).
    pub fn verify_uart_rx_handwritten_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in uart_rx_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) = compare_named_ports(sim.ports(), &abs_out, UART_RX_ARCH_PORTS)
            {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
            cycles += 1;
        }
        EquivStatus::Pass { cycles }
    }

    /// UART/SPI/I2C/AXI: generated FL ≡ tick via FormalEquivProduct (rst alphabet).
    pub fn verify_generated_rst_compare(&self, hir: FrozenHir) -> EquivStatus {
        FormalEquivProduct::new(0xC0FFEE, 8)
            .with_boolean_ports(&["rst"])
            .check_random_compare(hir)
    }

    /// Convenience: handwritten equiv path (for fixtures that supply their own FL).
    pub fn verify_handwritten<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
        stimuli: impl IntoIterator<Item = PortValues>,
    ) -> EquivStatus {
        check_functional_equiv(hir, abs, stimuli)
    }
}

fn compare_named_ports(
    left: &PortValues,
    right: &PortValues,
    names: &[&str],
) -> Result<(), Vec<PortMismatch>> {
    let mut l = PortValues::default();
    let mut r = PortValues::default();
    for name in names {
        if let Some(v) = left.get(name) {
            l.set(*name, v);
        }
        if let Some(v) = right.get(name) {
            r.set(*name, v);
        }
    }
    compare_port_values(&l, &r)
}

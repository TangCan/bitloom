//! FR103 — first-class IP dual-model matrix (NFR14 FIFO/UART/SPI/I2C/AXI).
//!
//! Cycle path = FrozenHir [`crate::Sim::tick`]. Functional path =
//! [`crate::GeneratedFunctional`] (generation path) **or** handwritten
//! [`SyncFifoFunctional`] when FR103 nails architectural FL for Mem-based FIFO.
//! **FR126** adds handwritten [`GpioFunctional`]; **FR135** adds handwritten
//! [`UartTxFunctional`]; **FR163** adds handwritten [`UartRxFunctional`] beyond
//! UartTx / Gpio / GeneratedFunctional alone; **FR168** adds handwritten
//! [`SpiMasterFunctional`] / [`I2cMasterFunctional`] / [`Axi4LiteSlaveFunctional`]
//! (all three; ≠ FR163 alone; ≠ GeneratedFunctional alone).
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

/// Architectural ports for SpiMaster handwritten FL (FR168).
const SPI_MASTER_ARCH_PORTS: &[&str] = &[
    "mosi_byte",
    "rx_data",
    "rx_valid",
    "busy",
    "cs_n",
    "sclk",
    "mosi",
];

/// Architectural ports for I2cMaster handwritten FL (FR168).
const I2C_MASTER_ARCH_PORTS: &[&str] = &[
    "tx_byte",
    "busy",
    "scl",
    "sda_out",
    "rx_data",
    "rx_valid",
    "ack_error",
];

/// Architectural ports for Axi4LiteSlave handwritten FL (FR168).
const AXI4_LITE_SLAVE_ARCH_PORTS: &[&str] = &[
    "s_axi_awready",
    "s_axi_wready",
    "s_axi_bresp",
    "s_axi_bvalid",
    "s_axi_arready",
    "s_axi_rdata",
    "s_axi_rresp",
    "s_axi_rvalid",
];

/// Handwritten `SpiMaster` FL (FR168) — Mode-0 single-byte path ≡ tick on
/// [`spi_master_dual_stimulus`]. Not GeneratedFunctional; not FR163 UartRx alone.
#[derive(Debug, Clone, Default)]
pub struct SpiMasterFunctional {
    hold: u64,
    shift: u64,
    rx_shift: u64,
    rx_hold: u64,
    busy_r: u64,
    bit_idx: u64,
    half: u64,
    bytes_left: u64,
    valid_r: u64,
    mosi_r: u64,
}

impl SpiMasterFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for SpiMasterFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let start = inputs.get("start").unwrap_or(0) != 0;
        let tx_data = inputs.get("tx_data").unwrap_or(0) & 0xff;
        let miso = inputs.get("miso").unwrap_or(0) & 1;
        let cpol = inputs.get("cpol").unwrap_or(0) & 1;
        let cpha = inputs.get("cpha").unwrap_or(0) & 1;
        let byte_count = inputs.get("byte_count").unwrap_or(0) & 0x7;

        if rst {
            *self = Self::default();
        } else {
            let busy = self.busy_r != 0;
            let accept = start && !busy;
            let eff_count = if byte_count == 0 { 1 } else { byte_count };
            let cpha0 = cpha == 0;
            let half0 = self.half == 0;
            let is_last_bit = self.bit_idx == 7;
            let is_last_byte = self.bytes_left == 1;
            let tx_msb = u64::from((tx_data & 0x80) != 0);
            let accept_mosi = if cpha0 { tx_msb } else { 0 };
            let shift_shl8 = (self.shift << 1) & 0xff;
            let sh8_msb = u64::from((shift_shl8 & 0x80) != 0);
            let sh_msb = u64::from((self.shift & 0x80) != 0);
            let rx_sampled = ((self.rx_shift << 1) & 0xff) | miso;
            let bit_p1 = (self.bit_idx.wrapping_add(1)) & 0xf;
            let bytes_m1 = (self.bytes_left.wrapping_sub(1)) & 0x7;

            let do_lead = busy && half0;
            let do_trail = busy && !half0;
            let sample_lead = do_lead && cpha0;
            let launch_lead = do_lead && !cpha0;
            let shift_trail = do_trail && cpha0;
            let sample_trail = do_trail && !cpha0;
            let finish_byte = do_trail && is_last_bit;
            let cont_frame = finish_byte && !is_last_byte;
            let end_frame = finish_byte && is_last_byte;

            let lead_shift = if launch_lead { shift_shl8 } else { self.shift };
            let lead_mosi = if launch_lead { sh_msb } else { self.mosi_r };
            let lead_rxsh = if sample_lead {
                rx_sampled
            } else {
                self.rx_shift
            };

            let tr_shift0 = if shift_trail { shift_shl8 } else { self.shift };
            let tr_mosi0 = if shift_trail { sh8_msb } else { self.mosi_r };
            let tr_rxsh0 = if sample_trail {
                rx_sampled
            } else {
                self.rx_shift
            };
            let tr_valid = u64::from(finish_byte);
            let tr_busy = if end_frame { 0 } else { 1 };
            let tr_bit = if finish_byte { 0 } else { bit_p1 };
            let tr_bytes = if cont_frame {
                bytes_m1
            } else {
                self.bytes_left
            };
            let tr_hold = if cont_frame { tx_data } else { self.hold };
            let tr_shift = if cont_frame { tx_data } else { tr_shift0 };
            let tr_mosi = if cont_frame { accept_mosi } else { tr_mosi0 };
            let tr_rxh = if finish_byte { tr_rxsh0 } else { self.rx_hold };
            let tr_rxsh = if finish_byte { 0 } else { tr_rxsh0 };

            let b_half = if do_lead { 1 } else { 0 };
            let b_bit = if do_trail { tr_bit } else { self.bit_idx };
            let b_bytes = if do_trail { tr_bytes } else { self.bytes_left };
            let b_hold = if do_trail { tr_hold } else { self.hold };
            let b_shift = if do_trail { tr_shift } else { lead_shift };
            let b_rxsh = if do_trail { tr_rxsh } else { lead_rxsh };
            let b_rxh = if do_trail { tr_rxh } else { self.rx_hold };
            let b_valid = if do_trail { tr_valid } else { 0 };
            let b_mosi = if do_trail { tr_mosi } else { lead_mosi };
            let b_busy = if do_trail { tr_busy } else { 1 };

            let nb_busy = if busy { b_busy } else { 0 };
            let nb_half = if busy { b_half } else { 0 };
            let nb_bit = if busy { b_bit } else { 0 };
            let nb_bytes = if busy { b_bytes } else { 0 };
            let nb_shift = if busy { b_shift } else { self.shift };
            let nb_rxsh = if busy { b_rxsh } else { self.rx_shift };
            let nb_hold = if busy { b_hold } else { self.hold };
            let nb_rxh = if busy { b_rxh } else { self.rx_hold };
            let nb_valid = if busy { b_valid } else { 0 };
            let nb_mosi = if busy { b_mosi } else { 0 };

            self.busy_r = if accept { 1 } else { nb_busy };
            self.half = if accept { 0 } else { nb_half };
            self.bit_idx = if accept { 0 } else { nb_bit };
            self.bytes_left = if accept { eff_count } else { nb_bytes };
            self.shift = if accept { tx_data } else { nb_shift };
            self.rx_shift = if accept { 0 } else { nb_rxsh };
            self.hold = if accept { tx_data } else { nb_hold };
            self.rx_hold = if accept { self.rx_hold } else { nb_rxh };
            self.valid_r = if accept { 0 } else { nb_valid };
            self.mosi_r = if accept { accept_mosi } else { nb_mosi };
        }

        let busy = self.busy_r != 0;
        let sclk = if busy { cpol ^ (self.half & 1) } else { cpol };
        let mut out = inputs.clone();
        out.set("mosi_byte", self.hold & 0xff);
        out.set("rx_data", self.rx_hold & 0xff);
        out.set("rx_valid", self.valid_r & 1);
        out.set("busy", self.busy_r & 1);
        out.set("cs_n", u64::from(!busy));
        out.set("sclk", sclk & 1);
        out.set("mosi", if busy { self.mosi_r & 1 } else { 0 });
        out
    }
}

/// Documented SpiMaster dual-model stimulus (reset + Mode-0 single-byte RX) — FR168.
pub fn spi_master_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame =
        |rst: u64, start: u64, tx_data: u64, miso: u64, cpol: u64, cpha: u64, bc: u64| {
            let mut pv = PortValues::default();
            pv.set("rst", rst);
            pv.set("start", start);
            pv.set("tx_data", tx_data);
            pv.set("miso", miso);
            pv.set("cpol", cpol);
            pv.set("cpha", cpha);
            pv.set("byte_count", bc);
            out.push(pv);
        };
    let tx = 0xa5u64;
    // MISO MSB-first → 0x3C (matches fr98_spi_mode0_byte_transfer_rx)
    let rx_bits = [0u64, 0, 1, 1, 1, 1, 0, 0];
    frame(1, 0, 0, 0, 0, 0, 1);
    frame(0, 0, 0, 0, 0, 0, 1);
    frame(0, 1, tx, rx_bits[0], 0, 0, 1);
    for &bit in &rx_bits {
        frame(0, 0, 0, bit, 0, 0, 1); // lead sample
        frame(0, 0, 0, bit, 0, 0, 1); // trail shift / finish
    }
    frame(0, 0, 0, 0, 0, 0, 1); // clear rx_valid
    out
}

/// Handwritten `I2cMaster` FL (FR168) — write+ACK path ≡ tick on
/// [`i2c_master_dual_stimulus`]. Not GeneratedFunctional; not FR163 alone.
#[derive(Debug, Clone, Default)]
pub struct I2cMasterFunctional {
    hold: u64,
    shift: u64,
    rx_shift: u64,
    rx_hold: u64,
    busy_r: u64,
    half: u64,
    bit_idx: u64,
    stage: u64,
    rw_r: u64,
    valid_r: u64,
    ack_err_r: u64,
}

impl I2cMasterFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for I2cMasterFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let start = inputs.get("start").unwrap_or(0) != 0;
        let addr = inputs.get("addr").unwrap_or(0) & 0xff;
        let rw = inputs.get("rw").unwrap_or(0) & 1;
        let tx_data = inputs.get("tx_data").unwrap_or(0) & 0xff;
        let sda_in = inputs.get("sda_in").unwrap_or(1) & 1;

        if rst {
            *self = Self::default();
        } else {
            let busy = self.busy_r != 0;
            let accept = start && !busy;
            let addr_byte = ((addr << 1) & 0xfe) | rw;
            let nack = sda_in != 0;
            let rw_read = self.rw_r != 0;
            let half0 = self.half == 0;
            let is_last_bit = self.bit_idx == 7;
            let shift_shl8 = (self.shift << 1) & 0xff;
            let rx_sampled = ((self.rx_shift << 1) & 0xff) | sda_in;
            let bit_p1 = (self.bit_idx.wrapping_add(1)) & 0xf;

            let mut n_busy = 0u64;
            let mut n_half = 0u64;
            let mut n_bit = 0u64;
            let mut n_stage = 0u64;
            let mut n_shift = self.shift;
            let mut n_hold = self.hold;
            let mut n_rxsh = self.rx_shift;
            let mut n_rxh = self.rx_hold;
            let mut n_valid = 0u64;
            let mut n_ackerr = self.ack_err_r;
            let mut n_rw = self.rw_r;

            if accept {
                n_busy = 1;
                n_half = 0;
                n_bit = 0;
                n_stage = 0;
                n_shift = addr_byte;
                n_hold = addr_byte;
                n_rxsh = 0;
                n_valid = 0;
                n_ackerr = 0;
                n_rw = rw;
            } else if busy {
                match self.stage {
                    0 => {
                        // START → ADDR
                        n_busy = 1;
                        n_half = 0;
                        n_bit = 0;
                        n_stage = 1;
                    }
                    1 => {
                        // ADDR
                        n_busy = 1;
                        if half0 {
                            n_half = 1;
                            n_bit = self.bit_idx;
                            n_stage = 1;
                        } else {
                            n_half = 0;
                            n_shift = shift_shl8;
                            if is_last_bit {
                                n_bit = 0;
                                n_stage = 2;
                            } else {
                                n_bit = bit_p1;
                                n_stage = 1;
                            }
                        }
                    }
                    2 => {
                        // AACK
                        n_busy = 1;
                        if half0 {
                            n_half = 1;
                            n_bit = self.bit_idx;
                            n_stage = 2;
                        } else {
                            n_half = 0;
                            n_bit = 0;
                            if nack {
                                n_stage = 5;
                                n_ackerr = 1;
                            } else {
                                n_stage = 3;
                                if rw_read {
                                    n_rxsh = 0;
                                } else {
                                    n_shift = tx_data;
                                    n_hold = tx_data;
                                }
                            }
                        }
                    }
                    3 => {
                        // DATA
                        n_busy = 1;
                        if half0 {
                            n_half = 1;
                            n_bit = self.bit_idx;
                            n_stage = 3;
                        } else {
                            n_half = 0;
                            n_shift = shift_shl8;
                            if rw_read {
                                n_rxsh = rx_sampled;
                            }
                            if is_last_bit {
                                n_bit = 0;
                                n_stage = 4;
                            } else {
                                n_bit = bit_p1;
                                n_stage = 3;
                            }
                        }
                    }
                    4 => {
                        // DACK
                        n_busy = 1;
                        if half0 {
                            n_half = 1;
                            n_bit = self.bit_idx;
                            n_stage = 4;
                        } else {
                            n_half = 0;
                            n_bit = self.bit_idx;
                            n_stage = 5;
                            if rw_read {
                                n_valid = 1;
                                n_rxh = self.rx_shift;
                            } else if nack {
                                n_ackerr = 1;
                            }
                        }
                    }
                    _ => {
                        // STOP (5+)
                        if half0 {
                            n_busy = 1;
                            n_half = 1;
                            n_bit = self.bit_idx;
                            n_stage = 5;
                        } else {
                            n_busy = 0;
                            n_half = 0;
                            n_bit = 0;
                            n_stage = 0;
                        }
                    }
                }
            }

            self.busy_r = n_busy;
            self.half = n_half;
            self.bit_idx = n_bit;
            self.stage = n_stage;
            self.shift = n_shift;
            self.hold = n_hold;
            self.rx_shift = n_rxsh;
            self.rx_hold = n_rxh;
            self.valid_r = n_valid;
            self.ack_err_r = n_ackerr;
            self.rw_r = n_rw;
        }

        let busy = self.busy_r != 0;
        let msb = u64::from((self.shift & 0x80) != 0);
        let sda_data = if self.rw_r != 0 { 1 } else { msb };
        let sda_mid = if self.stage == 1 { msb } else { sda_data };
        let is_ackph = self.stage == 2 || self.stage == 4;
        let sda_or_ack = if is_ackph { 1 } else { sda_mid };
        let is_lowdrv = self.stage == 0 || self.stage == 5;
        let sda_active = if is_lowdrv { 0 } else { sda_or_ack };
        let sda_out = if busy { sda_active } else { 1 };
        let scl_busy = if self.stage == 0 { 1 } else { self.half & 1 };
        let scl = if busy { scl_busy } else { 1 };

        let mut out = inputs.clone();
        out.set("tx_byte", self.hold & 0xff);
        out.set("busy", self.busy_r & 1);
        out.set("scl", scl & 1);
        out.set("sda_out", sda_out & 1);
        out.set("rx_data", self.rx_hold & 0xff);
        out.set("rx_valid", self.valid_r & 1);
        out.set("ack_error", self.ack_err_r & 1);
        out
    }
}

/// Documented I2cMaster dual-model stimulus (reset + write with ACK) — FR168.
pub fn i2c_master_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64, start: u64, addr: u64, rw: u64, tx_data: u64, sda_in: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("start", start);
        pv.set("addr", addr);
        pv.set("rw", rw);
        pv.set("tx_data", tx_data);
        pv.set("sda_in", sda_in);
        out.push(pv);
    };
    let addr = 0x50u64;
    let data = 0xa5u64;
    frame(1, 0, 0, 0, 0, 1);
    frame(0, 0, 0, 0, 0, 1);
    frame(0, 1, addr, 0, data, 0);
    // START→ADDR×16→AACK×2→DATA×16→DACK×2→STOP×2 = 39 post-start halves
    for _ in 0..39 {
        frame(0, 0, addr, 0, data, 0);
    }
    out
}

/// Handwritten `Axi4LiteSlave` FL (FR168) — write+read handshake ≡ tick on
/// [`axi4_lite_slave_dual_stimulus`]. Not GeneratedFunctional; not FR163 alone.
#[derive(Debug, Clone, Default)]
pub struct Axi4LiteSlaveFunctional {
    data0_r: u64,
    data1_r: u64,
    data2_r: u64,
    data3_r: u64,
    rdata_r: u64,
    bvalid_r: u64,
    rvalid_r: u64,
}

impl Axi4LiteSlaveFunctional {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AbstractionView for Axi4LiteSlaveFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let rst = inputs.get("rst").unwrap_or(0) != 0;
        let awaddr = inputs.get("s_axi_awaddr").unwrap_or(0) & 0xff;
        let awvalid = inputs.get("s_axi_awvalid").unwrap_or(0) != 0;
        let wdata = inputs.get("s_axi_wdata").unwrap_or(0) & 0xffff_ffff;
        let wstrb = inputs.get("s_axi_wstrb").unwrap_or(0) & 0xf;
        let wvalid = inputs.get("s_axi_wvalid").unwrap_or(0) != 0;
        let bready = inputs.get("s_axi_bready").unwrap_or(0) != 0;
        let araddr = inputs.get("s_axi_araddr").unwrap_or(0) & 0xff;
        let arvalid = inputs.get("s_axi_arvalid").unwrap_or(0) != 0;
        let rready = inputs.get("s_axi_rready").unwrap_or(0) != 0;

        if rst {
            *self = Self::default();
        } else {
            let aw_ready = self.bvalid_r == 0;
            let ar_ready = self.rvalid_r == 0 && self.bvalid_r == 0;
            let do_write = awvalid && aw_ready && wvalid;
            let do_read = !do_write && arvalid && ar_ready;
            let b_fire = self.bvalid_r != 0 && bready;
            let r_fire = self.rvalid_r != 0 && rready;

            let mut byte_mask = 0u64;
            if wstrb & 1 != 0 {
                byte_mask |= 0x0000_00ff;
            }
            if wstrb & 2 != 0 {
                byte_mask |= 0x0000_ff00;
            }
            if wstrb & 4 != 0 {
                byte_mask |= 0x00ff_0000;
            }
            if wstrb & 8 != 0 {
                byte_mask |= 0xff00_0000;
            }
            let byte_mask_n = (!byte_mask) & 0xffff_ffff;
            let wdata_m = wdata & byte_mask;

            let merge = |old: u64| (wdata_m) | (old & byte_mask_n);
            if do_write {
                match awaddr {
                    0x00 => self.data0_r = merge(self.data0_r),
                    0x04 => self.data1_r = merge(self.data1_r),
                    0x08 => self.data2_r = merge(self.data2_r),
                    0x0c => self.data3_r = merge(self.data3_r),
                    _ => {}
                }
            }

            let rdata_mux = match araddr {
                0x00 => self.data0_r,
                0x04 => self.data1_r,
                0x08 => self.data2_r,
                0x0c => self.data3_r,
                _ => 0,
            };
            if do_read {
                self.rdata_r = rdata_mux;
            }

            self.bvalid_r = if do_write {
                1
            } else if b_fire {
                0
            } else {
                self.bvalid_r
            };
            self.rvalid_r = if do_read {
                1
            } else if r_fire {
                0
            } else {
                self.rvalid_r
            };
        }

        let aw_ready = self.bvalid_r == 0;
        let ar_ready = self.rvalid_r == 0 && self.bvalid_r == 0;
        let mut out = inputs.clone();
        out.set("s_axi_awready", u64::from(aw_ready));
        out.set("s_axi_wready", u64::from(aw_ready));
        out.set("s_axi_bresp", 0);
        out.set("s_axi_bvalid", self.bvalid_r & 1);
        out.set("s_axi_arready", u64::from(ar_ready));
        out.set("s_axi_rdata", self.rdata_r & 0xffff_ffff);
        out.set("s_axi_rresp", 0);
        out.set("s_axi_rvalid", self.rvalid_r & 1);
        out
    }
}

/// Documented Axi4LiteSlave dual-model stimulus (reset + write + read) — FR168.
pub fn axi4_lite_slave_dual_stimulus() -> Vec<PortValues> {
    let mut out = Vec::new();
    let mut frame = |rst: u64,
                     awaddr: u64,
                     awvalid: u64,
                     wdata: u64,
                     wstrb: u64,
                     wvalid: u64,
                     bready: u64,
                     araddr: u64,
                     arvalid: u64,
                     rready: u64| {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("s_axi_awaddr", awaddr);
        pv.set("s_axi_awvalid", awvalid);
        pv.set("s_axi_wdata", wdata);
        pv.set("s_axi_wstrb", wstrb);
        pv.set("s_axi_wvalid", wvalid);
        pv.set("s_axi_bready", bready);
        pv.set("s_axi_araddr", araddr);
        pv.set("s_axi_arvalid", arvalid);
        pv.set("s_axi_rready", rready);
        out.push(pv);
    };
    frame(1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    frame(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    frame(0, 0x00, 1, 0xdead_beef, 0xf, 1, 0, 0, 0, 0);
    frame(0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
    frame(0, 0, 0, 0, 0, 0, 0, 0x00, 1, 0);
    frame(0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
    out
}

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

    /// FR168: handwritten `SpiMaster` FL ≡ tick on [`spi_master_dual_stimulus`].
    pub fn verify_spi_master_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = SpiMasterFunctional::new();
        let mut cycles = 0usize;
        for inputs in spi_master_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, SPI_MASTER_ARCH_PORTS)
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

    /// Deliberate mismatch / alternate FL ATDD for SpiMaster (FR168).
    pub fn verify_spi_master_handwritten_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in spi_master_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, SPI_MASTER_ARCH_PORTS)
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

    /// FR168: handwritten `I2cMaster` FL ≡ tick on [`i2c_master_dual_stimulus`].
    pub fn verify_i2c_master_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = I2cMasterFunctional::new();
        let mut cycles = 0usize;
        for inputs in i2c_master_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, I2C_MASTER_ARCH_PORTS)
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

    /// Deliberate mismatch / alternate FL ATDD for I2cMaster (FR168).
    pub fn verify_i2c_master_handwritten_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in i2c_master_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, I2C_MASTER_ARCH_PORTS)
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

    /// FR168: handwritten `Axi4LiteSlave` FL ≡ tick on [`axi4_lite_slave_dual_stimulus`].
    pub fn verify_axi4_lite_slave_handwritten(&self, hir: FrozenHir) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut fl = Axi4LiteSlaveFunctional::new();
        let mut cycles = 0usize;
        for inputs in axi4_lite_slave_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = fl.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, AXI4_LITE_SLAVE_ARCH_PORTS)
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

    /// Deliberate mismatch / alternate FL ATDD for Axi4LiteSlave (FR168).
    pub fn verify_axi4_lite_slave_handwritten_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let mut sim = Sim::new(hir);
        let mut cycles = 0usize;
        for inputs in axi4_lite_slave_dual_stimulus() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            sim.tick();
            let abs_out = abs.cycle(&inputs);
            if let Err(mismatches) =
                compare_named_ports(sim.ports(), &abs_out, AXI4_LITE_SLAVE_ARCH_PORTS)
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

//! FR103 — first-class IP dual-model matrix (NFR14 FIFO/UART/SPI/I2C/AXI).
//!
//! Cycle path = FrozenHir [`crate::Sim::tick`]. Functional path =
//! [`crate::GeneratedFunctional`] (generation path) **or** handwritten
//! [`SyncFifoFunctional`] when FR103 nails architectural FL for Mem-based FIFO.
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

/// Architectural ports compared for SyncFifo dual-model (avoid internal wires).
const SYNC_FIFO_ARCH_PORTS: &[&str] = &["full", "empty", "data_out"];

/// Architectural ports for Gpio handwritten FL (FR126).
const GPIO_ARCH_PORTS: &[&str] = &["pad_out", "rd_data"];

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

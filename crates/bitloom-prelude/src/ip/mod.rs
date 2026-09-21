//! First-class IP (FR37 / FR48 / FR82 / FR89 / FR98 / FR108 / FR120): SyncFifo,
//! UartTx, UartRx, SpiMaster, I2cMaster, Axi4LiteSlave, Gpio, GpioVip, black-box;
//! plus FR77 overlay [`Crc8Lut`] (Epic 29.3).
//!
//! Epic 34 / FR82 deepens five classes to **non-stub** synthesizable baselines.
//! Epic 38 / FR89 deepens UartTx programmable baud. Epic 43 / FR98 Stories
//! 43.2–43.5 add UART/SPI/I2C/AXI near-VIP. Epic 50 / FR108 delivers GPIO near-VIP
//! (Phase 12 optional G0 elevated). Epic 61 / FR120 delivers commercial VIP GPIO
//! ([`GpioVip`] beyond P1–P4). Those IP APIs take **no** generator closures.
//! Design crates reach IP via `bitloom_prelude::ip` only.
//!
//! **FR77 / Cap-R-63:** [`Crc8Lut`] accepts elaborate-time table closures
//! (`elaborate_with_table_fn`) on top of the Epic 27 Mem-init path; default poly
//! needs no closure. Closures dissolve before freeze (NFR36); synthesizable leg
//! uses [`SynthesizableClosure`] checks (D1).
//!
//! **FR131:** split by protocol; re-exported as `bitloom_prelude::ip::*`.
//! **FR139:** GPIO VIP/SocPad further split under `ip/gpio/{base,vip,socpad}`;
//! public paths unchanged (no cross-crate; AD-6 / C2 not selected).
//! **FR136:** [`ChipPadRing`] multi-peripheral / full-chip pad ring under
//! `ip/gpio/chip_ring` (beyond [`GpioSocPad`] alone).

mod axi;
mod axi_lite_csr;
mod blackbox;
mod crc;
mod csr;
mod gpio;
mod i2c;
mod param_sync_fifo;
mod rv_reg_slice;
mod spi;
mod sync_fifo;
mod uart;

pub use axi::*;
pub use axi_lite_csr::*;
pub use blackbox::*;
pub use crc::*;
pub use csr::*;
pub use gpio::*;
pub use i2c::*;
pub use param_sync_fifo::*;
pub use rv_reg_slice::*;
pub use spi::*;
pub use sync_fifo::*;
pub use uart::*;

#[cfg(test)]
mod tests;

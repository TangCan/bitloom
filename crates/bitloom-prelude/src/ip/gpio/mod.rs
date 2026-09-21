//! GPIO family submodule (FR139 / Epic 78): further split beyond FR131 protocol `ip/gpio`.
//!
//! - [`base`] — [`Gpio`] (FR108)
//! - [`vip`] — [`GpioVip`] (FR120)
//! - [`socpad`] — [`GpioSocPad`] (FR128)
//! - [`chip_ring`] — [`ChipPadRing`] (FR136 multi-peripheral / full-chip pad ring)
//!
//! Public paths remain `bitloom_prelude::ip::{Gpio,GpioVip,GpioSocPad,ChipPadRing}` via
//! re-export (C3 stable; no cross-crate / C2 not selected). Brand: Bitloom.

mod base;
mod chip_ring;
mod csr;
mod socpad;
mod vip;

pub use base::*;
pub use chip_ring::*;
pub use csr::*;
pub use socpad::*;
pub use vip::*;

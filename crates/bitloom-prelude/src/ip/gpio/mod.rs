//! GPIO family submodule (FR139 / Epic 78): further split beyond FR131 protocol `ip/gpio`.
//!
//! - [`base`] — [`Gpio`] (FR108)
//! - [`vip`] — [`GpioVip`] (FR120)
//! - [`socpad`] — [`GpioSocPad`] (FR128)
//!
//! Public paths remain `bitloom_prelude::ip::{Gpio,GpioVip,GpioSocPad}` via re-export
//! (C3 stable; no cross-crate / C2 not selected). Brand: Bitloom.

mod base;
mod socpad;
mod vip;

pub use base::*;
pub use socpad::*;
pub use vip::*;

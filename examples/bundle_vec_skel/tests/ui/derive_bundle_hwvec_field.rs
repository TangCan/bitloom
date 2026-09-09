//! Unsupported: HwVec field in derive — stable rhdl::E0180.
use bitloom_prelude::{Bundle, HwVec, UInt};

#[derive(Bundle)]
struct WithHwVec {
    lanes: HwVec<UInt<8>, 4>,
}

fn main() {}

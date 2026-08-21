//! OUT OF SCOPE: `HwVec<Bundle, _>` — element type must be ground.
use bitloom_prelude::rhdl::module;
use bitloom_prelude::{Bundle, Clock, GroundType, HwVec, Input, Reset};

struct Nested;

impl Bundle for Nested {
    fn leaves() -> &'static [(&'static str, GroundType)] {
        &[("x", GroundType::UInt { width: 8 })]
    }
}

#[module]
struct Bad {
    pub clk: Input<Clock>,
    pub rst: Input<Reset>,
    pub lanes: Input<HwVec<Nested, 2>>,
}

fn main() {}

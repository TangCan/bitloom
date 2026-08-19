use rhdl_prelude::rhdl::module;
use rhdl_prelude::UInt;

#[module]
struct Bad {
    x: UInt<8>,
}

fn main() {}

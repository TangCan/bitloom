//! Example design crate: depends only on `rhdl-prelude`.

use bitloom_prelude::rhdl::module;
use bitloom_prelude::{Clock, Elaboratable, Input, Output, Reset, UInt};

#[module]
pub struct CounterPorts {
    pub clk: Input<Clock>,
    pub rst: Input<Reset>,
    pub data_in: Input<UInt<8>>,
    pub data_out: Output<UInt<8>>,
}

/// Host entry for `cargo rhdl build --package counter_ports`.
pub fn rhdl_elaborate() -> Result<bitloom_prelude::FrozenHir, bitloom_prelude::Diagnostics> {
    CounterPorts::elaborate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_prelude::{Elaboratable, GroundType, PortDirection};

    #[test]
    fn elaborate_yields_frozen_hir_with_directed_ports() {
        let frozen = CounterPorts::elaborate().expect("elaborate");
        assert_eq!(frozen.abi_name, "CounterPorts");
        let m = &frozen.circuit().modules[0];
        assert_eq!(m.ports.len(), 4);
        assert_eq!(m.ports[0].name, "clk");
        assert_eq!(m.ports[0].direction, PortDirection::Input);
        assert_eq!(m.ports[0].ty, GroundType::Clock);
        assert_eq!(m.ports[1].ty, GroundType::Reset);
        assert_eq!(m.ports[2].ty, GroundType::UInt { width: 8 });
        assert_eq!(m.ports[3].direction, PortDirection::Output);
        assert_eq!(m.ports[3].ty, GroundType::UInt { width: 8 });
    }

    #[test]
    fn bare_uint_port_rejected_at_compile_time() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/bare_uint_port.rs");
    }

    #[test]
    fn unmarked_process_rejected_at_compile_time() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/unmarked_process.rs");
    }
}

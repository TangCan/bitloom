//! Tree IP (fifo_skel) + external black-box wrapper stub (FR37).

use rhdl_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// In-tree IP re-export.
pub use fifo_skel::SkidFifo;

/// Opaque vendor IP: emit-only black box (no FrozenHir body for the child).
pub struct ExtUartBlackBox;

impl Elaboratable for ExtUartBlackBox {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("ExtUartWrap");
        s.begin_module("ExtUartWrap", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("tx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx", GroundType::UInt { width: 1 }, Span::default());
        // Black-box: declare ports only; vendor netlist supplied separately.
        s.end_module();
        s.finish()
    }
}

pub fn vendor_blackbox_v() -> &'static str {
    "module vendor_uart(input clk, input rst, input [7:0] tx_data, output tx);\nendmodule\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhdl_prelude::Elaboratable;

    #[test]
    fn tree_ip_and_blackbox() {
        assert!(SkidFifo::elaborate().is_ok());
        let bb = ExtUartBlackBox::elaborate().unwrap();
        assert_eq!(bb.abi_name, "ExtUartWrap");
        assert!(vendor_blackbox_v().contains("vendor_uart"));
    }
}

//! Tree IP + black-box via `bitloom_prelude::ip` (FR37).

pub use bitloom_prelude::ip::{ExtBlackBox, SyncFifo, UartTx, vendor_blackbox_v};

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    #[test]
    fn fifo_uart_blackbox_elaborate_emit_tick() {
        for (hir, name) in [
            (SyncFifo::elaborate().unwrap(), "SyncFifo"),
            (UartTx::elaborate().unwrap(), "UartTx"),
            (ExtBlackBox::elaborate().unwrap(), "ExtBlackBox"),
        ] {
            assert_eq!(hir.abi_name, name);
            let art = emit(&hir);
            assert!(art.files.iter().any(|f| f.contents.contains(name)));
            let mut sim = Sim::new(hir);
            let mut pv = PortValues::default();
            pv.set("rst", 1);
            sim.set_inputs(pv);
            sim.tick();
        }
        assert!(vendor_blackbox_v().contains("vendor_ext_ip"));
        assert!(
            ExtBlackBox::elaborate().unwrap().circuit().modules[0]
                .body
                .is_empty()
        );
    }
}

//! Tree IP + black-box via `bitloom_prelude::ip` (FR37 / FR82).

pub use bitloom_prelude::ip::{ExtBlackBox, SyncFifo, UartRx, UartTx, vendor_blackbox_v};

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    #[test]
    fn fifo_uart_blackbox_elaborate_emit_tick() {
        // SyncFifo — depth-4 non-stub (FR82)
        {
            let hir = SyncFifo::elaborate().unwrap();
            assert_eq!(hir.abi_name, "SyncFifo");
            let art = emit(&hir);
            assert!(art.files.iter().any(|f| f.contents.contains("SyncFifo")));
            let mut sim = Sim::new(hir);
            let mut pv = PortValues::default();
            pv.set("rst", 1);
            pv.set("wr_en", 0);
            pv.set("rd_en", 0);
            pv.set("data_in", 0);
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
            pv.set("rst", 0);
            pv.set("wr_en", 1);
            pv.set("data_in", 0x5A);
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
            pv.set("wr_en", 0);
            pv.set("rd_en", 1);
            sim.set_inputs(pv);
            sim.settle();
            sim.tick();
            assert_eq!(sim.ports().get("data_out"), Some(0x5A));
        }

        // UartTx — FR82 8N1 + FR89 programmable baud_div demo (Epic 38)
        {
            let hir = UartTx::elaborate().unwrap();
            assert_eq!(hir.abi_name, "UartTx");
            let art = emit(&hir);
            let v = &art.files[0].contents;
            assert!(v.contains("UartTx") && v.contains("baud_div"));
            let baud_div = 1u64; // 2 clocks per bit
            let mut sim = Sim::new(hir);
            let mut pv = PortValues::default();
            pv.set("rst", 1);
            pv.set("wr_en", 0);
            pv.set("wr_data", 0);
            pv.set("baud_div", baud_div);
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
            pv.set("rst", 0);
            pv.set("wr_en", 1);
            pv.set("wr_data", 0x01);
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
            assert_eq!(sim.ports().get("tx_busy"), Some(1));
            assert_eq!(sim.ports().get("tx"), Some(0)); // start bit
            // Hold start for second clock of baud period
            pv.set("wr_en", 0);
            sim.set_inputs(pv);
            sim.settle();
            sim.tick();
            assert_eq!(
                sim.ports().get("tx"),
                Some(0),
                "ip_box: start bit holds for full baud_div period"
            );
        }

        // UartRx — FR98 near-VIP 8N1 (Epic 43.2)
        {
            let hir = UartRx::elaborate().unwrap();
            assert_eq!(hir.abi_name, "UartRx");
            let art = emit(&hir);
            let v = &art.files[0].contents;
            assert!(v.contains("UartRx") && v.contains("rd_valid") && v.contains("baud_div"));
            let mut sim = Sim::new(hir);
            let mut pv = PortValues::default();
            pv.set("rst", 1);
            pv.set("rx", 1);
            pv.set("baud_div", 0);
            sim.set_inputs(pv.clone());
            sim.settle();
            sim.tick();
            pv.set("rst", 0);
            sim.set_inputs(pv);
            sim.settle();
            sim.tick();
            assert_eq!(sim.ports().get("rx_busy"), Some(0));
        }

        // ExtBlackBox — opaque ports-only wrapper (retained)
        {
            let hir = ExtBlackBox::elaborate().unwrap();
            assert_eq!(hir.abi_name, "ExtBlackBox");
            let art = emit(&hir);
            assert!(art.files.iter().any(|f| f.contents.contains("ExtBlackBox")));
            let mut sim = Sim::new(hir);
            let mut pv = PortValues::default();
            pv.set("rst", 1);
            sim.set_inputs(pv);
            sim.tick();
            assert!(
                ExtBlackBox::elaborate().unwrap().circuit().modules[0]
                    .body
                    .is_empty()
            );
        }

        assert!(vendor_blackbox_v().contains("vendor_ext_ip"));
    }
}

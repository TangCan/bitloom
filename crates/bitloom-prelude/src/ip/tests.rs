use super::*;

use crate::{Elaboratable, SynthesizableClosureViolation};

use bitloom_hir::PortValues;

use bitloom_sim::Sim;

use bitloom_vlog::emit;

fn smoke_elaborate_emit_tick<T: Elaboratable>(abi: &str) {
    let hir = T::elaborate().expect("elaborate");

    assert_eq!(hir.abi_name, abi);

    let art = emit(&hir);

    assert!(
        art.files
            .iter()
            .any(|f| f.contents.contains(&format!("module {abi}"))),
        "emit must contain module {abi}"
    );

    let mut sim = Sim::new(hir);

    let mut pv = PortValues::default();

    pv.set("rst", 1);

    sim.set_inputs(pv);

    sim.tick();
}

fn fifo_drive(sim: &mut Sim, rst: u64, wr_en: u64, rd_en: u64, data_in: u64) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("wr_en", wr_en);

    pv.set("rd_en", rd_en);

    pv.set("data_in", data_in);

    sim.set_inputs(pv);

    // Same-cycle enables (`wr_en && !full`) need comb settle before the edge.

    sim.settle();

    sim.tick();
}

#[test]

fn sync_fifo_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<SyncFifo>("SyncFifo");

    let hir = SyncFifo::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module SyncFifo"), "emit .v");

    assert!(
        v.contains("full") && v.contains("empty") && v.contains("wr_en"),
        "non-stub ports in emit"
    );

    let mut sim = Sim::new(SyncFifo::elaborate().unwrap());

    // reset

    fifo_drive(&mut sim, 1, 0, 0, 0);

    assert_eq!(sim.ports().get("empty"), Some(1));

    assert_eq!(sim.ports().get("full"), Some(0));

    // push 0x11, 0x22

    fifo_drive(&mut sim, 0, 1, 0, 0x11);

    assert_eq!(sim.ports().get("empty"), Some(0));

    fifo_drive(&mut sim, 0, 1, 0, 0x22);

    // pop — dout tracks head via async mem read of rd_ptr

    fifo_drive(&mut sim, 0, 0, 1, 0);

    assert_eq!(sim.ports().get("data_out"), Some(0x11));

    fifo_drive(&mut sim, 0, 0, 1, 0);

    assert_eq!(sim.ports().get("data_out"), Some(0x22));

    assert_eq!(sim.ports().get("empty"), Some(1));
}

#[test]

fn sync_fifo_fills_then_blocks_write() {
    let mut sim = Sim::new(SyncFifo::elaborate().unwrap());

    fifo_drive(&mut sim, 1, 0, 0, 0);

    for b in [0xA0u64, 0xA1, 0xA2, 0xA3] {
        fifo_drive(&mut sim, 0, 1, 0, b);
    }

    assert_eq!(sim.ports().get("full"), Some(1));

    // write while full must not change occupancy / lose head

    fifo_drive(&mut sim, 0, 1, 0, 0xFF);

    assert_eq!(sim.ports().get("full"), Some(1));

    fifo_drive(&mut sim, 0, 0, 1, 0);

    assert_eq!(sim.ports().get("data_out"), Some(0xA0));
}

fn uart_drive(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64) {
    uart_drive_baud(sim, rst, wr_en, wr_data, 0);
}

fn uart_drive_baud(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64, baud_div: u64) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("wr_en", wr_en);

    pv.set("wr_data", wr_data);

    pv.set("baud_div", baud_div);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();
}

#[test]

fn uart_tx_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<UartTx>("UartTx");

    let hir = UartTx::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module UartTx"));

    assert!(
        v.contains("tx_busy") && v.contains("tx"),
        "serial tx in emit"
    );

    let mut sim = Sim::new(UartTx::elaborate().unwrap());

    uart_drive(&mut sim, 1, 0, 0);

    assert_eq!(sim.ports().get("tx"), Some(1)); // idle high

    assert_eq!(sim.ports().get("tx_busy"), Some(0));

    // accept byte 0xA5 = 0b1010_0101 → LSB first: 1,0,1,0,0,1,0,1

    uart_drive(&mut sim, 0, 1, 0xA5);

    assert_eq!(sim.ports().get("tx_busy"), Some(1));

    assert_eq!(sim.ports().get("tx_byte"), Some(0xA5));

    assert_eq!(sim.ports().get("tx"), Some(0)); // start bit

    let expected = [0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1]; // start + 8 data + stop

    // Current cycle already showed start (bit_idx=0). Next 9 cycles: data…stop.

    for (i, &bit) in expected.iter().enumerate().skip(1) {
        uart_drive(&mut sim, 0, 0, 0);

        assert_eq!(sim.ports().get("tx"), Some(bit), "frame bit {i} mismatch");
    }

    // Stop bit cycle leaves bit_idx==9 / busy==1; one idle tick clears busy.

    assert_eq!(sim.ports().get("tx_busy"), Some(1));

    uart_drive(&mut sim, 0, 0, 0);

    assert_eq!(sim.ports().get("tx_busy"), Some(0));

    assert_eq!(sim.ports().get("tx"), Some(1)); // idle

    // ignore wr_en while busy: start new frame, then wr_en with other data must not replace

    uart_drive(&mut sim, 0, 1, 0x3C);

    assert_eq!(sim.ports().get("tx_busy"), Some(1));

    uart_drive(&mut sim, 0, 1, 0xFF);

    assert_eq!(sim.ports().get("tx_byte"), Some(0x3C));
}

#[test]

fn uart_tx_programmable_baud_holds_bits() {
    let mut sim = Sim::new(UartTx::elaborate().unwrap());

    let baud_div = 1u64; // 2 clk/bit

    uart_drive_baud(&mut sim, 1, 0, 0, baud_div);

    uart_drive_baud(&mut sim, 0, 1, 0x01, baud_div);

    assert_eq!(sim.ports().get("tx"), Some(0)); // start

    uart_drive_baud(&mut sim, 0, 0, 0, baud_div);

    assert_eq!(sim.ports().get("tx"), Some(0), "start held");

    uart_drive_baud(&mut sim, 0, 0, 0, baud_div);

    assert_eq!(sim.ports().get("tx"), Some(1), "LSB after baud period");
}

fn uart_rx_drive(sim: &mut Sim, rst: u64, rx: u64, baud_div: u64) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("rx", rx);

    pv.set("baud_div", baud_div);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();
}

#[test]

fn uart_rx_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<UartRx>("UartRx");

    let hir = UartRx::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module UartRx"));

    assert!(
        v.contains("rd_data") && v.contains("rd_valid") && v.contains("baud_div"),
        "RX ports in emit"
    );

    // Idle high then falling edge + 8N1 for 0xA5 (LSB first)

    let mut sim = Sim::new(UartRx::elaborate().unwrap());

    uart_rx_drive(&mut sim, 1, 1, 0);

    uart_rx_drive(&mut sim, 0, 1, 0);

    // start + data bits of 0xA5 = 0b1010_0101 → LSB first: 1,0,1,0,0,1,0,1 + stop

    let bits = [0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1];

    uart_rx_drive(&mut sim, 0, bits[0], 0); // start edge

    for &b in &bits[1..] {
        uart_rx_drive(&mut sim, 0, b, 0);
    }

    assert_eq!(sim.ports().get("rd_valid"), Some(1));

    assert_eq!(sim.ports().get("rd_data"), Some(0xA5));
}

fn spi_drive(
    sim: &mut Sim,

    rst: u64,

    start: u64,

    tx_data: u64,

    miso: u64,

    cpol: u64,

    cpha: u64,

    byte_count: u64,
) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("start", start);

    pv.set("tx_data", tx_data);

    pv.set("miso", miso);

    pv.set("cpol", cpol);

    pv.set("cpha", cpha);

    pv.set("byte_count", byte_count);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();
}

#[test]

fn spi_master_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<SpiMaster>("SpiMaster");

    let hir = SpiMaster::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module SpiMaster"));

    assert!(
        v.contains("cs_n")
            && v.contains("sclk")
            && v.contains("mosi")
            && v.contains("cpol")
            && v.contains("rx_data"),
        "FR98 SPI must emit serial pins + CPOL/RX"
    );

    let mut sim = Sim::new(SpiMaster::elaborate().unwrap());

    spi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("busy"), Some(0));

    assert_eq!(sim.ports().get("cs_n"), Some(1));

    assert_eq!(sim.ports().get("sclk"), Some(0));

    // Mode-0 half-period: 0xA5 = 1010_0101

    let expected = [1u64, 0, 1, 0, 0, 1, 0, 1];

    spi_drive(&mut sim, 0, 1, 0xA5, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("busy"), Some(1));

    assert_eq!(sim.ports().get("mosi_byte"), Some(0xA5));

    assert_eq!(sim.ports().get("cs_n"), Some(0));

    assert_eq!(sim.ports().get("sclk"), Some(0), "Mode-0 setup half");

    assert_eq!(sim.ports().get("mosi"), Some(1));

    for (i, &bit) in expected.iter().enumerate() {
        // leading

        spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);

        assert_eq!(sim.ports().get("sclk"), Some(1), "leading bit {i}");

        assert_eq!(sim.ports().get("mosi"), Some(bit), "MOSI bit {i}");

        // trailing (or finish on last)

        spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);

        if i + 1 < expected.len() {
            assert_eq!(sim.ports().get("busy"), Some(1));

            assert_eq!(sim.ports().get("sclk"), Some(0));

            assert_eq!(sim.ports().get("mosi"), Some(expected[i + 1]));
        }
    }

    assert_eq!(sim.ports().get("busy"), Some(0));

    assert_eq!(sim.ports().get("cs_n"), Some(1));

    // busy-gated: start while busy must not replace hold

    spi_drive(&mut sim, 0, 1, 0x3C, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("busy"), Some(1));

    spi_drive(&mut sim, 0, 1, 0xFF, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("mosi_byte"), Some(0x3C));
}

fn i2c_drive(sim: &mut Sim, rst: u64, start: u64, addr: u64, rw: u64, tx_data: u64, sda_in: u64) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("start", start);

    pv.set("addr", addr);

    pv.set("rw", rw);

    pv.set("tx_data", tx_data);

    pv.set("sda_in", sda_in);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();
}

#[test]

fn i2c_master_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<I2cMaster>("I2cMaster");

    let hir = I2cMaster::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module I2cMaster"));

    assert!(
        v.contains("scl") && v.contains("sda_out") && v.contains("addr") && v.contains("ack_error"),
        "FR98 I2C must emit near-VIP ports"
    );

    let mut sim = Sim::new(I2cMaster::elaborate().unwrap());

    i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("busy"), Some(0));

    assert_eq!(sim.ports().get("scl"), Some(1));

    assert_eq!(sim.ports().get("sda_out"), Some(1));

    let addr = 0x50u64;

    let data = 0xA5u64;

    let addr_byte = addr << 1;

    i2c_drive(&mut sim, 0, 1, addr, 0, data, 0);

    assert_eq!(sim.ports().get("busy"), Some(1));

    assert_eq!(sim.ports().get("tx_byte"), Some(addr_byte));

    assert_eq!(sim.ports().get("sda_out"), Some(0)); // START

    assert_eq!(sim.ports().get("scl"), Some(1));

    let mut saw_scl_low = false;

    for _ in 0..64 {
        i2c_drive(&mut sim, 0, 0, addr, 0, data, 0);

        if sim.ports().get("scl") == Some(0) {
            saw_scl_low = true;
        }

        if sim.ports().get("busy") == Some(0) {
            break;
        }
    }

    assert!(saw_scl_low, "SCL must leave idle-high during transfer");

    assert_eq!(sim.ports().get("busy"), Some(0));

    assert_eq!(sim.ports().get("ack_error"), Some(0));

    // busy-gated: second start while busy must not replace hold

    i2c_drive(&mut sim, 0, 1, 0x11, 0, 0x22, 0);

    assert_eq!(sim.ports().get("busy"), Some(1));

    let held = sim.ports().get("tx_byte");

    i2c_drive(&mut sim, 0, 1, 0x7F, 0, 0xFF, 0);

    assert_eq!(sim.ports().get("tx_byte"), held);
}

fn axi_drive(
    sim: &mut Sim,

    rst: u64,

    awvalid: u64,

    wvalid: u64,

    wdata: u64,

    bready: u64,

    arvalid: u64,

    rready: u64,
) {
    let mut pv = PortValues::default();

    pv.set("rst", rst);

    pv.set("s_axi_awaddr", 0);

    pv.set("s_axi_awvalid", awvalid);

    pv.set("s_axi_wdata", wdata);

    pv.set("s_axi_wstrb", 0xF);

    pv.set("s_axi_wvalid", wvalid);

    pv.set("s_axi_bready", bready);

    pv.set("s_axi_araddr", 0);

    pv.set("s_axi_arvalid", arvalid);

    pv.set("s_axi_rready", rready);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();
}

#[test]

fn axi4_lite_slave_elaborate_emit_tick() {
    smoke_elaborate_emit_tick::<Axi4LiteSlave>("Axi4LiteSlave");

    let hir = Axi4LiteSlave::elaborate().unwrap();

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module Axi4LiteSlave"));

    assert!(
        v.contains("s_axi_awready") && v.contains("s_axi_bvalid") && v.contains("s_axi_rvalid"),
        "FR82 AXI-Lite must emit handshake ports"
    );

    let mut sim = Sim::new(Axi4LiteSlave::elaborate().unwrap());

    axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0);

    assert_eq!(sim.ports().get("s_axi_awready"), Some(1));

    assert_eq!(sim.ports().get("s_axi_bvalid"), Some(0));

    // write 0xDEAD_BEEF; hold bready low so bvalid sticks

    axi_drive(&mut sim, 0, 1, 1, 0xDEAD_BEEF, 0, 0, 0);

    assert_eq!(sim.ports().get("s_axi_bvalid"), Some(1));

    assert_eq!(sim.ports().get("s_axi_bresp"), Some(0));

    assert_eq!(sim.ports().get("s_axi_awready"), Some(0)); // blocked while bvalid

    axi_drive(&mut sim, 0, 0, 0, 0, 1, 0, 0);

    assert_eq!(sim.ports().get("s_axi_bvalid"), Some(0));

    assert_eq!(sim.ports().get("s_axi_awready"), Some(1));

    // read back

    axi_drive(&mut sim, 0, 0, 0, 0, 0, 1, 0);

    assert_eq!(sim.ports().get("s_axi_rvalid"), Some(1));

    assert_eq!(sim.ports().get("s_axi_rdata"), Some(0xDEAD_BEEF));

    axi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);

    assert_eq!(sim.ports().get("s_axi_rvalid"), Some(0));
}

#[test]

fn gpio_elaborate_emit_tick_ports() {
    smoke_elaborate_emit_tick::<Gpio>("Gpio");

    let hir = Gpio::elaborate().unwrap();

    let v = &emit(&hir).files[0].contents;

    assert!(v.contains("module Gpio"));

    for p in [
        "dir", "wr_en", "wr_data", "wr_mask", "pad_in", "pad_out", "rd_data",
    ] {
        assert!(v.contains(p), "missing port {p}");
    }
}

#[test]

fn gpio_masked_write_and_direction_readback() {
    let mut sim = Sim::new(Gpio::elaborate().unwrap());

    let mut pv = PortValues::default();

    pv.set("rst", 1);

    pv.set("dir", 0xff);

    pv.set("wr_en", 0);

    pv.set("wr_data", 0);

    pv.set("wr_mask", 0);

    pv.set("pad_in", 0);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    pv.set("rst", 0);

    // Full write 0xA5 with mask 0xFF, all outputs

    pv.set("wr_en", 1);

    pv.set("wr_data", 0xa5);

    pv.set("wr_mask", 0xff);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_out"), Some(0xa5));

    assert_eq!(sim.ports().get("rd_data"), Some(0xa5));

    // Masked write: low nibble only → 0xA5 becomes 0xAB

    pv.set("wr_data", 0x0b);

    pv.set("wr_mask", 0x0f);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_out"), Some(0xab));

    // Input direction on high nibble: pad_in 0x50 → rd_data 0x5B

    pv.set("wr_en", 0);

    pv.set("dir", 0x0f);

    pv.set("pad_in", 0x50);

    sim.set_inputs(pv);

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_out"), Some(0x0b));

    assert_eq!(sim.ports().get("rd_data"), Some(0x5b));
}

#[test]

fn gpio_vip_elaborate_emit_commercial_ports() {
    smoke_elaborate_emit_tick::<GpioVip>("GpioVip");

    let hir = GpioVip::elaborate().unwrap();

    let v = &emit(&hir).files[0].contents;

    assert!(v.contains("module GpioVip"));

    for p in [
        "dir",
        "wr_en",
        "set_en",
        "clr_en",
        "od",
        "irq_en",
        "irq_clear",
        "pad_oe",
        "irq_status",
        "irq_out",
    ] {
        assert!(v.contains(p), "missing commercial VIP port {p}");
    }
}

#[test]

fn gpio_vip_set_clear_irq_open_drain_tick() {
    let mut sim = Sim::new(GpioVip::elaborate().unwrap());

    let mut pv = PortValues::default();

    // reset

    for (k, v) in [
        ("rst", 1u64),
        ("dir", 0xff),
        ("wr_en", 0),
        ("wr_data", 0),
        ("wr_mask", 0),
        ("set_en", 0),
        ("set_data", 0),
        ("clr_en", 0),
        ("clr_data", 0),
        ("od", 0),
        ("pad_in", 0),
        ("irq_en", 0),
        ("irq_clear", 0),
    ] {
        pv.set(k, v);
    }

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    pv.set("rst", 0);

    // C3: atomic set 0x0f

    pv.set("set_en", 1);

    pv.set("set_data", 0x0f);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_out"), Some(0x0f));

    pv.set("set_en", 0);

    // C3: clear low nibble → 0

    pv.set("clr_en", 1);

    pv.set("clr_data", 0x0f);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_out"), Some(0x00));

    pv.set("clr_en", 0);

    // C2: open-drain — write 0x03, od=0x01 → bit0 Hi-Z (oe=0), bit1 drive

    pv.set("wr_en", 1);

    pv.set("wr_data", 0x03);

    pv.set("wr_mask", 0xff);

    pv.set("od", 0x01);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("pad_oe"), Some(0xfe)); // ~od&out for bit0

    assert_eq!(sim.ports().get("pad_out"), Some(0x02));

    pv.set("wr_en", 0);

    pv.set("od", 0);

    // C1: rising edge on bit0 with irq_en

    pv.set("irq_en", 0x01);

    pv.set("pad_in", 0);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    pv.set("pad_in", 1);

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("irq_status"), Some(0x01));

    assert_eq!(sim.ports().get("irq_out"), Some(1));

    // clear

    pv.set("irq_clear", 0x01);

    pv.set("pad_in", 1); // no new edge (prev already 1)

    sim.set_inputs(pv.clone());

    sim.settle();

    sim.tick();

    assert_eq!(sim.ports().get("irq_status"), Some(0x00));

    assert_eq!(sim.ports().get("irq_out"), Some(0));
}

#[test]

fn blackbox_elaborate_emit_tick_opaque() {
    smoke_elaborate_emit_tick::<ExtBlackBox>("ExtBlackBox");

    let hir = ExtBlackBox::elaborate().unwrap();

    // Opaque: no regs / processes in body — ports only.

    let m = &hir.circuit().modules[0];

    assert!(
        m.body.is_empty(),
        "black-box must not inline vendor HIR body"
    );

    assert!(vendor_blackbox_v().contains("vendor_ext_ip"));
}

#[test]

fn fifo_uart_api_has_no_generator_closure_params() {
    // Compile-time / surface check: Elaboratable::elaborate takes no Fn.

    let _ = SyncFifo::elaborate();

    let _ = UartTx::elaborate();

    let _ = SpiMaster::elaborate();

    let _ = I2cMaster::elaborate();

    let _ = Axi4LiteSlave::elaborate();

    let _ = Gpio::elaborate();

    let _ = GpioVip::elaborate();
}

#[test]

fn crc8_lut_default_elaborate_emit_tick() {
    let hir = Crc8Lut::elaborate().expect("default");

    assert_eq!(hir.abi_name, "Crc8Lut");

    let art = emit(&hir);

    let v = &art.files[0].contents;

    assert!(v.contains("module Crc8Lut"));

    assert!(v.contains("lut[0] = 0;"));

    assert!(v.contains(&format!(
        "lut[1] = {};",
        crc8_table_byte(1, Crc8Lut::DEFAULT_POLY)
    )));

    assert!(
        !v.to_lowercase().contains("closure") && !v.contains("||"),
        "NFR36: emit must not retain closure IR"
    );

    let mut sim = Sim::new(hir);

    let mut pv = PortValues::default();

    pv.set("rst", 1);

    sim.set_inputs(pv.clone());

    sim.tick();

    pv.set("rst", 0);

    pv.set("addr", 1);

    sim.set_inputs(pv);

    sim.tick(); // schedule SyncReadMem

    sim.tick(); // deliver into q / rdata

    assert_eq!(
        sim.ports().get("rdata"),
        Some(crc8_table_byte(1, Crc8Lut::DEFAULT_POLY) as u64)
    );
}

#[test]

fn crc8_lut_custom_poly_via_table_fn() {
    let poly = 0x1du8;

    let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| crc8_table_byte(i as u8, poly) as u64)
        .expect("custom");

    let mut sim = Sim::new(hir);

    let mut pv = PortValues::default();

    pv.set("rst", 1);

    sim.set_inputs(pv.clone());

    sim.tick();

    pv.set("rst", 0);

    pv.set("addr", 0xA5);

    sim.set_inputs(pv);

    sim.tick(); // schedule

    sim.tick(); // deliver

    assert_eq!(
        sim.ports().get("rdata"),
        Some(crc8_table_byte(0xA5, poly) as u64)
    );

    // Custom poly must differ from default at this address (proves customization).

    assert_ne!(
        crc8_table_byte(0xA5, poly),
        crc8_table_byte(0xA5, Crc8Lut::DEFAULT_POLY)
    );
}

#[test]

fn crc8_lut_synthesizable_closure_violation_is_clear_error() {
    let err = Crc8Lut::elaborate_with_table_fn(
        &[SynthesizableClosureViolation::heap("Vec in table Fn")],
        |_| 0,
    )
    .expect_err("must reject");

    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0143"),
        "expected E0143, got {err:?}"
    );
}

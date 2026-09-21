//! Story128.4 GPIO CSR public API and independent composition verification.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{CsrAccess, CsrOwner, Gpio, GpioCsr, Irq},
};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Stdio},
};
fn ports() -> BTreeMap<String, (PortDirection, GroundType)> {
    use PortDirection::{Input, Output};
    let mut p = BTreeMap::from([
        ("clk".into(), (Input, GroundType::Clock)),
        ("rst".into(), (Input, GroundType::Reset)),
    ]);
    for (n, w, d) in [
        ("req_valid", 1, Input),
        ("write", 1, Input),
        ("addr", 16, Input),
        ("wdata", 32, Input),
        ("wstrb", 4, Input),
        ("rsp_ready", 1, Input),
        ("pad_in", 32, Input),
        ("req_ready", 1, Output),
        ("rsp_valid", 1, Output),
        ("rdata", 32, Output),
        ("error", 2, Output),
        ("pad_out", 32, Output),
        ("pad_oe", 32, Output),
        ("raw_event", 1, Output),
    ] {
        p.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    p
}
#[test]
fn p0_exact_sixteen_ports_and_shared_standalone_equivalence() {
    let hir = GpioCsr::elaborate().unwrap();
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "GpioCsr")
        .unwrap();
    assert_eq!(top.ports.len(), 16);
    assert_eq!(
        top.ports
            .iter()
            .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
            .collect::<BTreeMap<_, _>>(),
        ports()
    );
    let mut s = ElaborateSession::new("GpioCsr");
    assert_eq!(
        GpioCsr::define_module(&mut s, String::from("GpioCsr")).unwrap(),
        "GpioCsr"
    );
    GpioCsr::define_module(&mut s, "GpioCsr").unwrap();
    assert_eq!(hir, s.finish().unwrap());
}
#[test]
fn p0_descriptor_exact_local_addresses_masks_and_owners() {
    use CsrAccess::{Ro, Rw, W1c, Wo};
    use CsrOwner::{External, Leaf};
    let block = GpioCsr::registers();
    block.validate().unwrap();
    assert_eq!(block.name, "GpioCsr");
    assert_eq!(block.registers.len(), 6);
    for (name, offset, access, owner, event) in [
        ("dir", 0, Rw, Leaf, None),
        ("out", 4, Rw, External, None),
        ("in", 8, Ro, External, None),
        ("set", 12, Wo, CsrOwner::None, None),
        ("clear", 16, Wo, CsrOwner::None, None),
        ("rise_event", 20, W1c, Leaf, Some("rise_bits")),
    ] {
        let r = block.registers.iter().find(|r| r.name == name).unwrap();
        assert_eq!(
            (r.offset, r.reset, r.access, r.owner),
            (offset, 0, access, owner)
        );
        assert_eq!(r.event.as_deref(), event);
        assert!(!r.read_reject && !r.write_reject);
        assert_eq!(r.fields.len(), 1);
        let f = &r.fields[0];
        assert_eq!(f.name, "bits");
        assert_eq!((f.mask, f.reset, f.access), (0xffffffff, 0, access));
    }
}
#[test]
fn p1_software_outputs_match_independent_local_address_golden() {
    let block = GpioCsr::registers();
    let c = block.emit_c_header().unwrap();
    let md = block.emit_markdown().unwrap();
    assert_eq!(c, GpioCsr::registers().emit_c_header().unwrap());
    assert_eq!(md, GpioCsr::registers().emit_markdown().unwrap());
    for (name, addr, access) in [
        ("DIR", "00000000", "RW"),
        ("OUT", "00000004", "RW"),
        ("IN", "00000008", "RO"),
        ("SET", "0000000c", "WO"),
        ("CLEAR", "00000010", "WO"),
        ("RISE_EVENT", "00000014", "W1C"),
    ] {
        for line in [
            format!("#define GPIOCSR_{name}_OFFSET UINT32_C(0x{addr})"),
            format!("#define GPIOCSR_{name}_MASK UINT32_C(0xffffffff)"),
            format!("#define GPIOCSR_{name}_BITS_MASK UINT32_C(0xffffffff)"),
            format!("#define GPIOCSR_{name}_RESET UINT32_C(0x00000000)"),
            format!("#define GPIOCSR_{name}_ACCESS \"{access}\""),
        ] {
            assert!(c.lines().any(|l| l == line), "missing {line}");
        }
    }
    for line in [
        "| dir | 0x0000 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | false |",
        "| out | 0x0004 | 0xffffffff | 0x00000000 | RW | External | — | false | false |",
        "| in | 0x0008 | 0xffffffff | 0x00000000 | RO | External | — | false | false |",
        "| set | 0x000c | 0xffffffff | 0x00000000 | WO | None | — | false | false |",
        "| clear | 0x0010 | 0xffffffff | 0x00000000 | WO | None | — | false | false |",
        "| rise_event | 0x0014 | 0xffffffff | 0x00000000 | W1C | Leaf | rise_bits | false | false |",
    ] {
        assert!(md.lines().any(|l| l == line), "missing {line}");
    }
    assert!(md.contains("Local byte offsets; caller supplies the base address."));
    assert!(!c.contains("UINT32_C(0x00000100)"));
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(
        c,
        fs::read_to_string(root.join("docs/ip/gpio-csr-registers.h")).unwrap()
    );
    assert_eq!(
        md,
        fs::read_to_string(root.join("docs/ip/gpio-csr-registers.md")).unwrap()
    );
}
#[test]
fn p0_invalid_names_and_conflicting_bodies_are_rejected() {
    for name in ["", "not a module", "module"] {
        let mut s = ElaborateSession::new("Top");
        assert_diag(
            &GpioCsr::define_module(&mut s, name).unwrap_err(),
            "rhdl::E0241",
            &["module name", "identifier"],
        );
        assert_diag(&s.finish().unwrap_err(), "rhdl::E0241", &["module name"]);
    }
    let mut s = ElaborateSession::new("GpioCsr");
    GpioCsr::define_module(&mut s, "GpioCsr").unwrap();
    assert_diag(
        &Gpio::define_module(&mut s, "GpioCsr").unwrap_err(),
        "rhdl::E0244",
        &["module 'GpioCsr'", "conflicts"],
    );
    assert_diag(
        &s.finish().unwrap_err(),
        "rhdl::E0244",
        &["module 'GpioCsr'"],
    );
}
#[test]
fn p0_shared_gpio_wiring_rejects_width_clock_type_and_direction() {
    for (bad, code, port, net) in [
        ("width", "rhdl::E0203", "gpio0.addr", "t0_addr"),
        ("type", "rhdl::E0256", "gpio0.clk", "bad_clk"),
        ("direction", "rhdl::E0257", "gpio0.pad_out", "t0_pad_out"),
    ] {
        assert_diag(&pair(Some(bad)).unwrap_err(), code, &[port, net]);
    }
    for same in [true, false] {
        let hir = pair_kind(None, same).unwrap();
        assert_eq!(
            hir.circuit()
                .modules
                .iter()
                .filter(|m| m.name == "SharedGpio")
                .count(),
            1
        );
        assert_eq!(
            hir.circuit()
                .modules
                .iter()
                .filter(|m| m.name == "OtherGpio")
                .count(),
            usize::from(!same)
        );
        // Additional renamed wrapper adds one definition; reused private leaf stays singular.
        if !same {
            assert_eq!(
                hir.circuit().modules.len(),
                pair_kind(None, true).unwrap().circuit().modules.len() + 1
            );
        }
    }
}
fn assert_diag(err: &bitloom_prelude::Diagnostics, code: &str, details: &[&str]) {
    assert!(
        err.0
            .iter()
            .any(|d| d.code == code && details.iter().all(|text| d.en.contains(text))),
        "expected {code} with {details:?}, got {err:?}"
    );
}

fn pair(bad: Option<&str>) -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    pair_kind(bad, false)
}

fn pair_kind(
    bad: Option<&str>,
    same_definition: bool,
) -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    let mut s = ElaborateSession::new("TwoGpios");
    GpioCsr::define_module(&mut s, "SharedGpio")?;
    GpioCsr::define_module(&mut s, "SharedGpio")?;
    if !same_definition {
        GpioCsr::define_module(&mut s, "OtherGpio")?;
    }
    let sp = Span::default();
    s.begin_module("TwoGpios", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut connects = vec![];
        for (name, (direction, mut ty)) in ports() {
            let net = if name == "clk" || name == "rst" {
                name.clone()
            } else {
                format!("t{lane}_{name}")
            };
            if lane == 0 && bad == Some("width") && name == "addr" {
                ty = GroundType::UInt { width: 8 };
            }
            if lane == 0 && bad == Some("type") && name == "clk" {
                s.add_input("bad_clk", GroundType::UInt { width: 1 }, sp);
                connects.push((name, "bad_clk".into()));
                continue;
            }
            if name != "clk" && name != "rst" {
                if direction == PortDirection::Input
                    || (lane == 0 && bad == Some("direction") && name == "pad_out")
                {
                    s.add_input(&net, ty, sp);
                } else {
                    s.add_output(&net, ty, sp);
                }
            }
            connects.push((name, net));
        }
        s.add_instance(
            format!("gpio{lane}"),
            if lane == 0 || same_definition {
                "SharedGpio"
            } else {
                "OtherGpio"
            },
            connects,
            vec![],
            sp,
        );
    }
    s.end_module();
    s.finish()
}

#[test]
fn p1_hierarchy_native_and_generated_reject_explicitly() {
    use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
    for hir in [
        GpioCsr::elaborate().unwrap(),
        pair(None).unwrap(),
        pair_kind(None, true).unwrap(),
        gpio_irq().unwrap(),
    ] {
        let mut errors = vec![];
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            errors.push(
                std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine))
                    .err()
                    .expect("hierarchy must reject"),
            );
        }
        errors.push(
            std::panic::catch_unwind(|| GeneratedFunctional::from_hir(&hir))
                .err()
                .expect("hierarchy must reject"),
        );
        for payload in errors {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("");
            assert!(
                message.contains("hierarchical simulation is unsupported"),
                "unexpected panic: {message}"
            );
        }
    }
}

fn run(dir: &Path, tool: &str, args: &[&str], label: &str) -> String {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "60s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required real RTL tool (missing tool is not a pass)");
    fs::write(
        dir.join(format!("{label}.json")),
        serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(), "start_utc_unix_ms":start_utc_unix_ms, "end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string(),
    )
    .unwrap();
    let output = fs::read_to_string(dir.join(format!("{label}.log"))).unwrap();
    assert!(
        status.success(),
        "{tool} {status}: {output}; {}",
        dir.display()
    );
    output
}

fn gpio_irq() -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    let mut s = ElaborateSession::new("GpioIrq");
    GpioCsr::define_module(&mut s, "ActualGpio")?;
    Irq::define_module(&mut s, "ActualIrq")?;
    let sp = Span::default();
    s.begin_module("GpioIrq", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut p = ports();
        if lane == 1 {
            for n in ["pad_in", "pad_out", "pad_oe", "raw_event"] {
                p.remove(n);
            }
            p.insert(
                "raw_events".into(),
                (PortDirection::Input, GroundType::UInt { width: 5 }),
            );
            p.insert(
                "irq".into(),
                (PortDirection::Output, GroundType::UInt { width: 1 }),
            );
        }
        let mut connections = vec![];
        for (name, (direction, ty)) in p {
            let net = if name == "clk" || name == "rst" {
                name.clone()
            } else {
                format!("t{lane}_{name}")
            };
            if lane == 1 && name == "raw_events" {
                s.declare_wire(&net, ty, sp);
            } else if name != "clk" && name != "rst" {
                if direction == PortDirection::Input {
                    s.add_input(&net, ty, sp);
                } else {
                    s.add_output(&net, ty, sp);
                }
            }
            connections.push((name, net));
        }
        s.add_instance(
            format!("peripheral{lane}"),
            if lane == 0 { "ActualGpio" } else { "ActualIrq" },
            connections,
            vec![],
            sp,
        );
    }
    for n in ["zero5", "gpio_bit"] {
        s.declare_wire(n, GroundType::UInt { width: 5 }, sp);
    }
    s.begin_combinational(sp);
    s.assign_lit("zero5", 0, sp);
    s.assign_lit("gpio_bit", 16, sp);
    s.assign_mux("t1_raw_events", "t0_raw_event", "gpio_bit", "zero5", sp);
    s.end_process();
    s.end_module();
    s.finish()
}
fn pair_tb(hir: &FrozenHir) -> String {
    let mut tb = tb_prefix(hir, "TwoGpios");
    tb.push_str(r#"
initial begin
 $dumpfile("trace.vcd"); $dumpvars(0,tb);
 rst=1; tick; rst=0;
 access0(1,0,32'h80000000,0); access1(1,0,1,0);
 access0(1,4,32'h80000000,0); access1(1,4,1,0);
 if(t0_pad_out!==32'h80000000 || t1_pad_out!==1 || t0_pad_oe!==32'h80000000 || t1_pad_oe!==1) $fatal(1,"independent output state");
 // Simultaneous snapshots, then only lane0 consumes its response.
 t0_req_valid=1; t0_write=0; t0_addr=4;
 t1_req_valid=1; t1_write=0; t1_addr=4; tick;
 t0_req_valid=0; t1_req_valid=0;
 if(t0_rdata!==32'h80000000 || t1_rdata!==1) $fatal(1,"independent read snapshots");
 t0_rsp_ready=1; tick; t0_rsp_ready=0;
 t1_pad_in=32'h80000000; t0_pad_in=1;
 access0(1,16,32'h80000000,0); access0(0,4,0,0);
 repeat(5) begin
 if(t1_rsp_valid!==1 || t1_req_ready!==0 || t1_rdata!==1 || t1_error!==0) $fatal(1,"held lane1 response");
 tick; end
 if(t0_pad_out!==0 || t1_pad_out!==1) $fatal(1,"cross-lane output contamination");
 t1_rsp_ready=1; tick; t1_rsp_ready=0;
 access0(0,20,0,1); access1(0,20,0,32'h80000000);
 access0(0,8,0,1); access1(0,8,0,32'h80000000);
 access0(1,20,1,0); access0(0,20,0,0); access1(0,20,0,32'h80000000);
 t0_req_valid=1; t0_write=0; t0_addr=20;
 t1_req_valid=1; t1_write=0; t1_addr=20; tick;
 t0_req_valid=0; t1_req_valid=0; rst=1; tick;
 t0_pad_in=0; t1_pad_in=0; rst=0; #1;
 if(t0_rsp_valid!==0 || t1_rsp_valid!==0 || t0_pad_out!==0 || t1_pad_out!==0 || t0_pad_oe!==0 || t1_pad_oe!==0 || t0_raw_event!==0 || t1_raw_event!==0) $fatal(1,"shared reset cancellation");
 access0(0,20,0,0); access1(0,20,0,0); access0(0,4,0,0); access1(0,4,0,0);
 $display("PASS GPIO composition independent state"); $finish;
end
initial begin #100000; $fatal(1,"watchdog"); end
endmodule
"#);
    tb
}
fn gpio_irq_tb(hir: &FrozenHir) -> String {
    let mut tb = tb_prefix(hir, "GpioIrq");
    tb.push_str(r#"
integer rises=0;
always @(posedge clk) if(!rst && t0_raw_event) rises=rises+1;
initial begin
 $dumpfile("trace.vcd"); $dumpvars(0,tb);
 rst=1; tick; rst=0; access1(1,4,16,0);
 t0_pad_in=32'h80000000; #1;
 if(t0_raw_event!==0) $fatal(1,"raw input bypass");
 tick; if(t0_raw_event!==0 || t1_irq!==0) $fatal(1,"first sync stage early event");
 tick; if(t0_raw_event!==1 || t1_irq!==0) $fatal(1,"sync2 rise phase");
 tick; if(t0_raw_event!==0 || t1_irq!==1 || rises!==1) $fatal(1,"event capture phase");
 access0(0,20,0,32'h80000000); access1(0,0,0,16);
 access1(1,4,0,0);
 if(t1_irq!==0 || t0_raw_event!==0 || rises!==1) $fatal(1,"mask disables only IRQ output");
 access1(0,0,0,16);
 access1(1,4,16,0);
 if(t1_irq!==1 || t0_raw_event!==0 || rises!==1) $fatal(1,"reenable lost pending without new edge");
 access1(1,0,16,0);
 repeat(8) begin tick; if(t1_irq!==0) $fatal(1,"sticky GPIO retriggered IRQ"); end
 access1(0,0,0,0); access1(0,12,0,0); access0(0,20,0,32'h80000000);
 t0_pad_in=0; repeat(3) tick;
 t0_pad_in=32'h80000000; repeat(3) tick;
 if(rises!==2 || t1_irq!==1) $fatal(1,"second genuine edge lost");
 access1(0,0,0,16); access0(0,20,0,32'h80000000);
 t0_pad_in=0; repeat(3) tick;
 t0_pad_in=32'h80000000; tick; tick;
 if(t0_raw_event!==1 || rises!==2) $fatal(1,"dual clear collision live rise precondition");
 t0_req_valid=1; t0_write=1; t0_addr=20; t0_wdata=32'h80000000; t0_wstrb=15;
 t1_req_valid=1; t1_write=1; t1_addr=0; t1_wdata=16; t1_wstrb=15; #1;
 if(t0_req_ready!==1 || t1_req_ready!==1) $fatal(1,"dual clear must commit together");
 tick; t0_req_valid=0; t1_req_valid=0;
 if(rises!==3 || t1_irq!==1 || t0_rsp_valid!==1 || t1_rsp_valid!==1 || t0_error!==0 || t1_error!==0) $fatal(1,"new rise must win both clears");
 t0_rsp_ready=1; t1_rsp_ready=1; tick; t0_rsp_ready=0; t1_rsp_ready=0;
 access0(0,20,0,32'h80000000); access1(0,0,0,16);
 t0_pad_in=0; repeat(3) tick;
 // Capture both responses on the first synchronization edge, then hold them
 // while the next edge makes raw_event live. Reset before event capture.
 t0_pad_in=32'h80000000;
 t0_req_valid=1; t0_write=0; t0_addr=20;
 t1_req_valid=1; t1_write=0; t1_addr=0; tick;
 t0_req_valid=0; t1_req_valid=0; tick;
 if(t0_raw_event!==1 || t0_rsp_valid!==1 || t1_rsp_valid!==1 || rises!==3) $fatal(1,"live pulse and held responses reset precondition");
 rst=1; #1;
 if(t0_raw_event!==0) $fatal(1,"reset raw event suppression");
 tick; t0_pad_in=0; rst=0; #1;
 if(t0_rsp_valid!==0 || t1_rsp_valid!==0 || t0_raw_event!==0 || t1_irq!==0 || rises!==3) $fatal(1,"GPIO IRQ reset cancellation");
 access0(0,20,0,0); access1(0,0,0,0); access1(0,4,0,0);
 $display("PASS GPIO composition actual GPIO raw event"); $finish;
end
initial begin #100000; $fatal(1,"watchdog"); end
endmodule
"#);
    tb
}
#[test]
fn p0_actual_two_gpio_rtl_shared_and_renamed_isolation() {
    for same in [true, false] {
        let h = pair_kind(None, same).unwrap();
        execute_composition(&h, "TwoGpios", &pair_tb(&h), &format!("pair-{same}"), false);
    }
}
#[test]
fn p0_actual_gpio_raw_to_irq4_never_retriggers_from_sticky() {
    let h = gpio_irq().unwrap();
    execute_composition(&h, "GpioIrq", &gpio_irq_tb(&h), "gpio-irq", false);
}
#[test]
#[ignore = "dedicated pinned firtool and JVM composition gate"]
fn p1_composition_firrtl_chisel_shared_renamed_actual_gpio_irq() {
    for same in [true, false] {
        let h = pair_kind(None, same).unwrap();
        execute_composition(&h, "TwoGpios", &pair_tb(&h), &format!("pair-{same}"), true);
    }
    let h = gpio_irq().unwrap();
    execute_composition(&h, "GpioIrq", &gpio_irq_tb(&h), "gpio-irq", true);
}
#[test]
fn p1_legacy_gpio_exact_ports_masked_write_reset_and_handwritten_fl_preserved() {
    use bitloom_hir::PortValues;
    use bitloom_sim::{AbstractionView, GpioFunctional, Sim, TickEngine};
    let h = Gpio::elaborate().unwrap();
    let top = h
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Gpio")
        .unwrap();
    let mut expected = BTreeMap::from([
        ("clk".into(), (PortDirection::Input, GroundType::Clock)),
        ("rst".into(), (PortDirection::Input, GroundType::Reset)),
    ]);
    for (n, w, d) in [
        ("dir", 8, PortDirection::Input),
        ("wr_en", 1, PortDirection::Input),
        ("wr_data", 8, PortDirection::Input),
        ("wr_mask", 8, PortDirection::Input),
        ("pad_in", 8, PortDirection::Input),
        ("pad_out", 8, PortDirection::Output),
        ("rd_data", 8, PortDirection::Output),
    ] {
        expected.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    assert_eq!(
        top.ports
            .iter()
            .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
            .collect::<BTreeMap<_, _>>(),
        expected
    );
    // Literal independent vectors: reset; full write; nibble merge; zero mask;
    // mixed direction immediate combinational pad read; reset wins write.
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut sim = Sim::with_engine(h.clone(), engine);
        let mut fl = GpioFunctional::new();
        for (rst, dir, en, data, mask, pad, out, read) in [
            (1, 255, 0, 0, 0, 0, 0, 0),
            (0, 255, 1, 165, 255, 0, 165, 165),
            (0, 255, 1, 11, 15, 0, 171, 171),
            (0, 255, 1, 0, 0, 0, 171, 171),
            (0, 15, 0, 0, 0, 80, 11, 91),
            (0, 15, 0, 0, 0, 160, 11, 171),
            (1, 255, 1, 255, 255, 0, 0, 0),
        ] {
            let mut p = PortValues::default();
            for (n, v) in [
                ("rst", rst),
                ("dir", dir),
                ("wr_en", en),
                ("wr_data", data),
                ("wr_mask", mask),
                ("pad_in", pad),
            ] {
                p.set(n, v);
            }
            let f = fl.cycle(&p);
            sim.set_inputs(p);
            sim.settle();
            sim.tick();
            for (n, v) in [("pad_out", out), ("rd_data", read)] {
                assert_eq!(f.get(n), Some(v));
                assert_eq!(sim.ports().get(n), Some(v));
            }
        }
        let mut p = PortValues::default();
        p.set("dir", 0);
        p.set("pad_in", 0x81);
        sim.set_inputs(p.clone());
        sim.settle();
        assert_eq!(sim.ports().get("rd_data"), Some(0x81));
        p.set("pad_in", 0x42);
        sim.set_inputs(p);
        sim.settle();
        assert_eq!(
            sim.ports().get("rd_data"),
            Some(0x42),
            "legacy input is combinational, no synchronizer added"
        );
    }
}
fn tb_prefix(hir: &FrozenHir, top_name: &str) -> String {
    use std::fmt::Write as _;
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top_name)
        .unwrap();
    let mut tb = String::from("module tb; reg clk=0, rst=0;\n");
    for p in &top.ports {
        if p.name == "clk" || p.name == "rst" {
            continue;
        }
        let GroundType::UInt { width } = p.ty else {
            panic!("unexpected type")
        };
        writeln!(
            tb,
            "{} [{}:0] {}{};",
            if p.direction == PortDirection::Input {
                "reg"
            } else {
                "wire"
            },
            width - 1,
            p.name,
            if p.direction == PortDirection::Input {
                "=0"
            } else {
                ""
            }
        )
        .unwrap();
    }
    writeln!(tb, "{top_name} dut(.*);").unwrap();
    tb.push_str("task tick; begin #5; clk=1; #5; clk=0; #1; end endtask\n");
    for lane in 0..2 {
        writeln!(tb,r#"task access{lane}(input wr,input [15:0] a,input [31:0] data,input [31:0] expected);
begin
 t{lane}_req_valid=1; t{lane}_write=wr; t{lane}_addr=a; t{lane}_wdata=data; t{lane}_wstrb=15; #1;
 if(t{lane}_req_ready !== 1) $fatal(1,"lane {lane} not ready");
 tick; t{lane}_req_valid=0;
 if(t{lane}_rsp_valid !== 1 || t{lane}_error !== 0) $fatal(1,"lane {lane} response");
 if(!wr && t{lane}_rdata !== expected) $fatal(1,"lane {lane} addr %h expected %h got %h",a,expected,t{lane}_rdata);
 t{lane}_rsp_ready=1; tick; t{lane}_rsp_ready=0;
 if(t{lane}_rsp_valid !== 0) $fatal(1,"lane {lane} response not consumed");
end endtask"#).unwrap();
    }
    tb
}

fn execute_composition(hir: &FrozenHir, top_name: &str, tb: &str, label: &str, backends: bool) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-gpio-api")
        .join(format!(
            "{label}-{backends}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    let rtl = bitloom_vlog::emit(hir)
        .files
        .into_iter()
        .map(|f| f.contents)
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("dut.v"), rtl).unwrap();
    if backends {
        composition_backends(hir, top_name, tb, &dir);
        return;
    }
    run(&dir, "iverilog", &["-V"], "iverilog-version");
    run(&dir, "vvp", &["-V"], "vvp-version");
    run(
        &dir,
        "iverilog",
        &["-g2012", "-s", "tb", "-o", "sim", "dut.v", "tb.sv"],
        "compile",
    );
    assert!(run(&dir, "vvp", &["sim"], "simulate").contains("PASS GPIO composition"));
}
fn composition_backends(hir: &FrozenHir, top_name: &str, tb: &str, dir: &Path) {
    let tools = std::env::var("RHDL_FIRTOOL_PATH").expect("pinned firtool directory");
    let firtool = format!("{tools}/firtool");
    assert!(run(dir, &firtool, &["--version"], "firtool-version").contains("firtool-1.159.0"));
    fs::write(
        dir.join("design.fir"),
        &bitloom_firrtl::emit(hir).files[0].contents,
    )
    .unwrap();
    run(
        dir,
        &firtool,
        &[
            "design.fir",
            "--verilog",
            "--disable-all-randomization",
            "--lowering-options=disallowLocalVariables",
            "-o",
            "firrtl.v",
        ],
        "firrtl-lower",
    );
    run(
        dir,
        "iverilog",
        &[
            "-g2012",
            "-s",
            "tb",
            "-o",
            "firrtl-sim",
            "firrtl.v",
            "tb.sv",
        ],
        "firrtl-compile",
    );
    assert!(run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("PASS GPIO composition"));
    fs::rename(dir.join("trace.vcd"), dir.join("firrtl.vcd")).unwrap();
    fs::create_dir_all(dir.join("src/main/scala")).unwrap();
    fs::create_dir_all(dir.join("project")).unwrap();
    fs::write(
        dir.join("src/main/scala/Design.scala"),
        &bitloom_firrtl::emit_chisel(hir).unwrap().files[0].contents,
    )
    .unwrap();
    fs::write(dir.join("src/main/scala/Main.scala"), format!("object PairMain extends App {{ circt.stage.ChiselStage.emitSystemVerilogFile(new {top_name}, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }}\n")).unwrap();
    fs::write(dir.join("build.sbt"), "scalaVersion := \"2.13.18\"\nlibraryDependencies += \"org.chipsalliance\" %% \"chisel\" % \"7.15.0\"\naddCompilerPlugin(\"org.chipsalliance\" % \"chisel-plugin\" % \"7.15.0\" cross CrossVersion.full)\n").unwrap();
    fs::write(
        dir.join("project/build.properties"),
        "sbt.version=1.10.11\n",
    )
    .unwrap();
    let log = fs::File::create(dir.join("sbt.log")).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args([
            "--kill-after=5s",
            "240s",
            "sbt",
            "-batch",
            "runMain PairMain",
        ])
        .env("CHISEL_FIRTOOL_PATH", &tools)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(dir.join("sbt.exit"), format!("{status}\n")).unwrap();
    fs::write(dir.join("sbt.json"),serde_json::json!({"command":["timeout","--kill-after=5s","240s","sbt","-batch","runMain PairMain"],"CHISEL_FIRTOOL_PATH":tools,"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    assert!(status.success(), "JVM pair failed: {}", dir.display());
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top_name)
        .unwrap();
    let connections = top
        .ports
        .iter()
        .map(|p| {
            let name = match p.name.as_str() {
                "clk" => "clock".into(),
                "rst" => "reset".into(),
                n => format!("io_{n}"),
            };
            format!(".{name}({})", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let chisel = dir.join("chisel");
    fs::write(
        chisel.join("tb.sv"),
        tb.replace("dut(.*)", &format!("dut({connections})")),
    )
    .unwrap();
    run(
        &chisel,
        "iverilog",
        &[
            "-g2012",
            "-I",
            ".",
            "-s",
            "tb",
            "-o",
            "simulation",
            "-f",
            "filelist.f",
            "tb.sv",
        ],
        "compile",
    );
    assert!(run(&chisel, "vvp", &["simulation"], "simulate").contains("PASS GPIO composition"));
    println!(
        "GPIO composition FIRRTL and JVM actual RTL PASS {}",
        dir.display()
    );
}

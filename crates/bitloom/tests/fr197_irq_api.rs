//! Story128.3 independent public surface and real two-instance Irq acceptance.
//! Fixed ATDD contract, software products, and real composition acceptance.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{CsrAccess, CsrOwner, Irq, Timer},
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
        ("raw_events", 5, Input),
        ("req_ready", 1, Output),
        ("rsp_valid", 1, Output),
        ("rdata", 32, Output),
        ("error", 2, Output),
        ("irq", 1, Output),
    ] {
        p.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    p
}

#[test]
fn p0_exact_fourteen_ports_and_shared_standalone_equivalence() {
    let hir = Irq::elaborate().unwrap();
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Irq")
        .unwrap();
    assert_eq!(top.ports.len(), 14);
    let actual: BTreeMap<_, _> = top
        .ports
        .iter()
        .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
        .collect();
    assert_eq!(actual, ports());
    let mut s = ElaborateSession::new("Irq");
    assert_eq!(
        Irq::define_module(&mut s, String::from("Irq")).unwrap(),
        "Irq"
    );
    Irq::define_module(&mut s, "Irq").unwrap();
    let shared = s.finish().unwrap();
    assert_eq!(
        hir, shared,
        "same body and no duplicate private definitions"
    );
}

#[test]
fn p0_register_descriptor_has_exact_local_map_masks_and_unique_owners() {
    use CsrAccess::{Ro, Rw, W1c, Wo};
    use CsrOwner::{External, Leaf};
    let block = Irq::registers();
    block.validate().unwrap();
    assert_eq!(block.name, "Irq");
    assert_eq!(block.registers.len(), 4);
    for (name, offset, access, owner, event) in [
        ("pending", 0, W1c, Leaf, Some("event_bits")),
        ("enable", 4, Rw, Leaf, None),
        ("test", 8, Wo, CsrOwner::None, None),
        ("raw", 12, Ro, External, None),
    ] {
        let r = block.registers.iter().find(|r| r.name == name).unwrap();
        assert_eq!(
            (r.offset, r.reset, r.access, r.owner),
            (offset, 0, access, owner)
        );
        assert_eq!(r.event.as_deref(), event);
        assert!(!r.read_reject && !r.write_reject);
        assert_eq!(r.fields.len(), 5);
        for (field, mask) in [
            ("timer", 1),
            ("uart_rx", 2),
            ("uart_tx", 4),
            ("uart_error", 8),
            ("gpio", 16),
        ] {
            let f = r.fields.iter().find(|f| f.name == field).unwrap();
            assert_eq!((f.mask, f.reset, f.access), (mask, 0, access));
        }
    }
}

#[test]
fn p1_software_outputs_match_handwritten_local_address_golden() {
    let block = Irq::registers();
    let c = block.emit_c_header().unwrap();
    let md = block.emit_markdown().unwrap();
    // These addresses, access tags and field masks are independent handwritten constants.
    for (register, address, access) in [
        ("PENDING", "00000000", "W1C"),
        ("ENABLE", "00000004", "RW"),
        ("TEST", "00000008", "WO"),
        ("RAW", "0000000c", "RO"),
    ] {
        for line in [
            format!("#define IRQ_{register}_OFFSET UINT32_C(0x{address})"),
            format!("#define IRQ_{register}_MASK UINT32_C(0x0000001f)"),
            format!("#define IRQ_{register}_RESET UINT32_C(0x00000000)"),
            format!("#define IRQ_{register}_ACCESS \"{access}\""),
        ] {
            assert!(c.lines().any(|l| l == line), "missing {line}");
        }
        for (field, mask) in [
            ("TIMER", "00000001"),
            ("UART_RX", "00000002"),
            ("UART_TX", "00000004"),
            ("UART_ERROR", "00000008"),
            ("GPIO", "00000010"),
        ] {
            let line = format!("#define IRQ_{register}_{field}_MASK UINT32_C(0x{mask})");
            assert!(c.lines().any(|l| l == line), "missing {line}");
        }
    }
    for line in [
        "| pending | 0x0000 | 0x0000001f | 0x00000000 | W1C | Leaf | event_bits | false | false |",
        "| enable | 0x0004 | 0x0000001f | 0x00000000 | RW | Leaf | — | false | false |",
        "| test | 0x0008 | 0x0000001f | 0x00000000 | WO | None | — | false | false |",
        "| raw | 0x000c | 0x0000001f | 0x00000000 | RO | External | — | false | false |",
    ] {
        assert!(md.lines().any(|l| l == line), "missing {line}");
    }
    assert!(md.contains("Local byte offsets; caller supplies the base address."));
    assert!(!c.contains("UINT32_C(0x00000300)"));
    assert!(!md.contains("| 0x0300 |"));
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // Runtime reads keep the first RED about the missing Irq API, not absent docs.
    assert_eq!(
        c,
        fs::read_to_string(root.join("docs/ip/irq-registers.h")).unwrap()
    );
    assert_eq!(
        md,
        fs::read_to_string(root.join("docs/ip/irq-registers.md")).unwrap()
    );
}

#[test]
fn p0_invalid_names_and_conflicting_bodies_are_rejected() {
    for name in ["", "not a module", "module"] {
        let mut s = ElaborateSession::new("Top");
        let err = Irq::define_module(&mut s, name).unwrap_err();
        assert_diag(&err, "rhdl::E0241", &["module name", "identifier"]);
        assert_diag(&s.finish().unwrap_err(), "rhdl::E0241", &["module name"]);
    }
    let mut s = ElaborateSession::new("Irq");
    Irq::define_module(&mut s, "Irq").unwrap();
    let err = bitloom_prelude::ip::CsrDecoder::define_module(&mut s, "Irq").unwrap_err();
    assert_diag(&err, "rhdl::E0244", &["module 'Irq'", "conflicts"]);
    assert_diag(&s.finish().unwrap_err(), "rhdl::E0244", &["module 'Irq'"]);

    let mut s = ElaborateSession::new("Irq");
    bitloom_prelude::ip::CsrDecoder::define_module(&mut s, "BitloomIrqCsr").unwrap();
    let err = Irq::define_module(&mut s, "Irq").unwrap_err();
    assert_diag(
        &err,
        "rhdl::E0244",
        &["module 'BitloomIrqCsr'", "conflicts"],
    );
    assert_diag(
        &s.finish().unwrap_err(),
        "rhdl::E0244",
        &["module 'BitloomIrqCsr'"],
    );
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
    let mut s = ElaborateSession::new("TwoIrqs");
    Irq::define_module(&mut s, "SharedIrq")?;
    Irq::define_module(&mut s, "SharedIrq")?;
    if !same_definition {
        Irq::define_module(&mut s, "OtherIrq")?;
    }
    let sp = Span::default();
    s.begin_module("TwoIrqs", sp);
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
                    || (lane == 0 && bad == Some("direction") && name == "irq")
                {
                    s.add_input(&net, ty, sp);
                } else {
                    s.add_output(&net, ty, sp);
                }
            }
            connects.push((name, net));
        }
        s.add_instance(
            format!("irq{lane}"),
            if lane == 0 || same_definition {
                "SharedIrq"
            } else {
                "OtherIrq"
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
fn p0_shared_irq_wiring_rejects_width_clock_type_and_output_target_conflicts() {
    for (bad, code, port, net) in [
        ("width", "rhdl::E0203", "irq0.addr", "t0_addr"),
        ("type", "rhdl::E0256", "irq0.clk", "bad_clk"),
        ("direction", "rhdl::E0257", "irq0.irq", "t0_irq"),
    ] {
        let err = pair(Some(bad)).expect_err("invalid Irq connection accepted");
        assert_diag(&err, code, &[port, net]);
    }
    let hir = pair(None).unwrap();
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "BitloomIrqCsr")
            .count(),
        1
    );
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "OtherIrq")
            .count(),
        1
    );
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "SharedIrq")
            .count(),
        1
    );
}

#[test]
fn p1_hierarchy_native_and_generated_reject_explicitly() {
    use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
    for hir in [
        Irq::elaborate().unwrap(),
        pair(None).unwrap(),
        pair_kind(None, true).unwrap(),
        timer_irq().unwrap(),
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

fn timer_irq() -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    let mut s = ElaborateSession::new("TimerIrq");
    Timer::define_module(&mut s, "ActualTimer")?;
    Irq::define_module(&mut s, "ActualIrq")?;
    let sp = Span::default();
    s.begin_module("TimerIrq", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut p = ports();
        if lane == 0 {
            p.remove("raw_events");
            p.remove("irq");
            p.insert(
                "match_event".into(),
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
            if lane == 0 {
                "ActualTimer"
            } else {
                "ActualIrq"
            },
            connections,
            vec![],
            sp,
        );
    }
    for name in ["zero5", "timer_bit"] {
        s.declare_wire(name, GroundType::UInt { width: 5 }, sp);
    }
    s.begin_combinational(sp);
    s.assign_lit("zero5", 0, sp);
    s.assign_lit("timer_bit", 1, sp);
    s.assign_mux("t1_raw_events", "t0_match_event", "timer_bit", "zero5", sp);
    s.end_process();
    s.end_module();
    s.finish()
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

fn pair_tb(hir: &FrozenHir) -> String {
    let mut tb = tb_prefix(hir, "TwoIrqs");
    tb.push_str(r#"
initial begin
 $dumpfile("trace.vcd"); $dumpvars(0,tb);
 rst=1; tick; rst=0;
 access0(1,4,1,0); access1(1,4,16,0);
 // Different hardware sources and different software requests on one edge.
 t0_raw_events=1; t1_raw_events=16;
 t0_req_valid=1; t0_write=1; t0_addr=8; t0_wdata=2; t0_wstrb=1;
 t1_req_valid=1; t1_write=0; t1_addr=12;
 #1; if(t0_req_ready !== 1 || t1_req_ready !== 1) $fatal(1,"simultaneous readiness");
 tick; t0_req_valid=0; t1_req_valid=0; t0_raw_events=0; t1_raw_events=0;
 if(t0_rsp_valid !== 1 || t1_rsp_valid !== 1 || t0_error !== 0 || t1_error !== 0 || t1_rdata !== 16) $fatal(1,"simultaneous transactions");
 if(t0_irq !== 1 || t1_irq !== 1) $fatal(1,"initial independent events");
 t0_rsp_ready=1; tick; t0_rsp_ready=0;
 // Lane1's RAW snapshot is held while lane0 changes its independent state.
 access0(0,0,0,3); access0(1,0,3,0); access0(0,0,0,0);
 if(t0_irq !== 0 || t1_irq !== 1) $fatal(1,"pending cross-lane contamination");
 t1_raw_events=4; tick; t1_raw_events=0;
 repeat(5) begin
  if(t1_rsp_valid !== 1 || t1_req_ready !== 0 || t1_rdata !== 16 || t1_error !== 0) $fatal(1,"asymmetric stalled snapshot");
  tick;
 end
 t1_rsp_ready=1; tick; t1_rsp_ready=0;
 access1(0,0,0,20); access1(0,12,0,0);
 access0(0,4,0,1); access1(0,4,0,16);
 // Each local clear affects only its own owner, not the shared definition.
 access1(1,0,16,0); access1(0,0,0,4);
 if(t1_irq !== 0) $fatal(1,"disabled remaining pending should not interrupt");
 access1(1,4,4,0); if(t1_irq !== 1) $fatal(1,"reenable lost pending");
 access0(1,8,1,0); if(t0_irq !== 1) $fatal(1,"lane0 TEST");
 // Both stalled responses and active events are cancelled by shared reset.
 t0_req_valid=1; t0_write=0; t0_addr=0;
 t1_req_valid=1; t1_write=0; t1_addr=0; tick;
 t0_req_valid=0; t1_req_valid=0; t0_raw_events=31; t1_raw_events=31;
 rst=1; tick; t0_raw_events=0; t1_raw_events=0; rst=0; #1;
 if(t0_rsp_valid !== 0 || t1_rsp_valid !== 0 || t0_irq !== 0 || t1_irq !== 0) $fatal(1,"shared reset cancellation");
 access0(0,0,0,0); access1(0,0,0,0); access0(0,4,0,0); access1(0,4,0,0);
 $display("PASS IRQ composition independent state"); $finish;
end
initial begin #100000; $fatal(1,"watchdog"); end
endmodule
"#);
    tb
}

fn timer_irq_tb(hir: &FrozenHir) -> String {
    let mut tb = tb_prefix(hir, "TimerIrq");
    tb.push_str(r#"
integer match_count=0;
always @(posedge clk) if(!rst && t0_match_event) match_count=match_count+1;
initial begin
 $dumpfile("trace.vcd"); $dumpvars(0,tb);
 rst=1; tick; rst=0;
 access1(1,4,1,0);
 access0(1,8,4,0); access0(1,0,1,0); // Actual one-shot Timer, compare=4.
 repeat(6) tick;
 if(match_count !== 1 || t1_irq !== 1) $fatal(1,"first real match did not reach IRQ");
 access0(0,12,0,1); access0(0,4,0,4); access1(0,0,0,1);
 access1(1,0,1,0); // Clear only IRQ while Timer EVENT remains sticky.
 repeat(12) begin tick; if(t1_irq !== 0) $fatal(1,"sticky Timer EVENT retriggered IRQ"); end
 access1(0,0,0,0); access1(0,12,0,0); access0(0,12,0,1);
 // Rearm the real Timer without clearing its local EVENT.
 access0(1,4,0,0); access0(1,0,1,0); repeat(6) tick;
 if(match_count !== 2 || t1_irq !== 1) $fatal(1,"second raw match lost");
 access0(0,12,0,1); access1(0,0,0,1);
 // Reset both modules while both have an outstanding read response.
 t0_req_valid=1; t0_write=0; t0_addr=12;
 t1_req_valid=1; t1_write=0; t1_addr=0; tick;
 t0_req_valid=0; t1_req_valid=0; rst=1; tick; rst=0; #1;
 if(t0_rsp_valid !== 0 || t1_rsp_valid !== 0 || t0_match_event !== 0 || t1_irq !== 0) $fatal(1,"Timer IRQ reset cancellation");
 access0(0,12,0,0); access1(0,0,0,0); access1(0,4,0,0);
 $display("PASS IRQ composition actual Timer raw event"); $finish;
end
initial begin #100000; $fatal(1,"watchdog"); end
endmodule
"#);
    tb
}

#[test]
fn p0_actual_two_irq_rtl_isolation_shared_and_renamed_definitions() {
    for same in [true, false] {
        let hir = pair_kind(None, same).unwrap();
        execute_composition(
            &hir,
            "TwoIrqs",
            &pair_tb(&hir),
            &format!("pair-{same}"),
            false,
        );
    }
}
#[test]
fn p0_actual_timer_raw_event_does_not_retrigger_from_local_sticky_event() {
    let hir = timer_irq().unwrap();
    execute_composition(&hir, "TimerIrq", &timer_irq_tb(&hir), "timer-irq", false);
}
#[test]
#[ignore = "dedicated pinned firtool and JVM gate"]
fn p1_composition_firrtl_chisel_shared_renamed_and_actual_timer() {
    for same in [true, false] {
        let hir = pair_kind(None, same).unwrap();
        execute_composition(
            &hir,
            "TwoIrqs",
            &pair_tb(&hir),
            &format!("pair-{same}"),
            true,
        );
    }
    let hir = timer_irq().unwrap();
    execute_composition(&hir, "TimerIrq", &timer_irq_tb(&hir), "timer-irq", true);
}

fn execute_composition(hir: &FrozenHir, top_name: &str, tb: &str, label: &str, backends: bool) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-irq-api")
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
    assert!(run(&dir, "vvp", &["sim"], "simulate").contains("PASS IRQ composition"));
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
    assert!(run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("PASS IRQ composition"));
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
    assert!(run(&chisel, "vvp", &["simulation"], "simulate").contains("PASS IRQ composition"));
    println!(
        "IRQ composition FIRRTL and JVM actual RTL PASS {}",
        dir.display()
    );
}

//! Story128.2 independent public surface and real two-instance Timer acceptance.
//! Active ATDD: missing Timer API is compile RED; behavior is not claimed run.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{CsrAccess, CsrOwner, Timer},
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
        ("req_ready", 1, Output),
        ("rsp_valid", 1, Output),
        ("rdata", 32, Output),
        ("error", 2, Output),
        ("match_event", 1, Output),
    ] {
        p.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    p
}

#[test]
fn p0_exact_thirteen_ports_and_shared_standalone_equivalence() {
    let hir = Timer::elaborate().unwrap();
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Timer")
        .unwrap();
    assert_eq!(top.ports.len(), 13);
    let actual: BTreeMap<_, _> = top
        .ports
        .iter()
        .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
        .collect();
    assert_eq!(actual, ports());
    let mut s = ElaborateSession::new("Timer");
    assert_eq!(
        Timer::define_module(&mut s, String::from("Timer")).unwrap(),
        "Timer"
    );
    Timer::define_module(&mut s, "Timer").unwrap();
    let shared = s.finish().unwrap();
    assert_eq!(
        hir, shared,
        "same body and no duplicate private definitions"
    );
}

#[test]
fn p0_register_descriptor_has_exact_local_map_masks_and_unique_owners() {
    use CsrAccess::{Rw, W1c};
    use CsrOwner::{External, Leaf};
    let block = Timer::registers();
    block.validate().unwrap();
    assert_eq!(block.name, "Timer");
    assert_eq!(block.registers.len(), 4);
    for (name, offset, mask, access, owner, event) in [
        ("ctrl", 0, 3, Rw, External, None),
        ("count", 4, 0xffff_ffff, Rw, External, None),
        ("compare", 8, 0xffff_ffff, Rw, Leaf, None),
        ("EVENT", 12, 1, W1c, Leaf, Some("match_bits")),
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
        assert_eq!(
            (f.name.as_str(), f.mask, f.reset, f.access),
            ("bits", mask, 0, access)
        );
    }
}

#[test]
fn p1_software_outputs_match_handwritten_local_address_golden() {
    let block = Timer::registers();
    let c = block.emit_c_header().unwrap();
    let md = block.emit_markdown().unwrap();
    for line in [
        "#define TIMER_CTRL_OFFSET UINT32_C(0x00000000)",
        "#define TIMER_COUNT_OFFSET UINT32_C(0x00000004)",
        "#define TIMER_COMPARE_OFFSET UINT32_C(0x00000008)",
        "#define TIMER_EVENT_OFFSET UINT32_C(0x0000000c)",
        "#define TIMER_CTRL_MASK UINT32_C(0x00000003)",
        "#define TIMER_COUNT_MASK UINT32_C(0xffffffff)",
        "#define TIMER_COMPARE_MASK UINT32_C(0xffffffff)",
        "#define TIMER_EVENT_MASK UINT32_C(0x00000001)",
        "#define TIMER_EVENT_ACCESS \"W1C\"",
    ] {
        assert!(c.lines().any(|l| l == line), "missing {line}");
    }
    for line in [
        "| ctrl | 0x0000 | 0x00000003 | 0x00000000 | RW | External | — | false | false |",
        "| count | 0x0004 | 0xffffffff | 0x00000000 | RW | External | — | false | false |",
        "| compare | 0x0008 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | false |",
        "| EVENT | 0x000c | 0x00000001 | 0x00000000 | W1C | Leaf | match_bits | false | false |",
    ] {
        assert!(md.lines().any(|l| l == line), "missing {line}");
    }
    assert!(md.contains("Local byte offsets; caller supplies the base address."));
    assert!(!c.contains("UINT32_C(0x00000200)"));
    assert!(!md.contains("| 0x0200 |"));
    assert_eq!(c, Timer::registers().emit_c_header().unwrap());
    assert_eq!(md, Timer::registers().emit_markdown().unwrap());
    assert_eq!(c, include_str!("../../../docs/ip/timer-registers.h"));
    assert_eq!(md, include_str!("../../../docs/ip/timer-registers.md"));
}

#[test]
fn p0_invalid_names_and_conflicting_bodies_are_rejected() {
    for name in ["", "not a module", "module"] {
        let mut s = ElaborateSession::new("Top");
        let err = Timer::define_module(&mut s, name).unwrap_err();
        assert_diag(&err, "rhdl::E0241", &["module name", "identifier"]);
        assert_diag(&s.finish().unwrap_err(), "rhdl::E0241", &["module name"]);
    }
    let mut s = ElaborateSession::new("Timer");
    Timer::define_module(&mut s, "Timer").unwrap();
    let err = bitloom_prelude::ip::CsrDecoder::define_module(&mut s, "Timer").unwrap_err();
    assert_diag(&err, "rhdl::E0244", &["module 'Timer'", "conflicts"]);
    assert_diag(&s.finish().unwrap_err(), "rhdl::E0244", &["module 'Timer'"]);

    let mut s = ElaborateSession::new("Timer");
    bitloom_prelude::ip::CsrDecoder::define_module(&mut s, "BitloomTimerCsr").unwrap();
    let err = Timer::define_module(&mut s, "Timer").unwrap_err();
    assert_diag(
        &err,
        "rhdl::E0244",
        &["module 'BitloomTimerCsr'", "conflicts"],
    );
    assert_diag(
        &s.finish().unwrap_err(),
        "rhdl::E0244",
        &["module 'BitloomTimerCsr'"],
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
    let mut s = ElaborateSession::new("TwoTimers");
    Timer::define_module(&mut s, "SharedTimer")?;
    Timer::define_module(&mut s, "SharedTimer")?;
    if !same_definition {
        Timer::define_module(&mut s, "OtherTimer")?;
    }
    let sp = Span::default();
    s.begin_module("TwoTimers", sp);
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
                    || (lane == 0 && bad == Some("direction") && name == "match_event")
                {
                    s.add_input(&net, ty, sp);
                } else {
                    s.add_output(&net, ty, sp);
                }
            }
            connects.push((name, net));
        }
        s.add_instance(
            format!("timer{lane}"),
            if lane == 0 || same_definition {
                "SharedTimer"
            } else {
                "OtherTimer"
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
fn p0_shared_timer_wiring_rejects_width_clock_type_and_output_target_conflicts() {
    for (bad, code, port, net) in [
        ("width", "rhdl::E0203", "timer0.addr", "t0_addr"),
        ("type", "rhdl::E0256", "timer0.clk", "bad_clk"),
        (
            "direction",
            "rhdl::E0257",
            "timer0.match_event",
            "t0_match_event",
        ),
    ] {
        let err = pair(Some(bad)).expect_err("invalid Timer connection accepted");
        assert_diag(&err, code, &[port, net]);
    }
    let hir = pair(None).unwrap();
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "BitloomTimerCsr")
            .count(),
        1
    );
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "OtherTimer")
            .count(),
        1
    );
    assert_eq!(
        hir.circuit()
            .modules
            .iter()
            .filter(|m| m.name == "SharedTimer")
            .count(),
        1
    );
}

#[test]
fn p1_hierarchy_native_and_generated_reject_explicitly() {
    use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
    for hir in [Timer::elaborate().unwrap(), pair(None).unwrap()] {
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
        serde_json::json!({"tool":tool,"args":args,"exit_code":status.code()}).to_string(),
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

#[test]
fn p0_actual_two_timer_rtl_has_independent_state_and_match_events() {
    for same_definition in [false, true] {
        actual_pair(same_definition, false);
    }
}

#[test]
#[ignore = "dedicated pinned firtool/JVM gate; run --ignored"]
fn p1_two_timer_firrtl_chisel_shared_and_renamed_definitions() {
    for same_definition in [false, true] {
        actual_pair(same_definition, true);
    }
}

fn actual_pair(same_definition: bool, backends: bool) {
    use std::fmt::Write as _;
    let hir = pair_kind(None, same_definition).unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-timer-api")
        .join(format!(
            "two-timers-{same_definition}-{backends}-{}",
            std::process::id()
        ));
    fs::create_dir_all(&dir).unwrap();
    let rtl = bitloom_vlog::emit(&hir)
        .files
        .into_iter()
        .map(|f| f.contents)
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("dut.v"), rtl).unwrap();
    let mut tb = String::from("module tb; reg clk=0, rst=0;\n");
    for lane in 0..2 {
        for (name, (d, ty)) in ports() {
            if name == "clk" || name == "rst" {
                continue;
            }
            let GroundType::UInt { width } = ty else {
                panic!("unexpected port type")
            };
            writeln!(
                tb,
                "{} [{}:0] t{lane}_{name}{};",
                if d == PortDirection::Input {
                    "reg"
                } else {
                    "wire"
                },
                width - 1,
                if d == PortDirection::Input { "=0" } else { "" }
            )
            .unwrap();
        }
    }
    tb.push_str("TwoTimers dut(.*); integer hits0=0,hits1=0;\n");
    tb.push_str("always @(posedge clk) if (!rst) begin if(t0_match_event) hits0=hits0+1; if(t1_match_event) hits1=hits1+1; end\n");
    tb.push_str("task tick; begin #5; clk=1; #5; clk=0; #1; end endtask\n");
    for lane in 0..2 {
        writeln!(tb, r#"task access{lane}(input wr,input [15:0] a,input [31:0] data,input [31:0] expected);
begin
 t{lane}_req_valid=1; t{lane}_write=wr; t{lane}_addr=a; t{lane}_wdata=data; t{lane}_wstrb=15; #1;
 if(t{lane}_req_ready !== 1) $fatal(1,"lane {lane} not ready");
 tick; t{lane}_req_valid=0;
 if(t{lane}_rsp_valid !== 1 || t{lane}_error !== 0) $fatal(1,"lane {lane} response");
 if(!wr && t{lane}_rdata !== expected) $fatal(1,"lane {lane} addr %h expected %h got %h",a,expected,t{lane}_rdata);
 t{lane}_rsp_ready=1; tick; t{lane}_rsp_ready=0;
end endtask"#).unwrap();
    }
    tb.push_str(
        r#"
initial begin
 $dumpfile("two-timers.vcd"); $dumpvars(0,tb);
 rst=1; tick; rst=0;
 // Both Timer instances accept on the same edge. Lane0
 // holds its read snapshot while lane1 consumes and completes another request.
 access0(1,4,32'h11223344,0); access1(1,4,32'h55667788,0);
 t0_req_valid=1; t1_req_valid=1; t0_write=0; t1_write=0;
 t0_addr=4; t1_addr=4; #1;
 if(t0_req_ready !== 1 || t1_req_ready !== 1) $fatal(1,"simultaneous ready");
 tick; t0_req_valid=0; t1_req_valid=0;
 if(t0_rsp_valid !== 1 || t1_rsp_valid !== 1 || t0_rdata !== 32'h11223344 || t1_rdata !== 32'h55667788) $fatal(1,"simultaneous snapshots");
 t1_rsp_ready=1; tick; t1_rsp_ready=0;
 access1(1,4,32'habcdef01,0); access1(0,4,0,32'habcdef01);
 repeat(7) begin
   if(t0_req_ready !== 0 || t0_rsp_valid !== 1 || t0_rdata !== 32'h11223344 || t0_error !== 0) $fatal(1,"lane0 stalled snapshot");
   tick;
 end
 t0_rsp_ready=1; tick; t0_rsp_ready=0;
 access0(0,4,0,32'h11223344);
 access0(1,4,32'h87654321,0); access1(1,4,32'h12345678,0);
 access0(0,4,0,32'h87654321); access1(0,4,0,32'h12345678);
 access0(1,4,0,0); access0(1,8,1,0); access0(1,0,3,0);
 repeat(8) tick;
 if(hits0 < 8 || hits1 != 0) $fatal(1,"event isolation %d %d",hits0,hits1);
 access1(0,4,0,32'h12345678); access1(0,12,0,0);
 access1(1,4,0,0); access1(1,8,2,0); access1(1,0,1,0);
 repeat(8) tick;
 if(hits1 != 1) $fatal(1,"one-shot isolation %d",hits1);
 access1(0,4,0,2); access1(0,0,0,0); access1(0,12,0,1);
 access0(0,12,0,1);
 rst=1; tick; rst=0;
 access0(0,4,0,0); access1(0,4,0,0);
 access0(0,12,0,0); access1(0,12,0,0);
 if(t0_match_event !== 0 || t1_match_event !== 0) $fatal(1,"reset event leak");
 $display("PASS two Timer instances independent state and raw events"); $finish;
end
initial begin #100000; $fatal(1,"watchdog"); end
endmodule
"#,
    );
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    if backends {
        pair_backends(&hir, &tb, &dir);
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
    assert!(
        run(&dir, "vvp", &["sim"], "simulate")
            .contains("PASS two Timer instances independent state and raw events")
    );
}

fn pair_backends(hir: &FrozenHir, tb: &str, dir: &Path) {
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
    assert!(
        run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("PASS two Timer instances")
    );
    fs::rename(dir.join("two-timers.vcd"), dir.join("firrtl.vcd")).unwrap();
    fs::create_dir_all(dir.join("src/main/scala")).unwrap();
    fs::create_dir_all(dir.join("project")).unwrap();
    fs::write(
        dir.join("src/main/scala/Design.scala"),
        &bitloom_firrtl::emit_chisel(hir).unwrap().files[0].contents,
    )
    .unwrap();
    fs::write(dir.join("src/main/scala/Main.scala"), "object PairMain extends App { circt.stage.ChiselStage.emitSystemVerilogFile(new TwoTimers, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }\n").unwrap();
    fs::write(dir.join("build.sbt"), "scalaVersion := \"2.13.18\"\nlibraryDependencies += \"org.chipsalliance\" %% \"chisel\" % \"7.15.0\"\naddCompilerPlugin(\"org.chipsalliance\" % \"chisel-plugin\" % \"7.15.0\" cross CrossVersion.full)\n").unwrap();
    fs::write(
        dir.join("project/build.properties"),
        "sbt.version=1.10.11\n",
    )
    .unwrap();
    let log = fs::File::create(dir.join("sbt.log")).unwrap();
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
    assert!(status.success(), "JVM pair failed: {}", dir.display());
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "TwoTimers")
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
    assert!(run(&chisel, "vvp", &["simulation"], "simulate").contains("PASS two Timer instances"));
    println!("TwoTimers FIRRTL and JVM actual RTL PASS {}", dir.display());
}

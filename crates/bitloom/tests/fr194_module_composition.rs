//! FR194：单 session 的真实模块层级，独立向量核验状态隔离。
use bitloom_prelude::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span, ip::Gpio,
};
use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
use std::{
    fmt::Write as _,
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};

fn register_body(s: &mut ElaborateSession, params: &[(String, u32)]) -> Result<(), Diagnostics> {
    let width = params.iter().find(|(key, _)| key == "WIDTH").unwrap().1;
    let span = Span::default();
    s.add_input("clk", GroundType::Clock, span);
    s.add_input("rst", GroundType::Reset, span);
    s.add_input("enable", GroundType::UInt { width: 1 }, span);
    s.add_input("data", GroundType::UInt { width }, span);
    s.add_output("out", GroundType::UInt { width }, span);
    s.declare_reg("state", GroundType::UInt { width }, span);
    s.begin_combinational(span);
    s.assign_net("out", "state", span);
    s.end_process();
    s.begin_sequential(span);
    s.assign_reg_d_mux("state", "enable", "data", "state", span);
    s.end_process();
    Ok(())
}

fn wrapper_body(s: &mut ElaborateSession, params: &[(String, u32)]) -> Result<(), Diagnostics> {
    let width = params.iter().find(|(key, _)| key == "WIDTH").unwrap().1;
    let span = Span::default();
    s.add_input("clk", GroundType::Clock, span);
    s.add_input("rst", GroundType::Reset, span);
    s.add_input("enable", GroundType::UInt { width: 1 }, span);
    s.add_input("data", GroundType::UInt { width }, span);
    s.add_output("out", GroundType::UInt { width }, span);
    s.add_instance(
        "leaf",
        format!("Register{width}"),
        ["clk", "rst", "enable", "data", "out"]
            .into_iter()
            .map(|p| (p.into(), p.into()))
            .collect(),
        vec![],
        span,
    );
    Ok(())
}

fn registers(top_first: bool, wrapped: bool) -> FrozenHir {
    fn children(s: &mut ElaborateSession, wrapped: bool) {
        for width in [8, 8, 16] {
            assert_eq!(
                s.define_module(
                    format!("Register{width}"),
                    vec![("WIDTH".into(), width)],
                    register_body,
                )
                .unwrap(),
                format!("Register{width}")
            );
            if wrapped {
                s.define_module(
                    format!("Wrapper{width}"),
                    vec![("WIDTH".into(), width)],
                    wrapper_body,
                )
                .unwrap();
            }
        }
    }
    fn top(s: &mut ElaborateSession, wrapped: bool) {
        let span = Span::default();
        s.begin_module("Registers", span);
        s.add_input("clk", GroundType::Clock, span);
        s.add_input("rst", GroundType::Reset, span);
        for (lane, width) in [8, 8, 16].into_iter().enumerate() {
            s.add_input(format!("enable{lane}"), GroundType::UInt { width: 1 }, span);
            s.add_input(format!("data{lane}"), GroundType::UInt { width }, span);
            s.add_output(format!("out{lane}"), GroundType::UInt { width }, span);
            s.add_instance(
                format!("lane{lane}"),
                format!("{}{width}", if wrapped { "Wrapper" } else { "Register" }),
                vec![
                    ("clk".into(), "clk".into()),
                    ("rst".into(), "rst".into()),
                    ("enable".into(), format!("enable{lane}")),
                    ("data".into(), format!("data{lane}")),
                    ("out".into(), format!("out{lane}")),
                ],
                vec![],
                span,
            );
        }
        s.end_module();
    }
    let mut s = ElaborateSession::new("Registers");
    if top_first {
        top(&mut s, wrapped);
        children(&mut s, wrapped);
    } else {
        children(&mut s, wrapped);
        top(&mut s, wrapped);
    }
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), if wrapped { 5 } else { 3 });
    hir
}

fn gpio_pair() -> FrozenHir {
    let mut s = ElaborateSession::new("TwoGpio");
    let name = Gpio::define_module(&mut s, "Gpio8").unwrap();
    assert_eq!(name, Gpio::define_module(&mut s, "Gpio8").unwrap());
    let span = Span::default();
    s.begin_module("TwoGpio", span);
    s.add_input("clk", GroundType::Clock, span);
    s.add_input("rst", GroundType::Reset, span);
    for lane in 0..2 {
        let mut connects = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (port, width) in [
            ("dir", 8),
            ("wr_en", 1),
            ("wr_data", 8),
            ("wr_mask", 8),
            ("pad_in", 8),
        ] {
            let net = format!("g{lane}_{port}");
            s.add_input(net.clone(), GroundType::UInt { width }, span);
            connects.push((port.into(), net));
        }
        for port in ["pad_out", "rd_data"] {
            let net = format!("g{lane}_{port}");
            s.add_output(net.clone(), GroundType::UInt { width: 8 }, span);
            connects.push((port.into(), net));
        }
        s.add_instance(format!("gpio{lane}"), name.clone(), connects, vec![], span);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    hir
}

fn next(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}

fn check(tb: &mut String, signal: &str, expected: u32, frame: usize, phase: &str) {
    writeln!(tb, "if ({signal} !== 32'd{expected}) $fatal(1, \"FR194 frame={frame} {phase} {signal} expected={expected} actual=%h\", {signal});").unwrap();
}

fn bounded(command: &mut Command, log_path: &Path, timeout: Duration) -> io::Result<ExitStatus> {
    let log = fs::File::create(log_path)?;
    command
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log));
    let mut child = command.spawn()?;
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            writeln!(
                fs::OpenOptions::new().append(true).open(log_path)?,
                "status={status}"
            )?;
            return Ok(status);
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            writeln!(
                fs::OpenOptions::new().append(true).open(log_path)?,
                "TIMEOUT after {timeout:?}"
            )?;
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!("tool timed out; log {}", log_path.display()),
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn run_rtl(label: &str, hir: &FrozenHir, tb: &str) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr194")
        .join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let emitted = bitloom_vlog::emit(hir);
    let design = emitted
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), design).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    let timeout = Duration::from_secs(60);
    let mut commands = String::new();
    for tool in ["iverilog", "vvp"] {
        let path = dir.join(format!("{tool}-version.log"));
        let status = bounded(Command::new(tool).arg("-V"), &path, timeout)
            .expect("FR194 RTL tool required and version probe must complete within 60s");
        assert!(
            status.success(),
            "{tool} version probe failed; {}",
            path.display()
        );
        writeln!(commands, "{tool} -V\n{}", fs::read_to_string(path).unwrap()).unwrap();
    }
    commands.push_str("iverilog -g2012 -s tb -o simulation design.v tb.sv\nvvp simulation\n");
    fs::write(dir.join("commands.log"), commands).unwrap();
    for (tool, args, filename) in [
        (
            "iverilog",
            vec![
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
            "compile.log",
        ),
        ("vvp", vec!["simulation"], "run.log"),
    ] {
        let path = dir.join(filename);
        let status = bounded(
            Command::new(tool).args(args).current_dir(&dir),
            &path,
            timeout,
        )
        .expect("FR194 RTL tool must exist and finish within 60s; artifacts preserved");
        let log = fs::read_to_string(&path).unwrap();
        assert!(
            status.success(),
            "FR194 {label}: {tool} failed; artifacts {}\n{log}",
            dir.display()
        );
        if tool == "vvp" {
            assert!(
                log.contains("FR194 PASS"),
                "simulation did not reach success marker"
            );
            println!("{label}: {log}artifacts={}", dir.display());
        }
    }
}

#[test]
fn stalled_tool_times_out_and_preserves_log() {
    let dir = std::env::temp_dir().join(format!("fr194-timeout-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let log = dir.join("timeout.log");
    let error = bounded(
        Command::new("sh").args(["-c", "echo started; exec sleep 10"]),
        &log,
        Duration::from_millis(100),
    )
    .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    let text = fs::read_to_string(log).unwrap();
    assert!(text.contains("TIMEOUT"));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rtl_same_and_different_parameters_preserve_instance_state_for_both_module_orders() {
    for (top_first, wrapped) in [(true, false), (false, false), (true, true)] {
        let hir = registers(top_first, wrapped);
        let mut tb = String::from(
            "module tb; reg clk=0,rst=0; reg enable0=0,enable1=0,enable2=0; reg[7:0] data0=0,data1=0;reg[15:0] data2=0;wire[7:0] out0,out1;wire[15:0] out2;Registers dut(.*);initial begin\n",
        );
        let mut seed = 0x1948_0816;
        let mut state = [0u32; 3];
        for frame in 0..192 {
            let reset = frame == 0 || frame == 95 || frame == 96;
            writeln!(tb, "rst={};", u32::from(reset)).unwrap();
            let mut enables = [false; 3];
            let mut data = [0; 3];
            for lane in 0..3 {
                enables[lane] = frame % (lane + 2) != 0;
                data[lane] = next(&mut seed) & if lane == 2 { 0xffff } else { 0xff };
                writeln!(
                    tb,
                    "enable{lane}={};data{lane}={};",
                    u32::from(enables[lane]),
                    data[lane]
                )
                .unwrap();
            }
            tb.push_str("#5;\n");
            if frame > 0 {
                for (lane, value) in state.iter().enumerate() {
                    check(&mut tb, &format!("out{lane}"), *value, frame, "pre");
                }
            }
            tb.push_str("clk=1;#5;\n");
            for lane in 0..3 {
                if reset {
                    state[lane] = 0;
                } else if enables[lane] {
                    state[lane] = data[lane];
                }
                check(&mut tb, &format!("out{lane}"), state[lane], frame, "post");
            }
            tb.push_str("clk=0;#5;\n");
        }
        tb.push_str(
            "$display(\"FR194 PASS registers cycles=192 seed=0x19480816\");$finish;end endmodule\n",
        );
        run_rtl(
            if wrapped {
                "registers-two-levels"
            } else if top_first {
                "registers-top-first"
            } else {
                "registers-child-first"
            },
            &hir,
            &tb,
        );
    }
}

#[test]
fn rtl_two_gpio_banks_keep_masked_writes_and_pad_reads_independent() {
    let hir = gpio_pair();
    let mut tb = String::from("module tb;reg clk=0,rst=0;\n");
    for lane in 0..2 {
        writeln!(tb, "reg[7:0] g{lane}_dir=0,g{lane}_wr_data=0,g{lane}_wr_mask=0,g{lane}_pad_in=0;reg g{lane}_wr_en=0;wire[7:0] g{lane}_pad_out,g{lane}_rd_data;").unwrap();
    }
    tb.push_str("TwoGpio dut(.*);initial begin\n");
    let mut seed = 0x1946_9100;
    let mut state = [[false; 8]; 2];
    for frame in 0..160 {
        let reset = frame == 0 || frame == 79;
        let mut dirs = [0; 2];
        let mut pads = [0; 2];
        let mut masks = [0; 2];
        let mut data = [0; 2];
        let mut enables = [false; 2];
        writeln!(tb, "rst={};", u32::from(reset)).unwrap();
        for lane in 0..2 {
            dirs[lane] = next(&mut seed) & 0xff;
            pads[lane] = next(&mut seed) & 0xff;
            masks[lane] = match frame % 4 {
                0 => 0,
                1 => 0xff,
                _ => next(&mut seed) & 0xff,
            };
            data[lane] = next(&mut seed) & 0xff;
            enables[lane] = (frame + lane) % 3 != 0;
            writeln!(tb, "g{lane}_dir={};g{lane}_pad_in={};g{lane}_wr_mask={};g{lane}_wr_data={};g{lane}_wr_en={};", dirs[lane], pads[lane], masks[lane], data[lane], u32::from(enables[lane])).unwrap();
        }
        for phase in ["pre", "post"] {
            if phase == "post" {
                tb.push_str("clk=1;#5;\n");
                // 独立按位参考：仅被选择的位接受写入，复位优先。
                for lane in 0..2 {
                    for bit in 0..8 {
                        if reset {
                            state[lane][bit] = false;
                        } else if enables[lane] && masks[lane] & (1 << bit) != 0 {
                            state[lane][bit] = data[lane] & (1 << bit) != 0;
                        }
                    }
                }
            } else {
                tb.push_str("#5;\n");
            }
            if phase == "pre" && frame == 0 {
                continue;
            }
            for lane in 0..2 {
                let mut pad_out = 0;
                let mut read = 0;
                for bit in 0..8 {
                    let is_output = dirs[lane] & (1 << bit) != 0;
                    if is_output && state[lane][bit] {
                        pad_out |= 1 << bit;
                    }
                    if if is_output {
                        state[lane][bit]
                    } else {
                        pads[lane] & (1 << bit) != 0
                    } {
                        read |= 1 << bit;
                    }
                }
                check(&mut tb, &format!("g{lane}_pad_out"), pad_out, frame, phase);
                check(&mut tb, &format!("g{lane}_rd_data"), read, frame, phase);
            }
        }
        tb.push_str("clk=0;#5;\n");
    }
    tb.push_str("$display(\"FR194 PASS gpio cycles=160 seed=0x19469100\");$finish;end endmodule\n");
    run_rtl("gpio-pair", &hir, &tb);
}

#[test]
fn gpio_legacy_entry_matches_shared_definition_and_preserves_port_contract() {
    let legacy = Gpio::elaborate().unwrap();
    let mut s = ElaborateSession::new("Gpio");
    Gpio::define_module(&mut s, "Gpio").unwrap();
    assert_eq!(legacy, s.finish().unwrap());
    let ports: Vec<_> = legacy.circuit().modules[0]
        .ports
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    assert_eq!(
        ports,
        [
            "clk", "rst", "dir", "wr_en", "wr_data", "wr_mask", "pad_in", "pad_out", "rd_data"
        ]
    );
}

#[test]
fn hierarchical_native_models_keep_explicit_unsupported_boundary() {
    for hir in [
        registers(true, false),
        registers(false, false),
        registers(true, true),
        gpio_pair(),
    ] {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            assert_hierarchy_error(
                std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine))
                    .err()
                    .expect("hierarchy must reject"),
            );
        }
        assert_hierarchy_error(
            std::panic::catch_unwind(|| GeneratedFunctional::from_hir(&hir))
                .err()
                .expect("hierarchy must reject"),
        );
    }
}

fn assert_hierarchy_error(payload: Box<dyn std::any::Any + Send>) {
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

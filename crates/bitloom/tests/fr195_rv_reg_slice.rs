//! Story 126.3 / FR195: active ATDD acceptance tests; no tool-presence skips.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span, ip::RvRegSlice,
};
use bitloom_sim::{Sim, TickEngine};
use std::{
    collections::VecDeque,
    fmt::Write as _,
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    time::Duration,
};

const SEEDS: [u64; 3] = [0x1263_1950_a551, 0xdead_beef_8012, 0x7359_2401_ffff];
#[derive(Clone, Copy, Debug, Default)]
struct Input {
    rst: bool,
    flush: bool,
    valid: bool,
    data: u64,
    ready: bool,
}
#[derive(Clone, Copy, Debug)]
struct Output {
    ready: bool,
    data: Option<u64>,
}
#[derive(Clone, Debug)]
struct Frame {
    input: Input,
    before: Output,
    after: Output,
}
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn mask(width: u32) -> u64 {
    u64::MAX >> (64 - width)
}
fn output(q: &VecDeque<u64>) -> Output {
    Output {
        ready: q.len() < 2,
        data: q.front().copied(),
    }
}

// This queue is an independent transaction model, with no knowledge of RTL registers.
fn trace(width: u32, initial_seed: u64) -> Vec<Frame> {
    let mut seed = initial_seed;
    let mut q = VecDeque::new();
    let mut frames = Vec::new();
    let (mut accepted, mut delivered, mut cancelled) = (0usize, 0usize, 0usize);
    let (mut epoch_in, mut epoch_out) = (0usize, 0usize);
    let mut pending = None;
    let mut occupancy = [0; 3];
    let (mut simultaneous, mut full_release, mut reset_live, mut flush_live, mut overlap) =
        (0, 0, 0, 0, 0);
    let (
        mut stall_run,
        mut longest_stall,
        mut high_delivered,
        mut throughput_run,
        mut longest_throughput,
    ) = (0, 0, 0, 0, 0);
    for cycle in 0..420 {
        let before = output(&q);
        occupancy[q.len()] += 1;
        let draining = cycle >= 400;
        let rst = matches!(cycle, 0 | 72 | 73 | 201);
        let flush = matches!(cycle, 64 | 73 | 74 | 160 | 202);
        let ready = if draining {
            true
        } else if cycle < 40 || (60..76).contains(&cycle) || (150..161).contains(&cycle) {
            false
        } else if cycle < 120 {
            true
        } else {
            next(&mut seed) & 3 != 0
        };
        if pending.is_none() && !draining && (cycle < 120 || next(&mut seed) & 3 != 0) {
            // Initial directed words exercise bit 63; later words vary every bit.
            let high = if cycle < 3 { 1u64 << (width - 1) } else { 0 };
            pending = Some((next(&mut seed) | high) & mask(width));
        }
        let input = Input {
            rst,
            flush,
            valid: pending.is_some(),
            data: pending.unwrap_or_else(|| next(&mut seed) & mask(width)),
            ready,
        };
        let pop = before.data.is_some() && ready && !rst && !flush;
        let push = input.valid && before.ready && !rst && !flush;
        if rst || flush {
            assert_eq!(
                epoch_in,
                epoch_out + q.len(),
                "epoch conservation before cancellation"
            );
            cancelled += q.len();
            reset_live += usize::from(rst && !q.is_empty());
            flush_live += usize::from(flush && !rst && !q.is_empty());
            overlap += usize::from(rst && flush);
            q.clear();
            pending = None;
            epoch_in = 0;
            epoch_out = 0;
        } else {
            simultaneous += usize::from(pop && push);
            full_release += usize::from(q.len() == 2 && pop && input.valid && !push);
            if pop {
                let word = q.pop_front().unwrap();
                high_delivered += usize::from(word & (1u64 << (width - 1)) != 0);
                delivered += 1;
                epoch_out += 1;
            }
            if push {
                q.push_back(input.data);
                accepted += 1;
                epoch_in += 1;
                pending = None;
            }
        }
        if before.data.is_some() && !ready && !rst && !flush {
            stall_run += 1;
        } else {
            stall_run = 0;
        }
        longest_stall = longest_stall.max(stall_run);
        if pop && push {
            throughput_run += 1;
        } else {
            throughput_run = 0;
        }
        longest_throughput = longest_throughput.max(throughput_run);
        assert_eq!(accepted, delivered + cancelled + q.len());
        assert_eq!(epoch_in, epoch_out + q.len());
        frames.push(Frame {
            input,
            before,
            after: output(&q),
        });
    }
    assert!(
        q.is_empty() && pending.is_none(),
        "must fully drain producer and DUT"
    );
    assert_eq!(accepted, delivered + cancelled);
    assert!(occupancy.into_iter().all(|n| n > 0));
    assert!(
        simultaneous > 0 && full_release > 0 && reset_live > 0 && flush_live > 0 && overlap > 0
    );
    assert!(longest_stall >= 30 && longest_throughput >= 20 && high_delivered > 0);
    // Enforce valid/data hold under backpressure, unless an edge cancels the producer epoch.
    for pair in frames.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        if a.input.valid && !a.before.ready && !a.input.rst && !a.input.flush {
            assert!(b.input.valid);
            assert_eq!(a.input.data, b.input.data);
        }
    }
    println!(
        "FR195 width={width} seed={initial_seed:#x} accepted={accepted} delivered={delivered} cancelled={cancelled} occupancy={occupancy:?} simultaneous={simultaneous} full_release={full_release} reset_live={reset_live} flush_live={flush_live} overlap={overlap} longest_stall={longest_stall} longest_throughput={longest_throughput}"
    );
    frames
}
fn inputs(i: Input) -> PortValues {
    let mut p = PortValues::default();
    for (name, value) in [
        ("clk", 0),
        ("rst", u64::from(i.rst)),
        ("flush", u64::from(i.flush)),
        ("input_valid", u64::from(i.valid)),
        ("input_data", i.data),
        ("output_ready", u64::from(i.ready)),
    ] {
        p.set(name, value);
    }
    p
}
fn native_check(sim: &Sim, expected: Output, context: &str) {
    assert_eq!(
        sim.ports().get("input_ready"),
        Some(u64::from(expected.ready)),
        "{context}"
    );
    assert_eq!(
        sim.ports().get("output_valid"),
        Some(u64::from(expected.data.is_some())),
        "{context}"
    );
    if let Some(word) = expected.data {
        assert_eq!(sim.ports().get("output_data"), Some(word), "{context}");
    }
}
fn single<const W: u32>() {
    let hir = RvRegSlice::<W>::elaborate().unwrap();
    for seed in SEEDS {
        let frames = trace(W, seed);
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(hir.clone(), engine);
            for (cycle, f) in frames.iter().enumerate() {
                let context = format!("width={W} seed={seed:#x} engine={engine:?} cycle={cycle}");
                if cycle > 0 {
                    // Probe all data/control inputs before the edge, including synchronous controls.
                    let old_data = sim.ports().get("output_data");
                    sim.set_inputs(inputs(Input {
                        rst: !f.input.rst,
                        flush: !f.input.flush,
                        valid: !f.input.valid,
                        ready: !f.input.ready,
                        data: (!f.input.data) & mask(W),
                    }));
                    sim.settle();
                    native_check(&sim, f.before, &context);
                    assert_eq!(
                        sim.ports().get("output_data"),
                        old_data,
                        "no input-to-payload combinational path: {context}"
                    );
                }
                sim.set_inputs(inputs(f.input));
                sim.settle();
                if cycle > 0 {
                    native_check(&sim, f.before, &context);
                }
                sim.tick();
                native_check(&sim, f.after, &context);
            }
        }
        let name = &hir.circuit().modules[0].name;
        let mut tb = format!(
            "module tb;reg clk=0;reg rst=0,flush=0,input_valid=0,output_ready=0;reg[{}:0] input_data=0;wire input_ready,output_valid;wire[{}:0] output_data;reg[{}:0] saved_data;{name} dut(.*);initial begin\n",
            W - 1,
            W - 1,
            W - 1
        );
        for (cycle, f) in frames.iter().enumerate() {
            if cycle > 0 {
                tb.push_str("saved_data=output_data;\n");
                drive(
                    &mut tb,
                    "",
                    Input {
                        rst: !f.input.rst,
                        flush: !f.input.flush,
                        valid: !f.input.valid,
                        ready: !f.input.ready,
                        data: (!f.input.data) & mask(W),
                    },
                );
                tb.push_str("#1;\n");
                check(&mut tb, "", f.before, cycle, "probe");
                tb.push_str(
                    "if(output_data !== saved_data) $fatal(1,\"combinational payload path\");\n",
                );
            }
            drive(&mut tb, "", f.input);
            tb.push_str("#1;\n");
            if cycle > 0 {
                check(&mut tb, "", f.before, cycle, "pre");
            }
            tb.push_str("clk=1;#1;\n");
            check(&mut tb, "", f.after, cycle, "post");
            tb.push_str("clk=0;#1;\n");
        }
        tb.push_str("$display(\"FR195 PASS\");$finish;end endmodule\n");
        run_rtl(&format!("single-w{W}-seed{seed:x}"), &hir, &tb);
    }
}
#[test]
fn p0_width1_native_and_real_rtl() {
    single::<1>();
}
#[test]
fn p0_width8_native_and_real_rtl() {
    single::<8>();
}
#[test]
fn p0_width32_native_and_real_rtl() {
    single::<32>();
}
#[test]
fn p0_width64_native_and_real_rtl() {
    single::<64>();
}

#[test]
fn p0_shared_definition_and_exact_port_contract() {
    type DefaultSlice = RvRegSlice;
    let hir = RvRegSlice::<32>::elaborate().unwrap();
    assert_eq!(
        DefaultSlice::elaborate().unwrap(),
        hir,
        "default width must be 32"
    );
    let name = &hir.circuit().modules[0].name;
    let mut session = ElaborateSession::new(name.clone());
    assert_eq!(
        RvRegSlice::<32>::define_module(&mut session, name).unwrap(),
        *name
    );
    assert_eq!(
        RvRegSlice::<32>::define_module(&mut session, name).unwrap(),
        *name
    );
    assert_eq!(hir, session.finish().unwrap());
    let ports = &hir.circuit().modules[0].ports;
    assert_eq!(ports.len(), 9);
    for (name, ty, direction) in [
        ("clk", GroundType::Clock, PortDirection::Input),
        ("rst", GroundType::Reset, PortDirection::Input),
        ("flush", GroundType::UInt { width: 1 }, PortDirection::Input),
        (
            "input_valid",
            GroundType::UInt { width: 1 },
            PortDirection::Input,
        ),
        (
            "input_data",
            GroundType::UInt { width: 32 },
            PortDirection::Input,
        ),
        (
            "output_ready",
            GroundType::UInt { width: 1 },
            PortDirection::Input,
        ),
        (
            "input_ready",
            GroundType::UInt { width: 1 },
            PortDirection::Output,
        ),
        (
            "output_valid",
            GroundType::UInt { width: 1 },
            PortDirection::Output,
        ),
        (
            "output_data",
            GroundType::UInt { width: 32 },
            PortDirection::Output,
        ),
    ] {
        let port = ports.iter().find(|p| p.name == name).expect(name);
        assert_eq!(port.ty, ty);
        assert_eq!(port.direction, direction);
    }
    let mut session = ElaborateSession::new("Conflict");
    RvRegSlice::<8>::define_module(&mut session, "Shared").unwrap();
    let error = RvRegSlice::<32>::define_module(&mut session, "Shared").unwrap_err();
    let diagnostic = format!("{error:?}");
    assert!(
        diagnostic.contains("WIDTH") && diagnostic.contains("8") && diagnostic.contains("32"),
        "complete WIDTH parameter identity: {diagnostic}"
    );
    assert!(
        format!("{error:?}").to_lowercase().contains("parameter"),
        "wrong identity diagnostic: {error:?}"
    );
}
fn invalid<const W: u32>() {
    let error = std::panic::catch_unwind(RvRegSlice::<W>::elaborate)
        .expect("invalid width must not panic")
        .unwrap_err();
    assert!(
        format!("{error:?}").to_lowercase().contains("width"),
        "wrong width diagnostic: {error:?}"
    );
    let result = std::panic::catch_unwind(|| {
        let mut s = ElaborateSession::new("Invalid");
        RvRegSlice::<W>::define_module(&mut s, "Invalid")
    })
    .expect("shared definition must not panic");
    let error = result.unwrap_err();
    assert!(format!("{error:?}").to_lowercase().contains("width"));
}
#[test]
fn p0_invalid_widths_are_diagnostics() {
    invalid::<0>();
    invalid::<65>();
    invalid::<{ u32::MAX }>();
}

fn pair() -> FrozenHir {
    let mut s = ElaborateSession::new("Pair");
    let name = RvRegSlice::<64>::define_module(&mut s, "Slice64").unwrap();
    assert_eq!(
        name,
        RvRegSlice::<64>::define_module(&mut s, "Slice64").unwrap()
    );
    let sp = Span::default();
    s.begin_module("Pair", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut connects = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (port, ty) in [
            ("flush", GroundType::UInt { width: 1 }),
            ("input_valid", GroundType::UInt { width: 1 }),
            ("input_data", GroundType::UInt { width: 64 }),
            ("output_ready", GroundType::UInt { width: 1 }),
        ] {
            let net = format!("s{lane}_{port}");
            s.add_input(net.clone(), ty, sp);
            connects.push((port.into(), net));
        }
        for (port, width) in [("input_ready", 1), ("output_valid", 1), ("output_data", 64)] {
            let net = format!("s{lane}_{port}");
            s.add_output(net.clone(), GroundType::UInt { width }, sp);
            connects.push((port.into(), net));
        }
        s.add_instance(format!("slice{lane}"), name.clone(), connects, vec![], sp);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    hir
}
#[test]
fn p0_real_rtl_two_instances_preserve_independent_state() {
    let hir = pair();
    let a = trace(64, SEEDS[0]);
    let b = trace(64, SEEDS[1]);
    let mut tb = String::from("module tb;reg clk=0,rst=0;\n");
    for lane in 0..2 {
        writeln!(tb,"reg s{lane}_rst=0,s{lane}_flush=0,s{lane}_input_valid=0,s{lane}_output_ready=0;reg[63:0] s{lane}_input_data=0;wire s{lane}_input_ready,s{lane}_output_valid;wire[63:0] s{lane}_output_data;").unwrap();
    }
    tb.push_str("Pair dut(.*);initial begin\n");
    // Offset lane 1 by 11 cycles, so ready, occupancy and cancellation differ.
    let idle = Input {
        rst: true,
        ..Input::default()
    };
    let empty = Output {
        ready: true,
        data: None,
    };
    for cycle in 0..a.len() + 11 {
        writeln!(tb, "rst={};", u8::from(cycle == 0)).unwrap();
        for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize)] {
            let f = cycle.checked_sub(offset).and_then(|n| frames.get(n));
            let mut input = f.map_or(idle, |f| f.input);
            // Shared domain reset; per-lane epoch cancellation uses flush.
            input.flush |= input.rst;
            input.rst = cycle == 0;
            drive(&mut tb, &format!("s{lane}_"), input);
        }
        tb.push_str("#1;\n");
        if cycle > 0 {
            for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize)] {
                let f = cycle.checked_sub(offset).and_then(|n| frames.get(n));
                check(
                    &mut tb,
                    &format!("s{lane}_"),
                    f.map_or(empty, |f| f.before),
                    cycle,
                    "pre",
                );
            }
        }
        tb.push_str("clk=1;#1;\n");
        for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize)] {
            let f = cycle.checked_sub(offset).and_then(|n| frames.get(n));
            check(
                &mut tb,
                &format!("s{lane}_"),
                f.map_or(empty, |f| f.after),
                cycle,
                "post",
            );
        }
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR195 PASS\");$finish;end endmodule\n");
    run_rtl("pair-w64", &hir, &tb);
}
#[test]
fn p1_native_hierarchy_remains_explicitly_unsupported() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let e = std::panic::catch_unwind(|| Sim::with_engine(pair(), engine))
            .err()
            .expect("hierarchy must reject");
        let message = e
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| e.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("hierarchical simulation is unsupported"),
            "unexpected panic: {message}"
        );
    }
}
fn drive(tb: &mut String, p: &str, i: Input) {
    writeln!(
        tb,
        "{p}rst={};{p}flush={};{p}input_valid={};{p}input_data=64'h{:016x};{p}output_ready={};",
        u8::from(i.rst),
        u8::from(i.flush),
        u8::from(i.valid),
        i.data,
        u8::from(i.ready)
    )
    .unwrap();
}
fn check(tb: &mut String, p: &str, o: Output, cycle: usize, phase: &str) {
    for (signal, value) in [
        ("input_ready", u64::from(o.ready)),
        ("output_valid", u64::from(o.data.is_some())),
    ]
    .into_iter()
    .chain(o.data.map(|v| ("output_data", v)))
    {
        writeln!(tb,"if ({p}{signal} !== 64'h{value:016x}) $fatal(1,\"FR195 cycle={cycle} phase={phase} {p}{signal} expected={value:x} actual=%h\",{p}{signal});").unwrap();
    }
}

fn bounded(command: &mut Command, log_path: &Path, timeout: Duration) -> io::Result<ExitStatus> {
    let log = fs::File::create(log_path)?;
    // GNU timeout creates and signals a process group, including tool children.
    let mut runner = Command::new("timeout");
    runner
        .arg("--kill-after=5s")
        .arg(format!("{}s", timeout.as_secs_f64()))
        .arg(command.get_program())
        .args(command.get_args());
    if let Some(dir) = command.get_current_dir() {
        runner.current_dir(dir);
    }
    for (key, value) in command.get_envs() {
        if let Some(value) = value {
            runner.env(key, value);
        } else {
            runner.env_remove(key);
        }
    }
    let status = runner
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .status()?;
    writeln!(
        fs::OpenOptions::new().append(true).open(log_path)?,
        "status={status}"
    )?;
    if matches!(status.code(), Some(124 | 137)) {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("tool timed out; log {}", log_path.display()),
        ));
    }
    Ok(status)
}

fn run_rtl(label: &str, hir: &FrozenHir, tb: &str) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr195")
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
            .expect("FR195 RTL tool required and version probe must complete within 60s");
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
        .expect("FR195 RTL tool must exist and finish within 60s; artifacts preserved");
        let log = fs::read_to_string(&path).unwrap();
        assert!(
            status.success(),
            "FR195 {label}: {tool} failed; artifacts {}\n{log}",
            dir.display()
        );
        if tool == "vvp" {
            assert!(
                log.contains("FR195 PASS"),
                "simulation did not reach success marker"
            );
            println!("{label}: {log}artifacts={}", dir.display());
        }
    }
}

#[test]
fn distinct_widths_coexist_in_one_session() {
    let mut session = ElaborateSession::new("Slice8");
    RvRegSlice::<8>::define_module(&mut session, "Slice8").unwrap();
    RvRegSlice::<32>::define_module(&mut session, "Slice32").unwrap();
    let hir = session.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    for (name, width, reference) in [
        ("Slice8", 8, RvRegSlice::<8>::elaborate().unwrap()),
        ("Slice32", 32, RvRegSlice::<32>::elaborate().unwrap()),
    ] {
        let module = hir
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap();
        for port in ["input_data", "output_data"] {
            assert_eq!(
                module.ports.iter().find(|p| p.name == port).unwrap().ty,
                GroundType::UInt { width }
            );
        }
        assert_eq!(module.body, reference.circuit().modules[0].body);
    }
}

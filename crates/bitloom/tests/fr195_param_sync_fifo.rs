//! Story 126.4 / FR195: active ATDD acceptance tests; no tool-presence skips.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span, ip::ParamSyncFifo,
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

const SEEDS: [u64; 3] = [0x1264_1950_a551, 0xdead_beef_8012, 0x7359_2401_ffff];
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
fn output(q: &VecDeque<u64>, depth: usize) -> Output {
    Output {
        ready: q.len() < depth,
        data: q.front().copied(),
    }
}

// This queue is an independent transaction model, with no knowledge of RTL registers.
fn trace(width: u32, depth: usize, initial_seed: u64) -> Vec<Frame> {
    let mut seed = initial_seed;
    let mut q = VecDeque::new();
    let mut frames = Vec::new();
    let (mut accepted, mut delivered, mut cancelled) = (0usize, 0usize, 0usize);
    let (mut epoch_in, mut epoch_out) = (0usize, 0usize);
    let mut pending = None;
    let mut occupancy = vec![0; depth + 1];
    let (mut seen_full, mut full_drains) = (false, 0usize);
    let (mut simultaneous, mut full_release, mut reset_live, mut flush_live, mut overlap) =
        (0, 0, 0, 0, 0);
    let (
        mut stall_run,
        mut longest_stall,
        mut high_delivered,
        mut throughput_run,
        mut longest_throughput,
    ) = (0, 0, 0, 0, 0);
    // Explicit cancellation cross-product, after the base trace has drained.
    // Each cancellation presents both input valid and output ready: any apparent
    // handshake on that edge must be cancelled, including a live flush output.
    let mut cancellation_tail = Vec::new();
    let occupancies = if depth == 1 {
        vec![0, depth]
    } else {
        vec![0, depth / 2, depth]
    };
    for (rst, flush) in [(true, false), (false, true), (true, true)] {
        for &resident in &occupancies {
            for word in 0..resident {
                cancellation_tail.push((
                    Input {
                        valid: true,
                        data: ((1u64 << (width - 1)) | word as u64) & mask(width),
                        ..Input::default()
                    },
                    None,
                ));
            }
            cancellation_tail.push((
                Input {
                    rst,
                    flush,
                    valid: true,
                    ready: true,
                    data: mask(width),
                },
                Some(resident),
            ));
        }
    }
    // Reuse after cancellation, then finish with twenty drain cycles.
    cancellation_tail.push((
        Input {
            valid: true,
            data: mask(width),
            ..Input::default()
        },
        None,
    ));
    cancellation_tail.extend((0..20).map(|_| {
        (
            Input {
                ready: true,
                ..Input::default()
            },
            None,
        )
    }));
    // Rows: reset-only / flush-only / overlap. Columns: empty / partial / full.
    let mut cancellation_occupancy = [[0usize; 3]; 3];
    let mut flush_output_handshake = 0usize;
    for cycle in 0..420 + cancellation_tail.len() {
        let before = output(&q, depth);
        occupancy[q.len()] += 1;
        let input = if let Some(&(input, expected_resident)) = cycle
            .checked_sub(420)
            .and_then(|offset| cancellation_tail.get(offset))
        {
            if cycle == 420 {
                assert!(
                    q.is_empty() && pending.is_none(),
                    "base trace must drain before cancellation matrix"
                );
            }
            if let Some(resident) = expected_resident {
                assert_eq!(q.len(), resident, "directed cancellation occupancy");
            }
            assert!(
                input.rst || input.flush || !input.valid || before.ready,
                "directed producer must not replace a stalled offer"
            );
            assert!(pending.is_none());
            pending = input.valid.then_some(input.data);
            input
        } else {
            let draining =
                cycle >= 400 || (180..200).contains(&cycle) || (300..320).contains(&cycle);
            let rst = matches!(cycle, 0 | 72 | 73 | 155 | 201);
            let flush = matches!(cycle, 64 | 73 | 74 | 155 | 160 | 202);
            let ready = if draining {
                true
            } else if cycle < 40
                || (60..76).contains(&cycle)
                || (140..180).contains(&cycle)
                || (260..300).contains(&cycle)
            {
                false
            } else if cycle < 120 {
                true
            } else {
                next(&mut seed) & 3 != 0
            };
            if pending.is_none()
                && !draining
                && (cycle < 120
                    || (140..180).contains(&cycle)
                    || (260..300).contains(&cycle)
                    || next(&mut seed) & 3 != 0)
            {
                // Initial directed words exercise bit 63; later words vary every bit.
                let high = if cycle < 3 { 1u64 << (width - 1) } else { 0 };
                pending = Some((next(&mut seed) | high) & mask(width));
            }
            Input {
                rst,
                flush,
                valid: pending.is_some(),
                data: pending.unwrap_or_else(|| next(&mut seed) & mask(width)),
                ready,
            }
        };
        let Input {
            rst, flush, ready, ..
        } = input;
        let pop = before.data.is_some() && ready && !rst && !flush;
        let push = input.valid && before.ready && !rst && !flush;
        if rst || flush {
            assert_eq!(
                epoch_in,
                epoch_out + q.len(),
                "epoch conservation before cancellation"
            );
            let cause = match (rst, flush) {
                (true, false) => 0,
                (false, true) => 1,
                (true, true) => 2,
                (false, false) => unreachable!(),
            };
            let occupancy_class = if q.is_empty() {
                0
            } else if q.len() == depth {
                2
            } else {
                1
            };
            cancellation_occupancy[cause][occupancy_class] += 1;
            flush_output_handshake += usize::from(flush && !rst && ready && before.data.is_some());
            cancelled += q.len();
            reset_live += usize::from(rst && !q.is_empty());
            flush_live += usize::from(flush && !rst && !q.is_empty());
            overlap += usize::from(rst && flush && !q.is_empty());
            q.clear();
            seen_full = false;
            pending = None;
            epoch_in = 0;
            epoch_out = 0;
        } else {
            simultaneous += usize::from(pop && push);
            full_release += usize::from(q.len() == depth && pop && input.valid && !push);
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
        if q.len() == depth {
            seen_full = true;
        }
        if seen_full && q.is_empty() {
            full_drains += 1;
            seen_full = false;
        }
        assert_eq!(accepted, delivered + cancelled + q.len());
        assert_eq!(epoch_in, epoch_out + q.len());
        frames.push(Frame {
            input,
            before,
            after: output(&q, depth),
        });
    }
    assert!(
        q.is_empty() && pending.is_none(),
        "must fully drain producer and DUT"
    );
    assert_eq!(accepted, delivered + cancelled);
    assert!(
        full_drains >= 2,
        "must repeatedly fill and drain: {full_drains}"
    );
    assert!(occupancy.iter().all(|n| *n > 0));
    for (cause, hits) in cancellation_occupancy.iter().enumerate() {
        assert!(
            hits[0] > 0 && hits[2] > 0,
            "empty/full cancellation cause={cause}: {hits:?}"
        );
        if depth == 1 {
            assert_eq!(hits[1], 0, "DEPTH1 cannot have partial occupancy");
        } else {
            assert!(hits[1] > 0, "partial cancellation cause={cause}: {hits:?}");
        }
    }
    assert!(
        flush_output_handshake > 0,
        "live flush must cancel apparent output transfer"
    );
    assert!(
        (if depth == 1 {
            simultaneous == 0
        } else {
            simultaneous > 0
        }) && full_release > 0
            && reset_live > 0
            && flush_live > 0
            && overlap > 0
    );
    assert!(longest_stall >= 30 && (depth == 1 || longest_throughput >= 20) && high_delivered > 0);
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
        "FR195 width={width} depth={depth} seed={initial_seed:#x} accepted={accepted} delivered={delivered} cancelled={cancelled} occupancy={occupancy:?} simultaneous={simultaneous} full_release={full_release} reset_live={reset_live} flush_live={flush_live} overlap={overlap} longest_stall={longest_stall} longest_throughput={longest_throughput} full_drains={full_drains} cycles={} cancellation_occupancy_reset_flush_overlap_empty_partial_full={cancellation_occupancy:?} flush_output_handshake={flush_output_handshake}",
        frames.len()
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
fn single<const W: u32, const D: u32>() {
    let hir = ParamSyncFifo::<W, D>::elaborate().unwrap();
    assert!(
        hir.circuit().modules[0]
            .body
            .iter()
            .all(|s| !matches!(s, bitloom_hir::Stmt::MemDecl { .. })),
        "register FIFO must not introduce memory"
    );
    for seed in SEEDS {
        let frames = trace(W, D as usize, seed);
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(hir.clone(), engine);
            for (cycle, f) in frames.iter().enumerate() {
                let context =
                    format!("width={W} depth={D} seed={seed:#x} engine={engine:?} cycle={cycle}");
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
                    if f.before.data.is_some() {
                        assert_eq!(
                            sim.ports().get("output_data"),
                            old_data,
                            "no input-to-valid-payload combinational path: {context}"
                        );
                    }
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
                if f.before.data.is_some() {
                    tb.push_str(
                        "if(output_data !== saved_data) $fatal(1,\"combinational valid payload path\");\n",
                    );
                }
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
        run_rtl(&format!("single-w{W}-d{D}-seed{seed:x}"), &hir, &tb);
    }
}
#[test]
fn p0_w1_d1_native_and_real_rtl() {
    single::<1, 1>();
}
#[test]
fn p0_w1_d2_native_and_real_rtl() {
    single::<1, 2>();
}
#[test]
fn p0_w1_d3_native_and_real_rtl() {
    single::<1, 3>();
}
#[test]
fn p0_w1_d4_native_and_real_rtl() {
    single::<1, 4>();
}
#[test]
fn p0_w1_d7_native_and_real_rtl() {
    single::<1, 7>();
}
#[test]
fn p0_w1_d16_native_and_real_rtl() {
    single::<1, 16>();
}
#[test]
fn p0_w8_d1_native_and_real_rtl() {
    single::<8, 1>();
}
#[test]
fn p0_w8_d2_native_and_real_rtl() {
    single::<8, 2>();
}
#[test]
fn p0_w8_d3_native_and_real_rtl() {
    single::<8, 3>();
}
#[test]
fn p0_w8_d4_native_and_real_rtl() {
    single::<8, 4>();
}
#[test]
fn p0_w8_d7_native_and_real_rtl() {
    single::<8, 7>();
}
#[test]
fn p0_w8_d16_native_and_real_rtl() {
    single::<8, 16>();
}
#[test]
fn p0_w32_d1_native_and_real_rtl() {
    single::<32, 1>();
}
#[test]
fn p0_w32_d2_native_and_real_rtl() {
    single::<32, 2>();
}
#[test]
fn p0_w32_d3_native_and_real_rtl() {
    single::<32, 3>();
}
#[test]
fn p0_w32_d4_native_and_real_rtl() {
    single::<32, 4>();
}
#[test]
fn p0_w32_d7_native_and_real_rtl() {
    single::<32, 7>();
}
#[test]
fn p0_w32_d16_native_and_real_rtl() {
    single::<32, 16>();
}
#[test]
fn p0_w64_d1_native_and_real_rtl() {
    single::<64, 1>();
}
#[test]
fn p0_w64_d2_native_and_real_rtl() {
    single::<64, 2>();
}
#[test]
fn p0_w64_d3_native_and_real_rtl() {
    single::<64, 3>();
}
#[test]
fn p0_w64_d4_native_and_real_rtl() {
    single::<64, 4>();
}
#[test]
fn p0_w64_d7_native_and_real_rtl() {
    single::<64, 7>();
}
#[test]
fn p0_w64_d16_native_and_real_rtl() {
    single::<64, 16>();
}
#[test]
fn p0_shared_definition_and_exact_port_contract() {
    type DefaultSlice = ParamSyncFifo;
    let hir = ParamSyncFifo::<32, 4>::elaborate().unwrap();
    assert_eq!(
        DefaultSlice::elaborate().unwrap(),
        hir,
        "default must be 32 × 4"
    );
    let name = &hir.circuit().modules[0].name;
    let mut session = ElaborateSession::new(name.clone());
    assert_eq!(
        ParamSyncFifo::<32>::define_module(&mut session, name).unwrap(),
        *name
    );
    assert_eq!(
        ParamSyncFifo::<32>::define_module(&mut session, name).unwrap(),
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
    ParamSyncFifo::<8>::define_module(&mut session, "Shared").unwrap();
    let error = ParamSyncFifo::<32>::define_module(&mut session, "Shared").unwrap_err();
    let diagnostic = format!("{error:?}");
    assert!(
        diagnostic.contains("WIDTH") && diagnostic.contains("8") && diagnostic.contains("32"),
        "complete WIDTH parameter identity: {diagnostic}"
    );
    assert!(
        format!("{error:?}").to_lowercase().contains("parameter"),
        "wrong identity diagnostic: {error:?}"
    );
    // Conflicts persist in the session; no candidate may be frozen afterward.
    assert_eq!(session.finish().unwrap_err(), error);
}
fn invalid<const W: u32, const D: u32>() {
    for shared in [false, true] {
        let result = std::panic::catch_unwind(|| {
            if shared {
                let mut s = ElaborateSession::new("Invalid");
                let error = ParamSyncFifo::<W, D>::define_module(&mut s, "Invalid").unwrap_err();
                // Failed definition must not leave a partial module or poison session.
                ParamSyncFifo::<8, 3>::define_module(&mut s, "Invalid").unwrap();
                assert_eq!(s.finish().unwrap().circuit().modules.len(), 1);
                error
            } else {
                ParamSyncFifo::<W, D>::elaborate().unwrap_err()
            }
        })
        .expect("invalid parameters must not panic");
        let diagnostic = format!("{result:?}").to_lowercase();
        for (parameter, value, maximum) in [("width", W, 64), ("depth", D, 16)] {
            if !(1..=maximum).contains(&value) {
                assert!(
                    result.0.iter().any(|entry| {
                        let entry = entry.en.to_lowercase();
                        entry.contains(parameter) && entry.contains(&format!("got {value}"))
                    }),
                    "missing {parameter}={value} diagnostic: {diagnostic}"
                );
            }
        }
    }
}
#[test]
fn p0_invalid_parameters_are_diagnostics_and_leave_no_partial_definition() {
    invalid::<0, 1>();
    invalid::<65, 16>();
    invalid::<{ u32::MAX }, 4>();
    invalid::<1, 0>();
    invalid::<64, 17>();
    invalid::<32, { u32::MAX }>();
    invalid::<0, 0>();
    invalid::<65, 17>();
}
#[test]
fn p0_depth_is_part_of_definition_identity() {
    let mut s = ElaborateSession::new("Shared");
    ParamSyncFifo::<8, 3>::define_module(&mut s, "Shared").unwrap();
    let error = ParamSyncFifo::<8, 7>::define_module(&mut s, "Shared").unwrap_err();
    let d = format!("{error:?}");
    assert!(
        d.contains("DEPTH") && d.contains('3') && d.contains('7'),
        "{d}"
    );
    assert_eq!(s.finish().unwrap_err(), error);
}

fn pair() -> FrozenHir {
    let mut s = ElaborateSession::new("Pair");
    let name = ParamSyncFifo::<64, 3>::define_module(&mut s, "Fifo64d3").unwrap();
    assert_eq!(
        name,
        ParamSyncFifo::<64, 3>::define_module(&mut s, "Fifo64d3").unwrap()
    );
    let specialized = ParamSyncFifo::<8, 7>::define_module(&mut s, "Fifo8d7").unwrap();
    let sp = Span::default();
    s.begin_module("Pair", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..3 {
        let width = if lane == 2 { 8 } else { 64 };
        let mut connects = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (port, ty) in [
            ("flush", GroundType::UInt { width: 1 }),
            ("input_valid", GroundType::UInt { width: 1 }),
            ("input_data", GroundType::UInt { width }),
            ("output_ready", GroundType::UInt { width: 1 }),
        ] {
            let net = format!("s{lane}_{port}");
            s.add_input(net.clone(), ty, sp);
            connects.push((port.into(), net));
        }
        for (port, width) in [
            ("input_ready", 1),
            ("output_valid", 1),
            ("output_data", width),
        ] {
            let net = format!("s{lane}_{port}");
            s.add_output(net.clone(), GroundType::UInt { width }, sp);
            connects.push((port.into(), net));
        }
        s.add_instance(
            format!("slice{lane}"),
            if lane == 2 {
                specialized.clone()
            } else {
                name.clone()
            },
            connects,
            vec![],
            sp,
        );
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 3);
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Pair")
        .unwrap();
    assert_eq!(
        top.ports
            .iter()
            .filter(|p| p.ty == GroundType::Clock)
            .count(),
        1
    );
    assert_eq!(
        top.ports
            .iter()
            .filter(|p| p.ty == GroundType::Reset)
            .count(),
        1
    );
    for stmt in &top.body {
        if let bitloom_hir::Stmt::Instance(i) = stmt {
            assert!(i.params.is_empty());
        }
    }
    hir
}
#[test]
fn p0_real_rtl_three_instances_preserve_independent_state() {
    let hir = pair();
    let a = trace(64, 3, SEEDS[0]);
    let b = trace(64, 3, SEEDS[1]);
    let c = trace(8, 7, SEEDS[2]);
    let mut tb = String::from("module tb;reg clk=0,rst=0;\n");
    for lane in 0..3 {
        let width = if lane == 2 { 8 } else { 64 };
        writeln!(tb,"reg s{lane}_rst=0,s{lane}_flush=0,s{lane}_input_valid=0,s{lane}_output_ready=0;reg[{}:0] s{lane}_input_data=0;wire s{lane}_input_ready,s{lane}_output_valid;wire[{}:0] s{lane}_output_data;", width-1,width-1).unwrap();
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
    for cycle in 0..a.len().max(b.len() + 11).max(c.len() + 23) {
        writeln!(tb, "rst={};", u8::from(cycle == 0)).unwrap();
        for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize), (2, &c, 23usize)] {
            let f = cycle.checked_sub(offset).and_then(|n| frames.get(n));
            let mut input = f.map_or(idle, |f| f.input);
            // Shared domain reset; per-lane epoch cancellation uses flush.
            input.flush |= input.rst;
            input.rst = cycle == 0;
            drive(&mut tb, &format!("s{lane}_"), input);
        }
        tb.push_str("#1;\n");
        if cycle > 0 {
            for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize), (2, &c, 23usize)] {
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
        for (lane, frames, offset) in [(0, &a, 0usize), (1, &b, 11usize), (2, &c, 23usize)] {
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
    // All preceding traces finish empty. Refill with distinct occupancies,
    // then cancel all three through the actual shared reset port. Continue with
    // independent traffic/backpressure and lane-local flush after that reset.
    let mut queues: [VecDeque<u64>; 3] = std::array::from_fn(|_| VecDeque::new());
    let mut accepted = [0; 3];
    let mut delivered = [0; 3];
    let mut cancelled = [0; 3];
    for cycle in 0..28 {
        let reset = cycle == 4;
        writeln!(tb, "rst={};", u8::from(reset)).unwrap();
        let mut after = [empty; 3];
        for lane in 0..3 {
            let width = if lane == 2 { 8 } else { 64 };
            let depth = if lane == 2 { 7 } else { 3 };
            let queue = &mut queues[lane];
            let before = output(queue, depth);
            if reset {
                assert_eq!(
                    queue.len(),
                    lane + 1,
                    "shared reset must cancel distinct live queues"
                );
            }
            let input = Input {
                rst: reset,
                flush: lane == 1 && matches!(cycle, 4 | 12),
                valid: cycle < lane + 1
                    || reset
                    || cycle == 6 + lane
                    || cycle == 9 + lane
                    || (lane == 1 && cycle == 14),
                data: ((0x8000_0000_0000_0080u64 | ((lane as u64) << 4)) + cycle as u64)
                    & mask(width),
                ready: reset || cycle >= 8 + lane * 4,
            };
            assert!(
                !input.valid || before.ready,
                "directed producer offers only with space"
            );
            drive(&mut tb, &format!("s{lane}_"), input);
            tb.push_str("#1;\n");
            check(
                &mut tb,
                &format!("s{lane}_"),
                before,
                cycle,
                "shared-reset-pre",
            );
            if input.rst || input.flush {
                cancelled[lane] += queue.len();
                queue.clear();
            } else {
                if input.ready && before.data.is_some() {
                    queue.pop_front();
                    delivered[lane] += 1;
                }
                if input.valid && before.ready {
                    queue.push_back(input.data);
                    accepted[lane] += 1;
                }
            }
            assert_eq!(
                accepted[lane],
                delivered[lane] + cancelled[lane] + queue.len()
            );
            after[lane] = output(queue, depth);
        }
        tb.push_str("clk=1;#1;\n");
        for (lane, expected) in after.into_iter().enumerate() {
            check(
                &mut tb,
                &format!("s{lane}_"),
                expected,
                cycle,
                "shared-reset-post",
            );
        }
        tb.push_str("clk=0;#1;\n");
    }
    for lane in 0..3 {
        assert!(queues[lane].is_empty());
        assert!(cancelled[lane] >= lane + 1 && delivered[lane] > 0);
        assert_eq!(accepted[lane], delivered[lane] + cancelled[lane]);
    }
    println!(
        "FR195 shared-reset accepted={accepted:?} delivered={delivered:?} cancelled={cancelled:?}"
    );
    tb.push_str("$display(\"FR195 PASS\");$finish;end endmodule\n");
    run_rtl("three-fifos", &hir, &tb);
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
        .join("../../target/fr195-fifo")
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
    ParamSyncFifo::<8>::define_module(&mut session, "Slice8").unwrap();
    ParamSyncFifo::<32>::define_module(&mut session, "Slice32").unwrap();
    let hir = session.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    for (name, width, reference) in [
        ("Slice8", 8, ParamSyncFifo::<8>::elaborate().unwrap()),
        ("Slice32", 32, ParamSyncFifo::<32>::elaborate().unwrap()),
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

#[test]
fn p0_legacy_fifo_known_ram_latency_full_and_reset_retention() {
    let hir = bitloom_prelude::ip::SyncFifo::elaborate().unwrap();
    let tb = r#"
module tb;
reg clk=0,rst=0,wr_en=0,rd_en=0;reg[7:0] data_in=0;
wire[7:0] data_out;wire full,empty;
SyncFifo dut(.*);
task edge_tick;begin #1;clk=1;#1;clk=0;#1;end endtask
initial begin
rst=1;edge_tick;
if(empty !== 1 || full !== 0 || data_out !== 0) $fatal(1,"legacy reset");
rst=0;wr_en=1;data_in=8'ha1;edge_tick;
// Uninitialized RAM read at this first write is deliberately not checked.
data_in=8'hb2;edge_tick;
if(data_out !== 8'ha1) $fatal(1,"legacy registered read");
data_in=8'hc3;edge_tick;data_in=8'hd4;edge_tick;
if(full !== 1 || empty !== 0) $fatal(1,"legacy full");
data_in=8'hee;edge_tick;
if(full !== 1) $fatal(1,"legacy rejected full write");
wr_en=0;rd_en=1;edge_tick;
if(data_out !== 8'ha1 || full !== 0) $fatal(1,"legacy first pop");
edge_tick;if(data_out !== 8'hb2) $fatal(1,"legacy second pop");
edge_tick;if(data_out !== 8'hc3) $fatal(1,"legacy third pop");
edge_tick;if(data_out !== 8'hd4 || empty !== 1) $fatal(1,"legacy last pop");
rd_en=0;rst=1;edge_tick;
if(data_out !== 0 || empty !== 1) $fatal(1,"legacy reset registers");
rst=0;edge_tick;
if(data_out !== 8'ha1) $fatal(1,"legacy RAM survives reset");
$display("FR195 PASS");$finish;
end endmodule
"#;
    run_rtl("legacy-known-ram", &hir, tb);
}

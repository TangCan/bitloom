//! Fixed-workload baseline. Run `cargo bench -p bitloom-sim --bench baseline`.
use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{Sim, TickEngine};
use std::{hint::black_box, time::Instant};

fn design(kind: &str) -> FrozenHir {
    let mut s = ElaborateSession::new(kind);
    let p = Span::default();
    s.begin_module("Baseline", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("constant", GroundType::UInt { width: 32 }, p);
    s.add_input("shift", GroundType::UInt { width: 32 }, p);
    s.add_input("addr", GroundType::UInt { width: 8 }, p);
    s.add_output("out", GroundType::UInt { width: 32 }, p);
    s.declare_reg("count", GroundType::UInt { width: 32 }, p);
    if kind == "memory" {
        s.declare_mem("ram", 256, 32, p);
    }
    if kind == "arithmetic" {
        for i in 0..16 {
            s.declare_wire(format!("n{i}"), GroundType::UInt { width: 32 }, p);
        }
    }
    s.begin_sequential(p);
    s.assign_reg_d_inc("count", p);
    if kind == "memory" {
        s.assign_mem_write("ram", "addr", "count", p);
    }
    s.end_process();
    s.begin_combinational(p);
    match kind {
        "counter" => s.assign_net("out", "count", p),
        "memory" => s.assign_mem_read("out", "ram", "addr", p),
        "arithmetic" => {
            let mut previous = "count".to_owned();
            for i in 0..16 {
                let dest = format!("n{i}");
                match i % 4 {
                    0 => s.assign_add(&dest, &previous, "constant", p),
                    1 => s.assign_xor(&dest, &previous, "constant", p),
                    2 => s.assign_shl(&dest, &previous, "shift", p),
                    _ => s.assign_shr(&dest, &previous, "shift", p),
                }
                previous = dest;
            }
            s.assign_net("out", previous, p);
        }
        _ => unreachable!(),
    }
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}

fn checksum_step(checksum: u64, value: u64) -> u64 {
    checksum.rotate_left(7).wrapping_add(value) ^ 0x517c_c1b7_2722_0a95
}

fn reference(kind: &str, cycles: u64) -> u64 {
    (1..=cycles).fold(0, |hash, cycle| {
        let mut value = cycle as u32;
        match kind {
            "memory" => value = value.wrapping_sub(1),
            "arithmetic" => {
                for _ in 0..4 {
                    value = ((value.wrapping_add(0x9e37_79b9) ^ 0x9e37_79b9) << 5) >> 5;
                }
            }
            _ => {}
        }
        checksum_step(hash, u64::from(value))
    })
}

fn measure(hir: &FrozenHir, engine: TickEngine, cycles: u64) -> (f64, u64) {
    let mut sim = Sim::with_engine(hir.clone(), engine);
    let mut inputs = PortValues::default();
    for (n, v) in [
        ("rst", 1),
        ("constant", 0x9e37_79b9),
        ("shift", 5),
        ("addr", 0),
    ] {
        inputs.set(n, v);
    }
    sim.set_inputs(inputs.clone());
    sim.tick();
    inputs.set("rst", 0);
    sim.set_inputs(inputs);
    let mut hash = 0;
    let start = Instant::now();
    for _ in 0..cycles {
        black_box(&mut sim).tick();
        hash = checksum_step(hash, sim.ports().get("out").unwrap());
    }
    (
        start.elapsed().as_secs_f64() * 1e9 / cycles as f64,
        black_box(hash),
    )
}

fn main() {
    let cycles = std::env::var("BITLOOM_BENCH_CYCLES")
        .map(|v| v.parse().unwrap())
        .unwrap_or(200_000);
    let samples: usize = std::env::var("BITLOOM_BENCH_SAMPLES")
        .map(|v| v.parse().unwrap())
        .unwrap_or(5);
    assert!(cycles > 0 && samples >= 3);
    println!(
        "cycles={cycles} samples={samples} warmup=20000 profile=release order=alternating-pairs"
    );
    println!("workload,requested_engine,effective_engine,sample,ns_per_cycle,checksum");
    for kind in ["counter", "arithmetic", "memory"] {
        let hir = design(kind);
        let expected = reference(kind, cycles);
        let engines = [TickEngine::Interpreter, TickEngine::Compiled];
        let mut timings = [Vec::new(), Vec::new()];
        // Warm both before measurement, then alternate which engine goes first
        // in each sample pair so gradual host drift is not tied to one engine.
        for engine in engines {
            let (_, warm_hash) = measure(&hir, engine, 20_000);
            assert_eq!(warm_hash, reference(kind, 20_000));
        }
        for sample in 0..samples {
            let order = if sample % 2 == 0 { [0, 1] } else { [1, 0] };
            for index in order {
                let engine = engines[index];
                let effective = if kind == "memory" {
                    "interpreter"
                } else {
                    engine.as_str()
                };
                let (ns, hash) = measure(&hir, engine, cycles);
                assert_eq!(
                    hash,
                    expected,
                    "{kind} {} sample {}",
                    engine.as_str(),
                    sample + 1
                );
                timings[index].push(ns);
                println!(
                    "{kind},{},{effective},{},{ns:.2},{hash:016x}",
                    engine.as_str(),
                    sample + 1
                );
            }
        }
        for (engine, timings) in engines.into_iter().zip(&mut timings) {
            timings.sort_by(f64::total_cmp);
            let middle = samples / 2;
            let median = if samples % 2 == 0 {
                (timings[middle - 1] + timings[middle]) / 2.0
            } else {
                timings[middle]
            };
            let effective = if kind == "memory" {
                "interpreter"
            } else {
                engine.as_str()
            };
            println!(
                "summary {kind} {} effective={effective}: min={:.2} median={median:.2} max={:.2} ns/cycle",
                engine.as_str(),
                timings[0],
                timings[samples - 1]
            );
        }
    }
}

//! Fixed-workload baseline. Run `cargo bench -p bitloom-sim --bench baseline`.
use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{Sim, TickEngine};
use std::{hint::black_box, time::Instant};

// A separate feature/build keeps allocator instrumentation out of normal timing.
#[cfg(feature = "bench-alloc")]
mod allocations {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
    pub struct Counting;
    static ENABLED: AtomicBool = AtomicBool::new(false);
    static CALLS: AtomicU64 = AtomicU64::new(0);
    static BYTES: AtomicU64 = AtomicU64::new(0);
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if ENABLED.load(Relaxed) {
                CALLS.fetch_add(1, Relaxed);
                BYTES.fetch_add(layout.size() as u64, Relaxed);
            }
            unsafe { System.alloc(layout) }
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            if ENABLED.load(Relaxed) {
                CALLS.fetch_add(1, Relaxed);
                BYTES.fetch_add(layout.size() as u64, Relaxed);
            }
            unsafe { System.alloc_zeroed(layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            if ENABLED.load(Relaxed) {
                CALLS.fetch_add(1, Relaxed);
                BYTES.fetch_add(size as u64, Relaxed);
            }
            unsafe { System.realloc(ptr, layout, size) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
    }
    pub fn start() {
        CALLS.store(0, Relaxed);
        BYTES.store(0, Relaxed);
        ENABLED.store(true, Relaxed);
    }
    pub fn stop() -> (u64, u64) {
        ENABLED.store(false, Relaxed);
        (CALLS.load(Relaxed), BYTES.load(Relaxed))
    }
}
#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOCATOR: allocations::Counting = allocations::Counting;

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
    if kind == "pipeline32" {
        for i in 0..32 {
            s.declare_reg(format!("q{i}"), GroundType::UInt { width: 32 }, p);
            s.declare_wire(format!("sum{i}"), GroundType::UInt { width: 32 }, p);
            s.declare_wire(format!("next{i}"), GroundType::UInt { width: 32 }, p);
        }
    }
    s.begin_sequential(p);
    s.assign_reg_d_inc("count", p);
    if kind == "pipeline32" {
        for i in 0..32 {
            s.assign_reg_d_from(format!("q{i}"), format!("next{i}"), p);
        }
    }
    if kind == "memory" {
        s.assign_mem_write("ram", "addr", "count", p);
    }
    s.end_process();
    s.begin_combinational(p);
    match kind {
        "counter" => s.assign_net("out", "count", p),
        "pipeline32" => {
            for i in (0..32).rev() {
                let previous = if i == 0 {
                    "count".into()
                } else {
                    format!("q{}", i - 1)
                };
                s.assign_xor(format!("next{i}"), format!("sum{i}"), "constant", p);
                s.assign_add(format!("sum{i}"), previous, "constant", p);
            }
            s.assign_net("out", "q31", p);
        }
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
    let mut pipeline = [0u32; 32];
    (1..=cycles).fold(0, |hash, cycle| {
        let mut value = cycle as u32;
        match kind {
            "pipeline32" => {
                let old = pipeline;
                for i in 0..32 {
                    let source = if i == 0 {
                        (cycle - 1) as u32
                    } else {
                        old[i - 1]
                    };
                    pipeline[i] = source.wrapping_add(0x9e37_79b9) ^ 0x9e37_79b9;
                }
                value = pipeline[31];
            }
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

#[inline(never)]
fn measure(hir: &FrozenHir, engine: TickEngine, cycles: u64) -> (f64, u64, u64, u64) {
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
    #[cfg(feature = "bench-alloc")]
    allocations::start();
    let start = Instant::now();
    for _ in 0..cycles {
        black_box(&mut sim).tick();
        hash = checksum_step(hash, sim.ports().get("out").unwrap());
    }
    let elapsed = start.elapsed().as_secs_f64();
    #[cfg(feature = "bench-alloc")]
    let (calls, bytes) = allocations::stop();
    #[cfg(not(feature = "bench-alloc"))]
    let (calls, bytes) = (0, 0);
    (elapsed * 1e9 / cycles as f64, black_box(hash), calls, bytes)
}

fn main() {
    let cycles = std::env::var("BITLOOM_BENCH_CYCLES")
        .map(|v| v.parse().unwrap())
        .unwrap_or(200_000);
    let samples: usize = std::env::var("BITLOOM_BENCH_SAMPLES")
        .map(|v| v.parse().unwrap())
        .unwrap_or(5);
    let warmup: u64 = std::env::var("BITLOOM_BENCH_WARMUP")
        .map(|v| v.parse().unwrap())
        .unwrap_or(20_000);
    let selected = std::env::var("BITLOOM_BENCH_WORKLOAD").unwrap_or_else(|_| "all".into());
    let selected_engine = std::env::var("BITLOOM_BENCH_ENGINE").unwrap_or_else(|_| "both".into());
    assert!(["all", "counter", "arithmetic", "memory", "pipeline32"].contains(&selected.as_str()));
    assert!(["both", "interpreter", "compiled"].contains(&selected_engine.as_str()));
    assert!(cycles > 0 && samples >= 3);
    println!(
        "cycles={cycles} samples={samples} warmup={warmup} profile=release order=alternating-pairs"
    );
    println!(
        "allocation_instrumentation={} (instrumented time is not native throughput)",
        cfg!(feature = "bench-alloc")
    );
    println!(
        "workload,requested_engine,effective_engine,sample,ns_per_cycle,checksum,allocation_calls,requested_bytes"
    );
    for kind in ["counter", "arithmetic", "memory", "pipeline32"] {
        if selected != "all" && selected != kind {
            continue;
        }
        let hir = design(kind);
        let expected = reference(kind, cycles);
        let engines = [TickEngine::Interpreter, TickEngine::Compiled];
        let mut timings = [Vec::new(), Vec::new()];
        // Warm both before measurement, then alternate which engine goes first
        // in each sample pair so gradual host drift is not tied to one engine.
        for engine in engines {
            if warmup == 0 || (selected_engine != "both" && selected_engine != engine.as_str()) {
                continue;
            }
            let (_, warm_hash, _, _) = measure(&hir, engine, warmup);
            assert_eq!(warm_hash, reference(kind, warmup));
        }
        for sample in 0..samples {
            let order = if sample % 2 == 0 { [0, 1] } else { [1, 0] };
            for index in order {
                let engine = engines[index];
                if selected_engine != "both" && selected_engine != engine.as_str() {
                    continue;
                }
                let effective = if kind == "memory" {
                    "interpreter"
                } else {
                    engine.as_str()
                };
                let (ns, hash, calls, bytes) = measure(&hir, engine, cycles);
                assert_eq!(
                    hash,
                    expected,
                    "{kind} {} sample {}",
                    engine.as_str(),
                    sample + 1
                );
                timings[index].push(ns);
                println!(
                    "{kind},{},{effective},{},{ns:.2},{hash:016x},{calls},{bytes}",
                    engine.as_str(),
                    sample + 1
                );
            }
        }
        for (engine, timings) in engines.into_iter().zip(&mut timings) {
            if timings.is_empty() {
                continue;
            }
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

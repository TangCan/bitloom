//! Story127.4 automate: bounded, legal delayed-responder public API replay.
//! The port oracle is independent of HIR operations and production address maps.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{Elaboratable, GroundType, ip::CsrDecoder};
use bitloom_sim::{Sim, TickEngine};
use std::{collections::BTreeMap, fmt::Write as _, fs};

#[path = "fr196_csr_decoder/common.rs"]
#[allow(dead_code)]
mod common;
use common::LEAVES;
type Values = BTreeMap<String, u64>;
const COMPLETIONS: usize = 128;
const MISS: usize = 4;

struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn inputs(&mut self) -> Values {
        let mut i = Values::from([
            ("rst".into(), 0),
            ("req_valid".into(), 0),
            ("write".into(), self.next() & 1),
            ("addr".into(), self.next() & 65535),
            ("wdata".into(), self.next() & 0xffff_ffff),
            ("wstrb".into(), self.next() & 15),
            ("rsp_ready".into(), 0),
        ]);
        for leaf in LEAVES {
            i.insert(format!("{leaf}_req_ready"), self.next() & 1);
            i.insert(format!("{leaf}_rsp_valid"), 0);
            i.insert(format!("{leaf}_rdata"), self.next() & 0xffff_ffff);
            i.insert(format!("{leaf}_error"), self.next() & 3);
        }
        i
    }
}

// One slot containing a committed port destination, or no outstanding request.
// Payloads are checked only while their valid is asserted; inactive payloads
// are intentionally absent from the expected map, not constrained to zero.
#[derive(Default)]
struct Oracle {
    slot: Option<usize>,
    completed: [usize; 5],
    completed_by_read_write_error: [[usize; 4]; 2],
    request_write: bool,
    canceled: [usize; 5],
    request_stalls: usize,
    response_waits: usize,
    response_stalls: usize,
}
impl Oracle {
    fn destination(addr: u64) -> usize {
        match addr {
            0x0000..=0x00ff => 0,
            0x0100..=0x01ff => 1,
            0x0200..=0x02ff => 2,
            0x0300..=0x03ff => 3,
            _ => MISS,
        }
    }
    fn edge(&mut self, i: &Values) -> Values {
        let reset = i["rst"] != 0;
        let target = Self::destination(i["addr"]);
        let ready = !reset
            && self.slot.is_none()
            && (target == MISS || i[&format!("{}_req_ready", LEAVES[target])] != 0);
        let response = match self.slot {
            Some(MISS) => Some((0, 3)),
            Some(owner) if i[&format!("{}_rsp_valid", LEAVES[owner])] != 0 => Some((
                i[&format!("{}_rdata", LEAVES[owner])],
                i[&format!("{}_error", LEAVES[owner])],
            )),
            _ => None,
        };
        let mut o = Values::from([
            ("req_ready".into(), u64::from(ready)),
            ("rsp_valid".into(), u64::from(response.is_some())),
        ]);
        if let Some((data, error)) = response {
            o.insert("rdata".into(), data);
            o.insert("error".into(), error);
        }
        for (owner, leaf) in LEAVES.iter().enumerate() {
            let valid = !reset && self.slot.is_none() && target == owner && i["req_valid"] != 0;
            o.insert(format!("{leaf}_req_valid"), u64::from(valid));
            o.insert(
                format!("{leaf}_rsp_ready"),
                u64::from(!reset && self.slot == Some(owner) && i["rsp_ready"] != 0),
            );
            if valid {
                o.insert(format!("{leaf}_addr"), i["addr"] - owner as u64 * 256);
                for field in ["write", "wdata", "wstrb"] {
                    o.insert(format!("{leaf}_{field}"), i[field]);
                }
            }
        }
        if reset {
            if let Some(owner) = self.slot.take() {
                self.canceled[owner] += 1;
            }
        } else if let Some(owner) = self.slot {
            if response.is_some() && i["rsp_ready"] != 0 {
                self.completed[owner] += 1;
                self.completed_by_read_write_error[usize::from(self.request_write)]
                    [response.unwrap().1 as usize] += 1;
                self.slot = None;
            } else if response.is_some() {
                self.response_stalls += 1;
            } else {
                self.response_waits += 1;
            }
        } else if i["req_valid"] != 0 {
            if ready {
                self.slot = Some(target);
                self.request_write = i["write"] != 0;
            } else {
                self.request_stalls += 1;
            }
        }
        o
    }
}

struct Replay {
    seed: u64,
    rng: Random,
    oracle: Oracle,
    sims: Vec<(TickEngine, Sim)>,
    trace: Vec<(Values, Values)>,
}
impl Replay {
    fn new(seed: u64) -> Self {
        let mut r = Self {
            seed,
            rng: Random(seed),
            oracle: Oracle::default(),
            trace: vec![],
            sims: [TickEngine::Interpreter, TickEngine::Compiled]
                .into_iter()
                .map(|engine| {
                    (
                        engine,
                        Sim::with_engine(CsrDecoder::elaborate().unwrap(), engine),
                    )
                })
                .collect(),
        };
        // Establish the synchronous reset edge before comparing any outputs.
        let mut reset = r.rng.inputs();
        reset.insert("rst".into(), 1);
        for (_, sim) in &mut r.sims {
            let mut p = PortValues::default();
            for (n, v) in &reset {
                p.set(n, *v);
            }
            sim.set_inputs(p);
            sim.tick();
        }
        r
    }
    fn step(&mut self, i: Values) {
        let expected = self.oracle.edge(&i);
        for (engine, sim) in &mut self.sims {
            let mut p = PortValues::default();
            for (n, v) in &i {
                p.set(n, *v);
            }
            sim.set_inputs(p);
            sim.settle();
            for (name, value) in &expected {
                assert_eq!(
                    sim.ports().get(name),
                    Some(*value),
                    "seed={:016x} cycle={} engine={engine:?} port={name}",
                    self.seed,
                    self.trace.len()
                );
            }
            sim.tick();
        }
        self.trace.push((i, expected));
    }
    fn idle(&mut self) {
        let i = self.rng.inputs();
        self.step(i);
    }
    // cancel=None completes; Some(false/true) cancels waiting/visible response.
    fn transaction(&mut self, owner: usize, cancel: Option<bool>) {
        assert!(self.oracle.slot.is_none());
        let mut request = self.rng.inputs();
        let addr = if owner == MISS {
            0x400 + self.rng.next() % 0xfc00
        } else {
            owner as u64 * 256 + (self.rng.next() & 255)
        };
        request.insert("req_valid".into(), 1);
        request.insert("addr".into(), addr);
        if owner != MISS {
            request.insert(format!("{}_req_ready", LEAVES[owner]), 0);
            for _ in 0..1 + self.rng.next() % 9 {
                self.step(request.clone());
            }
            request.insert(format!("{}_req_ready", LEAVES[owner]), 1);
        }
        let write = request["write"] != 0;
        self.step(request);
        assert_eq!(
            self.oracle.slot,
            Some(owner),
            "request must be accepted before leaf responds"
        );
        if owner != MISS {
            for _ in 0..17 + self.rng.next() % 48 {
                let mut i = self.rng.inputs();
                i.insert("rsp_ready".into(), self.rng.next() & 1);
                self.step(i);
            }
        }
        let arbitrary_data = self.rng.next() & 0xffff_ffff;
        let error = [0, 2, 3][(self.rng.next() % 3) as usize];
        // Legal visible response: failed reads return zero; failed writes may
        // carry arbitrary data. Invalid leaf payloads remain unconstrained.
        let data = if !write && error != 0 {
            0
        } else {
            arbitrary_data
        };
        let visible = cancel != Some(false);
        let stall_cycles = 1 + self.rng.next() % 17;
        // Peer holds response data/valid through every stalled cycle and the
        // eventual consumption/reset edge. Other inactive payloads keep moving.
        for cycle in 0..=stall_cycles {
            let mut i = self.rng.inputs();
            if owner != MISS && visible {
                i.insert(format!("{}_rsp_valid", LEAVES[owner]), 1);
                i.insert(format!("{}_rdata", LEAVES[owner]), data);
                i.insert(format!("{}_error", LEAVES[owner]), error);
            }
            if cycle == stall_cycles {
                i.insert("rsp_ready".into(), 1);
                i.insert("rst".into(), u64::from(cancel.is_some()));
            }
            self.step(i);
        }
        assert!(self.oracle.slot.is_none());
        // Recovery also rejects late duplicate responses after cancel/consume.
        for _ in 0..3 {
            self.idle();
        }
    }

    fn rtl(&self) {
        let hir = CsrDecoder::elaborate().unwrap();
        assert_eq!(hir.circuit().modules.len(), 1);
        let ports = &hir.circuit().modules[0].ports;
        let inputs: Vec<_> = ports
            .iter()
            .filter(|p| p.direction == PortDirection::Input && p.name != "clk")
            .collect();
        let outputs: Vec<_> = ports
            .iter()
            .filter(|p| p.direction == PortDirection::Output)
            .collect();
        let mut tb = String::from("module tb;\nreg clk=0;\n");
        for p in ports.iter().filter(|p| p.name != "clk") {
            let width = match p.ty {
                GroundType::UInt { width } => width,
                _ => 1,
            };
            let input = p.direction == PortDirection::Input;
            writeln!(
                tb,
                "{} [{}:0] {}{};",
                if input { "reg" } else { "wire" },
                width - 1,
                p.name,
                if input { "=0" } else { "" }
            )
            .unwrap();
        }
        writeln!(tb, "CsrDecoder dut(.*);\ninteger fd,n,cycle,j;\nreg [63:0] stimulus[0:{}];\nreg [63:0] expected[0:{}];\nreg masks[0:{}];",
            inputs.len()-1, outputs.len()-1, outputs.len()-1).unwrap();
        tb.push_str("initial begin\nrst=1;#2;clk=1;#2;clk=0;#2;\nfd=$fopen(\"trace.txt\",\"r\");if(!fd)$fatal(1,\"missing replay\");\n");
        writeln!(
            tb,
            "for(cycle=0;cycle<{};cycle=cycle+1) begin",
            self.trace.len()
        )
        .unwrap();
        writeln!(tb, "for(j=0;j<{};j=j+1) begin n=$fscanf(fd,\"%h\",stimulus[j]);if(n!=1)$fatal(1,\"short input trace\");end", inputs.len()).unwrap();
        writeln!(tb, "for(j=0;j<{};j=j+1) begin n=$fscanf(fd,\"%h %h\",expected[j],masks[j]);if(n!=2)$fatal(1,\"short expected trace\");end", outputs.len()).unwrap();
        for (j, p) in inputs.iter().enumerate() {
            writeln!(tb, "{}=stimulus[{j}];", p.name).unwrap();
        }
        tb.push_str("#1;\n");
        for (j, p) in outputs.iter().enumerate() {
            writeln!(tb, "if(masks[{j}] && {} !== expected[{j}]) $fatal(1,\"seed={:016x} cycle=%0d port={} actual=%h expected=%h\",cycle,{},expected[{j}]);",
                p.name, self.seed, p.name, p.name).unwrap();
        }
        tb.push_str("#1;clk=1;#2;clk=0;#2;\nend\n$fclose(fd);$display(\"DELAYED REPLAY PASS\");$finish;end\nendmodule\n");
        let mut trace = String::new();
        for (i, expected) in &self.trace {
            for p in &inputs {
                write!(trace, "{:x} ", i[&p.name]).unwrap();
            }
            for p in &outputs {
                write!(
                    trace,
                    "{:x} {:x} ",
                    expected.get(&p.name).copied().unwrap_or(0),
                    u64::from(expected.contains_key(&p.name))
                )
                .unwrap();
            }
            trace.push('\n');
        }
        let dir = common::dir(&format!("automate-delayed-{:016x}", self.seed));
        fs::write(
            dir.join("design.v"),
            &bitloom_vlog::emit(&hir).files[0].contents,
        )
        .unwrap();
        fs::write(dir.join("tb.sv"), tb).unwrap();
        fs::write(dir.join("trace.txt"), trace).unwrap();
        common::run(
            &dir,
            "iverilog",
            &[
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
            "compile",
        );
        assert!(common::run(&dir, "vvp", &["simulation"], "run").contains("DELAYED REPLAY PASS"));
        fs::write(
            dir.join("coverage.json"),
            serde_json::json!({
                "seed":format!("{:016x}",self.seed), "cycles":self.trace.len(),
                "completed_by_uart_gpio_timer_irq_miss":self.oracle.completed,
                "completed_by_read_write_error_0_1_2_3":self.oracle.completed_by_read_write_error,
                "canceled_by_uart_gpio_timer_irq_miss":self.oracle.canceled,
                "request_stall_cycles":self.oracle.request_stalls,
                "leaf_wait_cycles":self.oracle.response_waits,
                "response_stall_cycles":self.oracle.response_stalls,
                "engines":["Interpreter","Compiled","iverilog/vvp"]
            })
            .to_string(),
        )
        .unwrap();
    }
}

fn replay(seed: u64) {
    let mut r = Replay::new(seed);
    for n in 0..COMPLETIONS {
        // Balanced scheduling guarantees every destination independently of RNG.
        r.transaction(n % 5, None);
        if n < 9 {
            let owner = n / 2;
            r.transaction(owner, Some(n % 2 == 1 || owner == MISS));
        }
    }
    for _ in 0..31 {
        r.idle();
    }
    assert_eq!(r.oracle.completed.iter().sum::<usize>(), COMPLETIONS);
    assert_eq!(r.oracle.completed, [26, 26, 26, 25, 25]);
    for errors in r.oracle.completed_by_read_write_error {
        assert_eq!(errors[1], 0, "reserved EXOKAY is not a legal CSR response");
        for code in [0, 2, 3] {
            assert!(
                errors[code] > 0,
                "each read/write class must exercise error {code}"
            );
        }
    }
    assert_eq!(r.oracle.canceled, [2, 2, 2, 2, 1]);
    assert!(
        r.oracle.request_stalls > 0 && r.oracle.response_waits > 0 && r.oracle.response_stalls > 0
    );
    r.rtl();
}

#[test]
fn p0_delayed_replay_seed_12740001() {
    replay(0x1274_0001);
}
#[test]
fn p0_delayed_replay_seed_a55a1234() {
    replay(0xa55a_1234);
}
#[test]
fn p0_delayed_replay_seed_deadbeef() {
    replay(0xdead_beef);
}
#[test]
fn p0_delayed_replay_seed_5eedcafef00d() {
    replay(0x5eed_cafe_f00d);
}

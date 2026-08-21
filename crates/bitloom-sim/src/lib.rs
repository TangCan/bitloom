//! Cycle-accurate simulation over FrozenHir (AD-5, AD-15, AD-17).

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use bitloom_hir::{AssignExpr, AssignTarget, FrozenHir, GroundType, PortValues, ProcessKind, Stmt};

pub use bitloom_hir::PortValues as Values;

mod coverage;
pub use coverage::{Coverage, parse_report};
mod engine;
pub use engine::TickEngine;
mod equiv;
pub use equiv::{
    EquivStatus, check_functional_equiv, check_functional_equiv_generated, reset_then_run,
};

mod generate;
pub use generate::{
    GeneratedFunctional, emit_functional_crate, generate_functional_sim,
    generate_functional_sim_with_bin,
};

mod cycle;
pub use cycle::{
    CycleAccurateSim, check_generated_bridge, check_generated_bridge_with,
    emit_cycle_accurate_crate, generate_cycle_accurate_sim,
};

mod fst;
pub use fst::{FstError, resolve_vcd2fst};

/// Simulator state for one FrozenHir circuit.
pub struct Sim {
    hir: FrozenHir,
    regs: BTreeMap<String, u64>,
    mems: BTreeMap<String, Vec<u64>>,
    /// SyncReadMem: data captured last cycle, applied this cycle (latency 1).
    pending_mem_reads: BTreeMap<String, u64>,
    ports: PortValues,
    time: u64,
    vcd: Option<VcdWriter>,
    fst: Option<FstPlan>,
    engine: TickEngine,
    kernel: engine::CompiledKernel,
    coverage: Coverage,
}

struct FstPlan {
    converter: PathBuf,
    vcd: PathBuf,
    fst: PathBuf,
}

struct VcdWriter {
    file: File,
    vars: Vec<String>,
}

impl Sim {
    pub fn new(hir: FrozenHir) -> Self {
        Self::with_engine(hir, TickEngine::Interpreter)
    }

    pub fn with_engine(hir: FrozenHir, engine: TickEngine) -> Self {
        let mut regs = BTreeMap::new();
        let mut mems = BTreeMap::new();
        for m in &hir.circuit().modules {
            for stmt in &m.body {
                match stmt {
                    Stmt::RegDecl { name, .. } => {
                        regs.insert(name.clone(), 0);
                    }
                    Stmt::MemDecl {
                        name, depth, width, ..
                    } => {
                        let _ = width;
                        mems.insert(name.clone(), vec![0; *depth as usize]);
                    }
                    _ => {}
                }
            }
        }
        let kernel = engine::compile(&hir);
        Self {
            hir,
            regs,
            mems,
            pending_mem_reads: BTreeMap::new(),
            ports: PortValues::default(),
            time: 0,
            vcd: None,
            fst: None,
            engine,
            kernel,
            coverage: Coverage::default(),
        }
    }

    pub fn engine(&self) -> TickEngine {
        self.engine
    }

    pub fn enable_vcd(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "$timescale 1ns $end")?;
        writeln!(file, "$scope module {} $end", self.hir.abi_name)?;
        let mut vars = Vec::new();
        if let Some(m) = self.hir.circuit().modules.first() {
            for p in &m.ports {
                let w = match &p.ty {
                    GroundType::UInt { width } | GroundType::SInt { width } => *width,
                    _ => 1,
                };
                writeln!(file, "$var wire {w} {} {} $end", p.name, p.name)?;
                vars.push(p.name.clone());
            }
            for stmt in &m.body {
                if let Stmt::RegDecl { name, ty, .. } = stmt {
                    let w = match ty {
                        GroundType::UInt { width } | GroundType::SInt { width } => *width,
                        _ => 1,
                    };
                    writeln!(file, "$var reg {w} {name} {name} $end")?;
                    vars.push(name.clone());
                }
            }
        }
        writeln!(file, "$upscope $end")?;
        writeln!(file, "$enddefinitions $end")?;
        self.vcd = Some(VcdWriter { file, vars });
        self.dump_vcd()?;
        Ok(())
    }

    /// Optional FST: still dumps VCD, then converts with `vcd2fst` (AD-24).
    pub fn enable_fst(&mut self, path: impl AsRef<Path>) -> Result<(), FstError> {
        let converter = resolve_vcd2fst()?;
        self.enable_fst_with(path, converter)
    }

    /// Same as `enable_fst` with an explicit converter (tests / `RHDL_VCD2FST`).
    pub fn enable_fst_with(
        &mut self,
        path: impl AsRef<Path>,
        converter: impl AsRef<Path>,
    ) -> Result<(), FstError> {
        let fst = path.as_ref().to_path_buf();
        let vcd = fst.with_extension("vcd");
        self.enable_vcd(&vcd)?;
        self.fst = Some(FstPlan {
            converter: converter.as_ref().to_path_buf(),
            vcd,
            fst,
        });
        Ok(())
    }

    /// Flush VCD and, if FST was requested, run the documented converter.
    pub fn finish_waves(&mut self) -> Result<(), FstError> {
        if let Some(mut w) = self.vcd.take() {
            w.file.flush()?;
        }
        if let Some(plan) = self.fst.take() {
            fst::convert_vcd_to_fst(&plan.converter, &plan.vcd, &plan.fst)?;
        }
        Ok(())
    }

    fn dump_vcd(&mut self) -> io::Result<()> {
        let Some(vcd) = self.vcd.as_mut() else {
            return Ok(());
        };
        writeln!(vcd.file, "#{}", self.time)?;
        for name in &vcd.vars {
            let val = self
                .ports
                .get(name)
                .or_else(|| self.regs.get(name).copied())
                .unwrap_or(0);
            writeln!(vcd.file, "b{val:b} {name}")?;
        }
        Ok(())
    }

    pub fn set_inputs(&mut self, inputs: PortValues) {
        for (k, v) in inputs.values {
            self.ports.set(k, v);
        }
    }

    pub fn ports(&self) -> &PortValues {
        &self.ports
    }

    fn lookup(&self, name: &str) -> u64 {
        self.ports
            .get(name)
            .or_else(|| self.regs.get(name).copied())
            .unwrap_or(0)
    }

    fn eval(&self, expr: &AssignExpr) -> u64 {
        match expr {
            AssignExpr::Ref(n) => self.lookup(n),
            AssignExpr::Lit(v) => *v,
            AssignExpr::Inc(n) => self.lookup(n).wrapping_add(1),
            AssignExpr::Add(a, b) => self.lookup(a).wrapping_add(self.lookup(b)),
            AssignExpr::Sub(a, b) => self.lookup(a).wrapping_sub(self.lookup(b)),
            AssignExpr::And(a, b) => self.lookup(a) & self.lookup(b),
            AssignExpr::Or(a, b) => self.lookup(a) | self.lookup(b),
            AssignExpr::Xor(a, b) => self.lookup(a) ^ self.lookup(b),
            AssignExpr::Shl(a, b) => self.lookup(a) << (self.lookup(b) & 63),
            AssignExpr::Shr(a, b) => self.lookup(a) >> (self.lookup(b) & 63),
            AssignExpr::Eq(a, b) => u64::from(self.lookup(a) == self.lookup(b)),
            AssignExpr::Mux { sel, t, f } => {
                if self.lookup(sel) != 0 {
                    self.lookup(t)
                } else {
                    self.lookup(f)
                }
            }
            AssignExpr::MemRead { mem, addr } => {
                let a = self.lookup(addr) as usize;
                self.mems
                    .get(mem)
                    .and_then(|m| m.get(a).copied())
                    .unwrap_or(0)
            }
        }
    }

    fn reg_meta(&self, name: &str) -> (bool, bool) {
        for m in &self.hir.circuit().modules {
            for stmt in &m.body {
                if let Stmt::RegDecl {
                    name: n,
                    async_reset,
                    has_enable,
                    ..
                } = stmt
                {
                    if n == name {
                        return (*async_reset, *has_enable);
                    }
                }
            }
        }
        (false, false)
    }

    fn mem_is_sync(&self, name: &str) -> bool {
        for m in &self.hir.circuit().modules {
            for stmt in &m.body {
                if let Stmt::MemDecl {
                    name: n, sync_read, ..
                } = stmt
                {
                    if n == name {
                        return *sync_read;
                    }
                }
            }
        }
        false
    }

    fn enable_active(&self) -> bool {
        let has_en_port = self
            .hir
            .circuit()
            .modules
            .iter()
            .any(|m| m.ports.iter().any(|p| p.name == "en"));
        if has_en_port {
            self.lookup("en") != 0
        } else {
            true
        }
    }

    fn reset_active(&self, m: &bitloom_hir::Module) -> bool {
        let rst = m
            .ports
            .iter()
            .find(|p| matches!(p.ty, GroundType::Reset))
            .map(|p| p.name.as_str())
            .unwrap_or("rst");
        self.lookup(rst) != 0
    }

    /// One rising edge of the module Clock (AD-15). Driven by FrozenHir assigns.
    pub fn tick(&mut self) {
        match self.engine {
            TickEngine::Interpreter => self.tick_interpreter(),
            TickEngine::Compiled => self.tick_compiled(),
        }
        self.sample_coverage();
        self.time += 1;
        let _ = self.dump_vcd();
    }

    fn sample_coverage(&mut self) {
        let names: Vec<String> = {
            let mut n = Vec::new();
            if let Some(m) = self.hir.circuit().modules.first() {
                for p in &m.ports {
                    n.push(p.name.clone());
                }
                for stmt in &m.body {
                    if let Stmt::RegDecl { name, .. } = stmt {
                        n.push(name.clone());
                    }
                }
            }
            n
        };
        for name in names {
            let val = self.lookup(&name);
            self.coverage.sample(name, val);
        }
    }

    pub fn coverage_report(&self) -> String {
        self.coverage.report()
    }

    fn tick_interpreter(&mut self) {
        self.tick_sequential();
        self.tick_combinational();
    }

    fn tick_compiled(&mut self) {
        // Compiled schedule covers plain RegD/Net; mem / enable share the interpreter path.
        if self.mems.is_empty()
            && !self.hir.circuit().modules.iter().any(|m| {
                m.body.iter().any(|s| {
                    matches!(
                        s,
                        Stmt::RegDecl {
                            has_enable: true,
                            ..
                        }
                    )
                })
            })
        {
            let Some(m) = self.hir.circuit().modules.first().cloned() else {
                return;
            };
            let reset = self.reset_active(&m);
            let seq = self.kernel.seq.clone();
            let comb = self.kernel.comb.clone();
            for (name, expr) in seq {
                let next = if reset { 0 } else { self.eval(&expr) };
                self.regs.insert(name, next);
            }
            for (name, expr) in comb {
                let val = self.eval(&expr);
                self.ports.set(name, val);
            }
            return;
        }
        self.tick_interpreter();
    }

    fn tick_sequential(&mut self) {
        let Some(m) = self.hir.circuit().modules.first().cloned() else {
            return;
        };
        let reset = self.reset_active(&m);
        let enable = self.enable_active();

        // Apply SyncReadMem pending data from previous cycle (latency 1).
        let pending = std::mem::take(&mut self.pending_mem_reads);
        for (name, val) in pending {
            if reset {
                self.regs.insert(name, 0);
            } else {
                self.regs.insert(name, val);
            }
        }

        let mut next_pending = BTreeMap::new();
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                if p.kind != ProcessKind::Sequential {
                    continue;
                }
                for a in &p.assigns {
                    match &a.target {
                        AssignTarget::RegD(name) => {
                            let (_async_rst, has_en) = self.reg_meta(name);
                            if reset {
                                self.regs.insert(name.clone(), 0);
                                continue;
                            }
                            if has_en && !enable {
                                continue;
                            }
                            match &a.expr {
                                AssignExpr::MemRead { mem, addr } if self.mem_is_sync(mem) => {
                                    let val = self.eval(&AssignExpr::MemRead {
                                        mem: mem.clone(),
                                        addr: addr.clone(),
                                    });
                                    next_pending.insert(name.clone(), val);
                                }
                                _ => {
                                    let next = self.eval(&a.expr);
                                    self.regs.insert(name.clone(), next);
                                }
                            }
                        }
                        AssignTarget::MemWrite { mem, addr, we } => {
                            if reset {
                                continue;
                            }
                            if let Some(en) = we {
                                if self.lookup(en) == 0 {
                                    continue;
                                }
                            }
                            let a_idx = self.lookup(addr) as usize;
                            let data = self.eval(&a.expr);
                            if let Some(bank) = self.mems.get_mut(mem) {
                                if a_idx < bank.len() {
                                    bank[a_idx] = data;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        self.pending_mem_reads = next_pending;
    }

    fn tick_combinational(&mut self) {
        let Some(m) = self.hir.circuit().modules.first().cloned() else {
            return;
        };
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                if p.kind == ProcessKind::Combinational {
                    for a in &p.assigns {
                        if let AssignTarget::Net(name) = &a.target {
                            let val = self.eval(&a.expr);
                            self.ports.set(name.clone(), val);
                        }
                    }
                }
            }
        }
    }
}

/// A handwritten abstraction/bridge cycle (FR29). Compared only via `PortValues`.
pub trait AbstractionView {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues;
}

/// One named port that differed between two views.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortMismatch {
    pub name: String,
    pub left: Option<u64>,
    pub right: Option<u64>,
}

/// Compare `PortValues` on the intersection of keys (FR29: PortValues only, no TLM).
pub fn compare_port_values(left: &PortValues, right: &PortValues) -> Result<(), Vec<PortMismatch>> {
    let mut mismatches = Vec::new();
    for name in left.values.keys() {
        if !right.values.contains_key(name) {
            continue;
        }
        let l = left.get(name);
        let r = right.get(name);
        if l != r {
            mismatches.push(PortMismatch {
                name: name.clone(),
                left: l,
                right: r,
            });
        }
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(mismatches)
    }
}

/// Tick RTL and one handwritten view; fail when documented ports disagree.
pub fn check_mixed_both<A: AbstractionView>(
    sim: &mut Sim,
    abs: &mut A,
    inputs: PortValues,
) -> Result<(), Vec<PortMismatch>> {
    sim.set_inputs(inputs.clone());
    sim.tick();
    let abs_out = abs.cycle(&inputs);
    compare_port_values(sim.ports(), &abs_out)
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;

    fn counter_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Counter", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    fn passthrough_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Pass", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn tick_counts_after_reset() {
        let mut sim = Sim::new(counter_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(3));
    }

    #[test]
    fn tick_passthrough_non_counter() {
        let mut sim = Sim::new(passthrough_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("data_in", 0xA5);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0xA5));
        let mut pv2 = PortValues::default();
        pv2.set("data_in", 0x3C);
        sim.set_inputs(pv2);
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0x3C));
    }

    fn adder_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Add8", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_add("y", "a", "b", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn tick_same_width_add() {
        let mut sim = Sim::new(adder_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("a", 3);
        pv.set("b", 5);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("y"), Some(8));
    }

    #[test]
    fn vcd_written() {
        let dir = std::env::temp_dir().join("rhdl_sim_vcd_test.vcd");
        let mut sim = Sim::new(counter_hir());
        sim.enable_vcd(&dir).unwrap();
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        let text = std::fs::read_to_string(&dir).unwrap();
        assert!(text.contains("$var"));
        assert!(text.contains("count"));
        let _ = std::fs::remove_file(dir);
    }

    #[test]
    fn fst_off_still_writes_vcd() {
        let dir = std::env::temp_dir().join("rhdl_sim_fst_off.vcd");
        let mut sim = Sim::new(counter_hir());
        sim.enable_vcd(&dir).unwrap();
        sim.tick();
        let text = std::fs::read_to_string(&dir).unwrap();
        assert!(text.contains("$var"));
        assert!(!dir.with_extension("fst").is_file());
        let _ = std::fs::remove_file(&dir);
    }

    #[test]
    fn fst_missing_converter_errors() {
        let err =
            crate::fst::resolve_vcd2fst_from(None, Some("/no-such-rhdl-path".into())).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("vcd2fst"));
    }

    #[test]
    fn fst_via_documented_converter() {
        let tmp = std::env::temp_dir().join("rhdl_fst_stub");
        let _ = std::fs::create_dir_all(&tmp);
        let stub = tmp.join("vcd2fst");
        std::fs::write(&stub, "#!/bin/sh\ncp \"$1\" \"$2\"\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut p = std::fs::metadata(&stub).unwrap().permissions();
            p.set_mode(0o755);
            std::fs::set_permissions(&stub, p).unwrap();
        }
        let fst = tmp.join("wave.fst");
        let mut sim = Sim::new(counter_hir());
        sim.enable_fst_with(&fst, &stub).unwrap();
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.finish_waves().unwrap();
        assert!(fst.is_file());
        assert!(fst.with_extension("vcd").is_file());
        let _ = std::fs::remove_file(&fst);
        let _ = std::fs::remove_file(fst.with_extension("vcd"));
    }

    #[test]
    fn functional_model_matches_tick_portvalues() {
        struct CounterFm {
            count: u64,
        }
        impl CounterFm {
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {
                if inputs.get("rst").unwrap_or(0) != 0 {
                    self.count = 0;
                } else {
                    self.count = self.count.wrapping_add(1);
                }
                let mut out = inputs.clone();
                out.set("data_out", self.count);
                out
            }
        }

        let mut sim = Sim::new(counter_hir());
        let mut fm = CounterFm { count: 0 };
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        let _ = fm.cycle(&pv);
        pv.set("rst", 0);
        for _ in 0..5 {
            sim.set_inputs(pv.clone());
            sim.tick();
            let fm_out = fm.cycle(&pv);
            assert_eq!(sim.ports().get("data_out"), fm_out.get("data_out"));
        }
    }

    struct CounterAbs {
        count: u64,
    }
    impl AbstractionView for CounterAbs {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            if inputs.get("rst").unwrap_or(0) != 0 {
                self.count = 0;
            } else {
                self.count = self.count.wrapping_add(1);
            }
            let mut out = inputs.clone();
            out.set("data_out", self.count);
            out
        }
    }

    #[test]
    fn mixed_both_portvalues_match() {
        let mut sim = Sim::new(counter_hir());
        let mut abs = CounterAbs { count: 0 };
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
        pv.set("rst", 0);
        for _ in 0..4 {
            check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
        }
        assert_eq!(sim.ports().get("data_out"), Some(4));
    }

    #[test]
    fn mixed_both_mismatch_fails() {
        struct WrongAbs;
        impl AbstractionView for WrongAbs {
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {
                let mut out = inputs.clone();
                out.set("data_out", 99);
                out
            }
        }
        let mut sim = Sim::new(counter_hir());
        let mut abs = WrongAbs;
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        let err = check_mixed_both(&mut sim, &mut abs, pv).unwrap_err();
        assert!(err.iter().any(|m| m.name == "data_out"));
    }

    #[test]
    fn no_hir_to_tlm_api() {
        // No SystemC TLM-2.0 / emit_tlm product API on the sim surface (AD-5 / FR29).
        // This does *not* ban FR47 Rust functional-sim *generation* (Epic 21.3+).
        // Scan only the production surface above `#[cfg(test)]`.
        let src = include_str!("lib.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(!prod.contains("emit_tlm"));
        assert!(!prod.contains("to_tlm"));
        assert!(!prod.contains("TLM-2.0"));
        assert!(!prod.contains("systemc"));
    }

    #[test]
    fn dual_view_equiv_pass() {
        let mut abs = CounterAbs { count: 0 };
        let st = check_functional_equiv(counter_hir(), &mut abs, reset_then_run(5));
        assert!(st.is_pass());
        assert_eq!(st, EquivStatus::Pass { cycles: 6 });
    }

    #[test]
    fn dual_view_equiv_fail_on_deliberate_mismatch() {
        struct WrongAbs;
        impl AbstractionView for WrongAbs {
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {
                let mut out = inputs.clone();
                out.set("data_out", 42);
                out
            }
        }
        let st = check_functional_equiv(counter_hir(), &mut WrongAbs, reset_then_run(2));
        assert!(!st.is_pass());
        match st {
            EquivStatus::Fail { mismatches, .. } => {
                assert!(mismatches.iter().any(|m| m.name == "data_out"));
            }
            EquivStatus::Pass { .. } => panic!("expected fail"),
        }
    }

    fn collect_trace(engine: TickEngine, n: usize) -> Vec<Option<u64>> {
        let mut sim = Sim::with_engine(counter_hir(), engine);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        let mut out = vec![sim.ports().get("data_out")];
        for _ in 0..n {
            sim.set_inputs(pv.clone());
            sim.tick();
            out.push(sim.ports().get("data_out"));
        }
        out
    }

    #[test]
    fn interpreter_and_compiled_portvalues_match() {
        assert_eq!(
            collect_trace(TickEngine::Interpreter, 6),
            collect_trace(TickEngine::Compiled, 6)
        );
        assert_eq!(
            TickEngine::from_name("compiled"),
            Some(TickEngine::Compiled)
        );
        let mut a = Sim::with_engine(passthrough_hir(), TickEngine::Interpreter);
        let mut b = Sim::with_engine(passthrough_hir(), TickEngine::Compiled);
        let mut pv = PortValues::default();
        pv.set("data_in", 0x5A);
        a.set_inputs(pv.clone());
        b.set_inputs(pv);
        a.tick();
        b.tick();
        assert_eq!(a.ports().get("data_out"), b.ports().get("data_out"));
        assert_eq!(a.engine().as_str(), "interpreter");
    }

    #[test]
    fn coverage_has_hit_and_miss() {
        let mut sim = Sim::new(counter_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        let report = sim.coverage_report();
        assert!(report.starts_with("# bitloom-sim coverage v1"));
        let (hits, misses) = parse_report(&report);
        assert!(
            hits.iter().any(|h| h == "data_out" || h == "count"),
            "hits={hits:?}"
        );
        assert!(
            misses.iter().any(|m| m == "data_in" || m == "clk"),
            "misses={misses:?}"
        );
    }

    fn sync_read_mem_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Srm", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("addr", GroundType::UInt { width: 4 }, Span::default());
        s.add_input("wdata", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("we", GroundType::Bool, Span::default());
        s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());
        s.declare_sync_read_mem("ram", 16, 8, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("rdata", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        // Always write for this golden; we gate with we in a fuller surface later.
        s.assign_mem_write("ram", "addr", "wdata", Span::default());
        s.assign_reg_d_mem_read("q", "ram", "addr", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn sync_read_mem_read_latency_one() {
        let mut sim = Sim::new(sync_read_mem_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("addr", 3);
        pv.set("wdata", 0xAB);
        pv.set("we", 1);
        sim.set_inputs(pv.clone());
        sim.tick(); // write + schedule read of old (0)
        assert_eq!(sim.ports().get("rdata"), Some(0));
        sim.tick(); // pending read delivers written value
        assert_eq!(sim.ports().get("rdata"), Some(0xAB));
    }

    fn async_reset_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("AsyncCnt", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg_ex(
            "count",
            GroundType::UInt { width: 8 },
            true,
            false,
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn async_reset_assert_and_release_tick_golden() {
        let mut sim = Sim::new(async_reset_hir());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(2));
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0));
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(2));
    }

    fn enable_hir(has_enable: bool) -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("EnCnt", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("en", GroundType::Bool, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg_ex(
            "count",
            GroundType::UInt { width: 8 },
            false,
            has_enable,
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn enable_high_matches_ungated_counter() {
        let mut gated = Sim::new(enable_hir(true));
        let mut plain = Sim::new(enable_hir(false));
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("en", 1);
        for _ in 0..4 {
            gated.set_inputs(pv.clone());
            plain.set_inputs(pv.clone());
            gated.tick();
            plain.tick();
            assert_eq!(gated.ports().get("data_out"), plain.ports().get("data_out"));
        }
    }

    #[test]
    fn enable_low_holds_value() {
        let mut sim = Sim::new(enable_hir(true));
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("en", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(2));
        pv.set("en", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(2));
    }
}

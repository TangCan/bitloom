//! Cycle-accurate simulation over FrozenHir (AD-5, AD-15, AD-17).

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bitloom_hir::{AssignExpr, AssignTarget, FrozenHir, GroundType, PortValues, ProcessKind, Stmt};

pub use bitloom_hir::PortValues as Values;

mod coverage;
pub use coverage::{
    Coverage, parse_branch_report, parse_report, parse_state_report, write_coverage_artifacts,
};
mod engine;
use engine::ResolvedExpr;
pub use engine::TickEngine;
mod equiv;
pub use equiv::{
    EquivStatus, check_functional_equiv, check_functional_equiv_generated, reset_then_run,
};

mod shared_stimulus;
pub use shared_stimulus::SharedStimulusScoreboard;

mod formal_equiv;
pub use formal_equiv::FormalEquivProduct;

mod ip_dual;
pub use ip_dual::{
    Axi4LiteSlaveFunctional, GpioFunctional, I2cMasterFunctional, IpDualModelMatrix,
    SpiMasterFunctional, SyncFifoFunctional, UartRxFunctional, UartTxFunctional,
    axi4_lite_slave_dual_stimulus, gpio_dual_stimulus, i2c_master_dual_stimulus,
    spi_master_dual_stimulus, sync_fifo_dual_stimulus, uart_rx_dual_stimulus,
    uart_tx_dual_stimulus,
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

mod systemc_tlm;
pub use systemc_tlm::{
    SYSTEMC_PIN_VERSION, SystemcToolchain, build_and_run_tlm_lt_smoke, emit_systemc_tlm_lt,
    generate_systemc_tlm_lt, resolve_systemc,
};
mod systemc_tlm_at;
pub use systemc_tlm_at::{
    build_and_run_tlm_at_smoke, emit_systemc_tlm_at, generate_systemc_tlm_at,
};

/// Simulator state for one FrozenHir circuit.
///
/// Values use `u64` storage. Wider HIR signals retain their low 64 bits on
/// passthrough and writeback; this is not arbitrary-width integer simulation.
pub struct Sim {
    hir: Arc<FrozenHir>,
    widths: BTreeMap<String, u32>,
    signal_ids: HashMap<String, usize>,
    signal_values: Vec<u64>,
    regs: BTreeMap<String, u64>,
    mems: Vec<Vec<u64>>,
    memory_ids: HashMap<String, usize>,
    /// SyncReadMem: data captured last cycle, applied this cycle (latency 1).
    pending_mem_reads: BTreeMap<String, u64>,
    ports: PortValues,
    time: u64,
    vcd: Option<VcdWriter>,
    fst: Option<FstPlan>,
    engine: TickEngine,
    kernel: engine::CompiledKernel,
    coverage: Coverage,
    coverage_values: Vec<u64>,
    coverage_signal_ids: HashMap<String, usize>,
    coverage_seq_mux_slots: Vec<Option<usize>>,
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
        engine::validate_single_module(&hir).expect("Sim construction failed");
        let mut regs = BTreeMap::new();
        let mut mems = Vec::new();
        let mut memory_ids = HashMap::new();
        for m in &hir.circuit().modules {
            for stmt in &m.body {
                match stmt {
                    Stmt::RegDecl { name, .. } => {
                        regs.insert(name.clone(), 0);
                    }
                    Stmt::MemDecl {
                        name,
                        depth,
                        width,
                        init,
                        ..
                    } => {
                        let _ = width;
                        let words = match init {
                            Some(v) => v.clone(),
                            None => vec![0; *depth as usize],
                        };
                        let id = mems.len();
                        memory_ids.insert(name.clone(), id);
                        mems.push(words);
                    }
                    _ => {}
                }
            }
        }
        let mut coverage = Coverage::default();
        let mut coverage_signal_ids = HashMap::new();
        let mut mux_coverage_ids = HashMap::new();
        let mut coverage_seq_mux_slots = Vec::new();
        if let Some(m) = hir.circuit().modules.first() {
            for p in &m.ports {
                let id = coverage.register_signal_slot(&p.name);
                coverage_signal_ids.insert(p.name.clone(), id);
            }
            for stmt in &m.body {
                if let Stmt::RegDecl { name, .. } = stmt {
                    let id = coverage.register_signal_slot(name);
                    coverage_signal_ids.insert(name.clone(), id);
                }
                if let Stmt::Process(process) = stmt {
                    for assign in &process.assigns {
                        let mux_id = if let AssignExpr::Mux { sel, .. } = &assign.expr {
                            let id = *mux_coverage_ids
                                .entry(sel.clone())
                                .or_insert_with(|| coverage.register_mux_slot(sel));
                            Some(id)
                        } else {
                            None
                        };
                        if process.kind == ProcessKind::Sequential {
                            coverage_seq_mux_slots.push(mux_id);
                        }
                    }
                }
            }
        }
        let coverage_values = vec![0; coverage_signal_ids.len()];
        let mut signal_ids = HashMap::new();
        if let Some(module) = hir.circuit().modules.first() {
            for port in &module.ports {
                let id = signal_ids.len();
                signal_ids.entry(port.name.clone()).or_insert(id);
            }
            for stmt in &module.body {
                if let Stmt::RegDecl { name, .. } | Stmt::WireDecl { name, .. } = stmt {
                    let id = signal_ids.len();
                    signal_ids.entry(name.clone()).or_insert(id);
                }
            }
        }
        let signal_values = vec![0; signal_ids.len()];
        let widths = signal_widths(&hir);
        let kernel = engine::compile(&hir, &mux_coverage_ids, &signal_ids, &memory_ids, &widths);
        Self {
            widths,
            signal_ids,
            signal_values,
            hir: Arc::new(hir),
            regs,
            mems,
            memory_ids,
            pending_mem_reads: BTreeMap::new(),
            ports: PortValues::default(),
            time: 0,
            vcd: None,
            fst: None,
            engine,
            kernel,
            coverage,
            coverage_values,
            coverage_signal_ids,
            coverage_seq_mux_slots,
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
            let value = self.truncate(&k, v);
            self.set_signal_value(&k, value);
            self.publish_port_value(&k, value);
            self.update_coverage_value(&k, value);
        }
    }

    fn update_coverage_value(&mut self, name: &str, value: u64) {
        if let Some(&id) = self.coverage_signal_ids.get(name) {
            self.coverage_values[id] = value;
        }
    }

    pub fn ports(&self) -> &PortValues {
        &self.ports
    }

    fn lookup(&self, name: &str) -> u64 {
        self.signal_ids
            .get(name)
            .and_then(|&id| self.signal_values.get(id).copied())
            .or_else(|| self.ports.get(name))
            .or_else(|| self.regs.get(name).copied())
            .unwrap_or(0)
    }

    fn set_signal_value(&mut self, name: &str, value: u64) {
        if let Some(&id) = self.signal_ids.get(name) {
            self.set_signal_value_id(id, value);
        }
    }

    fn set_signal_value_id(&mut self, id: usize, value: u64) {
        if let Some(slot) = self.signal_values.get_mut(id) {
            *slot = value;
        }
    }

    fn publish_port_value(&mut self, name: &str, value: u64) {
        if let Some(current) = self.ports.values.get_mut(name) {
            if *current != value {
                *current = value;
            }
        } else {
            self.ports.values.insert(name.to_owned(), value);
        }
    }

    fn truncate(&self, name: &str, value: u64) -> u64 {
        mask_width(value, self.widths.get(name).copied().unwrap_or(64))
    }

    fn eval(&mut self, expr: &AssignExpr, mux_coverage_id: Option<usize>) -> u64 {
        match expr {
            AssignExpr::Ref(n) => self.lookup(n),
            AssignExpr::Lit(v) => *v,
            AssignExpr::Inc(n) => self.lookup(n).wrapping_add(1),
            AssignExpr::Add(a, b) => self.lookup(a).wrapping_add(self.lookup(b)),
            AssignExpr::Sub(a, b) => self.lookup(a).wrapping_sub(self.lookup(b)),
            AssignExpr::And(a, b) => self.lookup(a) & self.lookup(b),
            AssignExpr::Or(a, b) => self.lookup(a) | self.lookup(b),
            AssignExpr::Xor(a, b) => self.lookup(a) ^ self.lookup(b),
            AssignExpr::Shl(a, b) => self
                .lookup(a)
                .checked_shl(self.lookup(b).min(64) as u32)
                .unwrap_or(0),
            AssignExpr::Shr(a, b) => self
                .lookup(a)
                .checked_shr(self.lookup(b).min(64) as u32)
                .unwrap_or(0),
            AssignExpr::Ult { lhs, rhs, width } => u64::from(
                mask_width(self.lookup(lhs), *width) < mask_width(self.lookup(rhs), *width),
            ),
            AssignExpr::Slt { lhs, rhs, width } => {
                u64::from(signed_less(self.lookup(lhs), self.lookup(rhs), *width))
            }
            AssignExpr::Sar {
                value,
                shamt,
                width,
            } => arithmetic_right_shift(self.lookup(value), self.lookup(shamt), *width),
            AssignExpr::Slice { src, lo, width } => {
                let mask = if *width >= 64 {
                    u64::MAX
                } else {
                    (1u64 << width) - 1
                };
                (self.lookup(src) >> lo) & mask
            }
            AssignExpr::Concat {
                high,
                low,
                low_width,
            } => (self.lookup(high) << low_width) | self.lookup(low),
            AssignExpr::ZeroExtend { src, to_width, .. } => {
                self.lookup(src)
                    & if *to_width >= 64 {
                        u64::MAX
                    } else {
                        (1u64 << to_width) - 1
                    }
            }
            AssignExpr::SignExtend {
                src,
                from_width: from,
                to_width,
            } => {
                let value = self.lookup(src);
                let extended = if *from < 64 && value & (1u64 << (*from - 1)) != 0 {
                    value | (!0u64 << *from)
                } else {
                    value
                };
                extended
                    & if *to_width >= 64 {
                        u64::MAX
                    } else {
                        (1u64 << to_width) - 1
                    }
            }
            AssignExpr::Eq(a, b) => u64::from(self.lookup(a) == self.lookup(b)),
            AssignExpr::Mux { sel, t, f } => {
                let took_true = self.lookup(sel) != 0;
                if let Some(id) = mux_coverage_id {
                    self.coverage.sample_mux_slot(id, took_true);
                } else {
                    self.coverage.sample_mux_branch(sel, took_true);
                }
                if took_true {
                    self.lookup(t)
                } else {
                    self.lookup(f)
                }
            }
            AssignExpr::MemRead { mem, addr } => {
                let a = self.lookup(addr) as usize;
                self.mems
                    .get(self.memory_ids.get(mem).copied().unwrap_or(usize::MAX))
                    .and_then(|m| m.get(a).copied())
                    .unwrap_or(0)
            }
        }
    }

    fn eval_resolved(&mut self, expr: &ResolvedExpr, mux_coverage_id: Option<usize>) -> u64 {
        let read = |id: usize, values: &[u64]| values.get(id).copied().unwrap_or(0);
        match expr {
            ResolvedExpr::Ref(id) => read(*id, &self.signal_values),
            ResolvedExpr::Lit(v) => *v,
            ResolvedExpr::Inc(id) => read(*id, &self.signal_values).wrapping_add(1),
            ResolvedExpr::Add(a, b) => {
                read(*a, &self.signal_values).wrapping_add(read(*b, &self.signal_values))
            }
            ResolvedExpr::Sub(a, b) => {
                read(*a, &self.signal_values).wrapping_sub(read(*b, &self.signal_values))
            }
            ResolvedExpr::And(a, b) => {
                read(*a, &self.signal_values) & read(*b, &self.signal_values)
            }
            ResolvedExpr::Or(a, b) => read(*a, &self.signal_values) | read(*b, &self.signal_values),
            ResolvedExpr::Xor(a, b) => {
                read(*a, &self.signal_values) ^ read(*b, &self.signal_values)
            }
            ResolvedExpr::Shl(a, b) => read(*a, &self.signal_values)
                .checked_shl(read(*b, &self.signal_values).min(64) as u32)
                .unwrap_or(0),
            ResolvedExpr::Shr(a, b) => read(*a, &self.signal_values)
                .checked_shr(read(*b, &self.signal_values).min(64) as u32)
                .unwrap_or(0),
            ResolvedExpr::Ult { lhs, rhs, width } => u64::from(
                mask_width(read(*lhs, &self.signal_values), *width)
                    < mask_width(read(*rhs, &self.signal_values), *width),
            ),
            ResolvedExpr::Slt { lhs, rhs, width } => u64::from(signed_less(
                read(*lhs, &self.signal_values),
                read(*rhs, &self.signal_values),
                *width,
            )),
            ResolvedExpr::Sar {
                value,
                shamt,
                width,
            } => arithmetic_right_shift(
                read(*value, &self.signal_values),
                read(*shamt, &self.signal_values),
                *width,
            ),
            ResolvedExpr::Slice { src, lo, width } => {
                mask_width(read(*src, &self.signal_values) >> lo, *width)
            }
            ResolvedExpr::Concat {
                high,
                low,
                low_width,
            } => (read(*high, &self.signal_values) << low_width) | read(*low, &self.signal_values),
            ResolvedExpr::ZeroExtend { src, to_width } => {
                mask_width(read(*src, &self.signal_values), *to_width)
            }
            ResolvedExpr::SignExtend {
                src,
                from_width,
                to_width,
            } => {
                let value = read(*src, &self.signal_values);
                let ext = if *from_width < 64 && value & (1u64 << (*from_width - 1)) != 0 {
                    value | (!0u64 << *from_width)
                } else {
                    value
                };
                mask_width(ext, *to_width)
            }
            ResolvedExpr::Eq(a, b) => {
                u64::from(read(*a, &self.signal_values) == read(*b, &self.signal_values))
            }
            ResolvedExpr::Mux { sel, t, f } => {
                let took_true = read(*sel, &self.signal_values) != 0;
                if let Some(id) = mux_coverage_id {
                    self.coverage.sample_mux_slot(id, took_true);
                }
                read(if took_true { *t } else { *f }, &self.signal_values)
            }
            ResolvedExpr::MemRead { mem, addr } => self
                .mems
                .get(*mem)
                .and_then(|bank| bank.get(read(*addr, &self.signal_values) as usize).copied())
                .unwrap_or(0),
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
        self.tick_combinational();
        match self.engine {
            TickEngine::Interpreter => self.tick_interpreter(),
            TickEngine::Compiled => self.tick_compiled(),
        }
        self.sample_coverage();
        self.time += 1;
        let _ = self.dump_vcd();
    }

    fn sample_coverage(&mut self) {
        for (id, value) in self.coverage_values.iter().copied().enumerate() {
            self.coverage.sample_signal_slot(id, value);
        }
    }

    pub fn coverage_report(&self) -> String {
        self.coverage.report()
    }

    /// Borrow the live coverage recorder (FR105/FR109/FR114).
    pub fn coverage(&self) -> &Coverage {
        &self.coverage
    }

    /// Write FR114 `coverage.lcov` + `coverage.html` under `out_dir`.
    pub fn write_coverage_artifacts(&self, out_dir: &Path) -> io::Result<(PathBuf, PathBuf)> {
        write_coverage_artifacts(&self.coverage, out_dir)
    }

    /// Register FSM state labels for FR109 / C3 state-visit coverage.
    pub fn register_fsm_states<I, S>(&mut self, fsm: &str, states: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.coverage.register_fsm_states(fsm, states);
    }

    /// Record a visit to an FSM state (after [`Self::register_fsm_states`]).
    pub fn sample_state_visit(&mut self, fsm: &str, state: &str) {
        self.coverage.sample_state_visit(fsm, state);
    }

    fn tick_interpreter(&mut self) {
        self.tick_sequential();
        self.tick_combinational();
    }

    /// Recompute combinational nets from current ports/regs without a clock edge.
    ///
    /// `tick` settles automatically before sampling and after committing the edge.
    /// Use this method to observe combinational outputs without advancing time.
    pub fn settle(&mut self) {
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
            let hir = Arc::clone(&self.hir);
            let Some(m) = hir.circuit().modules.first() else {
                return;
            };
            let reset = self.reset_active(&m);
            let seq = std::mem::take(&mut self.kernel.seq);
            let comb = std::mem::take(&mut self.kernel.fast_comb);
            // NBA: evaluate all next values before committing register updates.
            let mut next_regs: BTreeMap<String, u64> = BTreeMap::new();
            for (name, expr, mux_coverage_id) in &seq {
                let next = if reset {
                    0
                } else {
                    self.eval(expr, *mux_coverage_id)
                };
                next_regs.insert(name.clone(), self.truncate(name, next));
            }
            for (name, val) in next_regs {
                self.set_signal_value(&name, val);
                self.regs.insert(name.clone(), val);
                self.update_coverage_value(&name, val);
            }
            for assign in &comb {
                let val = mask_width(
                    self.eval_resolved(&assign.expr, assign.mux_coverage_id),
                    assign.target_width,
                );
                self.set_signal_value_id(assign.target_id, val);
                self.publish_port_value(&assign.target_name, val);
                self.update_coverage_value(&assign.target_name, val);
            }
            self.kernel.seq = seq;
            self.kernel.fast_comb = comb;
            return;
        }
        self.tick_interpreter();
    }

    fn tick_sequential(&mut self) {
        let hir = Arc::clone(&self.hir);
        let Some(m) = hir.circuit().modules.first() else {
            return;
        };
        let reset = self.reset_active(&m);
        let enable = self.enable_active();

        // Pending synchronous reads join the simultaneous register commit;
        // downstream sequential RHS still observes the pre-edge register file.
        let mut next_regs = std::mem::take(&mut self.pending_mem_reads);
        if reset {
            for value in next_regs.values_mut() {
                *value = 0;
            }
        }
        let mut next_pending = BTreeMap::new();
        let mut writes = Vec::new();
        let mut seq_assign_index = 0;
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                if p.kind != ProcessKind::Sequential {
                    continue;
                }
                for a in &p.assigns {
                    let mux_coverage_id = self
                        .coverage_seq_mux_slots
                        .get(seq_assign_index)
                        .copied()
                        .flatten();
                    seq_assign_index += 1;
                    match &a.target {
                        AssignTarget::RegD(name) => {
                            let (_async_rst, has_en) = self.reg_meta(name);
                            if let AssignExpr::MemRead { mem, addr } = &a.expr {
                                if self.mem_is_sync(mem) {
                                    // The memory read stage samples every edge; the
                                    // destination register gates retirement at this edge.
                                    let value = self.eval(
                                        &AssignExpr::MemRead {
                                            mem: mem.clone(),
                                            addr: addr.clone(),
                                        },
                                        None,
                                    );
                                    next_pending.insert(name.clone(), self.truncate(name, value));
                                    if reset {
                                        next_regs.insert(name.clone(), 0);
                                    } else if has_en && !enable {
                                        next_regs.remove(name);
                                    }
                                    continue;
                                }
                            }
                            if reset {
                                next_regs.insert(name.clone(), 0);
                                continue;
                            }
                            if has_en && !enable {
                                continue;
                            }
                            let next = self.eval(&a.expr, mux_coverage_id);
                            next_regs.insert(name.clone(), self.truncate(name, next));
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
                            let data = self.eval(&a.expr, mux_coverage_id);
                            let data = self.truncate(mem, data);
                            writes.push((
                                self.memory_ids.get(mem).copied().unwrap_or(usize::MAX),
                                a_idx,
                                data,
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
        // Internal collision convention is read-before-write, independent of
        // statement order. FIRRTL collision results remain undefined.
        for (mem, index, value) in writes {
            if let Some(bank) = self.mems.get_mut(mem) {
                if let Some(word) = bank.get_mut(index) {
                    *word = value;
                }
            }
        }
        for (name, val) in next_regs {
            self.set_signal_value(&name, val);
            self.regs.insert(name.clone(), val);
            self.update_coverage_value(&name, val);
        }
        self.pending_mem_reads = next_pending;
    }

    fn tick_combinational(&mut self) {
        let comb = std::mem::take(&mut self.kernel.fast_comb);
        for assign in &comb {
            let value = mask_width(
                self.eval_resolved(&assign.expr, assign.mux_coverage_id),
                assign.target_width,
            );
            self.set_signal_value_id(assign.target_id, value);
            self.publish_port_value(&assign.target_name, value);
            self.update_coverage_value(&assign.target_name, value);
        }
        self.kernel.fast_comb = comb;
    }
}

fn signed_less(lhs: u64, rhs: u64, width: u32) -> bool {
    let lhs = mask_width(lhs, width);
    let rhs = mask_width(rhs, width);
    let sign = 1u64 << (width - 1);
    let lhs_negative = lhs & sign != 0;
    let rhs_negative = rhs & sign != 0;
    if lhs_negative != rhs_negative {
        lhs_negative
    } else {
        lhs < rhs
    }
}

fn signal_widths(hir: &FrozenHir) -> BTreeMap<String, u32> {
    let mut widths = BTreeMap::new();
    if let Some(module) = hir.circuit().modules.first() {
        for port in &module.ports {
            widths.insert(port.name.clone(), ground_width(&port.ty));
        }
        for stmt in &module.body {
            match stmt {
                Stmt::RegDecl { name, ty, .. } | Stmt::WireDecl { name, ty, .. } => {
                    widths.insert(name.clone(), ground_width(ty));
                }
                Stmt::MemDecl { name, width, .. } => {
                    widths.insert(name.clone(), *width);
                }
                _ => {}
            }
        }
    }
    widths
}
fn ground_width(ty: &GroundType) -> u32 {
    match ty {
        GroundType::UInt { width } | GroundType::SInt { width } => *width,
        _ => 1,
    }
}

fn mask_width(value: u64, width: u32) -> u64 {
    value
        & if width >= 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        }
}

fn arithmetic_right_shift(value: u64, shamt: u64, width: u32) -> u64 {
    let mask = if width >= 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };
    let value = value & mask;
    let negative = value & (1u64 << (width - 1)) != 0;
    if shamt >= u64::from(width) {
        return if negative { mask } else { 0 };
    }
    if shamt == 0 || !negative {
        return value >> shamt;
    }
    ((value >> shamt) | (!0u64 << (width - shamt as u32))) & mask
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
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(counter_hir(), engine);
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

    fn resolved_chain_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("ResolvedChain", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 3 }, Span::default());
        s.add_input("b", GroundType::UInt { width: 3 }, Span::default());
        s.add_input("sel", GroundType::Bool, Span::default());
        s.add_output("out", GroundType::UInt { width: 3 }, Span::default());
        s.add_output("lt", GroundType::Bool, Span::default());
        s.declare_wire("sum", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("picked", GroundType::UInt { width: 3 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_add("sum", "a", "b", Span::default());
        s.assign_mux("picked", "sel", "sum", "a", Span::default());
        s.assign_net("out", "picked", Span::default());
        s.assign_ult("lt", "a", "b", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    fn multi_bank_mem_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("MemBanks", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("addr", GroundType::UInt { width: 2 }, Span::default());
        s.add_output("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("b", GroundType::UInt { width: 8 }, Span::default());
        s.declare_mem_with_init("mem_a", 3, 8, vec![0x12, 0x34, 0x9a], Span::default());
        s.declare_mem_with_init("mem_b", 3, 8, vec![0x56, 0x78, 0xbc], Span::default());
        s.begin_combinational(Span::default());
        s.assign_mem_read("a", "mem_a", "addr", Span::default());
        s.assign_mem_read("b", "mem_b", "addr", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    fn expression_eval_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("ExpressionEval", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("shamt", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("sel", GroundType::Bool, Span::default());
        s.add_input("addr", GroundType::UInt { width: 2 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_mem_with_init("mem_a", 3, 8, vec![0x11, 0x22, 0x33], Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
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
        // FR29: no HIR→TLM *lowering* that replaces cycle-accurate tick.
        // FR101 may export `systemc_tlm` / `emit_systemc_tlm_lt` as a separate product path
        // (revised AD-5); that must not appear as `to_tlm` / bare `emit_tlm` on this surface.
        let src = include_str!("lib.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(!prod.contains("fn to_tlm") && !prod.contains("pub fn to_tlm"));
        assert!(!prod.contains("pub fn emit_tlm(") && !prod.contains("fn emit_tlm("));
        if prod.contains("mod systemc_tlm") || prod.contains("emit_systemc_tlm") {
            assert!(
                prod.contains("FR101") || include_str!("systemc_tlm.rs").contains("FR101"),
                "SystemC TLM surface must be labeled FR101"
            );
        }
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
    fn resolved_expression_chains_keep_widths_and_comparisons() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(resolved_chain_hir(), engine);
            let mut inputs = PortValues::default();
            inputs.set("a", 7);
            inputs.set("b", 3);
            inputs.set("sel", 1);
            sim.set_inputs(inputs);
            sim.settle();
            assert_eq!(sim.ports().get("sum"), Some(2));
            assert_eq!(sim.ports().get("out"), Some(2));
            assert_eq!(sim.ports().get("lt"), Some(0));
        }
    }

    #[test]
    fn resolved_evaluator_matches_ast_for_every_expression_variant() {
        let mut sim = Sim::new(expression_eval_hir());
        let mut inputs = PortValues::default();
        inputs.set("a", 0x91);
        inputs.set("b", 5);
        inputs.set("shamt", 2);
        inputs.set("sel", 1);
        inputs.set("addr", 2);
        sim.set_inputs(inputs);
        let signal_ids = sim.signal_ids.clone();
        let id = |name: &str| signal_ids[name];
        let mem = sim.memory_ids["mem_a"];
        let expressions = vec![
            (AssignExpr::Ref("a".into()), ResolvedExpr::Ref(id("a"))),
            (AssignExpr::Lit(13), ResolvedExpr::Lit(13)),
            (AssignExpr::Inc("a".into()), ResolvedExpr::Inc(id("a"))),
            (
                AssignExpr::Add("a".into(), "b".into()),
                ResolvedExpr::Add(id("a"), id("b")),
            ),
            (
                AssignExpr::Sub("a".into(), "b".into()),
                ResolvedExpr::Sub(id("a"), id("b")),
            ),
            (
                AssignExpr::And("a".into(), "b".into()),
                ResolvedExpr::And(id("a"), id("b")),
            ),
            (
                AssignExpr::Or("a".into(), "b".into()),
                ResolvedExpr::Or(id("a"), id("b")),
            ),
            (
                AssignExpr::Xor("a".into(), "b".into()),
                ResolvedExpr::Xor(id("a"), id("b")),
            ),
            (
                AssignExpr::Shl("a".into(), "shamt".into()),
                ResolvedExpr::Shl(id("a"), id("shamt")),
            ),
            (
                AssignExpr::Shr("a".into(), "shamt".into()),
                ResolvedExpr::Shr(id("a"), id("shamt")),
            ),
            (
                AssignExpr::Ult {
                    lhs: "a".into(),
                    rhs: "b".into(),
                    width: 8,
                },
                ResolvedExpr::Ult {
                    lhs: id("a"),
                    rhs: id("b"),
                    width: 8,
                },
            ),
            (
                AssignExpr::Slt {
                    lhs: "a".into(),
                    rhs: "b".into(),
                    width: 8,
                },
                ResolvedExpr::Slt {
                    lhs: id("a"),
                    rhs: id("b"),
                    width: 8,
                },
            ),
            (
                AssignExpr::Sar {
                    value: "a".into(),
                    shamt: "shamt".into(),
                    width: 8,
                },
                ResolvedExpr::Sar {
                    value: id("a"),
                    shamt: id("shamt"),
                    width: 8,
                },
            ),
            (
                AssignExpr::Slice {
                    src: "a".into(),
                    lo: 2,
                    width: 3,
                },
                ResolvedExpr::Slice {
                    src: id("a"),
                    lo: 2,
                    width: 3,
                },
            ),
            (
                AssignExpr::Concat {
                    high: "a".into(),
                    low: "b".into(),
                    low_width: 8,
                },
                ResolvedExpr::Concat {
                    high: id("a"),
                    low: id("b"),
                    low_width: 8,
                },
            ),
            (
                AssignExpr::ZeroExtend {
                    src: "a".into(),
                    from_width: 8,
                    to_width: 12,
                },
                ResolvedExpr::ZeroExtend {
                    src: id("a"),
                    to_width: 12,
                },
            ),
            (
                AssignExpr::SignExtend {
                    src: "a".into(),
                    from_width: 8,
                    to_width: 12,
                },
                ResolvedExpr::SignExtend {
                    src: id("a"),
                    from_width: 8,
                    to_width: 12,
                },
            ),
            (
                AssignExpr::Eq("a".into(), "b".into()),
                ResolvedExpr::Eq(id("a"), id("b")),
            ),
            (
                AssignExpr::Mux {
                    sel: "sel".into(),
                    t: "a".into(),
                    f: "b".into(),
                },
                ResolvedExpr::Mux {
                    sel: id("sel"),
                    t: id("a"),
                    f: id("b"),
                },
            ),
            (
                AssignExpr::MemRead {
                    mem: "mem_a".into(),
                    addr: "addr".into(),
                },
                ResolvedExpr::MemRead {
                    mem,
                    addr: id("addr"),
                },
            ),
        ];
        for (ast, resolved) in expressions {
            let expected = sim.eval(&ast, None);
            let actual = sim.eval_resolved(&resolved, None);
            assert_eq!(actual, expected, "expression {ast:?}");
        }
        sim.set_inputs({
            let mut values = PortValues::default();
            values.set("addr", 3);
            values
        });
        let ast = AssignExpr::MemRead {
            mem: "mem_a".into(),
            addr: "addr".into(),
        };
        let resolved = ResolvedExpr::MemRead {
            mem,
            addr: id("addr"),
        };
        assert_eq!(sim.eval_resolved(&resolved, None), sim.eval(&ast, None));
    }

    #[test]
    fn resolved_memory_slots_keep_banks_and_zero_on_oob() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(multi_bank_mem_hir(), engine);
            let mut inputs = PortValues::default();
            inputs.set("addr", 1);
            sim.set_inputs(inputs);
            sim.settle();
            assert_eq!(sim.ports().get("a"), Some(0x34));
            assert_eq!(sim.ports().get("b"), Some(0x78));
            let mut inputs = PortValues::default();
            inputs.set("addr", 3);
            sim.set_inputs(inputs);
            sim.settle();
            assert_eq!(sim.ports().get("a"), Some(0));
            assert_eq!(sim.ports().get("b"), Some(0));
        }
    }

    #[test]
    fn unresolved_fast_slots_keep_zero_fallback_without_panicking() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(passthrough_hir(), engine);
            assert_eq!(
                sim.eval_resolved(&ResolvedExpr::Ref(usize::MAX), None),
                0,
                "unresolved signal reads retain the zero fallback"
            );
            assert_eq!(
                sim.eval_resolved(
                    &ResolvedExpr::MemRead {
                        mem: usize::MAX,
                        addr: sim.signal_ids["data_in"],
                    },
                    None,
                ),
                0,
                "unresolved memory reads retain the zero fallback"
            );
            sim.kernel.fast_comb.push(engine::ResolvedAssign {
                target_name: "unknown_target".into(),
                target_id: usize::MAX,
                target_width: 64,
                mux_coverage_id: None,
                expr: ResolvedExpr::Ref(usize::MAX),
            });
            sim.tick_combinational();
            assert_eq!(sim.ports().get("unknown_target"), Some(0));
        }
    }

    #[test]
    fn portvalues_writeback_preserves_existing_keys_and_values() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(passthrough_hir(), engine);
            assert!(!sim.ports().values.contains_key("data_out"));
            sim.settle();
            assert_eq!(sim.ports().get("data_out"), Some(0));
            let mut inputs = PortValues::default();
            inputs.set("data_in", 0);
            inputs.set("extra_input", 17);
            sim.set_inputs(inputs);
            assert_eq!(sim.ports().get("extra_input"), Some(17));
            sim.settle();
            let keys_and_values = sim.ports().values.clone();
            sim.settle();
            assert_eq!(sim.ports().values, keys_and_values);
            let mut changed = PortValues::default();
            changed.set("data_in", 9);
            changed.set("extra_input", 23);
            sim.set_inputs(changed);
            sim.settle();
            assert_eq!(sim.ports().get("data_out"), Some(9));
            assert_eq!(sim.ports().get("extra_input"), Some(23));
        }
    }

    #[test]
    fn coverage_has_hit_and_miss() {
        let mut sim = Sim::new(counter_hir());
        assert!(
            sim.coverage().is_empty(),
            "registration alone is not a sample"
        );
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        let report = sim.coverage_report();
        assert!(
            report.starts_with("# bitloom-sim coverage v2")
                || report.starts_with("# bitloom-sim coverage v3"),
            "FR105/FR109 coverage header v2 (Mux) or v3 (with FSM)"
        );
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

    #[test]
    fn coverage_slots_are_updated_by_inputs_and_tick_writes() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(passthrough_hir(), engine);
            assert!(sim.coverage().is_empty());
            let mut inputs = PortValues::default();
            inputs.set("data_in", 7);
            sim.set_inputs(inputs);
            sim.tick();
            let mut inputs = PortValues::default();
            inputs.set("data_in", 8);
            sim.set_inputs(inputs);
            sim.tick();
            let (hits, misses) = parse_report(&sim.coverage_report());
            assert!(hits.contains(&"data_out".to_owned()));
            assert!(hits.contains(&"data_in".to_owned()));
            assert!(misses.contains(&"clk".to_owned()));
        }
    }

    fn mux_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("MuxOnly", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("sel", GroundType::Bool, Span::default());
        s.add_input("a", GroundType::Bool, Span::default());
        s.add_input("b", GroundType::Bool, Span::default());
        s.add_output("y", GroundType::Bool, Span::default());
        s.begin_combinational(Span::default());
        s.assign_mux("y", "sel", "a", "b", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    fn sequential_mux_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("SeqMux", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("sel", GroundType::Bool, Span::default());
        s.add_input("a", GroundType::Bool, Span::default());
        s.add_input("b", GroundType::Bool, Span::default());
        s.add_output("y", GroundType::Bool, Span::default());
        s.declare_reg("q", GroundType::Bool, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_mux("q", "sel", "a", "b", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn registering_mux_slots_does_not_create_branch_misses() {
        let sim = Sim::new(mux_hir());
        assert!(sim.coverage().is_empty());
        assert!(!sim.coverage_report().contains("branch_"));
    }

    #[test]
    fn resolved_mux_coverage_ids_match_for_both_tick_engines() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(mux_hir(), engine);
            let mut inputs = PortValues::default();
            inputs.set("sel", 1);
            sim.set_inputs(inputs.clone());
            sim.tick();
            let (hits, misses) = parse_branch_report(&sim.coverage_report());
            assert!(hits.contains(&"mux:sel:t".to_owned()));
            assert!(misses.contains(&"mux:sel:f".to_owned()));

            inputs.set("sel", 0);
            sim.set_inputs(inputs);
            sim.tick();
            let (hits, misses) = parse_branch_report(&sim.coverage_report());
            assert!(hits.contains(&"mux:sel:t".to_owned()));
            assert!(hits.contains(&"mux:sel:f".to_owned()));
            assert!(misses.is_empty());
        }
    }

    #[test]
    fn sequential_mux_coverage_ids_match_for_both_tick_engines() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(sequential_mux_hir(), engine);
            let mut inputs = PortValues::default();
            inputs.set("sel", 1);
            inputs.set("a", 1);
            sim.set_inputs(inputs.clone());
            sim.tick();
            let (hits, misses) = parse_branch_report(&sim.coverage_report());
            assert!(hits.contains(&"mux:sel:t".to_owned()));
            assert!(misses.contains(&"mux:sel:f".to_owned()));

            inputs.set("sel", 0);
            sim.set_inputs(inputs);
            sim.tick();
            let (hits, misses) = parse_branch_report(&sim.coverage_report());
            assert!(hits.contains(&"mux:sel:t".to_owned()));
            assert!(hits.contains(&"mux:sel:f".to_owned()));
            assert!(misses.is_empty());
        }
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
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(sync_read_mem_hir(), engine);
            let mut pv = PortValues::default();
            pv.set("rst", 0);
            pv.set("addr", 3);
            pv.set("wdata", 0xAB);
            pv.set("we", 1);
            sim.set_inputs(pv);
            sim.tick(); // write + schedule read of old (0)
            assert_eq!(sim.ports().get("rdata"), Some(0));
            sim.tick(); // retires the pre-write read; the new read sees 0xAB
            assert_eq!(sim.ports().get("rdata"), Some(0));
            sim.tick();
            assert_eq!(sim.ports().get("rdata"), Some(0xAB));
        }
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
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(async_reset_hir(), engine);
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
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut gated = Sim::with_engine(enable_hir(true), engine);
            let mut plain = Sim::with_engine(enable_hir(false), engine);
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
    }

    #[test]
    fn enable_low_holds_value() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(enable_hir(true), engine);
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

    #[test]
    fn reset_overrides_disabled_enable_and_disabled_enable_holds_register() {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(enable_hir(true), engine);
            let mut inputs = PortValues::default();
            inputs.set("rst", 0);
            inputs.set("en", 1);
            sim.set_inputs(inputs.clone());
            sim.tick();
            sim.tick();
            assert_eq!(sim.ports().get("data_out"), Some(2));

            inputs.set("rst", 1);
            inputs.set("en", 0);
            sim.set_inputs(inputs.clone());
            sim.tick();
            assert_eq!(sim.ports().get("data_out"), Some(0));

            inputs.set("rst", 0);
            inputs.set("en", 1);
            sim.set_inputs(inputs.clone());
            sim.tick();
            assert_eq!(sim.ports().get("data_out"), Some(1));
            inputs.set("en", 0);
            sim.set_inputs(inputs);
            sim.tick();
            sim.tick();
            assert_eq!(sim.ports().get("data_out"), Some(1));
        }
    }
}

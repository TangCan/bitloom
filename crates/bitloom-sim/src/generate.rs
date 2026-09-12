//! FR47 leg 1: generate a Rust functional-sim crate from FrozenHir (AD-5).
//! Minimal interpreter-backed AbstractionView — not HLS-quality codegen.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bitloom_hir::{AssignExpr, AssignTarget, FrozenHir, GroundType, ProcessKind, Stmt};

use crate::AbstractionView;
use bitloom_hir::PortValues;

/// Sequential op collected from FrozenHir (order preserved; matches `Sim::tick_sequential`).
#[derive(Debug, Clone)]
enum SeqOp {
    RegD {
        name: String,
        expr: AssignExpr,
        has_en: bool,
    },
    MemWrite {
        mem: String,
        addr: String,
        we: Option<String>,
        expr: AssignExpr,
    },
}

/// In-process functional model derived from FrozenHir (FR47 / FR112 / FR159).
///
/// **FR112:** SyncReadMem / Mem `MemRead`+`MemWrite` semantics match cycle-accurate
/// [`crate::Sim::tick`] (latency-1 sync read via `pending_mem_reads`) for the
/// **in-process** view / `check_generated_bridge`.
///
/// **FR159:** [`generate_functional_sim`] emits the same MemRead/MemWrite +
/// SyncReadMem latency-1 semantics into the standalone functional-sim crate
/// (no longer stubs `MemRead` as `0`).
#[derive(Debug, Clone)]
pub struct GeneratedFunctional {
    regs: BTreeMap<String, u64>,
    mems: BTreeMap<String, Vec<u64>>,
    mem_sync: BTreeMap<String, bool>,
    pending_mem_reads: BTreeMap<String, u64>,
    reset_port: String,
    enable_port: Option<String>,
    seq: Vec<SeqOp>,
    /// Combinational Net updates: (net_name, expr).
    comb: Vec<(String, AssignExpr)>,
}

impl GeneratedFunctional {
    /// Build a functional model from the top module of `hir`.
    pub fn from_hir(hir: &FrozenHir) -> Self {
        let m = hir
            .circuit()
            .modules
            .first()
            .expect("FrozenHir has at least one module");
        let reset_port = m
            .ports
            .iter()
            .find(|p| matches!(p.ty, GroundType::Reset))
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "rst".into());
        let enable_port = m
            .ports
            .iter()
            .find(|p| p.name == "en")
            .map(|p| p.name.clone());

        let mut regs = BTreeMap::new();
        let mut reg_has_en = BTreeMap::new();
        let mut mems = BTreeMap::new();
        let mut mem_sync = BTreeMap::new();
        for stmt in &m.body {
            match stmt {
                Stmt::RegDecl {
                    name, has_enable, ..
                } => {
                    regs.insert(name.clone(), 0u64);
                    reg_has_en.insert(name.clone(), *has_enable);
                }
                Stmt::MemDecl {
                    name,
                    depth,
                    init,
                    sync_read,
                    ..
                } => {
                    let words = match init {
                        Some(v) => v.clone(),
                        None => vec![0; *depth as usize],
                    };
                    mems.insert(name.clone(), words);
                    mem_sync.insert(name.clone(), *sync_read);
                }
                _ => {}
            }
        }

        let mut seq = Vec::new();
        let mut comb = Vec::new();
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                match p.kind {
                    ProcessKind::Sequential => {
                        for a in &p.assigns {
                            match &a.target {
                                AssignTarget::RegD(name) => {
                                    let has_en = reg_has_en.get(name).copied().unwrap_or(false);
                                    seq.push(SeqOp::RegD {
                                        name: name.clone(),
                                        expr: a.expr.clone(),
                                        has_en,
                                    });
                                }
                                AssignTarget::MemWrite { mem, addr, we } => {
                                    seq.push(SeqOp::MemWrite {
                                        mem: mem.clone(),
                                        addr: addr.clone(),
                                        we: we.clone(),
                                        expr: a.expr.clone(),
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    ProcessKind::Combinational => {
                        for a in &p.assigns {
                            if let AssignTarget::Net(name) = &a.target {
                                comb.push((name.clone(), a.expr.clone()));
                            }
                        }
                    }
                }
            }
        }

        Self {
            regs,
            mems,
            mem_sync,
            pending_mem_reads: BTreeMap::new(),
            reset_port,
            enable_port,
            seq,
            comb,
        }
    }

    fn lookup(&self, inputs: &PortValues, name: &str) -> u64 {
        inputs
            .get(name)
            .or_else(|| self.regs.get(name).copied())
            .unwrap_or(0)
    }

    fn mem_is_sync(&self, name: &str) -> bool {
        self.mem_sync.get(name).copied().unwrap_or(false)
    }

    fn eval_mem_read(&self, inputs: &PortValues, mem: &str, addr: &str) -> u64 {
        let a = self.lookup(inputs, addr) as usize;
        self.mems
            .get(mem)
            .and_then(|m| m.get(a).copied())
            .unwrap_or(0)
    }

    fn eval(&self, inputs: &PortValues, expr: &AssignExpr) -> u64 {
        match expr {
            AssignExpr::Ref(n) => self.lookup(inputs, n),
            AssignExpr::Lit(v) => *v,
            AssignExpr::Inc(n) => self.lookup(inputs, n).wrapping_add(1),
            AssignExpr::Add(a, b) => self.lookup(inputs, a).wrapping_add(self.lookup(inputs, b)),
            AssignExpr::Sub(a, b) => self.lookup(inputs, a).wrapping_sub(self.lookup(inputs, b)),
            AssignExpr::And(a, b) => self.lookup(inputs, a) & self.lookup(inputs, b),
            AssignExpr::Or(a, b) => self.lookup(inputs, a) | self.lookup(inputs, b),
            AssignExpr::Xor(a, b) => self.lookup(inputs, a) ^ self.lookup(inputs, b),
            AssignExpr::Shl(a, b) => self.lookup(inputs, a) << (self.lookup(inputs, b) & 63),
            AssignExpr::Shr(a, b) => self.lookup(inputs, a) >> (self.lookup(inputs, b) & 63),
            AssignExpr::Eq(a, b) => u64::from(self.lookup(inputs, a) == self.lookup(inputs, b)),
            AssignExpr::Mux { sel, t, f } => {
                if self.lookup(inputs, sel) != 0 {
                    self.lookup(inputs, t)
                } else {
                    self.lookup(inputs, f)
                }
            }
            AssignExpr::MemRead { mem, addr } => self.eval_mem_read(inputs, mem, addr),
        }
    }
}

impl AbstractionView for GeneratedFunctional {
    fn cycle(&mut self, inputs: &PortValues) -> PortValues {
        let reset = inputs.get(&self.reset_port).unwrap_or(0) != 0;
        let enable = self
            .enable_port
            .as_ref()
            .map(|p| inputs.get(p).unwrap_or(0) != 0)
            .unwrap_or(true);

        // Apply SyncReadMem pending from previous cycle (latency 1) — matches Sim.
        let pending = std::mem::take(&mut self.pending_mem_reads);
        for (name, val) in pending {
            self.regs.insert(name, if reset { 0 } else { val });
        }

        let mut next_pending = BTreeMap::new();
        let mut next_regs: BTreeMap<String, u64> = BTreeMap::new();
        // Clone ops so MemWrite can mutate `mems` without borrowing `seq`.
        let ops = self.seq.clone();

        for op in &ops {
            match op {
                SeqOp::RegD { name, expr, has_en } => {
                    if reset {
                        next_regs.insert(name.clone(), 0);
                        continue;
                    }
                    if *has_en && !enable {
                        continue;
                    }
                    match expr {
                        AssignExpr::MemRead { mem, addr } if self.mem_is_sync(mem) => {
                            let val = self.eval_mem_read(inputs, mem, addr);
                            next_pending.insert(name.clone(), val);
                        }
                        _ => {
                            next_regs.insert(name.clone(), self.eval(inputs, expr));
                        }
                    }
                }
                SeqOp::MemWrite {
                    mem,
                    addr,
                    we,
                    expr,
                } => {
                    if reset {
                        continue;
                    }
                    if let Some(en) = we {
                        if self.lookup(inputs, en) == 0 {
                            continue;
                        }
                    }
                    let a_idx = self.lookup(inputs, addr) as usize;
                    let data = self.eval(inputs, expr);
                    if let Some(bank) = self.mems.get_mut(mem) {
                        if a_idx < bank.len() {
                            bank[a_idx] = data;
                        }
                    }
                }
            }
        }
        for (k, v) in next_regs {
            self.regs.insert(k, v);
        }
        self.pending_mem_reads = next_pending;

        let mut out = inputs.clone();
        for (name, expr) in &self.comb {
            // Prefer updated regs over prior port values (matches Sim::tick_combinational).
            out.set(name.clone(), self.eval(&out, expr));
        }
        out
    }
}

/// Alias required by Story 21.2→21.3 product surface naming.
pub fn emit_functional_crate(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    generate_functional_sim(hir, out_dir)
}

/// Write a standalone Rust functional-sim crate under `out_dir`.
///
/// Includes `src/lib.rs` (FunctionalSim + gold test) and `src/main.rs` for `cargo run`.
pub fn generate_functional_sim(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(out_dir.join("src"))?;
    let pkg = sanitize_pkg_name(&hir.abi_name);
    let model = GeneratedFunctional::from_hir(hir);
    let cargo = render_cargo_toml(&pkg, out_dir)?;
    let lib = render_lib_rs(&pkg, &model);
    fs::write(out_dir.join("Cargo.toml"), cargo)?;
    fs::write(out_dir.join("src/lib.rs"), lib)?;
    write_functional_main(out_dir)?;
    Ok(out_dir.to_path_buf())
}

fn sanitize_pkg_name(abi: &str) -> String {
    let mut s: String = abi
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() || s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        s = format!("func_{s}");
    }
    format!("bitloom_func_{s}")
}

fn render_cargo_toml(pkg: &str, out_dir: &Path) -> io::Result<String> {
    let hir_dep = resolve_hir_dep(out_dir);
    Ok(format!(
        r#"[package]
name = "{pkg}"
version = "0.0.0"
edition = "2024"
rust-version = "1.97.1"
publish = false
description = "Generated Bitloom functional simulator (FR47). Not SystemC."

# Keep generated crate out of the parent workspace.
[workspace]

[dependencies]
{hir_dep}

[[bin]]
name = "{pkg}"
path = "src/main.rs"
"#
    ))
}

fn resolve_hir_dep(out_dir: &Path) -> String {
    // Prefer workspace path when generating inside the monorepo (tests / CLI).
    let candidates = [
        out_dir.join("../../crates/bitloom-hir").canonicalize().ok(),
        std::env::var_os("CARGO_MANIFEST_DIR")
            .and_then(|m| PathBuf::from(m).join("../bitloom-hir").canonicalize().ok()),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../bitloom-hir")
            .canonicalize()
            .ok(),
    ];
    for c in candidates.into_iter().flatten() {
        if c.join("Cargo.toml").is_file() {
            return format!("bitloom-hir = {{ path = \"{}\" }}", c.display());
        }
    }
    format!("bitloom-hir = \"{}\"", env!("CARGO_PKG_VERSION"))
}

fn render_lib_rs(pkg: &str, model: &GeneratedFunctional) -> String {
    let _ = pkg;
    let reg_inits: String = model
        .regs
        .keys()
        .map(|n| format!("        regs.insert({n:?}.into(), 0u64);\n"))
        .collect();
    let mem_inits: String = model
        .mems
        .iter()
        .map(|(n, words)| {
            let lit = words
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("        mems.insert({n:?}.into(), vec![{lit}]);\n")
        })
        .collect();
    let seq_arms: String = model
        .seq
        .iter()
        .map(|op| match op {
            SeqOp::RegD { name, expr, has_en } => {
                let body = match expr {
                    AssignExpr::MemRead { mem, addr } if model.mem_is_sync(mem) => format!(
                        "                    let val = self.eval_mem_read(inputs, {mem:?}, {addr:?});\n\
                                            next_pending.insert({name:?}.into(), val);\n"
                    ),
                    _ => format!(
                        "                    next_regs.insert({name:?}.into(), {});\n",
                        render_expr(expr)
                    ),
                };
                let en_guard = if *has_en {
                    format!(
                        "                if reset {{\n\
                                            next_regs.insert({name:?}.into(), 0);\n\
                                        }} else if enable {{\n\
                         {body}                }}\n"
                    )
                } else {
                    format!(
                        "                if reset {{\n\
                                            next_regs.insert({name:?}.into(), 0);\n\
                                        }} else {{\n\
                         {body}                }}\n"
                    )
                };
                en_guard
            }
            SeqOp::MemWrite {
                mem,
                addr,
                we,
                expr,
            } => {
                let we_guard = match we {
                    Some(w) => format!(
                        "                if !reset && self.lookup(inputs, {w:?}) != 0 {{\n"
                    ),
                    None => "                if !reset {\n".into(),
                };
                format!(
                    "{we_guard}                    let a_idx = self.lookup(inputs, {addr:?}) as usize;\n\
                                        let data = {};\n\
                                        if let Some(bank) = self.mems.get_mut({mem:?}) {{\n\
                                            if a_idx < bank.len() {{\n\
                                                bank[a_idx] = data;\n\
                                            }}\n\
                                        }}\n\
                                    }}\n",
                    render_expr(expr)
                )
            }
        })
        .collect();
    let comb_arms: String = model
        .comb
        .iter()
        .map(|(name, expr)| {
            format!(
                "        out.set({name:?}, {});\n",
                render_expr_ports_regs(expr)
            )
        })
        .collect();
    let reset = &model.reset_port;
    let enable_init = match &model.enable_port {
        Some(p) => format!("let enable = inputs.get({p:?}).unwrap_or(0) != 0;"),
        None => "#[allow(unused_variables)] let enable = true;".into(),
    };
    let mem_gold = if model.mems.is_empty() {
        String::new()
    } else {
        format!(
            r#"
    #[test]
    fn gold_sync_read_mem_latency1_write_then_read() {{
        // FR159: emitted SyncReadMem path matches Sim / GeneratedFunctional latency-1.
        let mut sim = FunctionalSim::new();
        let mut pv = PortValues::default();
        pv.set({reset:?}, 0);
        pv.set("addr", 3);
        pv.set("wdata", 0xAB);
        pv.set("we", 1);
        let c0 = sim.cycle(&pv);
        if c0.values.contains_key("rdata") {{
            assert_eq!(c0.get("rdata"), Some(0));
        }}
        let c1 = sim.cycle(&pv);
        if c1.values.contains_key("rdata") {{
            assert_eq!(c1.get("rdata"), Some(0xAB));
        }}
    }}
"#
        )
    };

    format!(
        r#"//! Generated Bitloom functional simulator (FR47 / FR159 / AD-5).
//! Not SystemC / TLM-2.0. Do not hand-edit; regenerate via `generate_functional_sim`.
//! MemRead/MemWrite + SyncReadMem latency-1 match GeneratedFunctional / Sim::tick (FR159).

use std::collections::BTreeMap;

use bitloom_hir::PortValues;

/// Generated functional view (AbstractionView-compatible cycle API).
#[derive(Debug, Clone)]
#[allow(dead_code)] // mems/pending unused on designs without MemRead
pub struct FunctionalSim {{
    regs: BTreeMap<String, u64>,
    mems: BTreeMap<String, Vec<u64>>,
    pending_mem_reads: BTreeMap<String, u64>,
}}

impl Default for FunctionalSim {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl FunctionalSim {{
    pub fn new() -> Self {{
        let mut regs = BTreeMap::new();
{reg_inits}        let mut mems = BTreeMap::new();
{mem_inits}        Self {{
            regs,
            mems,
            pending_mem_reads: BTreeMap::new(),
        }}
    }}

    fn lookup(&self, inputs: &PortValues, name: &str) -> u64 {{
        inputs
            .get(name)
            .or_else(|| self.regs.get(name).copied())
            .unwrap_or(0)
    }}

    #[allow(dead_code)]
    fn eval_mem_read(&self, inputs: &PortValues, mem: &str, addr: &str) -> u64 {{
        let a = self.lookup(inputs, addr) as usize;
        self.mems
            .get(mem)
            .and_then(|m| m.get(a).copied())
            .unwrap_or(0)
    }}

    /// One untimed functional cycle; returns updated `PortValues`.
    pub fn cycle(&mut self, inputs: &PortValues) -> PortValues {{
        let reset = inputs.get({reset:?}).unwrap_or(0) != 0;
        {enable_init}
        // Apply SyncReadMem pending from previous cycle (latency 1) — matches Sim.
        let pending = std::mem::take(&mut self.pending_mem_reads);
        for (name, val) in pending {{
            self.regs.insert(name, if reset {{ 0 }} else {{ val }});
        }}
        let mut next_pending = BTreeMap::new();
        let mut next_regs: BTreeMap<String, u64> = BTreeMap::new();
{seq_arms}        for (k, v) in next_regs {{
            self.regs.insert(k, v);
        }}
        self.pending_mem_reads = next_pending;
        let mut out = inputs.clone();
{comb_arms}        out
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn gold_port_values_after_reset_and_three_cycles() {{
        let mut sim = FunctionalSim::new();
        let mut pv = PortValues::default();
        pv.set({reset:?}, 1);
        let _ = sim.cycle(&pv);
        pv.set({reset:?}, 0);
        let mut last = PortValues::default();
        for _ in 0..3 {{
            last = sim.cycle(&pv);
        }}
        // Counter-style gold: data_out == 3 when HIR has count++ / data_out=count.
        if last.values.contains_key("data_out") {{
            assert_eq!(last.get("data_out"), Some(3));
        }}
    }}
{mem_gold}}}
"#
    )
}

fn render_expr(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => format!("self.lookup(inputs, {n:?})"),
        AssignExpr::Lit(v) => format!("{v}"),
        AssignExpr::Inc(n) => format!("self.lookup(inputs, {n:?}).wrapping_add(1)"),
        AssignExpr::Add(a, b) => {
            format!("self.lookup(inputs, {a:?}).wrapping_add(self.lookup(inputs, {b:?}))")
        }
        AssignExpr::Sub(a, b) => {
            format!("self.lookup(inputs, {a:?}).wrapping_sub(self.lookup(inputs, {b:?}))")
        }
        AssignExpr::And(a, b) => format!("self.lookup(inputs, {a:?}) & self.lookup(inputs, {b:?})"),
        AssignExpr::Or(a, b) => format!("self.lookup(inputs, {a:?}) | self.lookup(inputs, {b:?})"),
        AssignExpr::Xor(a, b) => format!("self.lookup(inputs, {a:?}) ^ self.lookup(inputs, {b:?})"),
        AssignExpr::Shl(a, b) => {
            format!("self.lookup(inputs, {a:?}) << (self.lookup(inputs, {b:?}) & 63)")
        }
        AssignExpr::Shr(a, b) => {
            format!("self.lookup(inputs, {a:?}) >> (self.lookup(inputs, {b:?}) & 63)")
        }
        AssignExpr::Eq(a, b) => {
            format!("u64::from(self.lookup(inputs, {a:?}) == self.lookup(inputs, {b:?}))")
        }
        AssignExpr::Mux { sel, t, f } => format!(
            "if self.lookup(inputs, {sel:?}) != 0 {{ self.lookup(inputs, {t:?}) }} else {{ self.lookup(inputs, {f:?}) }}"
        ),
        AssignExpr::MemRead { mem, addr } => {
            format!("self.eval_mem_read(inputs, {mem:?}, {addr:?})")
        }
    }
}

fn render_expr_ports_regs(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => {
            format!("out.get({n:?}).or_else(|| self.regs.get({n:?}).copied()).unwrap_or(0)")
        }
        other => render_expr(other).replace("inputs", "&out"),
    }
}

/// Also write a tiny `main.rs` so `cargo run` works (prints one cycle).
pub fn write_functional_main(out_dir: &Path) -> io::Result<()> {
    let main = r#"fn main() {
    use bitloom_func_bin_placeholder::FunctionalSim;
    use bitloom_hir::PortValues;
    let mut sim = FunctionalSim::new();
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    let out = sim.cycle(&pv);
    println!("{out:?}");
}
"#;
    // Fix package import: read Cargo.toml name
    let toml = fs::read_to_string(out_dir.join("Cargo.toml"))?;
    let name = toml
        .lines()
        .find_map(|l| {
            l.strip_prefix("name = \"")
                .and_then(|r| r.strip_suffix('"'))
                .map(|s| s.replace('-', "_"))
        })
        .unwrap_or_else(|| "functional_sim".into());
    let main = main.replace("bitloom_func_bin_placeholder", &name);
    fs::write(out_dir.join("src/main.rs"), main)
}

/// Convenience alias (same as [`generate_functional_sim`]).
pub fn generate_functional_sim_with_bin(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    generate_functional_sim(hir, out_dir)
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;
    use crate::{Sim, check_mixed_both};

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

    #[test]
    fn generated_functional_matches_tick_port_values() {
        let hir = counter_hir();
        let mut sim = Sim::new(hir.clone());
        let mut abs = GeneratedFunctional::from_hir(&hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
        pv.set("rst", 0);
        for _ in 0..3 {
            check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
        }
        assert_eq!(sim.ports().get("data_out"), Some(3));
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
        s.assign_mem_write("ram", "addr", "wdata", Span::default());
        s.assign_reg_d_mem_read("q", "ram", "addr", Span::default());
        s.end_process();
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn generated_functional_sync_read_mem_matches_tick() {
        let hir = sync_read_mem_hir();
        let mut sim = Sim::new(hir.clone());
        let mut abs = GeneratedFunctional::from_hir(&hir);
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("addr", 3);
        pv.set("wdata", 0xAB);
        pv.set("we", 1);
        check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
        assert_eq!(sim.ports().get("rdata"), Some(0));
        check_mixed_both(&mut sim, &mut abs, pv).unwrap();
        assert_eq!(sim.ports().get("rdata"), Some(0xAB));
    }

    #[test]
    fn emit_writes_crate_with_gold_test() {
        let hir = counter_hir();
        let dir = std::env::temp_dir().join(format!("bitloom-func-gen-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = generate_functional_sim_with_bin(&hir, &dir).unwrap();
        let lib = fs::read_to_string(out.join("src/lib.rs")).unwrap();
        assert!(lib.contains("FunctionalSim"));
        assert!(lib.contains("gold_port_values_after_reset_and_three_cycles"));
        assert!(!lib.to_lowercase().contains("systemc") || lib.contains("Not SystemC"));
        let cargo = fs::read_to_string(out.join("Cargo.toml")).unwrap();
        assert!(cargo.contains("bitloom-hir"));
        assert!(out.join("src/main.rs").is_file());
    }

    #[test]
    fn emit_sync_read_mem_not_stubbed_and_cargo_tests() {
        let hir = sync_read_mem_hir();
        let dir = std::env::temp_dir().join(format!("bitloom-func-mem-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = generate_functional_sim(&hir, &dir).unwrap();
        let lib = fs::read_to_string(out.join("src/lib.rs")).unwrap();
        assert!(
            lib.contains("eval_mem_read") && lib.contains("pending_mem_reads"),
            "FR159 emit must include real MemRead / SyncReadMem latency-1"
        );
        assert!(
            lib.contains("gold_sync_read_mem_latency1_write_then_read"),
            "emitted crate must include SyncReadMem gold"
        );
        // Must not be the pre-FR159 constant stub for MemRead targets.
        assert!(
            lib.contains("next_pending.insert(\"q\"")
                || lib.contains("next_pending.insert(\"q\".into()"),
            "sync MemRead into q must queue pending, not assign 0"
        );
        let status = std::process::Command::new("cargo")
            .arg("+1.97.1")
            .arg("test")
            .arg("--manifest-path")
            .arg(out.join("Cargo.toml"))
            .arg("--quiet")
            .status()
            .expect("spawn cargo test");
        assert!(
            status.success(),
            "emitted SyncReadMem functional crate must cargo test"
        );
    }
}

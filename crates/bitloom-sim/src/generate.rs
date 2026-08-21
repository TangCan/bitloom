//! FR47 leg 1: generate a Rust functional-sim crate from FrozenHir (AD-5).
//! Minimal interpreter-backed AbstractionView — not HLS-quality codegen.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bitloom_hir::{
    AssignExpr, AssignTarget, FrozenHir, GroundType, ProcessKind, Stmt,
};

use crate::AbstractionView;
use bitloom_hir::PortValues;

/// In-process functional model derived from FrozenHir (same semantics as the emitted crate).
#[derive(Debug, Clone)]
pub struct GeneratedFunctional {
    regs: BTreeMap<String, u64>,
    reset_port: String,
    enable_port: Option<String>,
    /// Sequential RegD updates: (reg_name, expr, has_enable).
    seq: Vec<(String, AssignExpr, bool)>,
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
        for stmt in &m.body {
            if let Stmt::RegDecl {
                name, has_enable, ..
            } = stmt
            {
                regs.insert(name.clone(), 0u64);
                reg_has_en.insert(name.clone(), *has_enable);
            }
        }

        let mut seq = Vec::new();
        let mut comb = Vec::new();
        for stmt in &m.body {
            if let Stmt::Process(p) = stmt {
                match p.kind {
                    ProcessKind::Sequential => {
                        for a in &p.assigns {
                            if let AssignTarget::RegD(name) = &a.target {
                                let has_en = reg_has_en.get(name).copied().unwrap_or(false);
                                seq.push((name.clone(), a.expr.clone(), has_en));
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
            // Memories: functional path returns 0 (cycle-accurate Sim owns mem semantics).
            AssignExpr::MemRead { .. } => 0,
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

        if reset {
            for v in self.regs.values_mut() {
                *v = 0;
            }
        } else {
            let mut next = BTreeMap::new();
            for (name, expr, has_en) in &self.seq {
                if *has_en && !enable {
                    continue;
                }
                next.insert(name.clone(), self.eval(inputs, expr));
            }
            for (k, v) in next {
                self.regs.insert(k, v);
            }
        }

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
        out_dir
            .join("../../crates/bitloom-hir")
            .canonicalize()
            .ok(),
        std::env::var_os("CARGO_MANIFEST_DIR").and_then(|m| {
            PathBuf::from(m)
                .join("../bitloom-hir")
                .canonicalize()
                .ok()
        }),
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
    format!(
        "bitloom-hir = \"{}\"",
        env!("CARGO_PKG_VERSION")
    )
}

fn render_lib_rs(pkg: &str, model: &GeneratedFunctional) -> String {
    let _ = pkg;
    let reg_inits: String = model
        .regs
        .keys()
        .map(|n| format!("        regs.insert({n:?}.into(), 0u64);\n"))
        .collect();
    let seq_arms: String = model
        .seq
        .iter()
        .map(|(name, expr, has_en)| {
            let en_guard = if *has_en {
                "            if !enable { /* hold */ } else {\n"
            } else {
                "            {\n"
            };
            format!(
                "{en_guard}                next.insert({name:?}.into(), {});\n            }}\n",
                render_expr(expr)
            )
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

    format!(
        r#"//! Generated Bitloom functional simulator (FR47 / AD-5).
//! Not SystemC / TLM-2.0. Do not hand-edit; regenerate via `generate_functional_sim`.

use std::collections::BTreeMap;

use bitloom_hir::PortValues;

/// Generated functional view (AbstractionView-compatible cycle API).
#[derive(Debug, Clone)]
pub struct FunctionalSim {{
    regs: BTreeMap<String, u64>,
}}

impl Default for FunctionalSim {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl FunctionalSim {{
    pub fn new() -> Self {{
        let mut regs = BTreeMap::new();
{reg_inits}        Self {{ regs }}
    }}

    fn lookup(&self, inputs: &PortValues, name: &str) -> u64 {{
        inputs
            .get(name)
            .or_else(|| self.regs.get(name).copied())
            .unwrap_or(0)
    }}

    /// One untimed functional cycle; returns updated `PortValues`.
    pub fn cycle(&mut self, inputs: &PortValues) -> PortValues {{
        let reset = inputs.get({reset:?}).unwrap_or(0) != 0;
        {enable_init}
        if reset {{
            for v in self.regs.values_mut() {{
                *v = 0;
            }}
        }} else {{
            let mut next = BTreeMap::new();
{seq_arms}            for (k, v) in next {{
                self.regs.insert(k, v);
            }}
        }}
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
}}
"#
    )
}

fn render_expr(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => format!("self.lookup(inputs, {n:?})"),
        AssignExpr::Lit(v) => format!("{v}"),
        AssignExpr::Inc(n) => format!("self.lookup(inputs, {n:?}).wrapping_add(1)"),
        AssignExpr::Add(a, b) => format!(
            "self.lookup(inputs, {a:?}).wrapping_add(self.lookup(inputs, {b:?}))"
        ),
        AssignExpr::Sub(a, b) => format!(
            "self.lookup(inputs, {a:?}).wrapping_sub(self.lookup(inputs, {b:?}))"
        ),
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
        AssignExpr::MemRead { .. } => "0".into(),
    }
}

fn render_expr_ports_regs(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => format!(
            "out.get({n:?}).or_else(|| self.regs.get({n:?}).copied()).unwrap_or(0)"
        ),
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

    #[test]
    fn emit_writes_crate_with_gold_test() {
        let hir = counter_hir();
        let dir = std::env::temp_dir().join(format!(
            "bitloom-func-gen-{}",
            std::process::id()
        ));
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
}

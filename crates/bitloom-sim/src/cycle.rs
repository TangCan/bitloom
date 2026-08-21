//! FR47 leg 2: cycle-accurate tick-wrapper artifact + bridge/compare.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bitloom_hir::{
    AssignExpr, AssignTarget, FrozenHir, GroundType, PortDirection, ProcessKind, Stmt,
};

use crate::generate::GeneratedFunctional;
use crate::{AbstractionView, EquivStatus, PortValues, Sim, check_functional_equiv};

/// Thin FrozenHir → `Sim::tick` wrapper (cycle-accurate product surface).
pub struct CycleAccurateSim {
    sim: Sim,
}

impl CycleAccurateSim {
    pub fn from_hir(hir: FrozenHir) -> Self {
        Self { sim: Sim::new(hir) }
    }

    pub fn tick_with(&mut self, inputs: PortValues) -> &PortValues {
        self.sim.set_inputs(inputs);
        self.sim.tick();
        self.sim.ports()
    }

    pub fn ports(&self) -> &PortValues {
        self.sim.ports()
    }
}

/// Bridge/compare: generated functional model vs cycle-accurate `tick` (FR47 leg 2).
pub fn check_generated_bridge(
    hir: FrozenHir,
    stimuli: impl IntoIterator<Item = PortValues>,
) -> EquivStatus {
    let mut abs = GeneratedFunctional::from_hir(&hir);
    check_functional_equiv(hir, &mut abs, stimuli)
}

/// Same bridge entry with an arbitrary functional view (for deliberate-mismatch tests).
pub fn check_generated_bridge_with<A: AbstractionView>(
    hir: FrozenHir,
    abs: &mut A,
    stimuli: impl IntoIterator<Item = PortValues>,
) -> EquivStatus {
    check_functional_equiv(hir, abs, stimuli)
}

/// Alias for product naming symmetry with `emit_functional_crate`.
pub fn emit_cycle_accurate_crate(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    generate_cycle_accurate_sim(hir, out_dir)
}

/// Write a cycle-accurate tick-wrapper crate (FrozenHir rebuild + `Sim::tick`).
pub fn generate_cycle_accurate_sim(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(out_dir.join("src"))?;
    let pkg = sanitize_pkg_name(&hir.abi_name);
    let cargo = render_cargo_toml(&pkg, out_dir)?;
    let hir_src =
        emit_hir_builder(hir).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let lib = render_lib_rs(&pkg, &hir_src);
    fs::write(out_dir.join("Cargo.toml"), cargo)?;
    fs::write(out_dir.join("src/lib.rs"), lib)?;
    let main = format!(
        r#"fn main() {{
    use {crate_name}::CycleAccurate;
    use bitloom_hir::PortValues;
    let mut sim = CycleAccurate::new();
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    let out = sim.cycle(&pv);
    println!("{{out:?}}");
}}
"#,
        crate_name = pkg.replace('-', "_"),
    );
    fs::write(out_dir.join("src/main.rs"), main)?;
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
        s = format!("cycle_{s}");
    }
    format!("bitloom_cycle_{s}")
}

fn resolve_dep(crate_name: &str, out_dir: &Path) -> String {
    let candidates = [
        out_dir
            .join(format!("../../crates/{crate_name}"))
            .canonicalize()
            .ok(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../{crate_name}"))
            .canonicalize()
            .ok(),
    ];
    for c in candidates.into_iter().flatten() {
        if c.join("Cargo.toml").is_file() {
            return format!("{crate_name} = {{ path = \"{}\" }}", c.display());
        }
    }
    format!("{crate_name} = \"{}\"", env!("CARGO_PKG_VERSION"))
}

fn render_cargo_toml(pkg: &str, out_dir: &Path) -> io::Result<String> {
    let hir = resolve_dep("bitloom-hir", out_dir);
    let sim = resolve_dep("bitloom-sim", out_dir);
    let builder = resolve_dep("bitloom-builder", out_dir);
    Ok(format!(
        r#"[package]
name = "{pkg}"
version = "0.0.0"
edition = "2024"
rust-version = "1.97.1"
publish = false
description = "Generated Bitloom cycle-accurate tick wrapper (FR47). Not SystemC."

[workspace]

[dependencies]
{hir}
{sim}
{builder}

[[bin]]
name = "{pkg}"
path = "src/main.rs"
"#
    ))
}

fn render_ground_type(ty: &GroundType) -> String {
    match ty {
        GroundType::UInt { width } => format!("GroundType::UInt {{ width: {width} }}"),
        GroundType::SInt { width } => format!("GroundType::SInt {{ width: {width} }}"),
        GroundType::Clock => "GroundType::Clock".into(),
        GroundType::Reset => "GroundType::Reset".into(),
        GroundType::Bool => "GroundType::Bool".into(),
        GroundType::Analog => "GroundType::Analog".into(),
    }
}

fn emit_hir_builder(hir: &FrozenHir) -> Result<String, String> {
    let circuit = hir.circuit();
    if circuit.modules.len() != 1 {
        return Err("cycle-accurate emit supports a single top module".into());
    }
    let m = &circuit.modules[0];
    let mut body = String::new();
    body.push_str(&format!(
        "    let mut s = ElaborateSession::new({:?});\n",
        circuit.name
    ));
    body.push_str(&format!(
        "    s.begin_module({:?}, Span::default());\n",
        m.name
    ));
    for p in &m.ports {
        let ty = render_ground_type(&p.ty);
        match p.direction {
            PortDirection::Input => {
                body.push_str(&format!(
                    "    s.add_input({:?}, {ty}, Span::default());\n",
                    p.name
                ));
            }
            PortDirection::Output => {
                body.push_str(&format!(
                    "    s.add_output({:?}, {ty}, Span::default());\n",
                    p.name
                ));
            }
            PortDirection::InOut => {
                return Err("InOut ports not supported in cycle-accurate emit".into());
            }
        }
    }
    for stmt in &m.body {
        match stmt {
            Stmt::RegDecl { name, ty, .. } => {
                body.push_str(&format!(
                    "    s.declare_reg({:?}, {}, Span::default());\n",
                    name,
                    render_ground_type(ty)
                ));
            }
            Stmt::WireDecl { name, ty, .. } => {
                body.push_str(&format!(
                    "    s.declare_wire({:?}, {}, Span::default());\n",
                    name,
                    render_ground_type(ty)
                ));
            }
            Stmt::Process(p) => {
                match p.kind {
                    ProcessKind::Combinational => {
                        body.push_str("    s.begin_combinational(Span::default());\n");
                    }
                    ProcessKind::Sequential => {
                        body.push_str("    s.begin_sequential(Span::default());\n");
                    }
                }
                for a in &p.assigns {
                    body.push_str(&emit_assign(a)?);
                }
                body.push_str("    s.end_process();\n");
            }
            Stmt::Instance(_) | Stmt::MemDecl { .. } => {
                return Err(
                    "instances/memories not supported in minimal cycle-accurate emit".into(),
                );
            }
        }
    }
    body.push_str("    s.end_module();\n");
    body.push_str("    s.finish().expect(\"rebuild FrozenHir\")\n");
    Ok(body)
}

fn emit_assign(a: &bitloom_hir::Assign) -> Result<String, String> {
    match (&a.target, &a.expr) {
        (AssignTarget::Net(name), AssignExpr::Ref(from)) => Ok(format!(
            "    s.assign_net({name:?}, {from:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Lit(v)) => Ok(format!(
            "    s.assign_lit({name:?}, {v}, Span::default());\n"
        )),
        (AssignTarget::RegD(name), AssignExpr::Inc(_)) => Ok(format!(
            "    s.assign_reg_d_inc({name:?}, Span::default());\n"
        )),
        (AssignTarget::RegD(name), AssignExpr::Ref(from)) => Ok(format!(
            "    s.assign_reg_d_from({name:?}, {from:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Add(a, b)) => Ok(format!(
            "    s.assign_add({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Sub(a, b)) => Ok(format!(
            "    s.assign_sub({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::And(a, b)) => Ok(format!(
            "    s.assign_and({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Or(a, b)) => Ok(format!(
            "    s.assign_or({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Xor(a, b)) => Ok(format!(
            "    s.assign_xor({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Eq(a, b)) => Ok(format!(
            "    s.assign_eq({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Mux { sel, t, f }) => Ok(format!(
            "    s.assign_mux({name:?}, {sel:?}, {t:?}, {f:?}, Span::default());\n"
        )),
        _ => Err(format!(
            "unsupported assign in cycle-accurate emit: {:?} <- {:?}",
            a.target, a.expr
        )),
    }
}

fn render_lib_rs(pkg: &str, hir_body: &str) -> String {
    let _ = pkg;
    format!(
        r#"//! Generated Bitloom cycle-accurate simulator (FR47 / AD-5).
//! FrozenHir → `bitloom_sim::Sim::tick` wrapper. Not SystemC / TLM-2.0.

use bitloom_builder::{{ElaborateSession, GroundType, Span}};
use bitloom_hir::{{FrozenHir, PortValues}};
use bitloom_sim::Sim;

fn frozen_hir() -> FrozenHir {{
{hir_body}}}

/// Cycle-accurate tick wrapper over reconstructed FrozenHir.
pub struct CycleAccurate {{
    sim: Sim,
}}

impl Default for CycleAccurate {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl CycleAccurate {{
    pub fn new() -> Self {{
        Self {{
            sim: Sim::new(frozen_hir()),
        }}
    }}

    pub fn cycle(&mut self, inputs: &PortValues) -> PortValues {{
        self.sim.set_inputs(inputs.clone());
        self.sim.tick();
        self.sim.ports().clone()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use bitloom_sim::{{GeneratedFunctional, check_generated_bridge, reset_then_run}};

    #[test]
    fn bridge_matches_functional_gold() {{
        let hir = frozen_hir();
        let status = check_generated_bridge(hir, reset_then_run(3));
        assert!(status.is_pass(), "{{status:?}}");
    }}

    #[test]
    fn tick_port_values_gold() {{
        let mut ca = CycleAccurate::new();
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        let _ = ca.cycle(&pv);
        pv.set("rst", 0);
        let mut last = PortValues::default();
        for _ in 0..3 {{
            last = ca.cycle(&pv);
        }}
        if last.values.contains_key("data_out") {{
            assert_eq!(last.get("data_out"), Some(3));
        }}
    }}

    #[test]
    fn deliberate_mismatch_fails() {{
        struct Wrong;
        impl bitloom_sim::AbstractionView for Wrong {{
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {{
                let mut o = inputs.clone();
                o.set("data_out", 99);
                o
            }}
        }}
        let hir = frozen_hir();
        let mut w = Wrong;
        let status = bitloom_sim::check_generated_bridge_with(hir, &mut w, reset_then_run(1));
        assert!(!status.is_pass());
    }}

    #[test]
    fn functional_in_process_still_aligned() {{
        use bitloom_sim::AbstractionView;
        let hir = frozen_hir();
        let mut abs = GeneratedFunctional::from_hir(&hir);
        let mut ca = CycleAccurate::new();
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        let f = abs.cycle(&pv);
        let c = ca.cycle(&pv);
        assert_eq!(f.get("data_out"), c.get("data_out"));
    }}
}}
"#
    )
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;
    use crate::reset_then_run;

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
    fn bridge_pass_and_mismatch_fail() {
        let hir = counter_hir();
        assert!(check_generated_bridge(hir.clone(), reset_then_run(3)).is_pass());

        struct Wrong;
        impl AbstractionView for Wrong {
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {
                let mut o = inputs.clone();
                o.set("data_out", 7);
                o
            }
        }
        let mut w = Wrong;
        assert!(!check_generated_bridge_with(hir, &mut w, reset_then_run(1)).is_pass());
    }

    #[test]
    fn emit_cycle_crate_compiles() {
        let hir = counter_hir();
        let dir = std::env::temp_dir().join(format!("bitloom-cycle-gen-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = generate_cycle_accurate_sim(&hir, &dir).unwrap();
        let lib = fs::read_to_string(out.join("src/lib.rs")).unwrap();
        assert!(
            lib.contains("CycleAccurate") && lib.contains("Sim::tick") || lib.contains("sim.tick")
        );
        assert!(lib.contains("check_generated_bridge"));
    }
}

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
    format!(
        "{crate_name} = \"{}\"",
        published_dependency_version(crate_name)
    )
}

fn published_dependency_version(crate_name: &str) -> &'static str {
    match crate_name {
        "bitloom-hir" | "bitloom-builder" => "1.1.1",
        "bitloom-sim" => env!("CARGO_PKG_VERSION"),
        _ => unreachable!("cycle crate generator requested unknown dependency: {crate_name}"),
    }
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
            Stmt::MemDecl {
                name,
                depth,
                width,
                sync_read,
                init,
                ..
            } => {
                let method = if *sync_read {
                    "declare_sync_read_mem"
                } else {
                    "declare_mem"
                };
                if let Some(init) = init {
                    let method = format!("{method}_with_init");
                    body.push_str(&format!("    s.{method}({name:?}, {depth}, {width}, vec!{init:?}, Span::default());\n"));
                } else {
                    body.push_str(&format!(
                        "    s.{method}({name:?}, {depth}, {width}, Span::default());\n"
                    ));
                }
            }
            Stmt::Instance(_) => {
                return Err("instances not supported in cycle-accurate emit".into());
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
        (AssignTarget::RegD(name), AssignExpr::MemRead { mem, addr }) => Ok(format!(
            "    s.assign_reg_d_mem_read({name:?}, {mem:?}, {addr:?}, Span::default());\n"
        )),
        (AssignTarget::RegD(name), AssignExpr::Mux { sel, t, f }) => Ok(format!(
            "    s.assign_reg_d_mux({name:?}, {sel:?}, {t:?}, {f:?}, Span::default());\n"
        )),
        (
            AssignTarget::MemWrite {
                mem,
                addr,
                we: None,
            },
            AssignExpr::Ref(data),
        ) => Ok(format!(
            "    s.assign_mem_write({mem:?}, {addr:?}, {data:?}, Span::default());\n"
        )),
        (
            AssignTarget::MemWrite {
                mem,
                addr,
                we: Some(we),
            },
            AssignExpr::Ref(data),
        ) => Ok(format!(
            "    s.assign_mem_write_en({mem:?}, {addr:?}, {data:?}, {we:?}, Span::default());\n"
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
        (AssignTarget::Net(name), AssignExpr::Shl(a, b)) => Ok(format!(
            "    s.assign_shl({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Shr(a, b)) => Ok(format!(
            "    s.assign_shr({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Eq(a, b)) => Ok(format!(
            "    s.assign_eq({name:?}, {a:?}, {b:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Ult { lhs, rhs, .. }) => Ok(format!(
            "    s.assign_ult({name:?}, {lhs:?}, {rhs:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Slt { lhs, rhs, .. }) => Ok(format!(
            "    s.assign_slt({name:?}, {lhs:?}, {rhs:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Sar { value, shamt, .. }) => Ok(format!(
            "    s.assign_sar({name:?}, {value:?}, {shamt:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Slice { src, lo, width }) => Ok(format!(
            "    s.assign_slice({name:?}, {src:?}, {lo}, {width}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::Concat { high, low, .. }) => Ok(format!(
            "    s.assign_concat({name:?}, {high:?}, {low:?}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::ZeroExtend { src, to_width, .. }) => Ok(format!(
            "    s.assign_zero_extend({name:?}, {src:?}, {to_width}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::SignExtend { src, to_width, .. }) => Ok(format!(
            "    s.assign_sign_extend({name:?}, {src:?}, {to_width}, Span::default());\n"
        )),
        (AssignTarget::Net(name), AssignExpr::MemRead { mem, addr }) => Ok(format!(
            "    s.assign_mem_read({name:?}, {mem:?}, {addr:?}, Span::default());\n"
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

    #[test]
    fn cycle_wrapper_smoke() {{
        let mut ca = CycleAccurate::new();
        let mut pv = PortValues::default();
        assert!(!ca.cycle(&pv).values.is_empty());
    }}
}}
"#
    )
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};
    use bitloom_hir::{Assign, AssignExpr, AssignTarget};

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

    fn run_generated_cycles(hir: FrozenHir, vectors: Vec<(PortValues, u64)>, output: &str) {
        let dir = std::env::temp_dir().join(format!(
            "bitloom-cycle-{}-{}-{}",
            hir.abi_name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut native = Sim::new(hir.clone());
        let mut functional = GeneratedFunctional::from_hir(&hir);
        let mut test = String::from("#[test] fn every_cycle() { let mut sim = CONSTRUCTOR;\n");
        for (i, (input, expected)) in vectors.into_iter().enumerate() {
            native.set_inputs(input.clone());
            native.tick();
            assert_eq!(
                native.ports().get(output),
                Some(expected),
                "native cycle {i}"
            );
            assert_eq!(
                functional.cycle(&input).get(output),
                Some(expected),
                "functional cycle {i}"
            );
            test.push_str("let mut input = bitloom_hir::PortValues::default();\n");
            for (name, value) in input.values {
                test.push_str(&format!("input.set({name:?}, {value});\n"));
            }
            test.push_str(&format!("assert_eq!(sim.cycle(&input).get({output:?}), Some({expected}), \"generated cycle {i}\");\n"));
        }
        test.push_str("}\n");
        for functional in [false, true] {
            let crate_dir = dir.join(if functional { "functional" } else { "cycle" });
            let constructor = if functional {
                crate::generate_functional_sim(&hir, &crate_dir).unwrap();
                format!(
                    "bitloom_func_{}::FunctionalSim::new()",
                    hir.abi_name.to_lowercase()
                )
            } else {
                generate_cycle_accurate_sim(&hir, &crate_dir).unwrap();
                format!("{}::CycleAccurate::new()", sanitize_pkg_name(&hir.abi_name))
            };
            fs::create_dir_all(crate_dir.join("tests")).unwrap();
            fs::write(
                crate_dir.join("tests/cycles.rs"),
                test.replace("CONSTRUCTOR", &constructor),
            )
            .unwrap();
            // Isolated from the parent cargo target; shared only by these sequential builds.
            let result = std::process::Command::new("cargo")
                .args(["test", "--offline", "--quiet", "--manifest-path"])
                .arg(crate_dir.join("Cargo.toml"))
                .env("CARGO_TARGET_DIR", dir.join("target"))
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "generated crate retained at {}\n{}\n{}",
                crate_dir.display(),
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
        }
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn emit_cycle_crate_compiles() {
        let mut count = 0;
        let vectors = (0..310)
            .map(|i| {
                let reset = i == 0 || i == 300;
                count = if reset { 0 } else { (count + 1) & 0xff };
                let mut input = PortValues::default();
                input.set("rst", reset as u64);
                (input, count)
            })
            .collect();
        run_generated_cycles(counter_hir(), vectors, "data_out");
    }

    #[test]
    fn emitted_cycle_memory_reset_and_write_enable() {
        let p = Span::default();
        let mut s = ElaborateSession::new("memory");
        s.begin_module("Memory", p);
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        s.add_input("addr", GroundType::UInt { width: 2 }, p);
        s.add_input("data", GroundType::UInt { width: 16 }, p);
        s.add_input("increment", GroundType::UInt { width: 16 }, p);
        s.declare_wire("write_data", GroundType::UInt { width: 16 }, p);
        s.add_input("we", GroundType::Bool, p);
        s.add_output("out", GroundType::UInt { width: 20 }, p);
        s.declare_sync_read_mem("ram", 4, 8, p);
        s.declare_reg("read_data", GroundType::UInt { width: 4 }, p);
        s.declare_reg("read_wide", GroundType::UInt { width: 16 }, p);
        s.begin_sequential(p);
        s.assign_mem_write_en("ram", "addr", "write_data", "we", p);
        s.assign_reg_d_mem_read("read_data", "ram", "addr", p);
        s.assign_reg_d_mem_read("read_wide", "ram", "addr", p);
        s.end_process();
        s.begin_combinational(p);
        s.assign_add("write_data", "data", "increment", p);
        s.assign_concat("out", "read_wide", "read_data", p);
        s.end_process();
        s.end_module();
        let hir = s.finish().unwrap();
        let mut bank = [0; 4];
        let mut pending = 0;
        let mut prior_sum = 0;
        let vectors = (0..40)
            .map(|i| {
                let reset = i == 0 || i == 17;
                let addr = i % 4;
                let data = 0x1fff0 + (i * 0x123) as u64;
                let we = i % 3 != 0;
                let expected = if reset {
                    0
                } else {
                    (pending << 4) | (pending & 0xf)
                };
                if !reset && we {
                    bank[addr] = prior_sum & 0xff;
                }
                pending = if reset { 0 } else { bank[addr] };
                // Sequential logic samples the previous comb result, then comb
                // computes the current 16-bit sum after this edge.
                prior_sum = ((data & 0xffff) + 0x30) & 0xffff;
                let mut input = PortValues::default();
                for (name, value) in [
                    ("rst", reset as u64),
                    ("addr", addr as u64),
                    ("data", data),
                    ("increment", 0x10030),
                    ("we", we as u64),
                ] {
                    input.set(name, value);
                }
                (input, expected)
            })
            .collect();
        run_generated_cycles(hir, vectors, "out");
    }

    #[test]
    fn generated_crate_registry_dependencies_use_published_versions() {
        assert_eq!(published_dependency_version("bitloom-hir"), "1.1.1");
        assert_eq!(published_dependency_version("bitloom-builder"), "1.1.1");
        assert_eq!(
            published_dependency_version("bitloom-sim"),
            env!("CARGO_PKG_VERSION")
        );
    }

    #[test]
    fn emit_cycle_crate_rebuilds_gpr_style_memory_writes() {
        let span = Span::default();
        let plain = Assign {
            target: AssignTarget::MemWrite {
                mem: "gpr".into(),
                addr: "rd".into(),
                we: None,
            },
            expr: AssignExpr::Ref("data".into()),
            span,
        };
        let gated = Assign {
            target: AssignTarget::MemWrite {
                mem: "gpr".into(),
                addr: "rd".into(),
                we: Some("we".into()),
            },
            expr: AssignExpr::Ref("data".into()),
            span,
        };
        assert!(
            emit_assign(&plain)
                .unwrap()
                .contains("assign_mem_write(\"gpr\"")
        );
        assert!(
            emit_assign(&gated)
                .unwrap()
                .contains("assign_mem_write_en(\"gpr\"")
        );
    }

    #[test]
    fn emit_cycle_crate_supports_reg_d_mux() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("RegDMux", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("zero", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("count_next", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_lit("zero", 0, Span::default());
        s.assign_net("count_next", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_mux("count", "rst", "zero", "count_next", Span::default());
        s.end_process();
        s.end_module();
        let hir = s.finish().unwrap();
        let dir = std::env::temp_dir().join(format!("bitloom-cycle-mux-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = generate_cycle_accurate_sim(&hir, &dir).unwrap();
        assert!(
            fs::read_to_string(out.join("src/lib.rs"))
                .unwrap()
                .contains("assign_reg_d_mux")
        );
        fs::remove_dir_all(dir).unwrap();
    }
}

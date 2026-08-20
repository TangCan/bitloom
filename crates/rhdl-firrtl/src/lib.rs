//! FIRRTL 6.0.0 text import/export (AD-3, AD-8). No firtool invocation.

use bitloom_hir::{
    Artifact, Assign, AssignExpr, AssignTarget, EmittedFile, FrozenHir, GroundType, Instance,
    Module, Port, PortConnect, PortDirection, Process, ProcessKind, Stmt,
};

mod chisel;
pub use chisel::{CHISEL_TARGET, ChiselGenError, emit_chisel};

/// Emit `<abi_name>.fir` with FIRRTL version 6.0.0 header.
pub fn emit(hir: &FrozenHir) -> Artifact {
    let mut body = String::from("FIRRTL version 6.0.0\n");
    body.push_str(&format!("circuit {} :\n", hir.abi_name));
    for m in &hir.circuit().modules {
        body.push_str(&emit_module(m));
    }
    let path = format!("{}.fir", hir.abi_name);
    Artifact {
        files: vec![EmittedFile {
            path: path.clone(),
            contents: body,
        }],
        filelist: vec![path],
    }
}

fn fir_type(ty: &GroundType) -> String {
    match ty {
        GroundType::UInt { width } => format!("UInt<{width}>"),
        GroundType::SInt { width } => format!("SInt<{width}>"),
        GroundType::Clock => "Clock".into(),
        GroundType::Reset => "Reset".into(),
        GroundType::Bool => "UInt<1>".into(),
        GroundType::Analog => "Analog".into(),
    }
}

fn emit_expr(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => n.clone(),
        AssignExpr::Lit(v) => format!("UInt({v})"),
        AssignExpr::Inc(n) => format!("add({n}, UInt(1))"),
        AssignExpr::Add(a, b) => format!("add({a}, {b})"),
        AssignExpr::Eq(a, b) => format!("eq({a}, {b})"),
        AssignExpr::Mux { sel, t, f } => format!("mux({sel}, {t}, {f})"),
        AssignExpr::MemRead { mem, addr } => format!("{mem}[{addr}]"),
    }
}

fn emit_module(m: &Module) -> String {
    let mut out = format!("  module {} :\n", m.name);
    for p in &m.ports {
        let dir = match p.direction {
            PortDirection::Input => "input",
            PortDirection::Output => "output",
            PortDirection::InOut => "inout",
        };
        out.push_str(&format!("    {} {}: {}\n", dir, p.name, fir_type(&p.ty)));
    }
    for stmt in &m.body {
        match stmt {
            Stmt::WireDecl { name, ty, .. } => {
                out.push_str(&format!("    wire {}: {}\n", name, fir_type(ty)));
            }
            Stmt::RegDecl {
                name,
                ty,
                clock,
                reset,
                ..
            } => {
                out.push_str(&format!(
                    "    reg {}: {}, {} with:\n      reset => ({}, UInt(0))\n",
                    name,
                    fir_type(ty),
                    clock,
                    reset
                ));
            }
            Stmt::MemDecl {
                name,
                depth,
                width,
                sync_read,
                ..
            } => {
                let ruw = if *sync_read { "smem" } else { "cmem" };
                out.push_str(&format!(
                    "    ; CHIRRTL-friendly {ruw} {name} : UInt<{width}>[{depth}]\n"
                ));
                let rlat = if *sync_read { 1 } else { 0 };
                out.push_str(&format!(
                    "    mem {name} :\n      data-type => UInt<{width}>\n      depth => {depth}\n      read-latency => {rlat}\n      write-latency => 1\n      readwriter => rw\n      read-under-write => undefined\n"
                ));
            }
            Stmt::Process(p) => {
                for a in &p.assigns {
                    match (&a.target, p.kind) {
                        (AssignTarget::Net(n), ProcessKind::Combinational) => {
                            out.push_str(&format!("    {n} <= {}\n", emit_expr(&a.expr)));
                        }
                        (AssignTarget::RegD(n), ProcessKind::Sequential) => {
                            out.push_str(&format!("    {n} <= {}\n", emit_expr(&a.expr)));
                        }
                        (AssignTarget::MemWrite { mem, addr }, ProcessKind::Sequential) => {
                            out.push_str(&format!(
                                "    ; mem write {mem}[{addr}] <= {}\n",
                                emit_expr(&a.expr)
                            ));
                        }
                        _ => {}
                    }
                }
            }
            Stmt::Instance(inst) => {
                out.push_str(&format!("    inst {} of {}\n", inst.name, inst.module));
                for c in &inst.connects {
                    if !c.dangling {
                        out.push_str(&format!(
                            "    {}.{} <= {}\n",
                            inst.name, c.child_port, c.parent_net
                        ));
                    }
                }
            }
        }
    }
    out
}

/// Import a FIRRTL 6 subset into FrozenHir via the same private freeze path.
pub fn import(text: &str) -> Result<FrozenHir, bitloom_hir::Diagnostics> {
    if !text.contains("FIRRTL version 6.0.0") {
        return Err(bitloom_hir::Diagnostics(vec![bitloom_hir::Diagnostic {
            span: bitloom_hir::Span::default(),
            code: "rhdl::E0401".into(),
            en: "FIRRTL import requires 'FIRRTL version 6.0.0' header".into(),
            zh: "FIRRTL 导入需要 'FIRRTL version 6.0.0' 头".into(),
        }]));
    }
    if text.contains("Analog") || text.contains("assert") || text.contains("chirrtl") {
        return Err(bitloom_hir::Diagnostics(vec![bitloom_hir::Diagnostic {
            span: bitloom_hir::Span::default(),
            code: "rhdl::E0402".into(),
            en: "import rejects Analog/InOut, properties, and CHIRRTL mem".into(),
            zh: "导入拒绝 Analog/InOut、property 与 CHIRRTL mem".into(),
        }]));
    }

    let mut circuit_name = String::from("Imported");
    let mut modules: Vec<Module> = Vec::new();
    let mut current: Option<Module> = None;
    let mut comb_assigns: Vec<Assign> = Vec::new();
    let mut seq_assigns: Vec<Assign> = Vec::new();

    let flush_processes = |m: &mut Module, comb: &mut Vec<Assign>, seq: &mut Vec<Assign>| {
        if !comb.is_empty() {
            m.body.push(Stmt::Process(Process {
                kind: ProcessKind::Combinational,
                assigns: std::mem::take(comb),
                span: bitloom_hir::Span::default(),
            }));
        }
        if !seq.is_empty() {
            m.body.push(Stmt::Process(Process {
                kind: ProcessKind::Sequential,
                assigns: std::mem::take(seq),
                span: bitloom_hir::Span::default(),
            }));
        }
    };

    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("circuit ") {
            circuit_name = rest.trim_end_matches(" :").trim().to_string();
        } else if let Some(rest) = line.strip_prefix("module ") {
            if let Some(mut m) = current.take() {
                flush_processes(&mut m, &mut comb_assigns, &mut seq_assigns);
                modules.push(m);
            }
            let name = rest.trim_end_matches(" :").trim().to_string();
            current = Some(Module {
                name,
                ports: Vec::new(),
                body: Vec::new(),
                span: bitloom_hir::Span::default(),
            });
        } else if let Some(m) = current.as_mut() {
            if let Some(rest) = line.strip_prefix("input ") {
                if let Some((name, ty)) = rest.split_once(':') {
                    m.ports.push(Port {
                        name: name.trim().to_string(),
                        direction: PortDirection::Input,
                        ty: parse_ty(ty.trim()),
                        span: bitloom_hir::Span::default(),
                    });
                }
            } else if let Some(rest) = line.strip_prefix("output ") {
                if let Some((name, ty)) = rest.split_once(':') {
                    m.ports.push(Port {
                        name: name.trim().to_string(),
                        direction: PortDirection::Output,
                        ty: parse_ty(ty.trim()),
                        span: bitloom_hir::Span::default(),
                    });
                }
            } else if let Some(rest) = line.strip_prefix("wire ") {
                if let Some((name, ty)) = rest.split_once(':') {
                    m.body.push(Stmt::WireDecl {
                        name: name.trim().to_string(),
                        ty: parse_ty(ty.trim()),
                        span: bitloom_hir::Span::default(),
                    });
                }
            } else if let Some(rest) = line.strip_prefix("reg ") {
                // reg count: UInt<8>, clk with:
                if let Some((lhs, _)) = rest.split_once(" with:") {
                    if let Some((name, ty_clk)) = lhs.split_once(':') {
                        let name = name.trim().to_string();
                        let mut parts = ty_clk.split(',');
                        let ty = parse_ty(parts.next().unwrap_or("UInt<1>").trim());
                        let clock = parts.next().unwrap_or("clk").trim().to_string();
                        m.body.push(Stmt::RegDecl {
                            name,
                            ty,
                            clock,
                            reset: "rst".into(),
                            async_reset: false,
                            has_enable: false,
                            span: bitloom_hir::Span::default(),
                        });
                    }
                }
            } else if let Some(rest) = line.strip_prefix("inst ") {
                // inst u0 of Child
                if let Some((iname, rest)) = rest.split_once(" of ") {
                    m.body.push(Stmt::Instance(Instance {
                        name: iname.trim().to_string(),
                        module: rest.trim().to_string(),
                        connects: Vec::new(),
                        params: Vec::new(),
                        span: bitloom_hir::Span::default(),
                    }));
                }
            } else if line.contains(" <= ") {
                let (lhs, rhs) = line.split_once(" <= ").unwrap();
                let lhs = lhs.trim();
                let rhs = rhs.trim();
                if let Some((inst, port)) = lhs.split_once('.') {
                    if let Some(Stmt::Instance(i)) = m
                        .body
                        .iter_mut()
                        .rev()
                        .find(|s| matches!(s, Stmt::Instance(ii) if ii.name == inst))
                    {
                        i.connects.push(PortConnect {
                            child_port: port.to_string(),
                            parent_net: rhs.to_string(),
                            span: bitloom_hir::Span::default(),
                            dangling: false,
                        });
                    }
                } else {
                    let expr = parse_expr(rhs);
                    let is_reg = m
                        .body
                        .iter()
                        .any(|s| matches!(s, Stmt::RegDecl { name, .. } if name == lhs));
                    let a = Assign {
                        target: if is_reg {
                            AssignTarget::RegD(lhs.to_string())
                        } else {
                            AssignTarget::Net(lhs.to_string())
                        },
                        expr,
                        span: bitloom_hir::Span::default(),
                    };
                    if is_reg {
                        seq_assigns.push(a);
                    } else {
                        comb_assigns.push(a);
                    }
                }
            }
        }
    }
    if let Some(mut m) = current.take() {
        flush_processes(&mut m, &mut comb_assigns, &mut seq_assigns);
        modules.push(m);
    }

    let mut owned = bitloom_hir::BuilderOwnedHir::new(circuit_name);
    for m in modules {
        owned.add_module(m);
    }
    bitloom_hir::seal_from_builder(owned)
}

fn parse_expr(s: &str) -> AssignExpr {
    if let Some(inner) = s.strip_prefix("add(").and_then(|x| x.strip_suffix(')')) {
        let parts: Vec<_> = inner.split(',').map(str::trim).collect();
        if parts.len() == 2 {
            if parts[1] == "UInt(1)" {
                return AssignExpr::Inc(parts[0].to_string());
            }
            return AssignExpr::Add(parts[0].to_string(), parts[1].to_string());
        }
    }
    if let Some(v) = s
        .strip_prefix("UInt(")
        .and_then(|x| x.strip_suffix(')'))
        .and_then(|x| x.parse().ok())
    {
        return AssignExpr::Lit(v);
    }
    AssignExpr::Ref(s.to_string())
}

fn parse_ty(s: &str) -> GroundType {
    match s {
        "Clock" => GroundType::Clock,
        "Reset" => GroundType::Reset,
        _ => {
            if let Some(w) = s
                .strip_prefix("UInt<")
                .and_then(|x| x.strip_suffix('>'))
                .and_then(|x| x.parse().ok())
            {
                GroundType::UInt { width: w }
            } else if let Some(w) = s
                .strip_prefix("SInt<")
                .and_then(|x| x.strip_suffix('>'))
                .and_then(|x| x.parse().ok())
            {
                GroundType::SInt { width: w }
            } else {
                GroundType::UInt { width: 1 }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;

    fn hierarchical_sample() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("r", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("r", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Top", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "x".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        s.finish().unwrap()
    }

    #[test]
    fn emit_has_version_header() {
        let art = emit(&hierarchical_sample());
        assert!(art.files[0].contents.starts_with("FIRRTL version 6.0.0"));
        assert_eq!(art.filelist[0], "Child.fir");
    }

    #[test]
    fn roundtrip_hierarchy_reg_and_instance() {
        let frozen = hierarchical_sample();
        let text = emit(&frozen).files[0].contents.clone();
        let back = import(&text).expect("import");
        assert_eq!(back.circuit().modules.len(), frozen.circuit().modules.len());
        let child = &back.circuit().modules[0];
        assert!(
            child
                .body
                .iter()
                .any(|s| matches!(s, Stmt::RegDecl { name, .. } if name == "r"))
        );
        assert!(child.body.iter().any(|s| matches!(
            s,
            Stmt::Process(p) if p.kind == ProcessKind::Combinational
                && p.assigns.iter().any(|a| matches!(&a.target, AssignTarget::Net(n) if n == "y"))
        )));
        let top = &back.circuit().modules[1];
        assert!(top.body.iter().any(|s| matches!(
            s,
            Stmt::Instance(i) if i.name == "u0" && i.module == "Child" && i.connects.len() == 4
        )));
    }

    fn counter_for_chisel() -> FrozenHir {
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
    fn chisel_best_effort_emits_scala() {
        assert_eq!(CHISEL_TARGET, "7.14.0");
        let art = emit_chisel(&counter_for_chisel()).unwrap();
        let scala = &art.files[0].contents;
        assert!(scala.contains("target Chisel 7.14.0"));
        assert!(scala.contains("NOT a round-trip contract"));
        assert!(scala.contains("class Counter extends Module"));
        assert!(scala.contains("io.data_out := count"));
        assert!(scala.contains("count := count + 1.U"));
        // FIRRTL path unchanged
        assert!(
            emit(&counter_for_chisel()).files[0]
                .contents
                .starts_with("FIRRTL version 6.0.0")
        );
    }

    #[test]
    fn chisel_best_effort_fails_on_instance() {
        let err = emit_chisel(&hierarchical_sample()).unwrap_err();
        assert_eq!(err.code, "rhdl::E0902");
    }

    #[test]
    fn chisel_best_effort_fails_on_mem() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("MemTop", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_sync_read_mem("ram", 4, 8, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "ram", Span::default());
        s.end_process();
        s.end_module();
        let err = emit_chisel(&s.finish().unwrap()).unwrap_err();
        assert_eq!(err.code, "rhdl::E0901");
        assert!(err.en.contains("mem"));
    }
}

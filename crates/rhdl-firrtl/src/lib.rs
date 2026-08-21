//! FIRRTL 6.0.0 text import/export (AD-3, AD-8). No firtool invocation.

use bitloom_hir::{
    Artifact, Assign, AssignExpr, AssignTarget, EmittedFile, FrozenHir, GroundType, Instance,
    Module, Port, PortConnect, PortDirection, Process, ProcessKind, Stmt,
};

mod chisel;
pub use chisel::{CHISEL_TARGET, ChiselGenError, FIRTOOL_TARGET, emit_chisel};

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
        AssignExpr::Sub(a, b) => format!("sub({a}, {b})"),
        AssignExpr::And(a, b) => format!("and({a}, {b})"),
        AssignExpr::Or(a, b) => format!("or({a}, {b})"),
        AssignExpr::Xor(a, b) => format!("xor({a}, {b})"),
        AssignExpr::Shl(a, b) => format!("dshl({a}, {b})"),
        AssignExpr::Shr(a, b) => format!("dshr({a}, {b})"),
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
                        (AssignTarget::MemWrite { mem, addr, we }, ProcessKind::Sequential) => {
                            let gate = we.as_ref().map(|e| format!(" if {e}")).unwrap_or_default();
                            out.push_str(&format!(
                                "    ; mem write{gate} {mem}[{addr}] <= {}\n",
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
    let mut import_errs: Vec<bitloom_hir::Diagnostic> = Vec::new();

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
                // Bitloom emit: `inst.port <= parent`. firtool/Chisel output often uses
                // `parent <= inst.port` for child outputs (FR46 reverse).
                let mut took_inst = false;
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
                    } else {
                        import_errs.push(bitloom_hir::Diagnostic {
                            span: bitloom_hir::Span::default(),
                            code: "rhdl::E0403".into(),
                            en: format!(
                                "import rejects unknown instance connect `{inst}.{port} <= …` (no matching `inst {inst}`)"
                            ),
                            zh: format!(
                                "导入拒绝未知实例连接 `{inst}.{port} <= …`（无匹配的 `inst {inst}`）"
                            ),
                        });
                    }
                    took_inst = true;
                } else if let Some((inst, port)) = rhs.split_once('.') {
                    if let Some(Stmt::Instance(i)) = m
                        .body
                        .iter_mut()
                        .rev()
                        .find(|s| matches!(s, Stmt::Instance(ii) if ii.name == inst))
                    {
                        i.connects.push(PortConnect {
                            child_port: port.to_string(),
                            parent_net: lhs.to_string(),
                            span: bitloom_hir::Span::default(),
                            dangling: false,
                        });
                    } else {
                        import_errs.push(bitloom_hir::Diagnostic {
                            span: bitloom_hir::Span::default(),
                            code: "rhdl::E0403".into(),
                            en: format!(
                                "import rejects unknown `… <= {inst}.{port}` (no matching `inst {inst}`; will not treat as ordinary assign)"
                            ),
                            zh: format!(
                                "导入拒绝未知 `… <= {inst}.{port}`（无匹配的 `inst {inst}`；不会降为普通赋值）"
                            ),
                        });
                    }
                    took_inst = true;
                }
                if !took_inst {
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

    if !import_errs.is_empty() {
        return Err(bitloom_hir::Diagnostics(import_errs));
    }

    let mut owned = bitloom_hir::BuilderOwnedHir::new(circuit_name);
    for m in modules {
        owned.add_module(m);
    }
    bitloom_hir::seal_from_builder(owned)
}

/// FR46: public port name / width / direction round-trip between two FrozenHir circuits.
/// Modules are matched by name; port order may differ.
pub fn ports_roundtrip_ok(expected: &FrozenHir, got: &FrozenHir) -> Result<(), String> {
    let exp: std::collections::HashMap<&str, &Module> = expected
        .circuit()
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();
    let got_map: std::collections::HashMap<&str, &Module> = got
        .circuit()
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();
    if exp.len() != got_map.len() {
        return Err(format!(
            "module count: expected {}, got {}",
            exp.len(),
            got_map.len()
        ));
    }
    for (name, em) in &exp {
        let gm = got_map
            .get(name)
            .ok_or_else(|| format!("missing module `{name}` after import"))?;
        if em.ports.len() != gm.ports.len() {
            return Err(format!(
                "module `{name}` port count: expected {}, got {}",
                em.ports.len(),
                gm.ports.len()
            ));
        }
        let mut gp: std::collections::HashMap<&str, &Port> =
            gm.ports.iter().map(|p| (p.name.as_str(), p)).collect();
        for ep in &em.ports {
            let p = gp
                .remove(ep.name.as_str())
                .ok_or_else(|| format!("module `{name}` missing port `{}`", ep.name))?;
            if p.direction != ep.direction {
                return Err(format!(
                    "module `{name}` port `{}` direction mismatch",
                    ep.name
                ));
            }
            if p.ty != ep.ty {
                return Err(format!(
                    "module `{name}` port `{}` type mismatch: {:?} vs {:?}",
                    ep.name, ep.ty, p.ty
                ));
            }
        }
    }
    Ok(())
}

/// FR46: instance graph round-trip (instance name, child module, connects).
/// Connects compared as unordered `(child_port, parent_net, dangling)` sets.
pub fn instance_graph_roundtrip_ok(expected: &FrozenHir, got: &FrozenHir) -> Result<(), String> {
    let exp: std::collections::HashMap<&str, &Module> = expected
        .circuit()
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();
    let got_map: std::collections::HashMap<&str, &Module> = got
        .circuit()
        .modules
        .iter()
        .map(|m| (m.name.as_str(), m))
        .collect();
    for (name, em) in &exp {
        let gm = got_map
            .get(name)
            .ok_or_else(|| format!("missing module `{name}`"))?;
        let e_insts: Vec<_> = em
            .body
            .iter()
            .filter_map(|s| match s {
                Stmt::Instance(i) => Some(i),
                _ => None,
            })
            .collect();
        let g_insts: Vec<_> = gm
            .body
            .iter()
            .filter_map(|s| match s {
                Stmt::Instance(i) => Some(i),
                _ => None,
            })
            .collect();
        if e_insts.len() != g_insts.len() {
            return Err(format!(
                "module `{name}` instance count: expected {}, got {}",
                e_insts.len(),
                g_insts.len()
            ));
        }
        let mut g_by_name: std::collections::HashMap<&str, &Instance> =
            g_insts.iter().map(|i| (i.name.as_str(), *i)).collect();
        for ei in e_insts {
            let gi = g_by_name
                .remove(ei.name.as_str())
                .ok_or_else(|| format!("module `{name}` missing instance `{}`", ei.name))?;
            if gi.module != ei.module {
                return Err(format!(
                    "instance `{}` module: expected {}, got {}",
                    ei.name, ei.module, gi.module
                ));
            }
            let eset: std::collections::HashSet<_> = ei
                .connects
                .iter()
                .map(|c| (c.child_port.as_str(), c.parent_net.as_str(), c.dangling))
                .collect();
            let gset: std::collections::HashSet<_> = gi
                .connects
                .iter()
                .map(|c| (c.child_port.as_str(), c.parent_net.as_str(), c.dangling))
                .collect();
            if eset != gset {
                let e_dangling: Vec<_> = ei
                    .connects
                    .iter()
                    .filter(|c| c.dangling)
                    .map(|c| c.child_port.as_str())
                    .collect();
                let g_dangling: Vec<_> = gi
                    .connects
                    .iter()
                    .filter(|c| c.dangling)
                    .map(|c| c.child_port.as_str())
                    .collect();
                if e_dangling != g_dangling {
                    return Err(format!(
                        "instance `{}` dangling connects mismatch (FIRRTL emit omits dangling; they cannot round-trip): expected dangling {e_dangling:?}, got {g_dangling:?}; full expected {eset:?}, got {gset:?}",
                        ei.name
                    ));
                }
                return Err(format!(
                    "instance `{}` connects mismatch: expected {eset:?}, got {gset:?}",
                    ei.name
                ));
            }
        }
    }
    Ok(())
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

    /// Port name / width / direction predicates for FR28 (skip clk/rst).
    fn assert_port_predicates(scala: &str, hir: &FrozenHir) {
        for m in &hir.circuit().modules {
            assert!(
                scala.contains(&format!("class {} extends Module", m.name)),
                "missing class for module {}",
                m.name
            );
            for p in &m.ports {
                if p.name == "clk" || p.name == "rst" {
                    continue;
                }
                let dir = match p.direction {
                    PortDirection::Input => "Input",
                    PortDirection::Output => "Output",
                    PortDirection::InOut => "Analog",
                };
                let ty = match &p.ty {
                    GroundType::UInt { width } => format!("UInt({width}.W)"),
                    GroundType::SInt { width } => format!("SInt({width}.W)"),
                    GroundType::Bool => "Bool()".into(),
                    GroundType::Clock => "Clock()".into(),
                    GroundType::Reset => "Reset()".into(),
                    GroundType::Analog => "Analog()".into(),
                };
                let field = format!("val {} = {dir}({ty})", p.name);
                assert!(
                    scala.contains(&field),
                    "port predicate failed for {}.{}: expected `{field}`",
                    m.name,
                    p.name
                );
            }
        }
    }

    /// Instance hierarchy predicates: Module(new Child) + directional connects.
    fn assert_hierarchy_predicates(scala: &str, hir: &FrozenHir) {
        let modules: std::collections::HashMap<&str, &Module> = hir
            .circuit()
            .modules
            .iter()
            .map(|m| (m.name.as_str(), m))
            .collect();
        for m in &hir.circuit().modules {
            for stmt in &m.body {
                let Stmt::Instance(inst) = stmt else {
                    continue;
                };
                assert!(
                    scala.contains(&format!("val {} = Module(new {})", inst.name, inst.module)),
                    "missing Module(new {}) for instance {}",
                    inst.module,
                    inst.name
                );
                let child = modules
                    .get(inst.module.as_str())
                    .expect("child module in circuit");
                for c in &inst.connects {
                    if c.dangling || c.child_port == "clk" || c.child_port == "rst" {
                        continue;
                    }
                    let port = child
                        .ports
                        .iter()
                        .find(|p| p.name == c.child_port)
                        .expect("child port");
                    let parent_ref = if m.ports.iter().any(|p| p.name == c.parent_net) {
                        format!("io.{}", c.parent_net)
                    } else {
                        c.parent_net.clone()
                    };
                    let child_io = format!("{}.io.{}", inst.name, c.child_port);
                    let expected = match port.direction {
                        PortDirection::Input => format!("{child_io} := {parent_ref}"),
                        PortDirection::Output => format!("{parent_ref} := {child_io}"),
                        PortDirection::InOut => continue,
                    };
                    assert!(
                        scala.contains(&expected),
                        "connect predicate failed: expected `{expected}`"
                    );
                }
            }
        }
    }

    #[test]
    fn chisel_fr28_flat_counter_emits_scala() {
        assert_eq!(CHISEL_TARGET, "7.14.0");
        assert_eq!(FIRTOOL_TARGET, "1.155.0");
        let hir = counter_for_chisel();
        let art = emit_chisel(&hir).unwrap();
        let scala = &art.files[0].contents;
        assert!(scala.contains("target Chisel 7.14.0"));
        assert!(scala.contains("firtool-1.155.0"));
        assert!(scala.contains("FR28 compilable Chisel"));
        assert!(scala.contains("class Counter extends Module"));
        assert!(scala.contains("io.data_out := count"));
        assert!(scala.contains("count := count + 1.U"));
        assert_port_predicates(scala, &hir);
        // FIRRTL path unchanged (AD-3)
        assert!(
            emit(&hir).files[0]
                .contents
                .starts_with("FIRRTL version 6.0.0")
        );
    }

    #[test]
    fn chisel_fr28_hierarchy_emits_module_new_and_connects() {
        let hir = hierarchical_sample();
        let art = emit_chisel(&hir).expect("hierarchy must emit; E0902 removed");
        let scala = &art.files[0].contents;
        assert!(scala.contains("val u0 = Module(new Child)"));
        assert!(scala.contains("u0.io.x := io.x"));
        assert!(scala.contains("io.y := u0.io.y"));
        assert!(!scala.contains("u0.io.clk"));
        assert!(!scala.contains("u0.io.rst"));
        assert_port_predicates(scala, &hir);
        assert_hierarchy_predicates(scala, &hir);
    }

    #[test]
    fn chisel_fr28_hierarchy_via_wire_parent_net() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Top", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("w", "x", Span::default());
        s.end_process();
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "w".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let hir = s.finish().unwrap();
        let scala = emit_chisel(&hir).unwrap().files[0].contents.clone();
        assert!(scala.contains("val w = Wire(UInt(8.W))"));
        assert!(scala.contains("u0.io.x := w"));
        assert!(!scala.contains("u0.io.x := io.w"));
        assert_hierarchy_predicates(&scala, &hir);
    }

    #[test]
    fn chisel_fr28_dangling_connect_omitted() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("spare", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
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
        s.add_dangling_input("u0", "spare", Span::default());
        s.end_module();
        let hir = s.finish().unwrap();
        let scala = emit_chisel(&hir).unwrap().files[0].contents.clone();
        assert!(scala.contains("u0.io.x := io.x"));
        assert!(
            !scala.contains("u0.io.spare"),
            "dangling child input must not appear as a connect"
        );
        assert_hierarchy_predicates(&scala, &hir);
    }

    #[test]
    fn chisel_fr28_fir_import_then_emit() {
        let frozen = hierarchical_sample();
        let fir = emit(&frozen).files[0].contents.clone();
        let back = import(&fir).expect("import .fir");
        let scala = emit_chisel(&back).expect("emit_chisel after import").files[0]
            .contents
            .clone();
        assert_port_predicates(&scala, &back);
        assert_hierarchy_predicates(&scala, &back);
        assert!(scala.contains("Module(new Child)"));
        assert!(scala.contains("target Chisel 7.14.0"));
        assert!(scala.contains("firtool-1.155.0"));
    }

    #[test]
    fn chisel_fr28_pin_locked() {
        assert_eq!(CHISEL_TARGET, "7.14.0");
        assert_eq!(FIRTOOL_TARGET, "1.155.0");
        let scala = emit_chisel(&counter_for_chisel()).unwrap().files[0]
            .contents
            .clone();
        assert!(scala.contains("Chisel 7.14.0"));
        assert!(scala.contains("firtool-1.155.0"));
    }

    #[test]
    fn chisel_fr28_fails_on_mem() {
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

    #[test]
    fn fr46_export_reimport_ports_and_instance_graph() {
        let frozen = hierarchical_sample();
        let fir = emit(&frozen).files[0].contents.clone();
        let back = import(&fir).expect("export→import");
        ports_roundtrip_ok(&frozen, &back).expect("FR46 port predicates");
        instance_graph_roundtrip_ok(&frozen, &back).expect("FR46 instance graph");
        // Re-emit FIRRTL and Chisel from imported HIR (same backend path as Bitloom surface).
        let re = emit(&back).files[0].contents.clone();
        assert!(re.starts_with("FIRRTL version 6.0.0"));
        let scala = emit_chisel(&back)
            .expect("emit_chisel after FR46 import")
            .files[0]
            .contents
            .clone();
        assert!(scala.contains("Module(new Child)"));
    }

    #[test]
    fn fr46_external_fir_firtool_style_connects_then_emit() {
        let fir = include_str!("../fixtures/external_hierarchy.fir");
        assert!(
            fir.contains("y <= u0.y"),
            "fixture must use firtool-style output connect"
        );
        let hir = import(fir).expect("import external .fir");
        let child = hir
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == "Child")
            .expect("Child");
        assert!(
            child
                .ports
                .iter()
                .any(|p| p.name == "x" && p.direction == PortDirection::Input)
        );
        let top = hir
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == "ExternalTop")
            .expect("ExternalTop");
        let inst = top
            .body
            .iter()
            .find_map(|s| match s {
                Stmt::Instance(i) if i.name == "u0" => Some(i),
                _ => None,
            })
            .expect("u0");
        assert_eq!(inst.module, "Child");
        assert!(
            inst.connects
                .iter()
                .any(|c| c.child_port == "y" && c.parent_net == "y" && !c.dangling)
        );
        let v = bitloom_vlog::emit(&hir);
        assert!(!v.files.is_empty());
        assert!(v.files[0].contents.contains("module"));
        let fir_out = emit(&hir).files[0].contents.clone();
        assert!(fir_out.starts_with("FIRRTL version 6.0.0"));
        let scala = emit_chisel(&hir).expect("chisel from external").files[0]
            .contents
            .clone();
        assert!(scala.contains("Module(new Child)"));
    }

    #[test]
    fn fr46_import_counter_then_tick() {
        let hir = counter_for_chisel();
        let fir = emit(&hir).files[0].contents.clone();
        let back = import(&fir).expect("import counter");
        ports_roundtrip_ok(&hir, &back).unwrap();
        let mut sim = bitloom_sim::Sim::new(back);
        let mut pv = bitloom_hir::PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(2));
    }

    #[test]
    fn fr46_rejects_unknown_parent_le_inst_port() {
        let fir = r#"FIRRTL version 6.0.0
circuit BadRhs :
  module BadRhs :
    input clock : Clock
    input reset : UInt<1>
    input x : UInt<8>
    output y : UInt<8>
    y <= missing.y
"#;
        let err = import(fir).expect_err("unknown rhs inst.port must reject");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0403"),
            "expected E0403, got {err}"
        );
        assert!(
            err.0.iter().any(|d| d.en.contains("missing.y")),
            "diagnostic should name unknown inst.port, got {err}"
        );
    }

    #[test]
    fn fr46_rejects_unknown_inst_port_le_parent() {
        let fir = r#"FIRRTL version 6.0.0
circuit BadLhs :
  module BadLhs :
    input clock : Clock
    input reset : UInt<1>
    input x : UInt<8>
    output y : UInt<8>
    ghost.x <= x
"#;
        let err = import(fir).expect_err("unknown lhs inst.port must reject");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0403"),
            "expected E0403, got {err}"
        );
        assert!(
            err.0.iter().any(|d| d.en.contains("ghost.x")),
            "diagnostic should name unknown inst.port, got {err}"
        );
    }

    #[test]
    fn fr46_dangling_connect_roundtrip_predicate_fails_clearly() {
        fn top_with_child(spare_dangling: bool) -> FrozenHir {
            let mut s = ElaborateSession::new("t");
            s.begin_module("Child", Span::default());
            s.add_input("clk", GroundType::Clock, Span::default());
            s.add_input("rst", GroundType::Reset, Span::default());
            s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
            if spare_dangling {
                s.add_input("spare", GroundType::UInt { width: 8 }, Span::default());
            }
            s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
            s.begin_combinational(Span::default());
            s.assign_net("y", "x", Span::default());
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
            if spare_dangling {
                s.add_dangling_input("u0", "spare", Span::default());
            }
            s.end_module();
            s.finish().unwrap()
        }

        let with_dangling = top_with_child(true);
        let fir = emit(&with_dangling).files[0].contents.clone();
        assert!(
            !fir.contains("u0.spare"),
            "FIRRTL emit must omit dangling instance connects, got:\n{fir}"
        );
        // Emit drops dangling marks; a re-imported graph matches the no-dangling shape.
        // Strengthened predicate must fail clearly rather than silently accepting.
        let without = top_with_child(false);
        let err = instance_graph_roundtrip_ok(&with_dangling, &without)
            .expect_err("dangling must fail FR46 instance-graph round-trip");
        assert!(
            err.contains("dangling"),
            "predicate message must call out dangling: {err}"
        );
    }
}

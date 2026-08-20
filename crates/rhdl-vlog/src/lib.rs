//! Yosys-friendly Verilog emitter (AD-8, AD-16).

use bitloom_hir::{
    Artifact, AssignExpr, AssignTarget, EmittedFile, FrozenHir, GroundType, PortDirection,
    ProcessKind, Stmt,
};

/// Emit `<abi_name>.v` from a frozen circuit.
pub fn emit(hir: &FrozenHir) -> Artifact {
    let mut body = String::new();
    for m in &hir.circuit().modules {
        body.push_str(&emit_module(m));
        body.push('\n');
    }
    let path = format!("{}.v", hir.abi_name);
    Artifact {
        files: vec![EmittedFile {
            path: path.clone(),
            contents: body,
        }],
        filelist: vec![path],
    }
}

fn verilog_ty(ty: &GroundType) -> String {
    match ty {
        GroundType::Clock | GroundType::Reset | GroundType::Bool | GroundType::Analog => {
            String::new()
        }
        GroundType::UInt { width } | GroundType::SInt { width } if *width == 1 => String::new(),
        GroundType::UInt { width } | GroundType::SInt { width } => format!("[{}:0] ", width - 1),
    }
}

fn emit_expr(expr: &AssignExpr) -> String {
    match expr {
        AssignExpr::Ref(n) => n.clone(),
        AssignExpr::Lit(v) => format!("{v}"),
        AssignExpr::Inc(n) => format!("{n} + 1"),
        AssignExpr::Add(a, b) => format!("{a} + {b}"),
        AssignExpr::Eq(a, b) => format!("({a} == {b})"),
        AssignExpr::Mux { sel, t, f } => format!("({sel} ? {t} : {f})"),
        AssignExpr::MemRead { mem, addr } => format!("{mem}[{addr}]"),
    }
}

fn reg_flags<'a>(m: &'a bitloom_hir::Module, name: &str) -> (bool, bool) {
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
    (false, false)
}

fn emit_module(m: &bitloom_hir::Module) -> String {
    let mut out = String::new();
    out.push_str(&format!("module {} (\n", m.name));
    let ports: Vec<String> = m
        .ports
        .iter()
        .map(|p| {
            let dir = match p.direction {
                PortDirection::Input => "input",
                PortDirection::Output => "output",
                PortDirection::InOut => "inout",
            };
            format!("  {} {}{}", dir, verilog_ty(&p.ty), p.name)
        })
        .collect();
    out.push_str(&ports.join(",\n"));
    out.push_str("\n);\n");

    let mut clk = None;
    let mut rst = None;
    for p in &m.ports {
        if matches!(p.ty, GroundType::Clock) {
            clk = Some(p.name.as_str());
        }
        if matches!(p.ty, GroundType::Reset) {
            rst = Some(p.name.as_str());
        }
    }

    for stmt in &m.body {
        match stmt {
            Stmt::WireDecl { name, ty, .. } => {
                out.push_str(&format!("  wire {}{};\n", verilog_ty(ty), name));
            }
            Stmt::RegDecl {
                name,
                ty,
                async_reset,
                has_enable,
                ..
            } => {
                let mut attrs = String::new();
                if *async_reset {
                    attrs.push_str(" /* async_reset */");
                }
                if *has_enable {
                    attrs.push_str(" /* clock_enable */");
                }
                out.push_str(&format!("  reg {}{}{};\n", verilog_ty(ty), name, attrs));
            }
            Stmt::MemDecl {
                name,
                depth,
                width,
                sync_read,
                ..
            } => {
                let kind = if *sync_read {
                    "/* SyncReadMem */"
                } else {
                    "/* Mem */"
                };
                out.push_str(&format!(
                    "  reg [{w}:0] {name} [0:{d}]; {kind}\n",
                    w = width.saturating_sub(1),
                    d = depth.saturating_sub(1),
                ));
            }
            Stmt::Process(p) => match p.kind {
                ProcessKind::Combinational => {
                    for a in &p.assigns {
                        if let AssignTarget::Net(n) = &a.target {
                            out.push_str(&format!("  assign {n} = {};\n", emit_expr(&a.expr)));
                        }
                    }
                }
                ProcessKind::Sequential => {
                    let clk = clk.unwrap_or("clk");
                    let rst = rst.unwrap_or("rst");
                    let mut async_regs = Vec::new();
                    let mut sync_regs = Vec::new();
                    let mut mem_writes = Vec::new();
                    for a in &p.assigns {
                        match &a.target {
                            AssignTarget::RegD(n) => {
                                let (is_async, _) = reg_flags(m, n);
                                if is_async {
                                    async_regs.push(a);
                                } else {
                                    sync_regs.push(a);
                                }
                            }
                            AssignTarget::MemWrite { .. } => mem_writes.push(a),
                            _ => {}
                        }
                    }
                    if !async_regs.is_empty() {
                        out.push_str(&format!(
                            "  always @(posedge {clk} or posedge {rst}) begin\n"
                        ));
                        out.push_str(&format!("    if ({rst}) begin\n"));
                        for a in &async_regs {
                            if let AssignTarget::RegD(n) = &a.target {
                                out.push_str(&format!("      {n} <= 0;\n"));
                            }
                        }
                        out.push_str("    end else begin\n");
                        for a in &async_regs {
                            if let AssignTarget::RegD(n) = &a.target {
                                let (_, has_en) = reg_flags(m, n);
                                let rhs = emit_expr(&a.expr);
                                if has_en {
                                    out.push_str(&format!("      if (en) {n} <= {rhs};\n"));
                                } else {
                                    out.push_str(&format!("      {n} <= {rhs};\n"));
                                }
                            }
                        }
                        out.push_str("    end\n  end\n");
                    }
                    if !sync_regs.is_empty() || !mem_writes.is_empty() {
                        out.push_str(&format!("  always @(posedge {clk}) begin\n"));
                        if !sync_regs.is_empty() {
                            out.push_str(&format!("    if ({rst}) begin\n"));
                            for a in &sync_regs {
                                if let AssignTarget::RegD(n) = &a.target {
                                    out.push_str(&format!("      {n} <= 0;\n"));
                                }
                            }
                            out.push_str("    end else begin\n");
                            for a in &sync_regs {
                                if let AssignTarget::RegD(n) = &a.target {
                                    let (_, has_en) = reg_flags(m, n);
                                    let rhs = emit_expr(&a.expr);
                                    if has_en {
                                        out.push_str(&format!("      if (en) {n} <= {rhs};\n"));
                                    } else {
                                        out.push_str(&format!("      {n} <= {rhs};\n"));
                                    }
                                }
                            }
                            out.push_str("    end\n");
                        }
                        for a in &mem_writes {
                            if let AssignTarget::MemWrite { mem, addr } = &a.target {
                                out.push_str(&format!(
                                    "    {mem}[{addr}] <= {};\n",
                                    emit_expr(&a.expr)
                                ));
                            }
                        }
                        out.push_str("  end\n");
                    }
                }
            },
            Stmt::Instance(inst) => {
                out.push_str(&format!("  {} {} (\n", inst.module, inst.name));
                let conns: Vec<String> = inst
                    .connects
                    .iter()
                    .filter(|c| !c.dangling)
                    .map(|c| format!("    .{}({})", c.child_port, c.parent_net))
                    .collect();
                out.push_str(&conns.join(",\n"));
                out.push_str("\n  );\n");
            }
        }
    }

    out.push_str("endmodule\n");
    out
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;

    fn passthrough_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("t");
        s.begin_module("PassThrough", Span::default());
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
    fn emit_abi_named_v_without_forbidden_syntax() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("CounterPorts", Span::default());
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
        let frozen = s.finish().unwrap();
        let art = emit(&frozen);
        assert_eq!(art.filelist, vec!["CounterPorts.v"]);
        let v = &art.files[0].contents;
        assert!(v.contains("module CounterPorts"));
        assert!(v.contains("always @(posedge clk)"));
        assert!(v.contains("input [7:0] data_in"));
        assert!(v.contains("assign data_out = count;"));
        assert!(v.contains("count <= count + 1;"));
        assert!(!v.contains("always_ff"));
        assert!(!v.contains("automatic"));
        assert!(!v.contains("logic "));
    }

    #[test]
    fn golden_passthrough_v_snapshot() {
        let art = emit(&passthrough_hir());
        let v = &art.files[0].contents;
        let expected = "\
module PassThrough (
  input clk,
  input rst,
  input [7:0] data_in,
  output [7:0] data_out
);
  assign data_out = data_in;
endmodule
";
        assert_eq!(v.trim(), expected.trim());
    }

    #[test]
    fn async_reset_emits_edge_sensitive_form() {
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
        let v = emit(&s.finish().unwrap()).files[0].contents.clone();
        assert!(v.contains("always @(posedge clk or posedge rst)"), "{v}");
        assert!(v.contains("if (rst) begin"), "{v}");
    }

    #[test]
    fn enable_emits_gated_update() {
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
            true,
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        let v = emit(&s.finish().unwrap()).files[0].contents.clone();
        assert!(v.contains("if (en) count <= count + 1;"), "{v}");
    }
}

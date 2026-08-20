//! HTML dump of FrozenHir for debugging (FR38 scaffold).
//!
//! Delivers: module/port HTML + instance hierarchy list.
//! LSP hover/goto: deferred (no language-server binary in this phase).

use bitloom_hir::{FrozenHir, Stmt};

pub fn to_html(hir: &FrozenHir) -> String {
    let mut out = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>RHDL HIR</title></head><body>\n",
    );
    out.push_str(&format!("<h1>{}</h1>\n", hir.abi_name));
    out.push_str("<h2>Modules</h2>\n<ul>\n");
    for m in &hir.circuit().modules {
        out.push_str(&format!("<li><strong>{}</strong><ul>\n", m.name));
        for p in &m.ports {
            out.push_str(&format!(
                "<li>port {} {:?} {:?}</li>\n",
                p.name, p.direction, p.ty
            ));
        }
        out.push_str("</ul></li>\n");
    }
    out.push_str("</ul>\n");
    out.push_str("<h2>Instance hierarchy</h2>\n<ul>\n");
    let mut any = false;
    for m in &hir.circuit().modules {
        for stmt in &m.body {
            if let Stmt::Instance(inst) = stmt {
                any = true;
                out.push_str(&format!(
                    "<li>{parent} → <strong>{name}</strong> : {module}</li>\n",
                    parent = m.name,
                    name = inst.name,
                    module = inst.module
                ));
            }
        }
    }
    if !any {
        out.push_str("<li><em>(no instances)</em></li>\n");
    }
    out.push_str("</ul>\n");
    out.push_str(
        "<p>LSP: deferred — no rhdl language-server binary in this phase; see docs/fr38-viz-lsp.md</p>\n</body></html>\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    use super::*;

    #[test]
    fn html_lists_ports() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 1 }, Span::default());
        s.end_module();
        let hir = s.finish().unwrap();
        let html = to_html(&hir);
        assert!(html.contains("<strong>M</strong>"));
        assert!(html.contains("port y"));
        assert!(html.contains("Instance hierarchy"));
    }

    #[test]
    fn html_lists_instance_hierarchy() {
        let mut s = ElaborateSession::new("Top");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 1 }, Span::default());
        s.end_module();
        s.begin_module("Top", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 1 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let hir = s.finish().unwrap();
        let html = to_html(&hir);
        assert!(html.contains("Top → <strong>u0</strong> : Child"), "{html}");
    }
}

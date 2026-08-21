//! HTML dump of FrozenHir for debugging and product visualize/doc (FR38 / FR49).
//!
//! Delivers: module/port HTML + instance hierarchy (list + Mermaid).
//! LSP hover/goto: deferred (no language-server binary in this phase).

use bitloom_hir::{FrozenHir, Stmt};

/// Render FrozenHir as a self-contained HTML hierarchy document (Bitloom product view).
pub fn to_html(hir: &FrozenHir) -> String {
    let mut out = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Bitloom hierarchy</title>\n\
         <style>body{font-family:system-ui,sans-serif;margin:1.5rem;line-height:1.45}\n\
         h1{font-size:1.4rem} h2{font-size:1.1rem;margin-top:1.5rem}\n\
         .brand{color:#0b3d5c;font-weight:700;letter-spacing:.02em}\n\
         pre.mermaid{background:#f6f8fa;padding:1rem;overflow:auto;border-radius:4px}\n\
         </style></head><body>\n",
    );
    out.push_str("<p class=\"brand\">Bitloom</p>\n");
    out.push_str(&format!("<h1>Hierarchy — {}</h1>\n", escape_html(&hir.abi_name)));
    out.push_str("<h2>Modules and ports</h2>\n<ul>\n");
    for m in &hir.circuit().modules {
        out.push_str(&format!("<li><strong>{}</strong><ul>\n", escape_html(&m.name)));
        for p in &m.ports {
            out.push_str(&format!(
                "<li>port {} {:?} {:?}</li>\n",
                escape_html(&p.name),
                p.direction,
                p.ty
            ));
        }
        out.push_str("</ul></li>\n");
    }
    out.push_str("</ul>\n");

    out.push_str("<h2>Instance hierarchy</h2>\n<ul>\n");
    let edges = instance_edges(hir);
    if edges.is_empty() {
        out.push_str("<li><em>(no instances)</em></li>\n");
    } else {
        for (parent, name, module) in &edges {
            out.push_str(&format!(
                "<li>{parent} → <strong>{name}</strong> : {module}</li>\n",
                parent = escape_html(parent),
                name = escape_html(name),
                module = escape_html(module)
            ));
        }
    }
    out.push_str("</ul>\n");

    out.push_str("<h2>Hierarchy diagram (Mermaid)</h2>\n");
    out.push_str("<pre class=\"mermaid\">\n");
    out.push_str(&mermaid_hierarchy(hir));
    out.push_str("</pre>\n");
    out.push_str(
        "<p>LSP: deferred — no Bitloom language-server binary in this phase; see docs/fr38-viz-lsp.md</p>\n\
         <p>Unrelated to <code>samitbasu/rhdl</code>.</p>\n</body></html>\n",
    );
    out
}

/// Collect `(parent_module, instance_name, child_module)` edges.
pub fn instance_edges(hir: &FrozenHir) -> Vec<(String, String, String)> {
    let mut edges = Vec::new();
    for m in &hir.circuit().modules {
        for stmt in &m.body {
            if let Stmt::Instance(inst) = stmt {
                edges.push((m.name.clone(), inst.name.clone(), inst.module.clone()));
            }
        }
    }
    edges
}

fn mermaid_hierarchy(hir: &FrozenHir) -> String {
    let mut s = String::from("flowchart TD\n");
    let top = &hir.abi_name;
    s.push_str(&format!("  {id}[\"{label}\"]\n", id = mmd_id(top), label = top));
    for (parent, name, module) in instance_edges(hir) {
        let child_label = format!("{name}:{module}");
        let child_id = mmd_id(&format!("{parent}__{name}"));
        s.push_str(&format!(
            "  {cid}[\"{clabel}\"]\n  {pid} --> {cid}\n",
            cid = child_id,
            clabel = child_label,
            pid = mmd_id(&parent),
        ));
    }
    s
}

fn mmd_id(raw: &str) -> String {
    let mut out = String::from("n_");
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
        assert!(html.contains("Bitloom"));
        assert!(html.contains("<strong>M</strong>"));
        assert!(html.contains("port y"));
        assert!(html.contains("Instance hierarchy"));
        assert!(html.contains("flowchart TD"));
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
        assert!(html.contains("u0:Child"), "{html}");
    }
}

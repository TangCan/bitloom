//! HTML dump of FrozenHir for debugging and product visualize/doc (FR38 / FR49).
//!
//! Delivers: module/port HTML + instance hierarchy (list + Mermaid).
//! Also: browsable timing HTML from tick samples / VCD (FR49 wave path).
//! LSP: product server is `bitloom-lsp` (FR99 / Epic 44); HTML hierarchy ≠ LSP.

use std::collections::BTreeMap;

use bitloom_hir::{FrozenHir, Stmt};

/// One time-step of signal values for the product timing view.
#[derive(Clone, Debug, Default)]
pub struct WaveSample {
    pub time: u64,
    pub values: BTreeMap<String, u64>,
}

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
    out.push_str(&format!(
        "<h1>Hierarchy — {}</h1>\n",
        escape_html(&hir.abi_name)
    ));
    out.push_str("<h2>Modules and ports</h2>\n<ul>\n");
    for m in &hir.circuit().modules {
        out.push_str(&format!(
            "<li><strong>{}</strong><ul>\n",
            escape_html(&m.name)
        ));
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
        "<p>LSP: see <code>bitloom-lsp</code> / docs/fr99-bitloom-lsp.md (FR99); HTML hierarchy ≠ LSP — docs/fr38-viz-lsp.md</p>\n\
         <p>Unrelated to <code>samitbasu/rhdl</code>.</p>\n</body></html>\n",
    );
    out
}

/// Interactive waveform HTML (FR104): browse / zoom·pan / signal search.
///
/// Self-contained product viewer — **not** static `timing.html` alone and **not**
/// GTKWave-only. Sibling of FR49 `timing_html`; signal names match VCD/`tick` dump.
pub fn interactive_wave_html(title: &str, samples: &[WaveSample]) -> String {
    let mut signal_names: Vec<String> = samples
        .iter()
        .flat_map(|s| s.values.keys().cloned())
        .collect();
    signal_names.sort();
    signal_names.dedup();

    let mut times: Vec<u64> = samples.iter().map(|s| s.time).collect();
    times.sort_unstable();
    times.dedup();

    // Embed samples as JSON: { "times": [...], "signals": { name: [v,...] } }
    let mut json = String::from("{\"times\":[");
    for (i, t) in times.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&t.to_string());
    }
    json.push_str("],\"signals\":{");
    for (si, name) in signal_names.iter().enumerate() {
        if si > 0 {
            json.push(',');
        }
        json.push('"');
        json.push_str(&escape_js_string(name));
        json.push_str("\":[");
        for (ti, t) in times.iter().enumerate() {
            if ti > 0 {
                json.push(',');
            }
            let v = samples
                .iter()
                .find(|s| s.time == *t)
                .and_then(|s| s.values.get(name))
                .copied()
                .unwrap_or(0);
            json.push_str(&v.to_string());
        }
        json.push(']');
    }
    json.push_str("}}");

    let mut out = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\">\
         <title>Bitloom interactive wave</title>\n\
         <style>\n\
         body{font-family:system-ui,sans-serif;margin:1rem;line-height:1.4;background:#f7fafc;color:#122}\n\
         .brand{color:#0b3d5c;font-weight:700;letter-spacing:.02em}\n\
         .toolbar{display:flex;flex-wrap:wrap;gap:.5rem;align-items:center;margin:0.75rem 0}\n\
         .toolbar input[type=search]{min-width:12rem;padding:.35rem .5rem}\n\
         .toolbar button{padding:.35rem .75rem;cursor:pointer}\n\
         #wave-canvas{display:block;width:100%;max-width:100%;background:#fff;\
         border:1px solid #c5d0da;border-radius:4px;cursor:crosshair;touch-action:none}\n\
         .hint{font-size:.85rem;color:#456}\n\
         </style></head><body>\n",
    );
    out.push_str("<p class=\"brand\">Bitloom</p>\n");
    out.push_str(&format!(
        "<h1>Interactive wave — {}</h1>\n",
        escape_html(title)
    ));
    out.push_str(
        "<p class=\"hint\">FR104 interactive viewer (browse / zoom·pan / search). \
         Static <code>timing.html</code> alone is <strong>not</strong> this path; \
         GTKWave/Surfer remain optional for sibling <code>wave.vcd</code> (AD-5/24), \
         not the sole completion path.</p>\n",
    );

    if samples.is_empty() || signal_names.is_empty() {
        out.push_str(
            "<div id=\"root\" data-bitloom-interactive-wave=\"empty\"><p><em>(no samples)</em></p></div>\n\
             </body></html>\n",
        );
        return out;
    }

    out.push_str(
        "<div id=\"root\" class=\"bitloom-interactive-wave\" data-bitloom-interactive-wave=\"1\">\n\
         <div class=\"toolbar\">\n\
         <label for=\"signal-search\">Search signals </label>\n\
         <input type=\"search\" id=\"signal-search\" placeholder=\"filter by name\" \
          aria-label=\"signal search filter\" />\n\
         <button type=\"button\" id=\"zoom-in\" title=\"zoom in\">Zoom +</button>\n\
         <button type=\"button\" id=\"zoom-out\" title=\"zoom out\">Zoom −</button>\n\
         <button type=\"button\" id=\"pan-left\" title=\"pan left\">Pan ←</button>\n\
         <button type=\"button\" id=\"pan-right\" title=\"pan right\">Pan →</button>\n\
         <button type=\"button\" id=\"zoom-fit\" title=\"fit all\">Fit</button>\n\
         <span id=\"viewport-label\" class=\"hint\"></span>\n\
         </div>\n\
         <canvas id=\"wave-canvas\" class=\"wave-canvas timeline\" width=\"960\" height=\"360\" \
          role=\"img\" aria-label=\"waveform timeline canvas\"></canvas>\n\
         </div>\n",
    );
    out.push_str("<script id=\"wave-data\" type=\"application/json\">");
    out.push_str(&json);
    out.push_str("</script>\n<script>\n");
    out.push_str(INTERACTIVE_WAVE_JS);
    out.push_str("\n</script>\n");
    out.push_str("<p>Unrelated to <code>samitbasu/rhdl</code>.</p>\n</body></html>\n");
    out
}

const INTERACTIVE_WAVE_JS: &str = include_str!("interactive_wave.js");

fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Browsable timing / wave HTML (product path — not GTKWave-only).
pub fn timing_html(title: &str, samples: &[WaveSample]) -> String {
    let mut signal_names: Vec<String> = samples
        .iter()
        .flat_map(|s| s.values.keys().cloned())
        .collect();
    signal_names.sort();
    signal_names.dedup();

    let mut out = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Bitloom timing</title>\n\
         <style>body{font-family:system-ui,sans-serif;margin:1.5rem;line-height:1.4}\n\
         .brand{color:#0b3d5c;font-weight:700}\n\
         table{border-collapse:collapse;font-size:0.9rem}\n\
         th,td{border:1px solid #ccc;padding:0.25rem 0.5rem;text-align:left}\n\
         th{background:#f0f4f8}\n\
         .lane{font-family:ui-monospace,monospace;white-space:pre}\n\
         </style></head><body>\n",
    );
    out.push_str("<p class=\"brand\">Bitloom</p>\n");
    out.push_str(&format!("<h1>Timing — {}</h1>\n", escape_html(title)));
    out.push_str(
        "<p>Product timing view from tick/VCD samples. GTKWave/Surfer remain optional \
         viewers for the sibling <code>.vcd</code>; they are <strong>not</strong> the sole path.</p>\n",
    );

    if samples.is_empty() || signal_names.is_empty() {
        out.push_str("<p><em>(no samples)</em></p>\n</body></html>\n");
        return out;
    }

    out.push_str("<h2>Value table</h2>\n<table>\n<tr><th>time</th>");
    for n in &signal_names {
        out.push_str(&format!("<th>{}</th>", escape_html(n)));
    }
    out.push_str("</tr>\n");
    for s in samples {
        out.push_str(&format!("<tr><td>{}</td>", s.time));
        for n in &signal_names {
            let v = s.values.get(n).copied().unwrap_or(0);
            out.push_str(&format!("<td>{v}</td>"));
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</table>\n");

    out.push_str("<h2>ASCII timing lanes</h2>\n<pre class=\"lane\">\n");
    for n in &signal_names {
        let mut lane = format!("{:>12} ", n);
        let mut prev: Option<u64> = None;
        for s in samples {
            let v = s.values.get(n).copied().unwrap_or(0);
            if prev == Some(v) {
                lane.push('-');
            } else {
                lane.push_str(&format!("[{v}]"));
                prev = Some(v);
            }
        }
        out.push_str(&lane);
        out.push('\n');
    }
    out.push_str("</pre>\n");
    out.push_str("<p>Unrelated to <code>samitbasu/rhdl</code>.</p>\n</body></html>\n");
    out
}

/// Parse a minimal VCD (as emitted by `bitloom-sim`) into wave samples.
pub fn samples_from_vcd(vcd: &str) -> Result<Vec<WaveSample>, String> {
    let mut samples = Vec::new();
    let mut current = WaveSample::default();
    let mut started = false;
    for line in vcd.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('#') {
            if started && !current.values.is_empty() {
                samples.push(current.clone());
            }
            let t: u64 = rest
                .split_whitespace()
                .next()
                .ok_or_else(|| "empty VCD time".to_string())?
                .parse()
                .map_err(|e| format!("bad VCD time: {e}"))?;
            current = WaveSample {
                time: t,
                values: current.values.clone(),
            };
            started = true;
        } else if let Some(rest) = line.strip_prefix('b') {
            let mut parts = rest.split_whitespace();
            let bits = parts
                .next()
                .ok_or_else(|| "VCD b-line missing bits".to_string())?;
            let name = parts
                .next()
                .ok_or_else(|| "VCD b-line missing name".to_string())?;
            let val = u64::from_str_radix(bits, 2).unwrap_or(0);
            current.values.insert(name.to_string(), val);
        }
    }
    if started {
        samples.push(current);
    }
    if samples.is_empty() {
        return Err("VCD contained no time samples".into());
    }
    Ok(samples)
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
    s.push_str(&format!(
        "  {id}[\"{label}\"]\n",
        id = mmd_id(top),
        label = top
    ));
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

    #[test]
    fn timing_html_from_samples_and_vcd() {
        let samples = vec![
            WaveSample {
                time: 0,
                values: BTreeMap::from([("x".into(), 1), ("y".into(), 1)]),
            },
            WaveSample {
                time: 1,
                values: BTreeMap::from([("x".into(), 2), ("y".into(), 2)]),
            },
        ];
        let html = timing_html("demo", &samples);
        assert!(html.contains("Bitloom") && html.contains("Timing"));
        assert!(html.contains("Value table") && html.contains("[1]"));

        let vcd = "\
$timescale 1ns $end
$scope module demo $end
$var wire 8 x x $end
$var wire 8 y y $end
$upscope $end
$enddefinitions $end
#0
b1 x
b1 y
#1
b10 x
b10 y
";
        let parsed = samples_from_vcd(vcd).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].values.get("x"), Some(&2));
        let from_vcd = timing_html("demo", &parsed);
        assert!(from_vcd.contains("Value table"));
    }

    #[test]
    fn interactive_wave_html_has_i1_i3_markers() {
        let samples = vec![
            WaveSample {
                time: 0,
                values: BTreeMap::from([("clk".into(), 0), ("x".into(), 1)]),
            },
            WaveSample {
                time: 1,
                values: BTreeMap::from([("clk".into(), 1), ("x".into(), 2)]),
            },
        ];
        let html = interactive_wave_html("demo", &samples);
        assert!(html.contains("Bitloom"));
        assert!(html.contains("data-bitloom-interactive-wave"));
        assert!(html.contains("wave-canvas") || html.contains("<canvas"));
        assert!(html.contains("zoom") || html.contains("Zoom"));
        assert!(html.contains("signal-search") || html.contains("search"));
    }
}

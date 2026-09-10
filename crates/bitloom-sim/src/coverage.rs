//! Stable, parseable simulation coverage (FR34 toggle + FR105 branch + FR109 FSM).

use std::collections::{BTreeMap, BTreeSet};

/// Toggle + Mux branch + FSM state-visit coverage over named signals / labels.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    prev: BTreeMap<String, u64>,
    toggled: BTreeSet<String>,
    seen: BTreeSet<String>,
    /// Decision points `mux:<sel>:t` / `mux:<sel>:f` that were taken at least once.
    branch_hits: BTreeSet<String>,
    /// Decision points registered (from Mux eval) but never taken.
    branch_seen: BTreeSet<String>,
    /// FSM state ids `fsm:<name>:<State>` visited at least once (FR109 / C3).
    state_hits: BTreeSet<String>,
    /// FSM states registered for visit tracking.
    state_seen: BTreeSet<String>,
}

impl Coverage {
    pub fn sample(&mut self, name: impl Into<String>, value: u64) {
        let name = name.into();
        self.seen.insert(name.clone());
        if let Some(old) = self.prev.get(&name)
            && *old != value
        {
            self.toggled.insert(name.clone());
        }
        self.prev.insert(name, value);
    }

    /// Register both arms of a Mux and record which arm was taken this eval.
    pub fn sample_mux_branch(&mut self, sel: &str, took_true: bool) {
        let t = format!("mux:{sel}:t");
        let f = format!("mux:{sel}:f");
        self.branch_seen.insert(t.clone());
        self.branch_seen.insert(f.clone());
        if took_true {
            self.branch_hits.insert(t);
        } else {
            self.branch_hits.insert(f);
        }
    }

    /// Register an explicit FSM state enum (or label set) for state-visit (FR109 / C3).
    pub fn register_fsm_states<I, S>(&mut self, fsm: &str, states: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for st in states {
            let id = format!("fsm:{fsm}:{}", st.into());
            self.state_seen.insert(id);
        }
    }

    /// Record that `state` of `fsm` was visited (must be registered first for miss tracking).
    pub fn sample_state_visit(&mut self, fsm: &str, state: &str) {
        let id = format!("fsm:{fsm}:{state}");
        self.state_seen.insert(id.clone());
        self.state_hits.insert(id);
    }

    pub fn hits(&self) -> impl Iterator<Item = &str> {
        self.toggled.iter().map(|s| s.as_str())
    }

    pub fn misses(&self) -> impl Iterator<Item = &str> {
        self.seen.difference(&self.toggled).map(|s| s.as_str())
    }

    pub fn branch_hits(&self) -> impl Iterator<Item = &str> {
        self.branch_hits.iter().map(|s| s.as_str())
    }

    pub fn branch_misses(&self) -> impl Iterator<Item = &str> {
        self.branch_seen
            .difference(&self.branch_hits)
            .map(|s| s.as_str())
    }

    pub fn state_hits(&self) -> impl Iterator<Item = &str> {
        self.state_hits.iter().map(|s| s.as_str())
    }

    pub fn state_misses(&self) -> impl Iterator<Item = &str> {
        self.state_seen
            .difference(&self.state_hits)
            .map(|s| s.as_str())
    }

    /// Stable line-oriented report.
    ///
    /// - No FSM registered → `# bitloom-sim coverage v2` (FR105 dialect).
    /// - FSM registered → `# bitloom-sim coverage v3` + `# FR109 C3 FSM/state-visit`
    ///   with `state_hit` / `state_miss` lines (toggle + branch retained).
    pub fn report(&self) -> String {
        let has_fsm = !self.state_seen.is_empty();
        let mut out = if has_fsm {
            String::from("# bitloom-sim coverage v3\n# FR109 C3 FSM/state-visit\n")
        } else {
            String::from("# bitloom-sim coverage v2\n")
        };
        for n in self.hits() {
            out.push_str(&format!("hit {n}\n"));
        }
        for n in self.misses() {
            out.push_str(&format!("miss {n}\n"));
        }
        for n in self.branch_hits() {
            out.push_str(&format!("branch_hit {n}\n"));
        }
        for n in self.branch_misses() {
            out.push_str(&format!("branch_miss {n}\n"));
        }
        for n in self.state_hits() {
            out.push_str(&format!("state_hit {n}\n"));
        }
        for n in self.state_misses() {
            out.push_str(&format!("state_miss {n}\n"));
        }
        out
    }

    /// True when no toggle/branch/state points were ever registered.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty() && self.branch_seen.is_empty() && self.state_seen.is_empty()
    }

    /// Emit LCOV (`coverage.lcov` dialect) for FR114 — one DA line per coverage point.
    ///
    /// Hits use `DA:line,1`; misses use `DA:line,0`. Synthetic `SF` path documents the
    /// Bitloom coverage namespace (not a Rust source map — Tywaves typed IDE is deferred).
    pub fn to_lcov(&self, source_file: &str) -> String {
        let mut points: Vec<(String, u64)> = Vec::new();
        for n in self.hits() {
            points.push((format!("toggle:{n}"), 1));
        }
        for n in self.misses() {
            points.push((format!("toggle:{n}"), 0));
        }
        for n in self.branch_hits() {
            points.push((n.to_string(), 1));
        }
        for n in self.branch_misses() {
            points.push((n.to_string(), 0));
        }
        for n in self.state_hits() {
            points.push((n.to_string(), 1));
        }
        for n in self.state_misses() {
            points.push((n.to_string(), 0));
        }
        points.sort_by(|a, b| a.0.cmp(&b.0));

        let mut out = String::from("TN:bitloom-sim\n");
        out.push_str(&format!("SF:{source_file}\n"));
        let mut lh = 0usize;
        let lf = points.len();
        for (i, (name, hits)) in points.iter().enumerate() {
            let line = i + 1;
            out.push_str(&format!("# {name}\n"));
            out.push_str(&format!("DA:{line},{hits}\n"));
            if *hits > 0 {
                lh += 1;
            }
        }
        out.push_str(&format!("LH:{lh}\n"));
        out.push_str(&format!("LF:{lf}\n"));
        out.push_str("end_of_record\n");
        out
    }

    /// In-tree Bitloom coverage GUI HTML (FR114) — browse hit/miss with search.
    pub fn coverage_gui_html(&self, title: &str) -> String {
        let mut rows = String::new();
        let mut push_row = |kind: &str, name: &str, hit: bool| {
            let status = if hit { "hit" } else { "miss" };
            rows.push_str(&format!(
                "<tr data-kind=\"{kind}\" data-name=\"{name}\" data-status=\"{status}\">\
                 <td>{kind}</td><td>{name}</td><td class=\"{status}\">{status}</td></tr>\n"
            ));
        };
        for n in self.hits() {
            push_row("toggle", n, true);
        }
        for n in self.misses() {
            push_row("toggle", n, false);
        }
        for n in self.branch_hits() {
            push_row("branch", n, true);
        }
        for n in self.branch_misses() {
            push_row("branch", n, false);
        }
        for n in self.state_hits() {
            push_row("state", n, true);
        }
        for n in self.state_misses() {
            push_row("state", n, false);
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<title>Bitloom coverage — {title}</title>
<style>
body {{ font-family: ui-sans-serif, system-ui, sans-serif; margin: 1.5rem; background: #f7f4ef; color: #1a1a1a; }}
h1 {{ font-size: 1.4rem; margin: 0 0 0.25rem; }}
.brand {{ letter-spacing: 0.04em; text-transform: uppercase; font-size: 0.75rem; color: #5c5346; }}
.hit {{ color: #0b6b3a; font-weight: 600; }}
.miss {{ color: #8a1f1f; font-weight: 600; }}
input {{ margin: 0.75rem 0; padding: 0.4rem 0.6rem; width: min(28rem, 100%); }}
table {{ border-collapse: collapse; width: min(48rem, 100%); background: #fff; }}
th, td {{ border: 1px solid #d9d2c5; padding: 0.35rem 0.55rem; text-align: left; }}
th {{ background: #ebe4d8; }}
</style>
</head>
<body data-bitloom-coverage-gui="fr114">
<p class="brand">Bitloom</p>
<h1>Coverage GUI — {title}</h1>
<p>FR114 LCOV companion view (≠ FR104 interactive wave; ≠ Tywaves). Filter by name:</p>
<input id="cov-search" type="search" placeholder="search coverage points…" />
<table>
<thead><tr><th>Kind</th><th>Point</th><th>Status</th></tr></thead>
<tbody id="cov-body">
{rows}</tbody>
</table>
<script>
const q = document.getElementById('cov-search');
const body = document.getElementById('cov-body');
q.addEventListener('input', () => {{
  const needle = q.value.toLowerCase();
  for (const tr of body.querySelectorAll('tr')) {{
    const name = (tr.getAttribute('data-name') || '').toLowerCase();
    tr.style.display = !needle || name.includes(needle) ? '' : 'none';
  }}
}});
</script>
</body>
</html>
"#
        )
    }
}

/// Write `coverage.lcov` + `coverage.html` for FR114. Fails if coverage is empty.
pub fn write_coverage_artifacts(
    cov: &Coverage,
    out_dir: &std::path::Path,
) -> std::io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    use std::fs;
    if cov.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bitloom-sim.coverage-empty: no coverage points — cannot claim FR114 (LCOV+GUI)",
        ));
    }
    fs::create_dir_all(out_dir)?;
    let lcov_path = out_dir.join("coverage.lcov");
    let html_path = out_dir.join("coverage.html");
    fs::write(&lcov_path, cov.to_lcov("bitloom://coverage/points"))?;
    fs::write(&html_path, cov.coverage_gui_html("sim"))?;
    Ok((lcov_path, html_path))
}

/// Parse FR34-compatible toggle `hit` / `miss` lines (ignores `branch_*` / `state_*`).
pub fn parse_report(text: &str) -> (Vec<String>, Vec<String>) {
    let mut hits = Vec::new();
    let mut misses = Vec::new();
    for line in text.lines() {
        if let Some(n) = line.strip_prefix("hit ") {
            hits.push(n.to_string());
        } else if let Some(n) = line.strip_prefix("miss ") {
            misses.push(n.to_string());
        }
    }
    (hits, misses)
}

/// Parse FR105 `branch_hit` / `branch_miss` lines.
pub fn parse_branch_report(text: &str) -> (Vec<String>, Vec<String>) {
    let mut hits = Vec::new();
    let mut misses = Vec::new();
    for line in text.lines() {
        if let Some(n) = line.strip_prefix("branch_hit ") {
            hits.push(n.to_string());
        } else if let Some(n) = line.strip_prefix("branch_miss ") {
            misses.push(n.to_string());
        }
    }
    (hits, misses)
}

/// Parse FR109 `state_hit` / `state_miss` lines.
pub fn parse_state_report(text: &str) -> (Vec<String>, Vec<String>) {
    let mut hits = Vec::new();
    let mut misses = Vec::new();
    for line in text.lines() {
        if let Some(n) = line.strip_prefix("state_hit ") {
            hits.push(n.to_string());
        } else if let Some(n) = line.strip_prefix("state_miss ") {
            misses.push(n.to_string());
        }
    }
    (hits, misses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_visit_hit_and_miss() {
        let mut c = Coverage::default();
        c.register_fsm_states("demo", ["Idle", "Busy", "Done"]);
        c.sample_state_visit("demo", "Idle");
        c.sample_state_visit("demo", "Busy");
        let r = c.report();
        assert!(r.contains("# bitloom-sim coverage v3"));
        assert!(r.contains("# FR109 C3 FSM/state-visit"));
        assert!(r.contains("state_hit fsm:demo:Idle"));
        assert!(r.contains("state_hit fsm:demo:Busy"));
        assert!(r.contains("state_miss fsm:demo:Done"));
        let (h, m) = parse_state_report(&r);
        assert!(h.contains(&"fsm:demo:Idle".into()));
        assert!(m.contains(&"fsm:demo:Done".into()));
    }

    #[test]
    fn lcov_and_gui_nonempty() {
        let mut c = Coverage::default();
        c.sample("y", 0);
        c.sample("y", 1);
        c.sample_mux_branch("sel", false);
        let lcov = c.to_lcov("bitloom://coverage/points");
        assert!(lcov.contains("end_of_record") && lcov.contains("DA:"));
        let html = c.coverage_gui_html("t");
        assert!(html.contains("data-bitloom-coverage-gui"));
    }
}

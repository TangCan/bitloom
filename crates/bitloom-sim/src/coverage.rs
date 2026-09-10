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
}

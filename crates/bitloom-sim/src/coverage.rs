//! Stable, parseable simulation coverage (FR34 toggle + FR105 branch extension).

use std::collections::{BTreeMap, BTreeSet};

/// Toggle + Mux branch coverage over named ports/regs sampled each tick.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    prev: BTreeMap<String, u64>,
    toggled: BTreeSet<String>,
    seen: BTreeSet<String>,
    /// Decision points `mux:<sel>:t` / `mux:<sel>:f` that were taken at least once.
    branch_hits: BTreeSet<String>,
    /// Decision points registered (from Mux eval) but never taken.
    branch_seen: BTreeSet<String>,
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

    /// Stable line-oriented report (`# bitloom-sim coverage v2`).
    ///
    /// Retains FR34 `hit` / `miss` toggle lines; adds FR105 `branch_hit` /
    /// `branch_miss` for Mux decision points.
    pub fn report(&self) -> String {
        let mut out = String::from("# bitloom-sim coverage v2\n");
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
        out
    }
}

/// Parse FR34-compatible toggle `hit` / `miss` lines (ignores `branch_*`).
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

//! Stable, parseable simulation coverage (FR34): at least one hit and one miss.

use std::collections::{BTreeMap, BTreeSet};

/// Toggle coverage over named ports/regs sampled each tick.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    prev: BTreeMap<String, u64>,
    toggled: BTreeSet<String>,
    seen: BTreeSet<String>,
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

    pub fn hits(&self) -> impl Iterator<Item = &str> {
        self.toggled.iter().map(|s| s.as_str())
    }

    pub fn misses(&self) -> impl Iterator<Item = &str> {
        self.seen.difference(&self.toggled).map(|s| s.as_str())
    }

    /// Stable line-oriented report (`# bitloom-sim coverage v1`).
    pub fn report(&self) -> String {
        let mut out = String::from("# bitloom-sim coverage v1\n");
        for n in self.hits() {
            out.push_str(&format!("hit {n}\n"));
        }
        for n in self.misses() {
            out.push_str(&format!("miss {n}\n"));
        }
        out
    }
}

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

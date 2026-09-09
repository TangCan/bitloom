//! FR100 formal-equivalence product path (NFR14 F1–F5).
//!
//! **F1 branch (i):** in-tree bounded prover API — beyond PortValues random
//! scoreboard alone. Exhaustive enumeration over a doc-pinned boolean input
//! alphabet × depth proves FL ≡ cycle-accurate `tick` within the bound.
//!
//! The automatic **random/compare** path is a companion (F3) and **≠** FR100
//! close by itself (F5). FR92 `SharedStimulusScoreboard` remains supporting /
//! not sufficient for FR100.
//!
//! Observation surface: `PortValues` only (AD-17). Design crates stay on
//! `bitloom-prelude`; this module lives in the toolchain (`bitloom-sim`).

use bitloom_hir::{FrozenHir, PortValues};

use crate::{
    AbstractionView, EquivStatus, check_functional_equiv, check_functional_equiv_generated,
    check_generated_bridge_with,
};

/// Product entry for FR100 automatic compare + bounded formal equivalence.
///
/// Not a rename of [`crate::SharedStimulusScoreboard`]: random sampling and
/// bounded exhaustive exploration are distinct surfaces; only the latter is the
/// F1-(i) formal product entry.
#[derive(Debug, Clone)]
pub struct FormalEquivProduct {
    /// Deterministic seed for [`Self::check_random_compare`].
    pub seed: u64,
    /// Number of stimulus frames for the random/compare companion path.
    pub random_cycles: usize,
    /// Boolean input ports forming the exhaustive alphabet (typically `rst`).
    boolean_ports: Vec<String>,
    /// Sequence length for bounded exhaustive exploration.
    exhaustive_depth: usize,
}

impl FormalEquivProduct {
    /// Build with seed + random cycle count; default alphabet empty until
    /// [`Self::with_boolean_ports`].
    pub fn new(seed: u64, random_cycles: usize) -> Self {
        Self {
            seed,
            random_cycles,
            boolean_ports: Vec::new(),
            exhaustive_depth: 0,
        }
    }

    /// Pin boolean ports that form the exhaustive / random alphabet.
    pub fn with_boolean_ports(mut self, ports: &[&str]) -> Self {
        self.boolean_ports = ports.iter().map(|p| (*p).to_string()).collect();
        self
    }

    /// Set bounded-exhaustive sequence depth (F1-(i) / F4 fixture scale).
    pub fn with_exhaustive_depth(mut self, depth: usize) -> Self {
        self.exhaustive_depth = depth;
        self
    }

    /// Alphabet of single-frame `PortValues` (cartesian product of 0/1 on each
    /// boolean port). Empty ports → one default empty frame.
    pub fn alphabet(&self) -> Vec<PortValues> {
        if self.boolean_ports.is_empty() {
            return vec![PortValues::default()];
        }
        let n = self.boolean_ports.len();
        assert!(
            n <= 12,
            "FormalEquivProduct alphabet supports at most 12 boolean ports (got {n}); F4 MVP scale"
        );
        let combinations = 1usize << n;
        let mut out = Vec::with_capacity(combinations);
        for mask in 0..combinations {
            let mut pv = PortValues::default();
            for (i, name) in self.boolean_ports.iter().enumerate() {
                let bit = ((mask >> i) & 1) as u64;
                pv.set(name, bit);
            }
            out.push(pv);
        }
        out
    }

    /// Deterministic random stimuli from the alphabet (LCG; companion F3 path).
    pub fn random_stimuli(&self) -> Vec<PortValues> {
        let alphabet = self.alphabet();
        let mut state = self.seed;
        let mut out = Vec::with_capacity(self.random_cycles.max(1));
        let n = self.random_cycles.max(1);
        for _ in 0..n {
            // Numerical Recipes LCG
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            let idx = (state as usize) % alphabet.len();
            out.push(alphabet[idx].clone());
        }
        out
    }

    /// F3 companion: automatic random PortValues compare (generated FL vs tick).
    ///
    /// Reproducible for a fixed seed. **Not** sufficient alone to close FR100 (F5).
    pub fn check_random_compare(&self, hir: FrozenHir) -> EquivStatus {
        check_functional_equiv_generated(hir, self.random_stimuli())
    }

    /// Same as [`Self::check_random_compare`] with an arbitrary functional view
    /// (deliberate-mismatch ATDD).
    pub fn check_random_compare_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        check_generated_bridge_with(hir, abs, self.random_stimuli())
    }

    /// F1-(i) product entry: exhaustive FL≡tick over all alphabet^depth sequences.
    ///
    /// Beyond random scoreboard sampling: every sequence of length
    /// `exhaustive_depth` drawn from the pinned alphabet is checked. First
    /// failing sequence returns `Fail` with readable `PortMismatch`s.
    pub fn check_bounded_exhaustive(&self, hir: FrozenHir) -> EquivStatus {
        let sequences = self.exhaustive_sequences();
        let mut total_cycles = 0usize;
        for seq in sequences {
            match check_functional_equiv_generated(hir.clone(), seq) {
                EquivStatus::Pass { cycles } => total_cycles = total_cycles.saturating_add(cycles),
                fail @ EquivStatus::Fail { .. } => return fail,
            }
        }
        EquivStatus::Pass {
            cycles: total_cycles,
        }
    }

    /// Bounded exhaustive with an arbitrary functional view (mismatch ATDD).
    pub fn check_bounded_exhaustive_with<A: AbstractionView>(
        &self,
        hir: FrozenHir,
        abs: &mut A,
    ) -> EquivStatus {
        let sequences = self.exhaustive_sequences();
        let mut total_cycles = 0usize;
        for seq in sequences {
            match check_functional_equiv(hir.clone(), abs, seq) {
                EquivStatus::Pass { cycles } => total_cycles = total_cycles.saturating_add(cycles),
                fail @ EquivStatus::Fail { .. } => return fail,
            }
        }
        EquivStatus::Pass {
            cycles: total_cycles,
        }
    }

    /// All sequences of length `exhaustive_depth` over [`Self::alphabet`].
    pub fn exhaustive_sequences(&self) -> Vec<Vec<PortValues>> {
        let alphabet = self.alphabet();
        let depth = self.exhaustive_depth;
        if depth == 0 {
            return vec![Vec::new()];
        }
        let mut sequences: Vec<Vec<PortValues>> = vec![Vec::new()];
        for _ in 0..depth {
            let mut next = Vec::with_capacity(sequences.len() * alphabet.len());
            for prefix in &sequences {
                for frame in &alphabet {
                    let mut seq = prefix.clone();
                    seq.push(frame.clone());
                    next.push(seq);
                }
            }
            sequences = next;
        }
        sequences
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_two_ports_has_four_frames() {
        let p = FormalEquivProduct::new(0, 0).with_boolean_ports(&["rst", "en"]);
        assert_eq!(p.alphabet().len(), 4);
    }

    #[test]
    fn exhaustive_depth_two_bool_has_four_sequences() {
        let p = FormalEquivProduct::new(0, 0)
            .with_boolean_ports(&["rst"])
            .with_exhaustive_depth(2);
        assert_eq!(p.exhaustive_sequences().len(), 4);
    }

    #[test]
    fn random_stimuli_reproducible() {
        let p = FormalEquivProduct::new(42, 5).with_boolean_ports(&["rst"]);
        assert_eq!(p.random_stimuli(), p.random_stimuli());
    }
}

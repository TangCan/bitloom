//! FR92 shared-stimulus scoreboard skeleton.
//!
//! One stimulus vector drives both the functional-sim path and cycle-accurate
//! `tick` (via [`crate::check_functional_equiv_generated`]). This is **not**
//! sufficient alone to close FR100 — see [`crate::FormalEquivProduct`] and
//! `docs/fr100-formal-equiv.md`. SystemC TLM-2.0 remains Epic 46 / FR101 (AD-5).
//!
//! Bridge adapter template for transaction→cycle handshakes remains
//! `bitloom_prelude::{StartWaitComplete, start_wait_complete}` (FR78).

use std::collections::{BTreeMap, BTreeSet};

use bitloom_hir::{FrozenHir, PortValues};

use crate::{EquivStatus, check_functional_equiv_generated, parse_state_report, reset_then_run};

/// Structured FSM coverage reported by one named simulation instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceCoverage {
    /// Original stable coverage report emitted by the instance.
    pub report: String,
    /// Sorted FSM-state coverage points visited by this instance.
    pub state_hits: Vec<String>,
    /// Sorted FSM-state coverage points not visited by this instance.
    pub state_misses: Vec<String>,
}

/// Stable cross-instance FSM coverage aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedCoverage {
    /// Per-instance reports, keyed by the caller-provided stable instance name.
    pub instances: BTreeMap<String, InstanceCoverage>,
    /// FSM states hit by at least one registered instance.
    pub state_hits: Vec<String>,
    /// FSM states missed by all registered instances that reported them.
    pub state_misses: Vec<String>,
}

/// Failure while registering coverage with a shared-stimulus scoreboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageRegistrationError {
    /// Reusing a name would overwrite a prior instance's coverage evidence.
    DuplicateInstance(String),
}

/// Shared stimulus / scoreboard fixture for multi-view co-check (FR92).
///
/// Hold a single `Vec<PortValues>` and run it through the generated functional
/// path vs `Sim::tick`. Expected outputs are the PortValues produced by both
/// sides under the same inputs (contrast / scoreboard), not a separate oracle
/// language.
#[derive(Debug, Clone)]
pub struct SharedStimulusScoreboard {
    /// Stimulus frames shared by functional and cycle-accurate paths.
    pub stimuli: Vec<PortValues>,
    coverage: BTreeMap<String, InstanceCoverage>,
}

impl SharedStimulusScoreboard {
    /// Build from an explicit shared stimulus vector.
    pub fn from_stimuli(stimuli: Vec<PortValues>) -> Self {
        Self {
            stimuli,
            coverage: BTreeMap::new(),
        }
    }

    /// Convenience: reset-high one cycle, then `n` cycles with `rst=0`.
    pub fn from_reset_then_run(n: usize) -> Self {
        Self::from_stimuli(reset_then_run(n))
    }

    /// Drive the **same** stimuli through FR47 generated functional vs `tick`.
    pub fn check_generated(&self, hir: FrozenHir) -> EquivStatus {
        check_functional_equiv_generated(hir, self.stimuli.clone())
    }

    /// Register the stable FSM coverage report of one named instance.
    ///
    /// The report is parsed through [`crate::parse_state_report`], preserving the raw text for
    /// inspection while exposing sorted state hits and misses for aggregation.
    pub fn register_coverage(
        &mut self,
        instance: impl Into<String>,
        report: impl Into<String>,
    ) -> Result<(), CoverageRegistrationError> {
        let instance = instance.into();
        if self.coverage.contains_key(&instance) {
            return Err(CoverageRegistrationError::DuplicateInstance(instance));
        }
        let report = report.into();
        let (state_hits, state_misses) = parse_state_report(&report);
        self.coverage.insert(
            instance,
            InstanceCoverage {
                report,
                state_hits,
                state_misses,
            },
        );
        Ok(())
    }

    /// Return deterministic per-instance and aggregate FSM coverage.
    pub fn aggregated_coverage(&self) -> AggregatedCoverage {
        let state_hits: BTreeSet<String> = self
            .coverage
            .values()
            .flat_map(|coverage| coverage.state_hits.iter().cloned())
            .collect();
        let state_misses: BTreeSet<String> = self
            .coverage
            .values()
            .flat_map(|coverage| coverage.state_misses.iter().cloned())
            .filter(|state| !state_hits.contains(state))
            .collect();
        AggregatedCoverage {
            instances: self.coverage.clone(),
            state_hits: state_hits.into_iter().collect(),
            state_misses: state_misses.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_state_coverage_without_hit_miss_overlap() {
        let mut board = SharedStimulusScoreboard::from_reset_then_run(2);
        board
            .register_coverage(
                "blink-n3",
                "state_hit fsm:BlinkState:Low\nstate_miss fsm:BlinkState:High\n",
            )
            .unwrap();
        board
            .register_coverage(
                "blink-n4",
                "state_hit fsm:BlinkState:High\nstate_miss fsm:BlinkState:Low\n",
            )
            .unwrap();

        let coverage = board.aggregated_coverage();
        assert_eq!(coverage.instances.len(), 2);
        assert_eq!(
            coverage.state_hits,
            ["fsm:BlinkState:High", "fsm:BlinkState:Low"]
        );
        assert!(coverage.state_misses.is_empty());
        assert_eq!(
            coverage.instances["blink-n3"].state_misses,
            ["fsm:BlinkState:High"]
        );
    }

    #[test]
    fn rejects_duplicate_instance_coverage() {
        let mut board = SharedStimulusScoreboard::from_reset_then_run(0);
        board.register_coverage("blink", "").unwrap();
        assert_eq!(
            board.register_coverage("blink", "").unwrap_err(),
            CoverageRegistrationError::DuplicateInstance("blink".into())
        );
    }
}

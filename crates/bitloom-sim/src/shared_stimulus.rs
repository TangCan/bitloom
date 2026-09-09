//! FR92 shared-stimulus scoreboard skeleton.
//!
//! One stimulus vector drives both the functional-sim path and cycle-accurate
//! `tick` (via [`crate::check_functional_equiv_generated`]). This is **not** an
//! automatic formal FL≡RTL product and **not** SystemC TLM-2.0 (AD-5).
//!
//! Bridge adapter template for transaction→cycle handshakes remains
//! `bitloom_prelude::{StartWaitComplete, start_wait_complete}` (FR78).

use bitloom_hir::{FrozenHir, PortValues};

use crate::{EquivStatus, check_functional_equiv_generated, reset_then_run};

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
}

impl SharedStimulusScoreboard {
    /// Build from an explicit shared stimulus vector.
    pub fn from_stimuli(stimuli: Vec<PortValues>) -> Self {
        Self { stimuli }
    }

    /// Convenience: reset-high one cycle, then `n` cycles with `rst=0`.
    pub fn from_reset_then_run(n: usize) -> Self {
        Self::from_stimuli(reset_then_run(n))
    }

    /// Drive the **same** stimuli through FR47 generated functional vs `tick`.
    pub fn check_generated(&self, hir: FrozenHir) -> EquivStatus {
        check_functional_equiv_generated(hir, self.stimuli.clone())
    }
}

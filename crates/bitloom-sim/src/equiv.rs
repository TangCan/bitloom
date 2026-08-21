//! Dual-view equivalence: functional vs cycle-accurate `tick` (FR30).
//!
//! Handwritten views remain supported. **P3 product acceptance** uses the FR47
//! **generated** path via [`check_functional_equiv_generated`].

use bitloom_hir::{FrozenHir, PortValues};

use crate::{AbstractionView, PortMismatch, Sim, check_mixed_both};

/// Result of a bounded functional ↔ tick equivalence run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquivStatus {
    Pass {
        cycles: usize,
    },
    Fail {
        cycle: usize,
        mismatches: Vec<PortMismatch>,
    },
}

impl EquivStatus {
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass { .. })
    }
}

/// Drive the same stimulus on FrozenHir `tick` and a functional view
/// (handwritten **or** generated `AbstractionView`).
/// Consistent PortValues → `Pass`; first divergence → `Fail`.
pub fn check_functional_equiv<A: AbstractionView>(
    hir: FrozenHir,
    abs: &mut A,
    stimuli: impl IntoIterator<Item = PortValues>,
) -> EquivStatus {
    let mut sim = Sim::new(hir);
    let mut cycles = 0usize;
    for inputs in stimuli {
        match check_mixed_both(&mut sim, abs, inputs) {
            Ok(()) => cycles += 1,
            Err(mismatches) => {
                return EquivStatus::Fail {
                    cycle: cycles,
                    mismatches,
                };
            }
        }
    }
    EquivStatus::Pass { cycles }
}

/// FR30 on the **generated** path (P3 acceptance): `GeneratedFunctional` vs `tick`.
///
/// Alias of [`crate::check_generated_bridge`]. Handwritten `check_functional_equiv`
/// remains available but is not the P3 gate.
pub fn check_functional_equiv_generated(
    hir: FrozenHir,
    stimuli: impl IntoIterator<Item = PortValues>,
) -> EquivStatus {
    crate::check_generated_bridge(hir, stimuli)
}

/// Reset-high one cycle, then `n` cycles with `rst=0`.
pub fn reset_then_run(n: usize) -> Vec<PortValues> {
    let mut out = Vec::with_capacity(n + 1);
    let mut rst = PortValues::default();
    rst.set("rst", 1);
    out.push(rst);
    for _ in 0..n {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        out.push(pv);
    }
    out
}

//! C ABI for cycle-accurate `tick` and a handwritten abstraction (FR33).

use std::ffi::CStr;
use std::ffi::c_char;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use rhdl_sim::Sim;

pub struct Handle {
    sim: Sim,
    inputs: PortValues,
    abs_count: u64,
    abs_out: PortValues,
}

fn counter_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("t");
    s.begin_module("Counter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}

fn abs_cycle(count: &mut u64, inputs: &PortValues) -> PortValues {
    if inputs.get("rst").unwrap_or(0) != 0 {
        *count = 0;
    } else {
        *count = count.wrapping_add(1);
    }
    let mut out = inputs.clone();
    out.set("data_out", *count);
    out
}

/// Rust-side golden used by tests and the C harness (rst pulse then 3 ticks → data_out=3).
pub fn rust_golden_data_out() -> u64 {
    let mut sim = Sim::new(counter_hir());
    let mut abs_count = 0u64;
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    sim.set_inputs(pv.clone());
    sim.tick();
    let _ = abs_cycle(&mut abs_count, &pv);
    pv.set("rst", 0);
    for _ in 0..3 {
        sim.set_inputs(pv.clone());
        sim.tick();
        let _ = abs_cycle(&mut abs_count, &pv);
    }
    assert_eq!(sim.ports().get("data_out"), Some(abs_count));
    sim.ports().get("data_out").unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn rhdl_sim_new() -> *mut Handle {
    Box::into_raw(Box::new(Handle {
        sim: Sim::new(counter_hir()),
        inputs: PortValues::default(),
        abs_count: 0,
        abs_out: PortValues::default(),
    }))
}

/// # Safety
/// `h` must be a pointer from `rhdl_sim_new` or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_free(h: *mut Handle) {
    if !h.is_null() {
        unsafe {
            drop(Box::from_raw(h));
        }
    }
}

fn name_of<'a>(name: *const c_char) -> Option<&'a str> {
    if name.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(name) }.to_str().ok()
}

/// # Safety
/// `h` from `rhdl_sim_new`; `name` is a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_set(h: *mut Handle, name: *const c_char, val: u64) {
    let Some(h) = (unsafe { h.as_mut() }) else {
        return;
    };
    let Some(n) = name_of(name) else { return };
    h.inputs.set(n, val);
    h.sim.set_inputs(h.inputs.clone());
}

/// # Safety
/// `h` from `rhdl_sim_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_tick(h: *mut Handle) {
    if let Some(h) = unsafe { h.as_mut() } {
        h.sim.set_inputs(h.inputs.clone());
        h.sim.tick();
    }
}

/// # Safety
/// `h` from `rhdl_sim_new`; `name` is a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_get(h: *mut Handle, name: *const c_char) -> u64 {
    let Some(h) = (unsafe { h.as_ref() }) else {
        return 0;
    };
    let Some(n) = name_of(name) else { return 0 };
    h.sim.ports().get(n).unwrap_or(0)
}

/// Handwritten abstraction cycle using the last `rhdl_sim_set` inputs.
///
/// # Safety
/// `h` from `rhdl_sim_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_abs_cycle(h: *mut Handle) {
    if let Some(h) = unsafe { h.as_mut() } {
        h.abs_out = abs_cycle(&mut h.abs_count, &h.inputs);
    }
}

/// # Safety
/// `h` from `rhdl_sim_new`; `name` is a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_abs_get(h: *mut Handle, name: *const c_char) -> u64 {
    let Some(h) = (unsafe { h.as_ref() }) else {
        return 0;
    };
    let Some(n) = name_of(name) else { return 0 };
    h.abs_out.get(n).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_golden_is_three() {
        assert_eq!(rust_golden_data_out(), 3);
    }
}

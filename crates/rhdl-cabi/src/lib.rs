//! C ABI for cycle-accurate `tick` and handwritten abstraction (FR33 / FR83).
//!
//! Documented DUTs: `Counter` (legacy `rhdl_sim_new`) and `Adder` via `rhdl_sim_new_dut`.

use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_char;
use std::ptr;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::Sim;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_error(msg: impl Into<String>) {
    let s = msg.into();
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(CString::new(s).unwrap_or_else(|_| {
            CString::new("rhdl-cabi: error message contained interior NUL").unwrap()
        }));
    });
}

fn clear_error() {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DutKind {
    Counter,
    Adder,
}

impl DutKind {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "Counter" => Some(Self::Counter),
            "Adder" => Some(Self::Adder),
            _ => None,
        }
    }

    fn hir(self) -> FrozenHir {
        match self {
            Self::Counter => counter_hir(),
            Self::Adder => adder_hir(),
        }
    }
}

pub struct Handle {
    dut: DutKind,
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

fn adder_hir() -> FrozenHir {
    let mut s = ElaborateSession::new("t");
    s.begin_module("Adder", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("sum", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_add("sum", "a", "b", Span::default());
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}

fn abs_cycle_counter(count: &mut u64, inputs: &PortValues) -> PortValues {
    if inputs.get("rst").unwrap_or(0) != 0 {
        *count = 0;
    } else {
        *count = count.wrapping_add(1);
    }
    let mut out = inputs.clone();
    out.set("data_out", *count);
    out
}

fn abs_cycle_adder(inputs: &PortValues) -> PortValues {
    let a = inputs.get("a").unwrap_or(0) & 0xff;
    let b = inputs.get("b").unwrap_or(0) & 0xff;
    let mut out = inputs.clone();
    out.set("sum", a.wrapping_add(b) & 0xff);
    out
}

fn new_handle(dut: DutKind) -> *mut Handle {
    clear_error();
    Box::into_raw(Box::new(Handle {
        dut,
        sim: Sim::new(dut.hir()),
        inputs: PortValues::default(),
        abs_count: 0,
        abs_out: PortValues::default(),
    }))
}

/// Rust-side golden used by tests and the C harness (rst pulse then 3 ticks → data_out=3).
pub fn rust_golden_data_out() -> u64 {
    let mut sim = Sim::new(counter_hir());
    let mut abs_count = 0u64;
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    sim.set_inputs(pv.clone());
    sim.tick();
    let _ = abs_cycle_counter(&mut abs_count, &pv);
    pv.set("rst", 0);
    for _ in 0..3 {
        sim.set_inputs(pv.clone());
        sim.tick();
        let _ = abs_cycle_counter(&mut abs_count, &pv);
    }
    assert_eq!(sim.ports().get("data_out"), Some(abs_count));
    sim.ports().get("data_out").unwrap()
}

/// FR83 Adder golden: a=5, b=7 → sum=12 on both RTL and abs views.
pub fn rust_golden_adder_sum() -> u64 {
    let mut sim = Sim::new(adder_hir());
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("a", 5);
    pv.set("b", 7);
    sim.set_inputs(pv.clone());
    sim.tick();
    let abs = abs_cycle_adder(&pv);
    let rtl = sim.ports().get("sum").unwrap();
    assert_eq!(rtl, abs.get("sum").unwrap());
    assert_eq!(rtl, 12);
    rtl
}

/// Legacy Counter factory (FR33).
#[unsafe(no_mangle)]
pub extern "C" fn rhdl_sim_new() -> *mut Handle {
    new_handle(DutKind::Counter)
}

/// Select a documented DUT by name (`"Counter"` or `"Adder"`).
///
/// Returns null and sets [`rhdl_last_error`] on unknown / invalid names.
///
/// # Safety
/// `dut_name` must be null or a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_new_dut(dut_name: *const c_char) -> *mut Handle {
    let Some(name) = name_of(dut_name) else {
        set_error("rhdl_sim_new_dut: dut_name is null or not valid UTF-8");
        return ptr::null_mut();
    };
    let Some(dut) = DutKind::parse(name) else {
        set_error(format!(
            "rhdl_sim_new_dut: unknown DUT '{name}' (documented: Counter, Adder)"
        ));
        return ptr::null_mut();
    };
    new_handle(dut)
}

/// Thread-local last error message, or null if none.
///
/// Pointer valid until the next `rhdl_*` call on this thread that clears/sets the error.
#[unsafe(no_mangle)]
pub extern "C" fn rhdl_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| match slot.borrow().as_ref() {
        Some(s) => s.as_ptr(),
        None => ptr::null(),
    })
}

/// # Safety
/// `h` must be a pointer from `rhdl_sim_new` / `rhdl_sim_new_dut` or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_free(h: *mut Handle) {
    if h.is_null() {
        set_error("rhdl_sim_free: null handle");
        return;
    }
    clear_error();
    unsafe {
        drop(Box::from_raw(h));
    }
}

fn name_of<'a>(name: *const c_char) -> Option<&'a str> {
    if name.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(name) }.to_str().ok()
}

/// # Safety
/// `h` from `rhdl_sim_new` / `rhdl_sim_new_dut`; `name` is a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_set(h: *mut Handle, name: *const c_char, val: u64) {
    let Some(h) = (unsafe { h.as_mut() }) else {
        set_error("rhdl_sim_set: null handle");
        return;
    };
    let Some(n) = name_of(name) else {
        set_error("rhdl_sim_set: port name is null or not valid UTF-8");
        return;
    };
    clear_error();
    h.inputs.set(n, val);
    h.sim.set_inputs(h.inputs.clone());
}

/// # Safety
/// `h` from `rhdl_sim_new` / `rhdl_sim_new_dut`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_tick(h: *mut Handle) {
    let Some(h) = (unsafe { h.as_mut() }) else {
        set_error("rhdl_sim_tick: null handle");
        return;
    };
    clear_error();
    h.sim.set_inputs(h.inputs.clone());
    h.sim.tick();
}

/// # Safety
/// `h` from `rhdl_sim_new` / `rhdl_sim_new_dut`; `name` is a NUL-terminated UTF-8 C string.
///
/// Returns `0` if the handle/name is invalid or the port is missing; check
/// [`rhdl_last_error`] to distinguish a real zero from failure (not silent success).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_sim_get(h: *mut Handle, name: *const c_char) -> u64 {
    let Some(h) = (unsafe { h.as_ref() }) else {
        set_error("rhdl_sim_get: null handle");
        return 0;
    };
    let Some(n) = name_of(name) else {
        set_error("rhdl_sim_get: port name is null or not valid UTF-8");
        return 0;
    };
    match h.sim.ports().get(n) {
        Some(v) => {
            clear_error();
            v
        }
        None => {
            set_error(format!("rhdl_sim_get: unknown or unset port '{n}'"));
            0
        }
    }
}

/// Handwritten abstraction cycle using the last `rhdl_sim_set` inputs.
///
/// # Safety
/// `h` from `rhdl_sim_new` / `rhdl_sim_new_dut`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_abs_cycle(h: *mut Handle) {
    let Some(h) = (unsafe { h.as_mut() }) else {
        set_error("rhdl_abs_cycle: null handle");
        return;
    };
    clear_error();
    h.abs_out = match h.dut {
        DutKind::Counter => abs_cycle_counter(&mut h.abs_count, &h.inputs),
        DutKind::Adder => abs_cycle_adder(&h.inputs),
    };
}

/// # Safety
/// `h` from `rhdl_sim_new` / `rhdl_sim_new_dut`; `name` is a NUL-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhdl_abs_get(h: *mut Handle, name: *const c_char) -> u64 {
    let Some(h) = (unsafe { h.as_ref() }) else {
        set_error("rhdl_abs_get: null handle");
        return 0;
    };
    let Some(n) = name_of(name) else {
        set_error("rhdl_abs_get: port name is null or not valid UTF-8");
        return 0;
    };
    match h.abs_out.get(n) {
        Some(v) => {
            clear_error();
            v
        }
        None => {
            set_error(format!("rhdl_abs_get: unknown or unset port '{n}'"));
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn rust_golden_is_three() {
        assert_eq!(rust_golden_data_out(), 3);
    }

    #[test]
    fn rust_golden_adder_is_twelve() {
        assert_eq!(rust_golden_adder_sum(), 12);
    }

    #[test]
    fn new_dut_unknown_sets_last_error() {
        let name = CString::new("Nope").unwrap();
        let h = unsafe { rhdl_sim_new_dut(name.as_ptr()) };
        assert!(h.is_null());
        let err = rhdl_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("unknown DUT"), "{msg}");
        assert!(msg.contains("Nope"), "{msg}");
    }

    #[test]
    fn new_dut_adder_works() {
        let name = CString::new("Adder").unwrap();
        let h = unsafe { rhdl_sim_new_dut(name.as_ptr()) };
        assert!(!h.is_null());
        assert!(rhdl_last_error().is_null());
        unsafe {
            let a = CString::new("a").unwrap();
            let b = CString::new("b").unwrap();
            let sum = CString::new("sum").unwrap();
            rhdl_sim_set(h, a.as_ptr(), 5);
            rhdl_sim_set(h, b.as_ptr(), 7);
            rhdl_sim_tick(h);
            rhdl_abs_cycle(h);
            assert_eq!(rhdl_sim_get(h, sum.as_ptr()), 12);
            assert_eq!(rhdl_abs_get(h, sum.as_ptr()), 12);
            assert!(rhdl_last_error().is_null());
            rhdl_sim_free(h);
        }
    }

    #[test]
    fn get_unknown_port_diagnoses() {
        let h = rhdl_sim_new();
        let name = CString::new("no_such_port").unwrap();
        let v = unsafe { rhdl_sim_get(h, name.as_ptr()) };
        assert_eq!(v, 0);
        let err = rhdl_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("no_such_port"), "{msg}");
        unsafe { rhdl_sim_free(h) };
    }
}

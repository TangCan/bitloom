//! FR194: definition identity and session lifecycle boundaries.
use bitloom_builder::{Diagnostics, ElaborateSession, GroundType, Span};

fn body(s: &mut ElaborateSession, params: &[(String, u32)]) -> Result<(), Diagnostics> {
    let width = params.iter().find(|p| p.0 == "WIDTH").map_or(8, |p| p.1);
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width }, Span::default());
    s.add_output("y", GroundType::UInt { width }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "x", Span::default());
    s.end_process();
    Ok(())
}
fn width(w: u32) -> Vec<(String, u32)> {
    vec![("WIDTH".into(), w)]
}
fn has(errors: &Diagnostics, code: &str) {
    assert!(
        errors
            .0
            .iter()
            .any(|d| d.code == code && !d.en.is_empty() && !d.zh.is_empty()),
        "{errors}"
    );
}

#[test]
fn reuses_full_parameters_independent_of_order_and_specializes_by_explicit_name() {
    let mut s = ElaborateSession::new("W8");
    let params = vec![("WIDTH".into(), 8), ("MODE".into(), 0)];
    s.define_module("W8", params.clone(), body).unwrap();
    let mut reversed = params;
    reversed.reverse();
    s.define_module("W8", reversed, body).unwrap();
    s.define_module("W16", width(16), body).unwrap();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    assert_eq!(
        hir.circuit().modules[0].ports[2].ty,
        GroundType::UInt { width: 8 }
    );
    assert_eq!(
        hir.circuit().modules[1].ports[2].ty,
        GroundType::UInt { width: 16 }
    );
}

#[test]
fn callback_receives_sorted_complete_parameters() {
    fn check(s: &mut ElaborateSession, p: &[(String, u32)]) -> Result<(), Diagnostics> {
        assert_eq!(p, &[("A".into(), u32::MAX), ("WIDTH".into(), 8)]);
        body(s, p)
    }
    let mut s = ElaborateSession::new("Top");
    s.define_module(
        "Top",
        vec![("WIDTH".into(), 8), ("A".into(), u32::MAX)],
        check,
    )
    .unwrap();
    s.finish().unwrap();
}

#[test]
fn conflicting_parameters_and_duplicate_keys_poison_finish() {
    let mut s = ElaborateSession::new("Top");
    s.define_module("Top", width(8), body).unwrap();
    has(
        &s.define_module("Top", width(16), body).unwrap_err(),
        "rhdl::E0244",
    );
    has(&s.finish().unwrap_err(), "rhdl::E0244");
    let mut s = ElaborateSession::new("Top");
    has(
        &s.define_module("Top", vec![("WIDTH".into(), 8), ("WIDTH".into(), 8)], body)
            .unwrap_err(),
        "rhdl::E0242",
    );
    has(&s.finish().unwrap_err(), "rhdl::E0242");
}

#[test]
fn same_parameters_cannot_hide_changed_body_or_source_spans() {
    fn changed(s: &mut ElaborateSession, p: &[(String, u32)]) -> Result<(), Diagnostics> {
        body(s, p)?;
        s.declare_wire("extra", GroundType::Bool, Span::default());
        Ok(())
    }
    fn moved(s: &mut ElaborateSession, p: &[(String, u32)]) -> Result<(), Diagnostics> {
        body(s, p)?;
        s.declare_wire("extra", GroundType::Bool, Span { start: 10, end: 20 });
        Ok(())
    }
    for (first, second) in [
        (
            body as fn(&mut ElaborateSession, &[(String, u32)]) -> Result<(), Diagnostics>,
            changed as _,
        ),
        (changed as _, moved as _),
    ] {
        let mut s = ElaborateSession::new("Top");
        s.define_module("Top", width(8), first).unwrap();
        has(
            &s.define_module("Top", width(8), second).unwrap_err(),
            "rhdl::E0244",
        );
        has(&s.finish().unwrap_err(), "rhdl::E0244");
    }
}

#[test]
fn repeated_body_is_executed_even_with_identical_function_pointer() {
    use std::sync::atomic::{AtomicU32, Ordering};
    static CALLS: AtomicU32 = AtomicU32::new(0);
    fn changing(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        body(s, &width(8 + CALLS.fetch_add(1, Ordering::SeqCst)))
    }
    let mut s = ElaborateSession::new("Top");
    s.define_module("Top", vec![], changing).unwrap();
    has(
        &s.define_module("Top", vec![], changing).unwrap_err(),
        "rhdl::E0244",
    );
    assert_eq!(CALLS.load(Ordering::SeqCst), 2);
}

#[test]
fn manual_definitions_cannot_be_silently_reused_in_either_order() {
    let mut s = ElaborateSession::new("Top");
    s.begin_module("Top", Span::default());
    body(&mut s, &[]).unwrap();
    s.end_module();
    has(
        &s.define_module("Top", vec![], body).unwrap_err(),
        "rhdl::E0244",
    );
    let mut s = ElaborateSession::new("Top");
    s.define_module("Top", vec![], body).unwrap();
    s.begin_module("Top", Span::default());
    body(&mut s, &[]).unwrap();
    s.end_module();
    has(&s.finish().unwrap_err(), "rhdl::E0250");
}

#[test]
fn active_module_cannot_be_overwritten_or_silently_discarded() {
    let mut s = ElaborateSession::new("Top");
    s.begin_module("Top", Span::default());
    has(
        &s.define_module("Other", vec![], body).unwrap_err(),
        "rhdl::E0240",
    );
    s.end_module();
    has(&s.finish().unwrap_err(), "rhdl::E0240");
    let mut s = ElaborateSession::new("Top");
    s.begin_module("Top", Span::default());
    body(&mut s, &[]).unwrap();
    s.begin_module("Other", Span::default());
    s.end_module();
    has(&s.finish().unwrap_err(), "rhdl::E0240");
    let mut s = ElaborateSession::new("Top");
    s.begin_module("Top", Span::default());
    body(&mut s, &[]).unwrap();
    has(&s.finish().unwrap_err(), "rhdl::E0240");
}

#[test]
fn callback_module_lifecycle_and_nested_helpers_are_rejected() {
    fn begin(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        s.begin_module("Other", Span::default());
        Ok(())
    }
    fn end(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        s.end_module();
        Ok(())
    }
    fn nested(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let _ = s.define_module("Other", vec![], body);
        Ok(())
    }
    fn open_process(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        s.begin_combinational(Span::default());
        Ok(())
    }
    fn failed(_: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        Err(Diagnostics::default())
    }
    for f in [begin, end, nested, open_process, failed] {
        let mut s = ElaborateSession::new("Top");
        assert!(s.define_module("Top", vec![], f).is_err());
        assert!(s.finish().is_err());
    }
}

#[test]
fn invalid_and_reserved_module_names_fail_before_emit() {
    for name in [
        "", "8Lane", "a-b", "a b", "模块", "module", "input", "logic", "wire",
    ] {
        let mut s = ElaborateSession::new(name);
        has(
            &s.define_module(name, vec![], body).unwrap_err(),
            "rhdl::E0241",
        );
        has(&s.finish().unwrap_err(), "rhdl::E0241");
    }
    for name in ["Lane_8", "lane", "_lane16"] {
        let mut s = ElaborateSession::new(name);
        s.define_module(name, vec![], body).unwrap();
        s.finish().unwrap();
    }
}

#[test]
fn explicit_top_can_be_defined_before_or_after_children() {
    fn top(s: &mut ElaborateSession, p: &[(String, u32)]) -> Result<(), Diagnostics> {
        body(s, p)?;
        s.add_instance(
            "u",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "x".into()),
            ],
            vec![],
            Span::default(),
        );
        Ok(())
    }
    for before in [true, false] {
        let mut s = ElaborateSession::new("Top");
        if before {
            s.define_module("Top", vec![], top).unwrap();
        }
        s.define_module("Child", vec![], body).unwrap();
        if !before {
            s.define_module("Top", vec![], top).unwrap();
        }
        assert_eq!(s.finish().unwrap().abi_name, "Top");
    }
}

#[test]
fn callback_diagnostics_survive_without_exact_duplicates() {
    fn nested(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        s.define_module("Nested", vec![], body)?;
        Ok(())
    }
    let mut session = ElaborateSession::new("Top");
    let immediate = session.define_module("Top", vec![], nested).unwrap_err();
    assert_eq!(
        immediate
            .0
            .iter()
            .filter(|d| d.code == "rhdl::E0240")
            .count(),
        1
    );
    assert_eq!(session.finish().unwrap_err(), immediate);

    fn diagnostic() -> bitloom_builder::Diagnostic {
        bitloom_builder::Diagnostic {
            code: "test::callback".into(),
            span: Span { start: 71, end: 89 },
            en: "distinct callback failure".into(),
            zh: "独立回调诊断".into(),
        }
    }
    fn fails(_: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let first = diagnostic();
        let mut distinct = first.clone();
        distinct.span.start = 72;
        Err(Diagnostics(vec![first.clone(), first, distinct]))
    }
    let mut session = ElaborateSession::new("Top");
    let immediate = session.define_module("Top", vec![], fails).unwrap_err();
    assert_eq!(
        immediate.0.iter().filter(|d| **d == diagnostic()).count(),
        1
    );
    assert_eq!(
        immediate
            .0
            .iter()
            .filter(|d| d.code == "test::callback")
            .count(),
        2
    );
    assert_eq!(session.finish().unwrap_err(), immediate);
}

#[test]
fn conflicting_definition_identifies_module_and_parameter_context() {
    let mut session = ElaborateSession::new("Lane");
    session.define_module("Lane", width(8), body).unwrap();
    let errors = session.define_module("Lane", width(16), body).unwrap_err();
    let diagnostic = errors.0.iter().find(|d| d.code == "rhdl::E0244").unwrap();
    for message in [&diagnostic.en, &diagnostic.zh] {
        for text in ["Lane", "WIDTH", "8", "16"] {
            assert!(message.contains(text), "{message}");
        }
    }
}

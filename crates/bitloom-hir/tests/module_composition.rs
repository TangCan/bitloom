//! FR194: direct freeze tests exercise imported HIR as well as builder output.
use bitloom_hir::*;
fn port(name: &str, direction: PortDirection, ty: GroundType) -> Port {
    Port {
        name: name.into(),
        direction,
        ty,
        span: Span::default(),
    }
}
fn module(name: &str) -> Module {
    Module {
        name: name.into(),
        ports: vec![
            port("clk", PortDirection::Input, GroundType::Clock),
            port("rst", PortDirection::Input, GroundType::Reset),
            port("x", PortDirection::Input, GroundType::UInt { width: 8 }),
            port("y", PortDirection::Output, GroundType::UInt { width: 8 }),
        ],
        body: vec![],
        span: Span::default(),
    }
}
fn connect(child: &str, parent: &str) -> PortConnect {
    PortConnect {
        child_port: child.into(),
        parent_net: parent.into(),
        span: Span::default(),
        dangling: false,
    }
}
fn instance(name: &str, child: &str) -> Instance {
    Instance {
        name: name.into(),
        module: child.into(),
        connects: vec![
            connect("clk", "clk"),
            connect("rst", "rst"),
            connect("x", "x"),
            connect("y", "y"),
        ],
        params: vec![],
        span: Span::default(),
    }
}
fn circuit() -> Vec<Module> {
    let mut top = module("Top");
    top.body.push(Stmt::Instance(instance("u", "Child")));
    vec![top, module("Child")]
}
fn inst(m: &mut Module) -> &mut Instance {
    match &mut m.body[0] {
        Stmt::Instance(i) => i,
        _ => panic!("test fixture"),
    }
}
fn freeze(top: &str, modules: Vec<Module>) -> Result<FrozenHir, Diagnostics> {
    let mut hir = BuilderOwnedHir::new(top);
    for module in modules {
        hir.add_module(module);
    }
    seal_from_builder(hir)
}
fn rejects(modules: Vec<Module>, code: &str) {
    let err = freeze("Top", modules).unwrap_err();
    assert!(
        err.0
            .iter()
            .any(|d| d.code == code && !d.en.is_empty() && !d.zh.is_empty()),
        "{err}"
    );
}

#[test]
fn rejects_duplicate_modules_ports_nets_instances_and_instance_net_collision() {
    let mut m = circuit();
    m.push(m[1].clone());
    rejects(m, "rhdl::E0250");
    let mut m = circuit();
    let duplicate = m[1].ports[2].clone();
    m[1].ports.push(duplicate);
    rejects(m, "rhdl::E0251");
    let mut m = circuit();
    m[0].body.push(Stmt::WireDecl {
        name: "x".into(),
        ty: GroundType::UInt { width: 8 },
        span: Span::default(),
    });
    rejects(m, "rhdl::E0251");
    let mut m = circuit();
    m[0].body.push(Stmt::Instance(instance("u", "Child")));
    rejects(m, "rhdl::E0252");
    let mut m = circuit();
    inst(&mut m[0]).name = "x".into();
    rejects(m, "rhdl::E0252");
    let mut m = circuit();
    m[0].body.push(Stmt::WireDecl {
        name: "u".into(),
        ty: GroundType::Bool,
        span: Span::default(),
    });
    rejects(m, "rhdl::E0252");
}

#[test]
fn rejects_missing_module_missing_input_unknown_net_duplicate_and_unknown_ports() {
    let mut m = circuit();
    inst(&mut m[0]).module = "Absent".into();
    rejects(m, "rhdl::E0201");
    let mut m = circuit();
    inst(&mut m[0]).connects.remove(2);
    rejects(m, "rhdl::E0202");
    let mut m = circuit();
    inst(&mut m[0]).connects[2].parent_net = "absent".into();
    rejects(m, "rhdl::E0204");
    let mut m = circuit();
    inst(&mut m[0]).connects.push(connect("x", "x"));
    rejects(m, "rhdl::E0253");
    for dangling in [true, false] {
        let mut m = circuit();
        let mut c = connect("absent", "also_absent");
        c.dangling = dangling;
        inst(&mut m[0]).connects.push(c);
        rejects(m, "rhdl::E0254");
    }
}

#[test]
fn dangling_and_unconnected_output_remain_legal_but_not_duplicate_dangling_ports() {
    let mut m = circuit();
    for c in &mut inst(&mut m[0]).connects {
        c.dangling = true;
        c.parent_net = "absent".into();
    }
    freeze("Top", m).unwrap();
    let mut m = circuit();
    inst(&mut m[0]).connects.pop();
    freeze("Top", m).unwrap();
    let mut m = circuit();
    let mut c = connect("x", "absent");
    c.dangling = true;
    inst(&mut m[0]).connects.push(c);
    rejects(m, "rhdl::E0253");
}

#[test]
fn widths_and_ground_types_checked_for_both_directions() {
    for index in [2, 3] {
        let mut m = circuit();
        m[0].ports[index].ty = GroundType::UInt { width: 16 };
        rejects(m, "rhdl::E0203");
        let mut m = circuit();
        m[0].ports[index].ty = GroundType::SInt { width: 8 };
        rejects(m, "rhdl::E0256");
    }
    for (child_port, parent_net) in [
        ("clk", "rst"),
        ("rst", "clk"),
        ("clk", "bit"),
        ("rst", "bit"),
    ] {
        let mut m = circuit();
        m[0].ports.push(port(
            "bit",
            PortDirection::Input,
            GroundType::UInt { width: 1 },
        ));
        let c = inst(&mut m[0])
            .connects
            .iter_mut()
            .find(|c| c.child_port == child_port)
            .unwrap();
        c.parent_net = parent_net.into();
        rejects(m, "rhdl::E0256");
    }
    for (parent_ty, child_ty) in [
        (GroundType::Bool, GroundType::UInt { width: 1 }),
        (GroundType::UInt { width: 1 }, GroundType::Bool),
        (GroundType::SInt { width: 8 }, GroundType::SInt { width: 8 }),
    ] {
        let mut m = circuit();
        for index in [2, 3] {
            m[0].ports[index].ty = parent_ty.clone();
            m[1].ports[index].ty = child_ty.clone();
        }
        freeze("Top", m).unwrap();
    }
}

fn reg(name: &str) -> Stmt {
    Stmt::RegDecl {
        name: name.into(),
        ty: GroundType::UInt { width: 8 },
        clock: "clk".into(),
        reset: "rst".into(),
        async_reset: false,
        has_enable: false,
        span: Span::default(),
    }
}
#[test]
fn child_outputs_only_drive_wires_or_parent_outputs() {
    for parent_net in ["x", "q"] {
        let mut m = circuit();
        m[0].body.push(reg("q"));
        inst(&mut m[0]).connects[3].parent_net = parent_net.into();
        rejects(m, "rhdl::E0257");
    }
    let mut m = circuit();
    m[0].body.push(Stmt::WireDecl {
        name: "w".into(),
        ty: GroundType::UInt { width: 8 },
        span: Span::default(),
    });
    inst(&mut m[0]).connects[3].parent_net = "w".into();
    freeze("Top", m).unwrap();
    freeze("Top", circuit()).unwrap();
}

#[test]
fn child_inputs_can_read_parent_outputs_and_registers() {
    for parent_net in ["y", "q"] {
        let mut m = circuit();
        m[0].body.push(reg("q"));
        inst(&mut m[0]).connects[2].parent_net = parent_net.into();
        freeze("Top", m).unwrap();
    }
}

#[test]
fn detects_instance_instance_and_instance_process_multiple_drivers() {
    let mut m = circuit();
    m[0].body.push(Stmt::Instance(instance("other", "Child")));
    rejects(m, "rhdl::E0140");
    let mut m = circuit();
    m[0].body.push(Stmt::Process(Process {
        kind: ProcessKind::Combinational,
        assigns: vec![Assign {
            target: AssignTarget::Net("y".into()),
            expr: AssignExpr::Ref("x".into()),
            span: Span::default(),
        }],
        span: Span::default(),
    }));
    rejects(m, "rhdl::E0140");
    let mut m = circuit();
    m[1].ports.push(port(
        "y2",
        PortDirection::Output,
        GroundType::UInt { width: 8 },
    ));
    inst(&mut m[0]).connects.push(connect("y2", "y"));
    rejects(m, "rhdl::E0140");
}

#[test]
fn identical_local_names_across_modules_and_distinct_output_nets_are_legal() {
    let mut m = circuit();
    for module in &mut m {
        module.body.push(reg("q"));
    }
    m[0].ports.push(port(
        "z",
        PortDirection::Output,
        GroundType::UInt { width: 8 },
    ));
    let mut other = instance("other", "Child");
    other.connects[3].parent_net = "z".into();
    m[0].body.push(Stmt::Instance(other));
    freeze("Top", m).unwrap();
}

#[test]
fn rejects_explicit_top_cycle_unreachable_cycle_and_self_cycle() {
    let mut m = circuit();
    m[1].body.push(Stmt::Instance(instance("back", "Top")));
    rejects(m, "rhdl::E0255");
    let mut m = circuit();
    let mut hidden = module("Hidden");
    hidden
        .body
        .push(Stmt::Instance(instance("self_ref", "Hidden")));
    m.push(hidden);
    rejects(m, "rhdl::E0255");
    let mut m = vec![module("Top")];
    m[0].body.push(Stmt::Instance(instance("self_ref", "Top")));
    rejects(m, "rhdl::E0255");
    let mut m = circuit();
    let mut a = module("A");
    let mut b = module("B");
    a.body.push(Stmt::Instance(instance("b", "B")));
    b.body.push(Stmt::Instance(instance("a", "A")));
    m.extend([a, b]);
    rejects(m, "rhdl::E0255");
}

#[test]
fn explicit_top_wins_otherwise_unique_root_required_and_declaration_order_irrelevant() {
    for reverse in [true, false] {
        let mut m = circuit();
        if reverse {
            m.reverse();
        }
        assert_eq!(freeze("other_circuit_name", m).unwrap().abi_name, "Top");
    }
    let mut m = circuit();
    m.push(module("Unused"));
    assert_eq!(freeze("Top", m.clone()).unwrap().abi_name, "Top");
    let err = freeze("missing", m).unwrap_err();
    assert!(err.0.iter().any(|d| d.code == "rhdl::E0002"));
}

#[test]
fn special_io_uses_selected_top_including_unique_root_fallback() {
    let mut m = circuit();
    m[0].ports
        .push(port("pad", PortDirection::InOut, GroundType::Analog));
    freeze("unmatched_circuit_name", m).unwrap();
    let mut m = circuit();
    m[1].ports
        .push(port("pad", PortDirection::InOut, GroundType::Analog));
    rejects(m, "rhdl::E0270");
}

#[test]
fn deep_graph_does_not_use_recursive_traversal() {
    const DEPTH: usize = 4000;
    let mut modules = Vec::with_capacity(DEPTH);
    for i in 0..DEPTH {
        let mut m = module(&format!("M{i}"));
        if i + 1 < DEPTH {
            m.body
                .push(Stmt::Instance(instance("next", &format!("M{}", i + 1))));
        }
        modules.push(m);
    }
    assert_eq!(freeze("unmatched", modules).unwrap().abi_name, "M0");
}

#[test]
fn cycle_diagnostic_names_bounded_cycle_or_blocked_residual() {
    let mut modules = vec![module("Top")];
    let mut hidden = module("HiddenCycle");
    hidden
        .body
        .push(Stmt::Instance(instance("self_ref", "HiddenCycle")));
    for index in 0..10 {
        let name = format!("Blocked{index}");
        let mut child = instance(&format!("child{index}"), &name);
        child.connects.pop(); // The leaf output is intentionally unused.
        hidden.body.push(Stmt::Instance(child));
        modules.push(module(&name));
    }
    modules.insert(1, hidden);
    let errors = freeze("Top", modules).unwrap_err();
    let diagnostic = errors.0.iter().find(|d| d.code == "rhdl::E0255").unwrap();
    assert!(diagnostic.en.contains("cycle/blocked residual"));
    assert!(diagnostic.zh.contains("环或受环阻塞"));
    for message in [&diagnostic.en, &diagnostic.zh] {
        assert!(message.contains("HiddenCycle"), "{message}");
        assert!(message.contains("Blocked0"), "{message}");
        assert!(!message.contains("Blocked9"), "{message}");
        assert!(message.contains("11"), "{message}");
    }
}

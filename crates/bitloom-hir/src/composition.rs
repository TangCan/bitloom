//! FR194: shared freeze validation for builder and imported hierarchy.
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

fn error(diags: &mut Diagnostics, span: Span, code: &str, en: String, zh: String) {
    diags.push(Diagnostic {
        span,
        code: code.into(),
        en,
        zh,
    });
}

pub(super) fn validate_hierarchy(circuit: &Circuit, diags: &mut Diagnostics) -> Option<String> {
    let mut modules = HashMap::new();
    for (index, module) in circuit.modules.iter().enumerate() {
        if modules.insert(module.name.as_str(), index).is_some() {
            error(
                diags,
                module.span,
                "rhdl::E0250",
                format!("duplicate module '{}'", module.name),
                format!("模块 '{}' 重复", module.name),
            );
        }
    }
    let mut children = vec![Vec::new(); circuit.modules.len()];
    let mut incoming = vec![0usize; circuit.modules.len()];
    for (index, parent) in circuit.modules.iter().enumerate() {
        validate_instances(circuit, &modules, parent, diags);
        for statement in &parent.body {
            if let Stmt::Instance(instance) = statement {
                if let Some(&child) = modules.get(instance.module.as_str()) {
                    children[index].push(child);
                    incoming[child] += 1;
                }
            }
        }
    }
    let roots: Vec<_> = incoming
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| (n == 0).then_some(i))
        .collect();
    let top = modules.get(circuit.name.as_str()).copied().or_else(|| {
        if roots.len() == 1 {
            Some(roots[0])
        } else {
            None
        }
    });
    if top.is_none() {
        error(
            diags,
            Span::default(),
            "rhdl::E0002",
            format!(
                "circuit needs an explicit top or exactly one root (found {})",
                roots.len()
            ),
            format!("电路需要显式顶层或唯一根模块（找到 {} 个根）", roots.len()),
        );
    }
    // Kahn traversal includes unreachable definitions and avoids recursive stack
    // growth. Multiple instances of the same child count as separate edges.
    let mut ready: VecDeque<_> = roots.into();
    let mut visited = 0;
    while let Some(parent) = ready.pop_front() {
        visited += 1;
        for &child in &children[parent] {
            incoming[child] -= 1;
            if incoming[child] == 0 {
                ready.push_back(child);
            }
        }
    }
    if visited != circuit.modules.len() {
        const RESIDUAL_LIMIT: usize = 8;
        let residual_count = circuit.modules.len() - visited;
        let residual = incoming
            .iter()
            .enumerate()
            .filter(|(_, count)| **count != 0)
            .take(RESIDUAL_LIMIT)
            .map(|(index, _)| circuit.modules[index].name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        error(
            diags,
            Span::default(),
            "rhdl::E0255",
            format!(
                "recursive module instance graph (including unreachable definitions); cycle/blocked residual modules (showing up to {RESIDUAL_LIMIT} of {residual_count}): {residual}"
            ),
            format!(
                "模块实例图存在环（包括顶层不可达定义）；环或受环阻塞的残留模块（共 {residual_count} 个，最多列出 {RESIDUAL_LIMIT} 个）：{residual}"
            ),
        );
    }
    top.map(|i| circuit.modules[i].name.clone())
}

fn validate_instances(
    circuit: &Circuit,
    modules: &HashMap<&str, usize>,
    parent: &Module,
    diags: &mut Diagnostics,
) {
    let mut names = HashSet::new();
    let mut nets = HashMap::new();
    for port in &parent.ports {
        if !names.insert(port.name.as_str()) {
            error(
                diags,
                port.span,
                "rhdl::E0251",
                format!(
                    "duplicate port or local net '{}.{}'",
                    parent.name, port.name
                ),
                format!("端口或局部网 '{}.{}' 重复", parent.name, port.name),
            );
        }
        nets.insert(
            port.name.as_str(),
            (&port.ty, port.direction == PortDirection::Output),
        );
    }
    for stmt in &parent.body {
        let (name, span) = match stmt {
            Stmt::WireDecl { name, ty, span } => {
                nets.insert(name.as_str(), (ty, true));
                (name, span)
            }
            Stmt::RegDecl { name, ty, span, .. } => {
                nets.insert(name.as_str(), (ty, false));
                (name, span)
            }
            Stmt::MemDecl { name, span, .. } => (name, span),
            _ => continue,
        };
        if !names.insert(name.as_str()) {
            error(
                diags,
                *span,
                "rhdl::E0251",
                format!("duplicate port or local net '{}.{name}'", parent.name),
                format!("端口或局部网 '{}.{name}' 重复", parent.name),
            );
        }
    }
    let mut driven = HashSet::new();
    for stmt in &parent.body {
        if let Stmt::Process(process) = stmt {
            for assign in &process.assigns {
                match &assign.target {
                    AssignTarget::Net(name) | AssignTarget::RegD(name) => {
                        driven.insert(name.as_str());
                    }
                    AssignTarget::MemWrite { .. } => {}
                }
            }
        }
    }
    for stmt in &parent.body {
        let Stmt::Instance(inst) = stmt else {
            continue;
        };
        if !names.insert(inst.name.as_str()) {
            error(
                diags,
                inst.span,
                "rhdl::E0252",
                format!(
                    "duplicate instance or local net collision '{}.{}'",
                    parent.name, inst.name
                ),
                format!("实例名重复或与局部网冲突 '{}.{}'", parent.name, inst.name),
            );
        }
        let Some(&index) = modules.get(inst.module.as_str()) else {
            error(
                diags,
                inst.span,
                "rhdl::E0201",
                format!("unknown child module '{}'", inst.module),
                format!("未知子模块 '{}'", inst.module),
            );
            continue;
        };
        let child = &circuit.modules[index];
        let ports: HashMap<_, _> = child.ports.iter().map(|p| (p.name.as_str(), p)).collect();
        let mut connected = HashSet::new();
        for c in &inst.connects {
            if !connected.insert(c.child_port.as_str()) {
                error(
                    diags,
                    c.span,
                    "rhdl::E0253",
                    format!("duplicate connection '{}.{}'", inst.name, c.child_port),
                    format!("连接 '{}.{}' 重复", inst.name, c.child_port),
                );
            }
            let Some(port) = ports.get(c.child_port.as_str()) else {
                error(
                    diags,
                    c.span,
                    "rhdl::E0254",
                    format!("unknown child port '{}.{}'", inst.name, c.child_port),
                    format!("未知子模块端口 '{}.{}'", inst.name, c.child_port),
                );
                continue;
            };
            // Dangling is an explicit connection opt-out, never a port-name opt-out.
            if c.dangling {
                continue;
            }
            let Some(&(parent_ty, writable)) = nets.get(c.parent_net.as_str()) else {
                error(
                    diags,
                    c.span,
                    "rhdl::E0204",
                    format!(
                        "cannot resolve parent net '{}' for '{}.{}'",
                        c.parent_net, inst.name, port.name
                    ),
                    format!(
                        "连接 '{}.{}' 时无法解析父网 '{}'",
                        inst.name, port.name, c.parent_net
                    ),
                );
                continue;
            };
            if width_of(parent_ty) != width_of(&port.ty) {
                error(
                    diags,
                    c.span,
                    "rhdl::E0203",
                    format!(
                        "width mismatch connecting '{}' to '{}.{}'",
                        c.parent_net, inst.name, port.name
                    ),
                    format!(
                        "连接 '{}' 与 '{}.{}' 的位宽不匹配",
                        c.parent_net, inst.name, port.name
                    ),
                );
            } else if !compatible_types(parent_ty, &port.ty) {
                error(
                    diags,
                    c.span,
                    "rhdl::E0256",
                    format!(
                        "type mismatch connecting '{}' ({parent_ty:?}) to '{}.{}' ({:?})",
                        c.parent_net, inst.name, port.name, port.ty
                    ),
                    format!(
                        "连接 '{}' 与 '{}.{}' 的类型不匹配",
                        c.parent_net, inst.name, port.name
                    ),
                );
            }
            if port.direction == PortDirection::Output {
                if !writable {
                    error(
                        diags,
                        c.span,
                        "rhdl::E0257",
                        format!(
                            "child output '{}.{}' must drive a wire or parent output, not '{}'",
                            inst.name, port.name, c.parent_net
                        ),
                        format!(
                            "子模块输出 '{}.{}' 只能驱动 wire 或父输出，不能驱动 '{}'",
                            inst.name, port.name, c.parent_net
                        ),
                    );
                }
                if !driven.insert(c.parent_net.as_str()) {
                    error(
                        diags,
                        c.span,
                        "rhdl::E0140",
                        format!("multiple instance/process drivers for '{}'", c.parent_net),
                        format!("网 '{}' 存在多个实例或过程驱动", c.parent_net),
                    );
                }
            }
        }
        for port in &child.ports {
            if port.direction == PortDirection::Input && !connected.contains(port.name.as_str()) {
                error(
                    diags,
                    inst.span,
                    "rhdl::E0202",
                    format!(
                        "undriven child input '{}.{}' (mark dangling if intentional)",
                        inst.name, port.name
                    ),
                    format!(
                        "子模块输入 '{}.{}' 未驱动（若故意悬空请标记 dangling）",
                        inst.name, port.name
                    ),
                );
            }
        }
    }
}

fn compatible_types(a: &GroundType, b: &GroundType) -> bool {
    // Bool is represented as an unsigned one-bit data signal by both backends.
    // Clock and Reset are control types, not interchangeable one-bit data.
    a == b
        || matches!(
            (a, b),
            (GroundType::Bool, GroundType::UInt { width: 1 })
                | (GroundType::UInt { width: 1 }, GroundType::Bool)
        )
}

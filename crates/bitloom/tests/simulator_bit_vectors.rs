use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::{FrozenHir, PortValues};
use bitloom_sim::{AbstractionView, GeneratedFunctional, Sim, TickEngine, generate_functional_sim};
use std::{fs, process::Command};

fn design(width: u32) -> FrozenHir {
    let mut s = ElaborateSession::new("vectors");
    let p = Span::default();
    s.begin_module("Vectors", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    for n in ["a", "b", "shift"] {
        s.add_input(n, GroundType::UInt { width }, p);
    }
    s.add_input("signed_a", GroundType::SInt { width }, p);
    s.add_input("signed_b", GroundType::SInt { width }, p);
    for n in [
        "left",
        "right",
        "sum",
        "diff",
        "and_o",
        "or_o",
        "xor_o",
        "count",
        "after_sum",
    ] {
        s.add_output(n, GroundType::UInt { width }, p);
    }
    s.add_output("sar", GroundType::SInt { width }, p);
    s.add_output("signed_left", GroundType::SInt { width }, p);
    s.add_output("signed_right", GroundType::SInt { width }, p);
    s.add_output("eq", GroundType::Bool, p);
    s.add_output("ult", GroundType::Bool, p);
    s.add_output("slt", GroundType::Bool, p);
    s.add_output("zext", GroundType::UInt { width: 64 }, p);
    s.add_output("sext", GroundType::SInt { width: 64 }, p);
    s.declare_reg("state", GroundType::UInt { width }, p);
    s.begin_sequential(p);
    s.assign_reg_d_inc("state", p);
    s.end_process();
    s.begin_combinational(p);
    s.assign_shl("left", "a", "shift", p);
    s.assign_shr("right", "a", "shift", p);
    s.assign_sar("sar", "signed_a", "shift", p);
    s.assign_shl("signed_left", "signed_a", "shift", p);
    s.assign_shr("signed_right", "signed_a", "shift", p);
    s.assign_add("sum", "a", "b", p);
    s.assign_shr("after_sum", "sum", "shift", p);
    s.assign_sub("diff", "a", "b", p);
    s.assign_and("and_o", "a", "b", p);
    s.assign_or("or_o", "a", "b", p);
    s.assign_xor("xor_o", "a", "b", p);
    s.assign_ult("ult", "a", "b", p);
    s.assign_slt("slt", "signed_a", "signed_b", p);
    if width < 64 {
        s.assign_zero_extend("zext", "a", 64, p);
        s.assign_sign_extend("sext", "signed_a", 64, p);
    } else {
        s.assign_net("zext", "a", p);
        s.assign_net("sext", "signed_a", p);
    }
    s.assign_eq("eq", "a", "b", p);
    s.assign_net("count", "state", p);
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}

#[test]
fn bit_vectors_match_reference_and_emitted_rust_and_rtl() {
    let root = std::env::temp_dir().join(format!(
        "bitloom-vectors-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    for width in [1, 8, 32, 64] {
        let mask = u64::MAX >> (64 - width);
        let h = design(width);
        let mut sims = [
            Sim::new(h.clone()),
            Sim::with_engine(h.clone(), TickEngine::Compiled),
        ];
        let mut functional = GeneratedFunctional::from_hir(&h);
        let mut rust = String::from(
            "#[test] fn vectors() { let mut s = bitloom_func_vectors::FunctionalSim::new();\n",
        );
        let mut rtl = String::from("module tb; reg clk=0, rst=0;\n");
        for n in ["a", "b", "shift", "signed_a", "signed_b"] {
            rtl += &format!("reg [{}:0] {n};\n", width - 1);
        }
        for n in [
            "left",
            "right",
            "sum",
            "diff",
            "and_o",
            "or_o",
            "xor_o",
            "count",
            "after_sum",
            "sar",
            "signed_left",
            "signed_right",
        ] {
            rtl += &format!("wire [{}:0] {n};\n", width - 1);
        }
        rtl += "wire eq, ult, slt; wire [63:0] zext, sext; Vectors dut(.*); initial begin\n";
        let mut count = 0;
        for reset_input in [1, 0, 3, 2] {
            let reset = reset_input & 1 != 0;
            for a in [0, 1, mask >> 1, mask] {
                for shift in [
                    0,
                    width as u64 - 1,
                    width as u64,
                    63,
                    64,
                    65,
                    1u64 << 32,
                    mask,
                ]
                .map(|v| v & mask)
                {
                    let b = 1 & mask;
                    let mut input = PortValues::default();
                    for (n, v) in [
                        ("rst", reset_input),
                        ("a", if width < 64 { a | (1 << width) } else { a }),
                        ("b", b),
                        ("signed_a", if width < 64 { a | (1 << width) } else { a }),
                        ("signed_b", b),
                        (
                            "shift",
                            if width < 64 {
                                shift | (1 << width)
                            } else {
                                shift
                            },
                        ),
                    ] {
                        input.set(n, v);
                    }
                    count = if reset { 0 } else { (count + 1) & mask };
                    let sum = a.wrapping_add(b) & mask;
                    let sar = if shift >= width as u64 {
                        if a & (1 << (width - 1)) != 0 { mask } else { 0 }
                    } else {
                        ((((a << (64 - width)) as i64) >> shift) as u64 >> (64 - width)) & mask
                    };
                    let expected = [
                        ("a", a),
                        ("signed_a", a),
                        ("b", b),
                        ("signed_b", b),
                        ("shift", shift),
                        ("rst", reset as u64),
                        ("left", if shift >= 64 { 0 } else { (a << shift) & mask }),
                        ("right", if shift >= 64 { 0 } else { a >> shift }),
                        ("sar", sar),
                        (
                            "signed_left",
                            if shift >= 64 { 0 } else { (a << shift) & mask },
                        ),
                        ("signed_right", if shift >= 64 { 0 } else { a >> shift }),
                        ("sum", sum),
                        ("after_sum", if shift >= 64 { 0 } else { sum >> shift }),
                        ("diff", a.wrapping_sub(b) & mask),
                        ("and_o", a & b),
                        ("or_o", a | b),
                        ("xor_o", a ^ b),
                        ("eq", (a == b) as u64),
                        ("count", count),
                        ("ult", (a < b) as u64),
                        (
                            "slt",
                            ((((a << (64 - width)) as i64) >> (64 - width))
                                < (((b << (64 - width)) as i64) >> (64 - width)))
                                as u64,
                        ),
                        ("zext", a),
                        ("sext", ((a << (64 - width)) as i64 >> (64 - width)) as u64),
                    ];
                    let f = functional.cycle(&input);
                    for sim in &mut sims {
                        sim.set_inputs(input.clone());
                        sim.tick();
                        for (n, v) in expected {
                            assert_eq!(
                                sim.ports().get(n),
                                Some(v),
                                "native {width} {n} a={a} shift={shift}"
                            );
                        }
                    }
                    for (n, v) in expected {
                        assert_eq!(
                            f.get(n),
                            Some(v),
                            "functional {width} {n} a={a} shift={shift}"
                        );
                    }
                    rust += "let mut p = bitloom_hir::PortValues::default();\n";
                    for (n, v) in &input.values {
                        rust += &format!("p.set({n:?}, {v});\n");
                        rtl += &format!("{n} = 64'h{v:x};\n");
                    }
                    rust += "let o = s.cycle(&p);\n";
                    rtl += "#5; clk=1; #5;\n";
                    for (n, v) in expected {
                        rust += &format!("assert_eq!(o.get({n:?}), Some({v}));\n");
                        rtl += &format!(
                            "if ({n} !== 64'h{v:x}) $fatal(1, \"{width} {n} a={a} shift={shift}\");\n"
                        );
                    }
                    rtl += "clk=0;\n";
                }
            }
        }
        rust += "}\n";
        rtl += "$finish; end endmodule\n";
        let dir = root.join(width.to_string());
        generate_functional_sim(&h, &dir).unwrap();
        fs::create_dir_all(dir.join("tests")).unwrap();
        fs::write(dir.join("tests/vectors.rs"), rust).unwrap();
        let output = Command::new("cargo")
            .args(["test", "--offline", "--quiet", "--manifest-path"])
            .arg(dir.join("Cargo.toml"))
            .env("CARGO_TARGET_DIR", root.join("target"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(
            dir.join("design.v"),
            bitloom_vlog::emit(&h).files[0].contents.clone(),
        )
        .unwrap();
        fs::write(dir.join("tb.v"), rtl).unwrap();
        if Command::new("iverilog").arg("-V").output().is_ok() {
            let output = Command::new("iverilog")
                .current_dir(&dir)
                .args(["-g2012", "-s", "tb", "-o", "run", "design.v", "tb.v"])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new("vvp")
                    .current_dir(&dir)
                    .arg("run")
                    .status()
                    .unwrap()
                    .success()
            );
        } else {
            assert!(
                std::env::var_os("BITLOOM_REQUIRE_RTL").is_none(),
                "RTL comparison requires iverilog and vvp"
            );
            eprintln!("RTL comparison not run: install iverilog and vvp");
        }

        if let Some(tool_dir) = std::env::var_os("RHDL_FIRTOOL_PATH") {
            fs::write(
                dir.join("design.fir"),
                bitloom_firrtl::emit(&h).files[0].contents.clone(),
            )
            .unwrap();
            let output = Command::new(std::path::Path::new(&tool_dir).join("firtool"))
                .arg(dir.join("design.fir"))
                .args([
                    "--verilog",
                    "--disable-all-randomization",
                    "--lowering-options=disallowLocalVariables",
                ])
                .arg("-o")
                .arg(dir.join("lowered.v"))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let output = Command::new("iverilog")
                .current_dir(&dir)
                .args([
                    "-g2012",
                    "-s",
                    "tb",
                    "-o",
                    "lowered-run",
                    "lowered.v",
                    "tb.v",
                ])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new("vvp")
                    .current_dir(&dir)
                    .arg("lowered-run")
                    .status()
                    .unwrap()
                    .success()
            );
        } else {
            assert!(
                std::env::var_os("BITLOOM_REQUIRE_FIRRTL").is_none(),
                "FIRRTL comparison requires RHDL_FIRTOOL_PATH"
            );
            eprintln!("FIRRTL comparison not run: set RHDL_FIRTOOL_PATH to product-pin directory");
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn bool_enable_uses_declared_one_bit() {
    let p = Span::default();
    let mut s = ElaborateSession::new("enable");
    s.begin_module("Enabled", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("en", GroundType::Bool, p);
    s.add_output("out", GroundType::UInt { width: 8 }, p);
    s.declare_reg_ex("q", GroundType::UInt { width: 8 }, false, true, p);
    s.begin_sequential(p);
    s.assign_reg_d_inc("q", p);
    s.end_process();
    s.begin_combinational(p);
    s.assign_net("out", "q", p);
    s.end_process();
    s.end_module();
    let h = s.finish().unwrap();
    let vectors: Vec<_> = [(3, 3, 0), (2, 2, 0), (2, 3, 1), (0, 0, 1), (0, 1, 2)]
        .into_iter()
        .map(|(reset, enable, value)| {
            let mut input = PortValues::default();
            input.set("rst", reset);
            input.set("en", enable);
            let mut expected = PortValues::default();
            expected.set("out", value);
            expected.set("rst", reset & 1);
            expected.set("en", enable & 1);
            (input, expected)
        })
        .collect();
    verify_functional_crate(&h, &vectors);
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut native = Sim::with_engine(h.clone(), engine);
        let mut functional = GeneratedFunctional::from_hir(&h);
        for (reset, enable, expected) in [(3, 3, 0), (2, 2, 0), (2, 3, 1), (0, 0, 1), (0, 1, 2)] {
            let mut input = PortValues::default();
            input.set("rst", reset);
            input.set("en", enable);
            native.set_inputs(input.clone());
            native.tick();
            let output = functional.cycle(&input);
            assert_eq!(native.ports().get("out"), Some(expected));
            assert_eq!(output.get("out"), Some(expected));
            for key in ["rst", "en"] {
                assert_eq!(native.ports().get(key), output.get(key));
            }
        }
    }
}
fn temporary_dir(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "bitloom-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn verify_functional_crate(hir: &FrozenHir, vectors: &[(PortValues, PortValues)]) {
    let dir = temporary_dir(&hir.abi_name);
    generate_functional_sim(hir, &dir).unwrap();
    let mut source = format!(
        "#[test] fn vectors() {{ let mut s = bitloom_func_{}::FunctionalSim::new();\n",
        hir.abi_name.to_lowercase()
    );
    for (input, expected) in vectors {
        source += "let mut p = bitloom_hir::PortValues::default();\n";
        for (name, value) in &input.values {
            source += &format!("p.set({name:?}, {value});\n");
        }
        source += "let output = s.cycle(&p);\n";
        for (name, value) in &expected.values {
            source += &format!("assert_eq!(output.get({name:?}), Some({value}));\n");
        }
    }
    source += "}\n";
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(dir.join("tests/vectors.rs"), source).unwrap();
    let output = Command::new("cargo")
        .args(["test", "--offline", "--quiet", "--manifest-path"])
        .arg(dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn uint128_passthrough_preserves_low_u64_word() {
    let p = Span::default();
    let mut s = ElaborateSession::new("wide");
    s.begin_module("Wide", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("data", GroundType::UInt { width: 128 }, p);
    s.add_output("out", GroundType::UInt { width: 128 }, p);
    s.begin_combinational(p);
    s.assign_net("out", "data", p);
    s.end_process();
    s.end_module();
    let h = s.finish().unwrap();
    let vectors: Vec<_> = [0, 1, 1 << 63, u64::MAX]
        .into_iter()
        .map(|value| {
            let mut input = PortValues::default();
            input.set("data", value);
            let mut expected = input.clone();
            expected.set("out", value);
            (input, expected)
        })
        .collect();
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut native = Sim::with_engine(h.clone(), engine);
        let mut functional = GeneratedFunctional::from_hir(&h);
        for (input, expected) in &vectors {
            native.set_inputs(input.clone());
            native.tick();
            let output = functional.cycle(input);
            for (name, value) in &expected.values {
                assert_eq!(native.ports().get(name), Some(*value));
                assert_eq!(output.get(name), Some(*value));
            }
        }
    }
    verify_functional_crate(&h, &vectors);
}

#[test]
fn mixed_width_logical_shifts_match_rtl_and_roundtrip() {
    for (value_width, shift_width, output_width) in
        [(32, 1, 32), (8, 8, 16), (8, 1, 16), (16, 8, 8)]
    {
        let p = Span::default();
        let mut s = ElaborateSession::new("mixed");
        s.begin_module("Mixed", p);
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        s.add_input("a", GroundType::UInt { width: value_width }, p);
        s.add_input("sa", GroundType::SInt { width: value_width }, p);
        s.add_input("shift", GroundType::UInt { width: shift_width }, p);
        s.add_output(
            "left",
            GroundType::UInt {
                width: output_width,
            },
            p,
        );
        s.add_output(
            "sleft",
            GroundType::SInt {
                width: output_width,
            },
            p,
        );
        s.add_output(
            "sright",
            GroundType::SInt {
                width: output_width,
            },
            p,
        );
        s.begin_combinational(p);
        s.assign_shl("left", "a", "shift", p);
        s.assign_shl("sleft", "sa", "shift", p);
        s.assign_shr("sright", "sa", "shift", p);
        s.end_process();
        s.end_module();
        let h = s.finish().unwrap();
        let fir = bitloom_firrtl::emit(&h).files[0].contents.clone();
        let back = bitloom_firrtl::import(&fir).unwrap();
        let mut native = Sim::new(h.clone());
        let mut imported = Sim::new(back);
        let mut functional = GeneratedFunctional::from_hir(&h);
        let mut vectors = Vec::new();
        let mut tb = format!(
            "module tb; reg clk=0, rst=0; reg [{}:0] a, sa; reg [{}:0] shift; wire [{}:0] left, sleft, sright; Mixed dut(.*); initial begin\n",
            value_width - 1,
            shift_width - 1,
            output_width - 1
        );
        for a in [1, (1u64 << value_width) - 1] {
            for shift in [0, 1, 7, 8, 15, 16, 32, 65, 255]
                .into_iter()
                .filter(|v| *v < (1 << shift_width))
            {
                let mut input = PortValues::default();
                input.set("a", a);
                input.set("sa", a);
                input.set("shift", shift);
                let mut expected = PortValues::default();
                let mask = (1 << output_width) - 1;
                for (n, v) in [
                    (
                        "left",
                        a.checked_shl(shift.min(64) as u32).unwrap_or(0) & mask,
                    ),
                    (
                        "sleft",
                        a.checked_shl(shift.min(64) as u32).unwrap_or(0) & mask,
                    ),
                    (
                        "sright",
                        a.checked_shr(shift.min(64) as u32).unwrap_or(0) & mask,
                    ),
                ] {
                    expected.set(n, v);
                }
                native.set_inputs(input.clone());
                native.tick();
                imported.set_inputs(input.clone());
                imported.tick();
                let f = functional.cycle(&input);
                tb += &format!("a=64'h{a:x}; sa=64'h{a:x}; shift=64'h{shift:x}; #1;\n");
                for (name, value) in &expected.values {
                    assert_eq!(native.ports().get(name), Some(*value));
                    assert_eq!(imported.ports().get(name), Some(*value));
                    assert_eq!(f.get(name), Some(*value));
                    tb += &format!(
                        "if ({name} !== 64'h{value:x}) $fatal(1, \"{name} widths={value_width}/{shift_width}/{output_width} a={a} shift={shift}\");\n"
                    );
                }
                vectors.push((input, expected));
            }
        }
        tb += "$finish; end endmodule\n";
        verify_functional_crate(&h, &vectors);
        let dir = temporary_dir("mixed-shifts");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tb.v"), tb).unwrap();
        fs::write(
            dir.join("direct.v"),
            bitloom_vlog::emit(&h).files[0].contents.clone(),
        )
        .unwrap();
        let has_rtl = Command::new("iverilog").arg("-V").output().is_ok();
        assert!(
            has_rtl || std::env::var_os("BITLOOM_REQUIRE_RTL").is_none(),
            "iverilog required"
        );
        if has_rtl {
            run_iverilog(&dir, "direct.v");
        } else {
            eprintln!("mixed-width RTL not run: missing iverilog");
        }
        if let Some(tool_dir) = std::env::var_os("RHDL_FIRTOOL_PATH") {
            fs::write(dir.join("design.fir"), fir).unwrap();
            let output = Command::new(std::path::Path::new(&tool_dir).join("firtool"))
                .arg(dir.join("design.fir"))
                .args([
                    "--verilog",
                    "--disable-all-randomization",
                    "--lowering-options=disallowLocalVariables",
                    "-o",
                ])
                .arg(dir.join("lowered.v"))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            run_iverilog(&dir, "lowered.v");
        } else {
            assert!(
                std::env::var_os("BITLOOM_REQUIRE_FIRRTL").is_none(),
                "firtool required"
            );
            eprintln!("mixed-width FIRRTL not run: missing RHDL_FIRTOOL_PATH");
        }
        fs::remove_dir_all(dir).unwrap();
    }
}

fn run_iverilog(dir: &std::path::Path, design: &str) {
    let output = Command::new("iverilog")
        .current_dir(dir)
        .args(["-g2012", "-s", "tb", "-o", "run", design, "tb.v"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        Command::new("vvp")
            .current_dir(dir)
            .arg("run")
            .status()
            .unwrap()
            .success()
    );
}

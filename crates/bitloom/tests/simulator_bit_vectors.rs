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
    s.add_output("signed_state", GroundType::SInt { width }, p);
    s.add_output("signed_count", GroundType::SInt { width }, p);
    s.add_output("eq", GroundType::Bool, p);
    s.add_output("ult", GroundType::Bool, p);
    s.add_output("slt", GroundType::Bool, p);
    s.add_output("zext", GroundType::UInt { width: 64 }, p);
    s.add_output("sext", GroundType::SInt { width: 64 }, p);
    s.add_output("sext_bits", GroundType::UInt { width: 64 }, p);
    s.add_output("zext_signed", GroundType::SInt { width: 64 }, p);
    s.declare_reg("state", GroundType::UInt { width }, p);
    s.declare_reg("signed_q", GroundType::SInt { width }, p);
    s.declare_reg("signed_counter", GroundType::SInt { width }, p);
    s.begin_sequential(p);
    s.assign_reg_d_inc("state", p);
    s.assign_reg_d_inc("signed_counter", p);
    s.assign_reg_d_from("signed_q", "signed_a", p);
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
        s.assign_sign_extend("sext_bits", "signed_a", 64, p);
        s.assign_zero_extend("zext_signed", "signed_a", 64, p);
    } else {
        s.assign_net("zext", "a", p);
        s.assign_net("sext", "signed_a", p);
        s.assign_net("sext_bits", "a", p);
        s.assign_net("zext_signed", "signed_a", p);
    }
    s.assign_eq("eq", "a", "b", p);
    s.assign_net("count", "state", p);
    s.assign_net("signed_state", "signed_q", p);
    s.assign_net("signed_count", "signed_counter", p);
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
            "signed_state",
            "signed_count",
        ] {
            rtl += &format!("wire [{}:0] {n};\n", width - 1);
        }
        rtl += "wire eq, ult, slt; wire [63:0] zext, sext, sext_bits, zext_signed; Vectors dut(.*); initial begin\n";
        let mut count = 0;
        for reset_input in [1, 0, 3, 2] {
            let reset = reset_input & 1 != 0;
            for a in [0, 1, mask >> 1, mask] {
                for shift in [
                    0,
                    1,
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
                        ("signed_state", if reset { 0 } else { a }),
                        ("sum", sum),
                        ("after_sum", if shift >= 64 { 0 } else { sum >> shift }),
                        ("diff", a.wrapping_sub(b) & mask),
                        ("and_o", a & b),
                        ("or_o", a | b),
                        ("xor_o", a ^ b),
                        ("eq", (a == b) as u64),
                        ("count", count),
                        ("signed_count", count),
                        ("ult", (a < b) as u64),
                        (
                            "slt",
                            ((((a << (64 - width)) as i64) >> (64 - width))
                                < (((b << (64 - width)) as i64) >> (64 - width)))
                                as u64,
                        ),
                        ("zext", a),
                        ("sext", ((a << (64 - width)) as i64 >> (64 - width)) as u64),
                        (
                            "sext_bits",
                            ((a << (64 - width)) as i64 >> (64 - width)) as u64,
                        ),
                        ("zext_signed", a),
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
        export_chisel_case(&h, &rtl, &format!("same_{width}"));
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
        export_chisel_case(
            &h,
            &tb,
            &format!("mixed_{value_width}_{shift_width}_{output_width}"),
        );
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

// The strict script compiles all cases in one JVM, then executes each emitted RTL
// testbench. The exact same independently computed vectors drive every backend.
fn export_chisel_case(hir: &FrozenHir, tb: &str, name: &str) {
    let Some(root) = std::env::var_os("BITLOOM_CHISEL_CASES") else {
        return;
    };
    let dir = std::path::Path::new(&root).join(name);
    fs::create_dir_all(&dir).unwrap();
    let scala = bitloom_firrtl::emit_chisel(hir).unwrap().files[0]
        .contents
        .clone();
    fs::write(dir.join("Design.scala"), format!("package {name}\n{scala}")).unwrap();
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == hir.abi_name)
        .unwrap_or(&hir.circuit().modules[0]);
    let connects = top
        .ports
        .iter()
        .map(|p| {
            let port = match p.name.as_str() {
                "clk" => "clock".into(),
                "rst" => "reset".into(),
                n => format!("io_{n}"),
            };
            format!(".{port}({})", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let tb = tb.replace("dut(.*)", &format!("dut({connects})"));
    fs::write(dir.join("tb.v"), tb).unwrap();
    fs::write(dir.join("top.txt"), &top.name).unwrap();
}

#[test]
fn reverse_dag_feedback_and_simultaneous_registers() {
    const SEED: u64 = 0x91b3_20d4;
    let p = Span::default();
    let mut s = ElaborateSession::new("boundaries");
    s.begin_module("Boundaries", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("load", GroundType::Bool, p);
    for name in ["a", "b"] {
        s.add_input(name, GroundType::UInt { width: 8 }, p);
    }
    for name in ["out", "acc", "first", "second"] {
        s.add_output(name, GroundType::UInt { width: 8 }, p);
    }
    for name in ["q", "x", "y"] {
        s.declare_reg(name, GroundType::UInt { width: 8 }, p);
    }
    s.declare_wire("next", GroundType::UInt { width: 8 }, p);
    let mut seed = SEED;
    let mut deps = Vec::new();
    for i in 0..24 {
        s.declare_wire(format!("n{i}"), GroundType::UInt { width: 8 }, p);
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        deps.push(if i == 0 {
            None
        } else {
            Some((seed as usize) % i)
        });
    }
    s.begin_sequential(p);
    s.assign_reg_d_from("q", "next", p);
    s.assign_reg_d_mux("x", "load", "a", "y", p);
    s.assign_reg_d_mux("y", "load", "b", "x", p);
    s.end_process();
    s.begin_combinational(p);
    s.assign_net("out", "n23", p);
    s.assign_net("acc", "q", p);
    s.assign_net("first", "x", p);
    s.assign_net("second", "y", p);
    s.assign_add("next", "q", "a", p);
    for i in (0..24).rev() {
        let from = deps[i].map(|d| format!("n{d}")).unwrap_or("a".into());
        if i % 2 == 0 {
            s.assign_add(format!("n{i}"), from, "b", p);
        } else {
            s.assign_xor(format!("n{i}"), from, "a", p);
        }
    }
    s.end_process();
    s.end_module();
    let hir = s.finish().unwrap();
    let mut state = [0u64; 3];
    let mut vectors = Vec::new();
    for cycle in 0..40u64 {
        let reset = cycle == 0 || cycle == 19;
        let load = cycle % 7 == 1;
        let a = (cycle * 73 + 3) & 255;
        let b = (cycle * 31 + 9) & 255;
        state = if reset {
            [0; 3]
        } else {
            [
                (state[0] + a) & 255,
                if load { a } else { state[2] },
                if load { b } else { state[1] },
            ]
        };
        let mut nodes = [0; 24];
        for i in 0..24 {
            let v = deps[i].map(|d| nodes[d]).unwrap_or(a);
            nodes[i] = if i % 2 == 0 { (v + b) & 255 } else { v ^ a };
        }
        let mut input = PortValues::default();
        for (n, v) in [
            ("rst", reset as u64),
            ("load", load as u64),
            ("a", a),
            ("b", b),
        ] {
            input.set(n, v);
        }
        let mut expected = PortValues::default();
        for (n, v) in [
            ("out", nodes[23]),
            ("acc", state[0]),
            ("first", state[1]),
            ("second", state[2]),
        ] {
            expected.set(n, v);
        }
        vectors.push((input, expected));
    }
    verify_boundary_models(&hir, &vectors, &format!("dag-seed-{SEED:x}"), true);
}

fn verify_boundary_models(
    hir: &FrozenHir,
    vectors: &[(PortValues, PortValues)],
    label: &str,
    rtl: bool,
) {
    verify_boundary_models_with_rtl_tail(hir, vectors, label, rtl, None);
}

fn verify_boundary_models_with_rtl_tail(
    hir: &FrozenHir,
    vectors: &[(PortValues, PortValues)],
    label: &str,
    rtl: bool,
    rtl_tail: Option<&str>,
) {
    let mut functional = GeneratedFunctional::from_hir(hir);
    let mut sims = [
        Sim::new(hir.clone()),
        Sim::with_engine(hir.clone(), TickEngine::Compiled),
    ];
    for (cycle, (input, expected)) in vectors.iter().enumerate() {
        let result = functional.cycle(input);
        for (n, v) in &expected.values {
            assert_eq!(
                result.get(n),
                Some(*v),
                "{label} functional cycle={cycle} input={input:?} net={n}"
            );
        }
        for sim in &mut sims {
            sim.set_inputs(input.clone());
            sim.tick();
            for (n, v) in &expected.values {
                assert_eq!(
                    sim.ports().get(n),
                    Some(*v),
                    "{label} {:?} cycle={cycle} input={input:?} net={n}",
                    sim.engine()
                );
            }
        }
    }
    verify_functional_crate(hir, vectors);
    let dir = temporary_dir(label);
    bitloom_sim::generate_cycle_accurate_sim(hir, &dir).unwrap();
    let mut source = format!(
        "#[test] fn vectors() {{ let mut s = bitloom_cycle_{}::CycleAccurate::new();\n",
        hir.abi_name.to_lowercase()
    );
    for (input, expected) in vectors {
        source += "let mut p = bitloom_hir::PortValues::default();\n";
        for (n, v) in &input.values {
            source += &format!("p.set({n:?},{v});\n");
        }
        source += "let o=s.cycle(&p);\n";
        for (n, v) in &expected.values {
            source += &format!("assert_eq!(o.get({n:?}),Some({v}));\n");
        }
    }
    source += "}\n";
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(dir.join("tests/vectors.rs"), source).unwrap();
    let result = Command::new("cargo")
        .args(["test", "--offline", "--quiet", "--manifest-path"])
        .arg(dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{label} generated cycle crate {}\n{}",
        dir.display(),
        String::from_utf8_lossy(&result.stderr)
    );
    if rtl {
        let m = &hir.circuit().modules[0];
        let mut tb = String::from("module tb;\n");
        for port in &m.ports {
            let direction = if port.direction == bitloom_hir::PortDirection::Input {
                "reg"
            } else {
                "wire"
            };
            let width = match port.ty {
                GroundType::UInt { width } | GroundType::SInt { width } => width,
                _ => 1,
            };
            tb += &format!("{direction} [{}:0] {};\n", width - 1, port.name);
        }
        tb += &format!("{} dut(.*); initial begin clk=0;\n", m.name);
        for (cycle, (input, expected)) in vectors.iter().enumerate() {
            for (n, v) in &input.values {
                tb += &format!("{n}=64'h{v:x};\n");
            }
            tb += "#5; clk=1; #5;\n";
            for (n, v) in &expected.values {
                tb += &format!(
                    "if ({n} !== 64'h{v:x}) $fatal(1, \"{label} cycle={cycle} net={n}\");\n"
                );
            }
            tb += "clk=0;\n";
        }
        if let Some(tail) = rtl_tail {
            tb += tail;
        }
        tb += "$finish; end endmodule\n";
        export_chisel_case(hir, &tb, &label.replace('-', "_"));
        fs::write(dir.join("tb.v"), tb).unwrap();
        fs::write(
            dir.join("design.v"),
            &bitloom_vlog::emit(hir).files[0].contents,
        )
        .unwrap();
        if Command::new("iverilog").arg("-V").output().is_ok() {
            run_iverilog(&dir, "design.v");
        } else {
            assert!(
                std::env::var_os("BITLOOM_REQUIRE_RTL").is_none(),
                "iverilog required"
            );
        }
        // FIRRTL does not yet preserve the asynchronous reset metadata.
        // The asynchronous probe below targets direct Verilog and Chisel.
        if !label.starts_with("memory_") && rtl_tail.is_none() {
            verify_firrtl_rtl(hir, &dir);
        }
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn memory_read_before_write_is_order_independent_in_internal_models() {
    for sync in [false, true] {
        for write_first in [false, true] {
            let p = Span::default();
            let mut s = ElaborateSession::new("memory_boundary");
            s.begin_module("MemoryBoundary", p);
            s.add_input("clk", GroundType::Clock, p);
            s.add_input("rst", GroundType::Reset, p);
            s.add_input("addr", GroundType::UInt { width: 2 }, p);
            s.add_input("data", GroundType::UInt { width: 8 }, p);
            s.add_input("we", GroundType::Bool, p);
            s.add_input("en", GroundType::Bool, p);
            s.add_output("out", GroundType::UInt { width: 8 }, p);
            if sync {
                s.declare_sync_read_mem("ram", 4, 8, p);
            } else {
                s.declare_mem("ram", 4, 8, p);
            }
            s.declare_reg_ex("q", GroundType::UInt { width: 8 }, true, true, p);
            s.begin_sequential(p);
            if write_first {
                s.assign_mem_write_en("ram", "addr", "data", "we", p);
            }
            s.assign_reg_d_mem_read("q", "ram", "addr", p);
            if !write_first {
                s.assign_mem_write_en("ram", "addr", "data", "we", p);
            }
            s.end_process();
            s.begin_combinational(p);
            s.assign_net("out", "q", p);
            s.end_process();
            s.end_module();
            let hir = s.finish().unwrap();
            let mut bank = [0u64; 4];
            let mut q = 0;
            let mut pending = None;
            let mut vectors = Vec::new();
            for cycle in 0..32u64 {
                let reset = cycle == 0 || cycle == 17;
                let addr = (cycle / 3 % 4) as usize;
                let data = (cycle * 13) & 255;
                let we = cycle % 3 != 2;
                let en = cycle % 5 != 3;
                let read = bank[addr];
                if reset {
                    q = 0;
                } else if sync {
                    if en {
                        if let Some(v) = pending {
                            q = v;
                        }
                    }
                } else if en {
                    q = read;
                }
                if sync {
                    pending = Some(read);
                }
                if !reset && we {
                    bank[addr] = data;
                }
                let mut input = PortValues::default();
                for (n, v) in [
                    ("rst", reset as u64),
                    ("addr", addr as u64),
                    ("data", data),
                    ("we", we as u64),
                    ("en", en as u64),
                ] {
                    input.set(n, v);
                }
                let mut expected = PortValues::default();
                expected.set("out", q);
                vectors.push((input, expected));
            }
            verify_boundary_models(
                &hir,
                &vectors,
                &format!("memory-sync{sync}-writefirst{write_first}"),
                false,
            );
        }
    }
}

#[test]
fn hierarchy_is_compiled_but_simulation_rejects_every_module_order() {
    let child = "  module Child :\n    input clk: Clock\n    input rst: Reset\n    input x: UInt<8>\n    output y: UInt<8>\n    regreset q: UInt<8>, clk, rst, UInt(0)\n    q <= x\n    y <= q\n";
    let parent = "  module Hierarchy :\n    input clk: Clock\n    input rst: Reset\n    input x: UInt<8>\n    output y: UInt<8>\n    wire child_y: UInt<8>\n    regreset q: UInt<8>, clk, rst, UInt(0)\n    inst u of Child\n    u.clk <= clk\n    u.rst <= rst\n    u.x <= x\n    child_y <= u.y\n    q <= child_y\n    y <= q\n";
    for (index, body) in [format!("{child}{parent}"), format!("{parent}{child}")]
        .iter()
        .enumerate()
    {
        let hir = bitloom_firrtl::import(&format!(
            "FIRRTL version 6.0.0\ncircuit Hierarchy :\n{body}"
        ))
        .unwrap();
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            assert!(
                std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine)).is_err(),
                "hierarchy must not silently execute the first module"
            );
        }
        assert!(std::panic::catch_unwind(|| GeneratedFunctional::from_hir(&hir)).is_err());
        let dir = temporary_dir("hierarchy");
        assert!(generate_functional_sim(&hir, &dir).is_err());
        assert!(bitloom_sim::generate_cycle_accurate_sim(&hir, &dir).is_err());
        fs::create_dir_all(&dir).unwrap();
        let mut tb = String::from(
            "module tb; reg clk=0,rst=1; reg [7:0] x=0; wire [7:0] y; Hierarchy dut(.*); initial begin #5;clk=1;#5;clk=0;rst=0;\n",
        );
        for i in 1..10 {
            let expected = if i == 1 { 0 } else { (i - 1) * 17 };
            tb += &format!(
                "x={};#5;clk=1;#5;if(y!==8'd{expected}) $fatal(1,\"hierarchy order={index} cycle={i}\");clk=0;\n",
                i * 17
            );
        }
        tb += "$finish;end endmodule\n";
        export_chisel_case(&hir, &tb, &format!("hierarchy_{index}"));
        fs::write(dir.join("tb.v"), tb).unwrap();
        fs::write(
            dir.join("design.v"),
            &bitloom_vlog::emit(&hir).files[0].contents,
        )
        .unwrap();
        if Command::new("iverilog").arg("-V").output().is_ok() {
            run_iverilog(&dir, "design.v");
        } else {
            assert!(std::env::var_os("BITLOOM_REQUIRE_RTL").is_none());
        }
        verify_firrtl_rtl(&hir, &dir);
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn noncolliding_memory_edges_match_independent_pipeline_and_rtl() {
    for sync in [true, false] {
        let p = Span::default();
        let mut s = ElaborateSession::new("memory_edges");
        s.begin_module("MemoryEdges", p);
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        for n in ["rd", "wr"] {
            s.add_input(n, GroundType::UInt { width: 2 }, p);
        }
        s.add_input("data", GroundType::UInt { width: 8 }, p);
        s.add_input("we", GroundType::Bool, p);
        s.add_input("en", GroundType::Bool, p);
        // User names collide with both naive target-name and ordinal read-stage
        // temporaries. The emitter must reserve every user declaration.
        s.add_input("__bitloom_read_q", GroundType::Bool, p);
        s.add_input("__bitloom_read_1", GroundType::Bool, p);
        for n in ["out", "always_out", "downstream"] {
            s.add_output(n, GroundType::UInt { width: 8 }, p);
        }
        if sync {
            s.declare_sync_read_mem("__bitloom_read_q_", 4, 8, p);
        } else {
            s.declare_mem("__bitloom_read_q_", 4, 8, p);
        }
        s.declare_reg_ex("q", GroundType::UInt { width: 8 }, false, true, p);
        s.declare_reg("plain", GroundType::UInt { width: 8 }, p);
        s.declare_reg("q_", GroundType::UInt { width: 8 }, p);
        s.declare_wire("__bitloom_read_1_", GroundType::UInt { width: 8 }, p);
        s.begin_sequential(p);
        s.assign_mem_write_en("__bitloom_read_q_", "wr", "data", "we", p);
        s.assign_reg_d_mem_read("q", "__bitloom_read_q_", "rd", p);
        s.assign_reg_d_mem_read("plain", "__bitloom_read_q_", "rd", p);
        s.assign_reg_d_from("q_", "q", p);
        s.end_process();
        s.begin_combinational(p);
        s.assign_lit("__bitloom_read_1_", 0, p);
        s.assign_net("out", "q", p);
        s.assign_net("always_out", "plain", p);
        s.assign_net("downstream", "q_", p);
        s.end_process();
        s.end_module();
        let hir = s.finish().unwrap();
        let mut bank = [0u64; 4];
        let mut stage = 0;
        let mut q = 0;
        let mut vectors = Vec::new();
        for cycle in 0..36u64 {
            let reset = cycle == 0 || cycle == 15;
            let en = cycle % 4 != 2;
            let we = cycle % 3 != 2;
            let wr = (cycle % 4) as usize;
            let rd = (wr + 2) % 4;
            let data = (cycle * 17) & 255;
            let read = bank[rd];
            let sampled = if sync { stage } else { read };
            let old_q = q;
            q = if reset {
                0
            } else if en {
                sampled
            } else {
                q
            };
            let plain = if reset { 0 } else { sampled };
            let down = if reset { 0 } else { old_q };
            stage = read;
            if !reset && we {
                bank[wr] = data;
            }
            let mut input = PortValues::default();
            for (n, v) in [
                ("__bitloom_read_q", 0),
                ("__bitloom_read_1", 0),
                ("rst", reset as u64),
                ("en", en as u64),
                ("we", we as u64),
                ("wr", wr as u64),
                ("rd", rd as u64),
                ("data", data),
            ] {
                input.set(n, v);
            }
            // RTL memories start undefined. Ignore outputs until every address
            // has been written and both pipeline stages have filled.
            let mut expected = PortValues::default();
            if cycle > 12 {
                for (n, v) in [("out", q), ("always_out", plain), ("downstream", down)] {
                    expected.set(n, v);
                }
            }
            vectors.push((input, expected));
        }
        verify_boundary_models(&hir, &vectors, &format!("memory_edges_sync{sync}"), true);
    }
}

fn verify_firrtl_rtl(hir: &FrozenHir, dir: &std::path::Path) {
    let Some(tools) = std::env::var_os("RHDL_FIRTOOL_PATH") else {
        assert!(
            std::env::var_os("BITLOOM_REQUIRE_FIRRTL").is_none(),
            "firtool required"
        );
        eprintln!("FIRRTL boundary RTL not executed: RHDL_FIRTOOL_PATH unset");
        return;
    };
    fs::write(
        dir.join("design.fir"),
        &bitloom_firrtl::emit(hir).files[0].contents,
    )
    .unwrap();
    let output = Command::new(std::path::Path::new(&tools).join("firtool"))
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
        "firtool={:?} artifacts={}\n{}",
        tools,
        dir.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    run_iverilog(dir, "lowered.v");
}

#[test]
fn recursive_hierarchy_without_top_returns_diagnostic() {
    let source = "FIRRTL version 6.0.0\ncircuit Missing :\n  module Loop :\n    input clk: Clock\n    input rst: Reset\n    inst again of Loop\n    again.clk <= clk\n    again.rst <= rst\n";
    let result = bitloom_firrtl::import(source);
    assert!(
        result
            .unwrap_err()
            .0
            .iter()
            .any(|d| d.code == "rhdl::E0002")
    );
}

#[test]
fn last_effective_net_assignment_precedes_dependency_ordering() {
    for feedback in [false, true] {
        let p = Span::default();
        let mut s = ElaborateSession::new("LastNet");
        s.begin_module("LastNet", p);
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        for n in ["data_in", "other"] {
            s.add_input(n, GroundType::UInt { width: 8 }, p);
        }
        for n in ["a", "b"] {
            s.add_output(n, GroundType::UInt { width: 8 }, p);
        }
        s.begin_combinational(p);
        s.assign_net("a", "b", p);
        s.assign_net("a", "data_in", p);
        s.assign_net("b", if feedback { "a" } else { "other" }, p);
        s.end_process();
        s.end_module();
        let hir = s.finish().unwrap();
        let vectors: Vec<_> = [(3, 11), (87, 19), (255, 0)]
            .into_iter()
            .map(|(value, other)| {
                let mut input = PortValues::default();
                for (n, v) in [("rst", 0), ("data_in", value), ("other", other)] {
                    input.set(n, v);
                }
                let mut expected = PortValues::default();
                expected.set("a", value);
                expected.set("b", if feedback { value } else { other });
                (input, expected)
            })
            .collect();
        verify_boundary_models(&hir, &vectors, &format!("last_net_{feedback}"), true);
    }
}

#[test]
fn repeated_reg_reads_and_memory_overwrites_keep_last_driver_and_all_writes() {
    for sync in [false, true] {
        let p = Span::default();
        let mut s = ElaborateSession::new("LastReg");
        s.begin_module("LastReg", p);
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        for n in ["r0", "r1", "w0", "w1"] {
            s.add_input(n, GroundType::UInt { width: 2 }, p);
        }
        for n in ["data", "other"] {
            s.add_input(n, GroundType::UInt { width: 8 }, p);
        }
        for n in ["oread", "omem", "oref"] {
            s.add_output(n, GroundType::UInt { width: 8 }, p);
        }
        for n in ["qread", "qmem", "qref"] {
            s.declare_reg(n, GroundType::UInt { width: 8 }, p);
        }
        if sync {
            s.declare_sync_read_mem("bank", 4, 8, p);
        } else {
            s.declare_mem("bank", 4, 8, p);
        }
        s.begin_sequential(p);
        s.assign_reg_d_mem_read("qread", "bank", "r0", p);
        s.assign_reg_d_mem_read("qread", "bank", "r1", p);
        s.assign_reg_d_from("qmem", "data", p);
        s.assign_reg_d_mem_read("qmem", "bank", "r0", p);
        s.assign_reg_d_mem_read("qref", "bank", "r1", p);
        s.assign_reg_d_from("qref", "data", p);
        s.assign_mem_write("bank", "w0", "data", p);
        s.assign_mem_write("bank", "w1", "other", p);
        s.end_process();
        s.begin_combinational(p);
        for (out, reg) in [("oread", "qread"), ("omem", "qmem"), ("oref", "qref")] {
            s.assign_net(out, reg, p);
        }
        s.end_process();
        s.end_module();
        let hir = s.finish().unwrap();
        let writes = hir.circuit().modules[0]
            .body
            .iter()
            .filter_map(|s| {
                if let bitloom_hir::Stmt::Process(p) = s {
                    Some(p)
                } else {
                    None
                }
            })
            .flat_map(|p| &p.assigns)
            .filter(|a| matches!(a.target, bitloom_hir::AssignTarget::MemWrite { .. }))
            .count();
        assert_eq!(
            writes, 2,
            "normalization must preserve both write side effects"
        );
        let mut bank = [0; 4];
        let mut pending = [0; 2];
        let mut vectors = Vec::new();
        for cycle in 0..20u64 {
            let reset = cycle == 0 || cycle == 12;
            let w0 = cycle as usize % 4;
            let w1 = (w0 + 1) % 4;
            let r0 = (w0 + 2) % 4;
            let r1 = (w0 + 3) % 4;
            let data = cycle * 7;
            let other = cycle * 11;
            let sampled = [bank[r1], bank[r0]];
            let outputs = if reset {
                [0; 3]
            } else {
                [
                    if sync { pending[0] } else { sampled[0] },
                    if sync { pending[1] } else { sampled[1] },
                    data,
                ]
            };
            pending = sampled;
            if !reset {
                bank[w0] = data;
                bank[w1] = other;
            }
            let mut input = PortValues::default();
            for (n, v) in [
                ("rst", reset as u64),
                ("data", data),
                ("other", other),
                ("r0", r0 as u64),
                ("r1", r1 as u64),
                ("w0", w0 as u64),
                ("w1", w1 as u64),
            ] {
                input.set(n, v);
            }
            let mut expected = PortValues::default();
            if cycle >= 6 {
                for (n, v) in [
                    ("oread", outputs[0]),
                    ("omem", outputs[1]),
                    ("oref", outputs[2]),
                ] {
                    expected.set(n, v);
                }
            }
            vectors.push((input, expected));
        }
        verify_boundary_models(&hir, &vectors, &format!("memory_last_reg_{sync}"), true);
    }
}

#[test]
fn long_reversed_chain_constructs_and_evaluates() {
    const LENGTH: usize = 4096;
    let p = Span::default();
    let mut s = ElaborateSession::new("LongChain");
    s.begin_module("LongChain", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("data_in", GroundType::UInt { width: 8 }, p);
    s.add_output("out", GroundType::UInt { width: 8 }, p);
    for i in 0..LENGTH {
        s.declare_wire(format!("n{i}"), GroundType::UInt { width: 8 }, p);
    }
    s.begin_combinational(p);
    s.assign_net("out", format!("n{}", LENGTH - 1), p);
    for i in (0..LENGTH).rev() {
        s.assign_net(
            format!("n{i}"),
            if i == 0 {
                "data_in".into()
            } else {
                format!("n{}", i - 1)
            },
            p,
        );
    }
    s.end_process();
    s.end_module();
    let hir = s.finish().unwrap();
    let mut input = PortValues::default();
    input.set("data_in", 37);
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut sim = Sim::with_engine(hir.clone(), engine);
        sim.set_inputs(input.clone());
        sim.tick();
        assert_eq!(sim.ports().get("out"), Some(37));
    }
    assert_eq!(
        GeneratedFunctional::from_hir(&hir).cycle(&input).get("out"),
        Some(37)
    );
}

#[test]
fn asynchronous_reset_clears_rtl_without_a_clock_edge() {
    let p = Span::default();
    let mut s = ElaborateSession::new("AsyncReset");
    s.begin_module("AsyncReset", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    s.add_input("data", GroundType::UInt { width: 8 }, p);
    s.add_output("out", GroundType::UInt { width: 8 }, p);
    s.declare_reg_ex("q", GroundType::UInt { width: 8 }, true, false, p);
    s.begin_sequential(p);
    s.assign_reg_d_from("q", "data", p);
    s.end_process();
    s.begin_combinational(p);
    s.assign_net("out", "q", p);
    s.end_process();
    s.end_module();
    let hir = s.finish().unwrap();
    let vectors: Vec<_> = [(1, 0, 0), (0, 0xa5, 0xa5)]
        .into_iter()
        .map(|(reset, data, out)| {
            let mut input = PortValues::default();
            input.set("rst", reset);
            input.set("data", data);
            let mut expected = PortValues::default();
            expected.set("out", out);
            (input, expected)
        })
        .collect();
    verify_boundary_models_with_rtl_tail(
        &hir,
        &vectors,
        "async_reset",
        true,
        Some(
            "// Clock stays low throughout reset assertion.\n#2; if (out !== 8'ha5) $fatal(1, \"load failed\"); rst=1; #2; if (clk !== 0 || out !== 0) $fatal(1, \"async reset did not clear before edge\");\n",
        ),
    );
}

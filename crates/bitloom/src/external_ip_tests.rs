use super::*;

fn adapter_source() -> String {
    [
        "module fifo_v3 #(\n",
        "  parameter int DATA_WIDTH = 32,\n",
        "  parameter int FifoDepth = 8,\n",
        "  parameter type dtype                = logic [DATA_WIDTH-1:0],\n",
        ") (\n",
        "  input  dtype  data_i,\n",
        "  output dtype  data_o,\n",
        ");\n",
        "  dtype [FifoDepth - 1:0] mem_n, mem_q;\n",
        "  always_comb begin\n",
        "    mem_n           = mem_q;\n",
        "  end\n",
        "  always_ff @(posedge clk_i) begin\n",
        "    if (!rst_ni) mem_q <= {FifoDepth{dtype'('0)}};\n",
        "    else mem_q <= mem_n;\n",
        "  end\n",
        "endmodule\n",
    ]
    .concat()
}

fn closure_fixture() -> (Manifest, Vec<SourceLock>) {
    let manifest: Manifest = serde_json::from_str(include_str!(
        "../../../ip/external/pulp-common-cells-fifo-v3.source.json"
    ))
    .expect("canonical manifest parses");
    let lock: LockFile = serde_json::from_str(include_str!(
        "../../../ip/external/pulp-common-cells-fifo-v3.source.lock.json"
    ))
    .expect("canonical lock parses");
    (manifest, lock.sources)
}

#[test]
fn p0_adapter_transforms_the_exact_locked_shape() {
    let adapted = adapt_fifo_for_yosys(&adapter_source()).expect("exact shape adapts");
    for expected in [
        "// dtype fixed by bitloom-yosys-sv-compat v1",
        "input  logic [DATA_WIDTH-1:0] data_i,",
        "output logic [DATA_WIDTH-1:0] data_o,",
        "logic [DATA_WIDTH-1:0] mem_n [FifoDepth - 1:0], mem_q [FifoDepth - 1:0];",
        "for (int unsigned i = 0; i < FifoDepth; i++) mem_n[i] = mem_q[i];",
        "for (int unsigned i = 0; i < FifoDepth; i++) mem_q[i] <= '0;",
        "for (int unsigned i = 0; i < FifoDepth; i++) mem_q[i] <= mem_n[i];",
    ] {
        assert!(
            adapted.contains(expected),
            "missing transformed fragment {expected:?}"
        );
    }
    assert!(!adapted.contains("parameter type dtype"));
}

#[test]
fn p0_adapter_rejects_missing_and_duplicated_fragments() {
    let source = adapter_source();
    for fragment in [
        "parameter type dtype                = logic [DATA_WIDTH-1:0],",
        "input  dtype  data_i,",
        "output dtype  data_o,",
        "dtype [FifoDepth - 1:0] mem_n, mem_q;",
        "mem_n           = mem_q;",
        "mem_q <= {FifoDepth{dtype'('0)}};",
        "mem_q <= mem_n;",
    ] {
        let missing = source.replacen(fragment, "shape drift", 1);
        let error = adapt_fifo_for_yosys(&missing).expect_err("missing fragment must fail");
        assert!(error.contains("bitloom.external-ip.adapter"), "{error}");

        let duplicated = format!("{source}\n{fragment}\n");
        let error = adapt_fifo_for_yosys(&duplicated).expect_err("duplicate fragment must fail");
        assert!(error.contains("bitloom.external-ip.adapter"), "{error}");
    }
}

#[test]
fn p0_relative_path_validation_rejects_escape_and_ambiguity() {
    for invalid in ["", "/absolute", "../escape", "nested/../escape", "./local"] {
        let error = validate_relative(invalid, "test").expect_err("unsafe path must fail");
        assert!(error.contains("bitloom.external-ip.path"), "{error}");
    }
    for valid in ["src/fifo_v3.sv", "include/common_cells", "common_cells"] {
        validate_relative(valid, "test").expect("normal relative path is accepted");
    }
}

#[test]
fn p0_yosys_argument_validation_rejects_metacharacters() {
    for invalid in [
        "/tmp/a b.sv",
        "/tmp/a;check.sv",
        "/tmp/a$HOME.sv",
        "/tmp/a\\b.sv",
        "/tmp/a\"b.sv",
        "/tmp/a\nb.sv",
    ] {
        let error = yosys_path(Path::new(invalid)).expect_err("unsafe Yosys path must fail");
        assert!(error.contains("bitloom.external-ip.compile"), "{error}");
    }
    assert_eq!(
        yosys_path(Path::new("/tmp/cache-1/fifo_v3.yosys.sv")).unwrap(),
        "/tmp/cache-1/fifo_v3.yosys.sv"
    );
}

#[test]
fn p0_closure_digest_binds_all_identity_dimensions() {
    let (manifest, sources) = closure_fixture();
    let baseline = closure_digest(&manifest, &sources);
    assert_eq!(baseline, sources[0].closure_digest);

    let (manifest, mut changed) = closure_fixture();
    changed[0].url.push_str("/mirror");
    assert_ne!(baseline, closure_digest(&manifest, &changed));

    let (manifest, mut changed) = closure_fixture();
    changed[0].r#ref = "v1.40.1".into();
    assert_ne!(baseline, closure_digest(&manifest, &changed));

    let (manifest, mut changed) = closure_fixture();
    changed[0].commit = "0000000000000000000000000000000000000000".into();
    assert_ne!(baseline, closure_digest(&manifest, &changed));

    let (manifest, mut changed) = closure_fixture();
    changed[0].tree_digest = "00".repeat(32);
    assert_ne!(baseline, closure_digest(&manifest, &changed));

    let (mut changed_manifest, sources) = closure_fixture();
    changed_manifest.dependencies[0].declared_version = "9.9.9".into();
    assert_ne!(baseline, closure_digest(&changed_manifest, &sources));
}

#[test]
fn p0_bender_dependency_parser_is_exact_and_section_scoped() {
    let inline = "package:\n  name: demo\ndependencies:\n  child: { git: \"https://example.test/child.git\", version: 0.2.0 } # pinned\nsources:\n  child: { git: \"https://evil.test/x.git\", version: 9 }\n";
    let block = "dependencies:\n  child:\n    git: 'https://example.test/child.git'\n    version: \"0.2.0\"\n";
    let baseline = parse_dependencies(inline).unwrap();
    assert_eq!(baseline, parse_dependencies(block).unwrap());
    assert_eq!(baseline["child"]["version"], "0.2.0");
    assert_ne!(
        baseline,
        parse_dependencies(&inline.replace("version: 0.2.0", "version: 0.2.01")).unwrap()
    );
    assert!(
        parse_dependencies(
            "sources:\n  child: { git: \"https://example.test/child.git\", version: 0.2.0 }\n"
        )
        .unwrap()
        .is_empty()
    );
    let extra = inline.replace(
        "sources:",
        "  extra: { git: \"https://example.test/extra.git\", version: 1 }\nsources:",
    );
    assert_ne!(baseline, parse_dependencies(&extra).unwrap());
    for invalid in [
        "dependencies:\n  child: { git: x, version: 1, version: 2 }\n",
        "dependencies:\n  child: { git: x, version: 1, rev: 2 }\n",
        "dependencies:\n  child: { git: x, version: 1 }\n  child: { git: x, version: 1 }\n",
        "dependencies:\n  child: { git: x, version: 1 }\ndependencies: {}\n",
        "dependencies:\n  child: &alias\n",
        "dependencies:\n  child:\n    git: x\n    version: 1\n    nested:\n      git: x\n",
    ] {
        assert!(
            parse_dependencies(invalid)
                .unwrap_err()
                .contains("bitloom.external-ip.dependency"),
            "{invalid}"
        );
    }
}

#[test]
fn p0_compile_contract_rejects_unsupported_shape_parameter_and_reset() {
    let (manifest, _) = closure_fixture();
    let original = serde_json::to_value(&manifest.compile).unwrap();
    for (field, value) in [("DEPTH", "9"), ("DATA_WIDTH", "16"), ("FALL_THROUGH", "1")] {
        let mut changed = original.clone();
        changed["parameters"][field] = value.into();
        let compile = serde_json::from_value(changed).unwrap();
        assert!(
            validate_pilot_compile(&compile)
                .unwrap_err()
                .contains("compile-contract")
        );
    }
    for (field, value) in [
        ("resetKind", "synchronous"),
        ("resetPolarity", "active-high"),
        ("clockPort", "data_i"),
    ] {
        let mut changed = original.clone();
        changed["clockReset"][field] = value.into();
        assert!(
            validate_pilot_compile(&serde_json::from_value(changed).unwrap())
                .unwrap_err()
                .contains("compile-contract")
        );
    }
    for field in ["name", "direction", "width"] {
        let mut changed = original.clone();
        changed["ports"][0][field] = "drift".into();
        assert!(
            validate_pilot_compile(&serde_json::from_value(changed).unwrap())
                .unwrap_err()
                .contains("compile-contract")
        );
    }
}

#[test]
fn p0_source_default_guard_rejects_parameter_default_drift() {
    let source = [
        "`include \"common_cells/assertions.svh\"",
        "module fifo_v3 #(",
        "parameter bit          FALL_THROUGH = 1'b0,",
        "parameter int unsigned DATA_WIDTH   = 32,",
        "parameter int unsigned DEPTH        = 8,",
        "parameter type dtype                = logic [DATA_WIDTH-1:0],",
        "parameter int unsigned ADDR_DEPTH   = (DEPTH > 1) ? $clog2(DEPTH) : 1",
        "input  logic  clk_i",
        "input  logic  rst_ni",
        "input  logic  flush_i",
        "input  logic  testmode_i",
        "output logic  full_o",
        "output logic  empty_o",
        "output logic  [ADDR_DEPTH-1:0] usage_o",
        "input  dtype  data_i",
        "input  logic  push_i",
        "output dtype  data_o",
        "input  logic  pop_i",
    ]
    .join("\n");
    verify_pilot_source(&source).unwrap();
    for (from, to) in [("= 1'b0", "= 1'b1"), ("= 32,", "= 16,"), ("= 8,", "= 9,")] {
        assert!(
            verify_pilot_source(&source.replace(from, to))
                .unwrap_err()
                .contains("compile-contract")
        );
    }
}

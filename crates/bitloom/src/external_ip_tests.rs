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
fn p0_binding_is_real_frozen_hierarchy_and_preserves_emitted_parent() {
    let (manifest, _) = closure_fixture();
    let hir = fifo_wrapper_hir(&manifest.compile).unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    let emitted = bitloom_vlog::emit(&hir).files.remove(0).contents;
    let parent = emit_fifo_wrapper(&manifest.compile).unwrap();
    assert!(emitted.contains(parent.trim()));
    assert!(parent.contains("module BitloomExternalFifo ("));
    assert!(parent.contains("  fifo_v3 fifo ("));
    assert!(!parent.contains("module fifo_v3 ("));
    assert!(!parent.contains("assign "));
    for port in &manifest.compile.ports {
        assert!(parent.contains(&format!(".{}({})", port.name, port.name)));
    }
    assert_eq!(
        parent,
        link_fifo_wrapper(&hir, &emitted, &manifest.compile).unwrap()
    );
    for mutant in [
        String::new(),
        emitted.replace(".data_o(data_o)", ".data_o(data_i)"),
        emitted.replace("fifo_v3 fifo", "empty_stub fifo"),
        emitted.replace("endmodule", "assign data_o = 0;\nendmodule"),
        emitted.replace("input [31:0] data_i", "input [30:0] data_i"),
    ] {
        assert!(
            link_fifo_wrapper(&hir, &mutant, &manifest.compile)
                .unwrap_err()
                .contains("bitloom.external-ip.binding")
        );
    }
}

#[test]
fn p0_binding_rejects_unsupported_shape_parameter_and_reset() {
    let (manifest, _) = closure_fixture();
    let original = serde_json::to_value(&manifest.compile).unwrap();
    for (field, value) in [("DEPTH", "9"), ("DATA_WIDTH", "16"), ("FALL_THROUGH", "1")] {
        let mut changed = original.clone();
        changed["parameters"][field] = value.into();
        let compile = serde_json::from_value(changed).unwrap();
        assert!(
            emit_fifo_wrapper(&compile)
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
            emit_fifo_wrapper(&serde_json::from_value(changed).unwrap())
                .unwrap_err()
                .contains("compile-contract")
        );
    }
    for field in ["name", "direction", "width"] {
        let mut changed = original.clone();
        changed["ports"][0][field] = "drift".into();
        assert!(
            emit_fifo_wrapper(&serde_json::from_value(changed).unwrap())
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

#[test]
fn p0_binding_output_does_not_overwrite_protected_inputs() {
    let root = temp_path(&std::env::temp_dir(), "binding-protection");
    let cache = root.join("cache");
    fs::create_dir_all(&cache).unwrap();
    let manifest = root.join("manifest.json");
    let lock = root.join("lock.json");
    fs::write(&manifest, "manifest").unwrap();
    fs::write(&lock, "lock").unwrap();
    for path in [&manifest, &lock, &cache.join("out.json")] {
        assert!(
            guard_binding_output(path, &manifest, &lock, &cache)
                .unwrap_err()
                .contains("path")
        );
    }
    guard_binding_output(&root.join("binding.json"), &manifest, &lock, &cache).unwrap();
    #[cfg(unix)]
    {
        let link = root.join("symlink.json");
        std::os::unix::fs::symlink(&manifest, &link).unwrap();
        assert!(guard_binding_output(&link, &manifest, &lock, &cache).is_err());
    }
    assert_eq!(fs::read_to_string(&manifest).unwrap(), "manifest");
    assert_eq!(fs::read_to_string(&lock).unwrap(), "lock");
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "requires actual iverilog/vvp; run in the dedicated FR200 gate"]
fn p0_behavior_oracle_kills_empty_and_zero_output_wrappers() {
    let (manifest, _) = closure_fixture();
    let good = emit_fifo_wrapper(&manifest.compile).unwrap();
    let header_end = good.find(");").unwrap() + 2;
    let empty = format!("{}\nendmodule\n", &good[..header_end]);
    let zero = format!(
        "{}\nassign full_o=0; assign empty_o=1; assign usage_o=0; assign data_o=0;\nendmodule\n",
        &good[..header_end]
    );
    let iv = tool_path("iverilog").unwrap();
    let vvp = tool_path("vvp").unwrap();
    let (base, runtime) = iverilog_runtime(&iv).unwrap();
    for name in ["ivl", "ivlpp", "vvp.tgt", "vvp.conf"] {
        assert!(runtime.contains_key(&Path::new(&base).join(name).to_string_lossy().into_owned()));
    }
    for (name, mutant) in [("empty", empty), ("zero-output", zero)] {
        let error = simulate_fifo("", &mutant, Path::new("."), &iv, &base, &vvp)
            .expect_err("oracle must kill stub wrapper");
        assert!(error.contains("simulation failed"), "{error}");
        println!("wrapper_mutation={name} result=rejected diagnostic={error}");
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

struct CacheFixture {
    root: PathBuf,
    cache: PathBuf,
    path: PathBuf,
    lock: LockFile,
    closure: String,
    staging: Vec<String>,
}
impl CacheFixture {
    fn new(published: bool) -> Self {
        let root = temp_path(&std::env::temp_dir(), "cache-integrity");
        let cache = root.join("cache");
        let path = root.join("source.lock.json");
        let mut lock: LockFile = serde_json::from_str(include_str!(
            "../../../ip/external/pulp-common-cells-fifo-v3.source.lock.json"
        ))
        .unwrap();
        lock.sources.truncate(1);
        let source = &mut lock.sources[0];
        source.files = vec![FileLock {
            path: "LICENSE".into(),
            sha256: sha256(b"license"),
        }];
        source.tree_digest = tree_digest(&source.files);
        source.license.sha256 = sha256(b"license");
        source.license.notice_paths.clear();
        let closure = format!("closures/v1-{}", source.closure_digest);
        source.cache_path = format!(
            "{closure}/{}-{}-{}",
            source.name, source.commit, source.tree_digest
        );
        let staging = vec!["staged".into()];
        let tree = cache.join(if published {
            &source.cache_path
        } else {
            &staging[0]
        });
        fs::create_dir_all(&tree).unwrap();
        fs::write(tree.join("LICENSE"), b"license").unwrap();
        if published {
            fs::create_dir_all(cache.join(&staging[0])).unwrap();
            fs::write(cache.join(&staging[0]).join("LICENSE"), b"license").unwrap();
        }
        fs::create_dir_all(cache.join(".staging-licenses")).unwrap();
        fs::write(
            cache
                .join(".staging-licenses")
                .join(format!("{}-LICENSE", source.name)),
            b"license",
        )
        .unwrap();
        let archive = root.join(&source.license.archive_path);
        fs::create_dir_all(archive.parent().unwrap()).unwrap();
        fs::write(archive, b"license").unwrap();
        Self {
            root,
            cache,
            path,
            lock,
            closure,
            staging,
        }
    }
    fn publish(&self, already_locked: bool) -> Result<(), String> {
        finalize_lock(
            &self.lock,
            &self.staging,
            &self.cache,
            &self.closure,
            &self.path,
            &json_bytes(&self.lock).unwrap(),
            already_locked,
        )
    }
    fn verify(&self) -> Result<(), String> {
        verify_source(&self.lock.sources[0], &self.cache, &self.path, None)
    }
}
impl Drop for CacheFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn cache_integrity_license_failure_preserves_reused_closure() {
    for old_lock in [false, true] {
        let f = CacheFixture::new(true);
        f.verify().unwrap();
        let bytes = json_bytes(&f.lock).unwrap();
        if old_lock {
            fs::write(&f.path, &bytes).unwrap();
        }
        let archive = f.root.join(&f.lock.sources[0].license.archive_path);
        fs::write(&archive, b"conflicting archive").unwrap();
        assert!(f.publish(old_lock).unwrap_err().contains("license-drift"));
        assert_eq!(
            fs::read(f.cache.join(&f.lock.sources[0].cache_path).join("LICENSE")).unwrap(),
            b"license"
        );
        if old_lock {
            assert_eq!(fs::read(&f.path).unwrap(), bytes);
        } else {
            assert!(!f.path.exists());
        }
        fs::write(archive, b"license").unwrap();
        f.verify().unwrap();
    }
}

#[test]
fn cache_integrity_license_failure_removes_only_new_closure() {
    let f = CacheFixture::new(false);
    fs::write(
        f.root.join(&f.lock.sources[0].license.archive_path),
        b"conflict",
    )
    .unwrap();
    assert!(f.publish(false).unwrap_err().contains("license-drift"));
    assert!(!f.cache.join(&f.closure).exists());
    assert!(!f.path.exists());
}

#[test]
fn cache_integrity_new_lock_verification_failure_preserves_shared_closure() {
    let f = CacheFixture::new(true);
    let file = f.cache.join(&f.lock.sources[0].cache_path).join("LICENSE");
    fs::write(&file, b"preexisting drift").unwrap();
    assert!(f.publish(false).unwrap_err().contains("content-drift"));
    assert_eq!(fs::read(&file).unwrap(), b"preexisting drift");
    assert!(!f.path.exists());
}

#[test]
fn cache_integrity_valid_publication_and_reuse() {
    for published in [false, true] {
        let f = CacheFixture::new(published);
        f.publish(false).unwrap();
        f.verify().unwrap();
        assert_eq!(fs::read(&f.path).unwrap(), json_bytes(&f.lock).unwrap());
    }
}

#[cfg(unix)]
#[test]
fn cache_integrity_rejects_all_symlink_directory_levels() {
    for level in 0..=3 {
        let f = CacheFixture::new(true);
        f.verify().unwrap();
        let selected = match level {
            0 => f.cache.clone(),
            1 => f.cache.join("closures"),
            2 => f.cache.join(&f.closure),
            _ => f.cache.join(&f.lock.sources[0].cache_path),
        };
        let outside = f.root.join("outside");
        fs::rename(&selected, &outside).unwrap();
        std::os::unix::fs::symlink(&outside, &selected).unwrap();
        let error = f
            .verify()
            .expect_err("symlinked cache path must be rejected");
        assert!(error.contains("bitloom.external-ip.path"), "{error}");
        assert!(outside.exists());
    }
}

#[test]
fn cache_integrity_lock_write_failure_respects_cache_ownership() {
    for published in [false, true] {
        let f = CacheFixture::new(published);
        fs::create_dir(&f.path).unwrap();
        fs::write(f.path.join("preserve"), b"existing destination").unwrap();
        assert!(f.publish(false).unwrap_err().contains("publish"));
        assert_eq!(f.cache.join(&f.closure).exists(), published);
        assert_eq!(
            fs::read(f.path.join("preserve")).unwrap(),
            b"existing destination"
        );
        if published {
            f.verify().unwrap();
        }
    }
}

#[cfg(unix)]
#[test]
fn cache_integrity_publication_rejects_static_and_dangling_links() {
    for level in 0..=2 {
        for dangling in [false, true] {
            let f = CacheFixture::new(true);
            let selected = match level {
                0 => f.cache.clone(),
                1 => f.cache.join("closures"),
                _ => f.cache.join(&f.closure),
            };
            let outside = f.root.join("outside");
            fs::rename(&selected, &outside).unwrap();
            let target = if dangling {
                f.root.join("missing")
            } else {
                outside.clone()
            };
            std::os::unix::fs::symlink(&target, &selected).unwrap();
            let before = fs::read_dir(&outside).unwrap().count();
            let error = publish_closure_cache(&f.lock.sources, &f.staging, &f.cache, &f.closure)
                .unwrap_err();
            assert!(error.contains("bitloom.external-ip.path"), "{error}");
            assert_eq!(fs::read_dir(&outside).unwrap().count(), before);
            assert!(!f.path.exists());
        }
    }
}

#[cfg(unix)]
#[test]
fn cache_integrity_publication_failure_cleans_owned_staging() {
    let f = CacheFixture::new(true);
    let selected = f.cache.join(&f.closure);
    let outside = f.root.join("outside");
    fs::rename(&selected, &outside).unwrap();
    std::os::unix::fs::symlink(&outside, &selected).unwrap();
    assert!(
        f.publish(false)
            .unwrap_err()
            .contains("bitloom.external-ip.path")
    );
    assert!(!f.cache.join(&f.staging[0]).exists());
    assert!(!f.cache.join(".staging-licenses").exists());
    assert!(outside.exists());
    assert!(selected.is_symlink());
}

#[cfg(unix)]
#[test]
fn cache_integrity_lock_rejects_cache_anchor_before_fetch() {
    let root = temp_path(&std::env::temp_dir(), "cache-anchor");
    fs::create_dir_all(root.join("outside")).unwrap();
    let cache = root.join("cache");
    std::os::unix::fs::symlink(root.join("outside"), &cache).unwrap();
    let manifest = root.join("manifest.json");
    fs::write(
        &manifest,
        include_bytes!("../../../ip/external/pulp-common-cells-fifo-v3.source.json"),
    )
    .unwrap();
    assert!(
        lock(&manifest, &root.join("lock.json"), &cache)
            .unwrap_err()
            .contains("bitloom.external-ip.path")
    );
    assert_eq!(fs::read_dir(root.join("outside")).unwrap().count(), 0);
    fs::remove_dir_all(root).unwrap();
}

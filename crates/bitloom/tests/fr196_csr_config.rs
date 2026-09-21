//! Story 127.2: active public-API ATDD, independent software constants.
use bitloom_prelude::{
    ElaborateSession,
    ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister},
};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

fn bank() -> CsrBlock {
    CsrBlock {
        name: "Probe".into(),
        registers: [
            (
                "control",
                0,
                CsrAccess::Rw,
                CsrOwner::Leaf,
                0x80ff00ff,
                false,
                false,
                Option::<&str>::None,
            ),
            (
                "status",
                4,
                CsrAccess::Ro,
                CsrOwner::External,
                0xff,
                true,
                false,
                Option::<&str>::None,
            ),
            (
                "tx",
                8,
                CsrAccess::Wo,
                CsrOwner::None,
                0xff,
                false,
                true,
                Option::<&str>::None,
            ),
            (
                "events",
                12,
                CsrAccess::W1c,
                CsrOwner::Leaf,
                0x8000000f,
                false,
                false,
                Some("hw_events"),
            ),
            (
                "counter",
                16,
                CsrAccess::Rw,
                CsrOwner::External,
                0xffffffff,
                false,
                true,
                Option::<&str>::None,
            ),
        ]
        .into_iter()
        .map(
            |(name, offset, access, owner, mask, read_reject, write_reject, event)| CsrRegister {
                name: name.into(),
                offset,
                reset: 0,
                access,
                owner,
                event: event.map(str::to_owned),
                read_reject,
                write_reject,
                fields: vec![CsrField {
                    name: "bits".into(),
                    mask,
                    reset: 0,
                    access,
                }],
            },
        )
        .collect(),
    }
}

fn rejected(label: &str, config: &CsrBlock) {
    assert!(
        !config.validate().expect_err(label).0.is_empty(),
        "{label}: empty diagnostics"
    );
    assert!(
        config.emit_markdown().is_err(),
        "{label}: markdown accepted"
    );
    assert!(config.emit_c_header().is_err(), "{label}: C accepted");
    assert!(
        config.elaborate("CsrProbe").is_err(),
        "{label}: RTL accepted"
    );
    // A failed host prevalidation must not poison or allocate into the session.
    let mut session = ElaborateSession::new("CsrProbe");
    assert!(
        config.define_module(&mut session, "CsrProbe").is_err(),
        "{label}"
    );
    bank().define_module(&mut session, "CsrProbe").expect(label);
    assert_eq!(
        session.finish().expect(label).circuit().modules.len(),
        1,
        "{label}"
    );
}

#[test]
fn p0_offsets_masks_resets_and_access_are_checked_before_all_emitters() {
    bank().validate().unwrap();
    let mut last = bank();
    last.registers[4].offset = 0xfffc;
    last.validate().unwrap();
    last.elaborate("LastWord").unwrap();
    for offset in [1, 2, 3, 4, 0xffff, 0x10000, u32::MAX] {
        let mut b = bank();
        b.registers[0].offset = offset;
        rejected(&format!("offset {offset:#x}"), &b);
    }
    for mask in [0, 0x1_0000_0000, u64::MAX] {
        let mut b = bank();
        b.registers[0].fields[0].mask = mask;
        rejected(&format!("mask {mask:#x}"), &b);
    }
    let mut b = bank();
    b.registers[0].fields.push(CsrField {
        name: "overlap".into(),
        mask: 1,
        reset: 0,
        access: CsrAccess::Rw,
    });
    rejected("overlapping fields", &b);
    for (reg_reset, field_reset) in [(1, 0), (0, 1), (1, 1), (0x100, 0x100)] {
        let mut b = bank();
        b.registers[0].reset = reg_reset;
        b.registers[0].fields[0].reset = field_reset;
        rejected("nonzero or inconsistent reset", &b);
    }
    let mut b = bank();
    b.registers[0].fields[0].access = CsrAccess::Ro;
    rejected("mixed access", &b);
    let mut b = bank();
    b.registers.clear();
    rejected("empty block", &b);
    let mut b = bank();
    b.registers[0].fields.clear();
    rejected("empty fields", &b);
}

#[test]
fn p0_owner_event_and_reject_contract_is_exhaustive() {
    for access in [CsrAccess::Rw, CsrAccess::Ro, CsrAccess::Wo, CsrAccess::W1c] {
        for owner in [CsrOwner::Leaf, CsrOwner::External, CsrOwner::None] {
            let mut b = bank();
            b.registers.truncate(1);
            let r = &mut b.registers[0];
            r.access = access;
            r.fields[0].access = access;
            r.owner = owner;
            r.event = (access == CsrAccess::W1c).then(|| "event_source".into());
            if matches!(
                (access, owner),
                (CsrAccess::Rw, CsrOwner::Leaf | CsrOwner::External)
                    | (CsrAccess::Ro, CsrOwner::External)
                    | (CsrAccess::Wo, CsrOwner::None)
                    | (CsrAccess::W1c, CsrOwner::Leaf)
            ) {
                b.validate().unwrap();
                b.elaborate("OwnerCase").unwrap();
            } else {
                rejected("illegal owner", &b);
            }
        }
    }
    for event in [
        Option::None,
        Some(""),
        Some("1bad"),
        Some("module"),
        Some("req_valid"),
        Some("control_value"),
    ] {
        let mut b = bank();
        b.registers[3].event = event.map(str::to_owned);
        rejected("invalid event", &b);
    }
    for index in [0, 1, 2, 4] {
        let mut b = bank();
        b.registers[index].event = Some("unexpected_event".into());
        rejected("non-W1C event", &b);
    }
    let mut b = bank();
    let mut r = b.registers[3].clone();
    r.name = "other_events".into();
    r.offset = 20;
    b.registers.push(r);
    rejected("duplicate event", &b);
    let mut b = bank();
    b.registers[2].read_reject = true;
    rejected("WO read reject", &b);
    let mut b = bank();
    b.registers[1].write_reject = true;
    rejected("RO write reject", &b);
}

#[test]
fn p0_names_and_generated_names_cannot_collide_or_be_silently_renamed() {
    for name in [
        "",
        "1abc",
        "_hidden",
        "a__b",
        "a-b",
        "空",
        "module",
        "wire",
        "typedef",
        "int",
        "endcase",
        "endfunction",
        "always_ff",
        "restrict",
    ] {
        for target in 0..3 {
            let mut b = bank();
            match target {
                0 => b.name = name.into(),
                1 => b.registers[0].name = name.into(),
                _ => b.registers[0].fields[0].name = name.into(),
            }
            rejected(&format!("name {name:?} target {target}"), &b);
        }
    }
    for name in ["control", "CONTROL"] {
        let mut b = bank();
        b.registers[1].name = name.into();
        rejected("duplicate/macro register name", &b);
    }
    for name in ["bits", "BITS"] {
        let mut b = bank();
        b.registers[0].fields[0].mask = 1;
        b.registers[0].fields.push(CsrField {
            name: name.into(),
            mask: 2,
            reset: 0,
            access: CsrAccess::Rw,
        });
        rejected("duplicate/macro field name", &b);
    }
    // A_OFFSET_MASK is both a register mask and A's OFFSET field mask.
    let mut b = bank();
    b.registers[0].name = "A".into();
    b.registers[0].fields[0].name = "OFFSET".into();
    b.registers[1].name = "A_OFFSET".into();
    rejected("generated macro collision", &b);
    let mut b = bank();
    b.registers[3].event = Some("counter_write_commit".into());
    rejected("event versus generated port", &b);
}

#[test]
fn p0_full_configuration_identity_rejects_same_module_name_and_poisons_session() {
    let original = bank();
    let mut variants = Vec::new();
    let mut b = original.clone();
    b.name = "Other".into();
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].name = "renamed".into();
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].fields[0].name = "renamed_bits".into();
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].fields[0].mask = 0x800000ff;
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].owner = CsrOwner::External;
    variants.push(b);
    let mut b = original.clone();
    b.registers[3].event = Some("other_event".into());
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].read_reject = true;
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].write_reject = true;
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].offset = 20;
    variants.push(b);
    let mut b = original.clone();
    b.registers[0].access = CsrAccess::Ro;
    b.registers[0].fields[0].access = CsrAccess::Ro;
    b.registers[0].owner = CsrOwner::External;
    variants.push(b);
    // Same effective mask with a different field partition must still differ.
    let mut b = original.clone();
    b.registers[0].fields[0].mask = 0xff;
    b.registers[0].fields.push(CsrField {
        name: "upper".into(),
        mask: 0x80ff0000,
        reset: 0,
        access: CsrAccess::Rw,
    });
    variants.push(b);
    for changed in variants {
        changed.validate().unwrap();
        let mut s = ElaborateSession::new("CsrProbe");
        assert_eq!(
            original.define_module(&mut s, "CsrProbe").unwrap(),
            "CsrProbe"
        );
        assert_eq!(
            original.define_module(&mut s, "CsrProbe").unwrap(),
            "CsrProbe"
        );
        let error = changed.define_module(&mut s, "CsrProbe").unwrap_err();
        assert!(error.0.iter().any(|d| d.code == "rhdl::E0244"), "{error}");
        assert!(s.finish().is_err(), "helper collision must poison session");
        let mut s = ElaborateSession::new("First");
        original.define_module(&mut s, "First").unwrap();
        changed.define_module(&mut s, "Second").unwrap();
        assert_eq!(s.finish().unwrap().circuit().modules.len(), 2);
    }
}

fn rtl(block: &CsrBlock) -> Vec<(String, String)> {
    bitloom_vlog::emit(&block.elaborate("CsrProbe").unwrap())
        .files
        .into_iter()
        .map(|f| (f.path, f.contents))
        .collect()
}

#[test]
fn p1_reordering_is_canonical_for_hir_rtl_markdown_header_and_session_reuse() {
    let mut b = bank();
    b.registers[0].fields[0].mask = 0xff;
    b.registers[0].fields.push(CsrField {
        name: "upper".into(),
        mask: 0x80ff0000,
        reset: 0,
        access: CsrAccess::Rw,
    });
    let mut reversed = b.clone();
    reversed.registers.reverse();
    for r in &mut reversed.registers {
        r.fields.reverse();
    }
    assert_eq!(
        b.elaborate("CsrProbe").unwrap(),
        reversed.elaborate("CsrProbe").unwrap()
    );
    assert_eq!(rtl(&b), rtl(&reversed));
    assert_eq!(rtl(&b), rtl(&b));
    assert_eq!(
        b.emit_markdown().unwrap(),
        reversed.emit_markdown().unwrap()
    );
    assert_eq!(
        b.emit_c_header().unwrap(),
        reversed.emit_c_header().unwrap()
    );
    assert_eq!(b.emit_markdown().unwrap(), b.emit_markdown().unwrap());
    assert_eq!(b.emit_c_header().unwrap(), b.emit_c_header().unwrap());
    let mut s = ElaborateSession::new("CsrProbe");
    b.define_module(&mut s, "CsrProbe").unwrap();
    reversed.define_module(&mut s, "CsrProbe").unwrap();
    assert_eq!(s.finish().unwrap().circuit().modules.len(), 1);
}

fn compile(dir: &Path, source: &str, label: &str) {
    compile_expected(dir, source, label, true);
}
fn compile_expected(dir: &Path, source: &str, label: &str, success: bool) {
    fs::write(dir.join(format!("{label}.c")), source).unwrap();
    let log_path = dir.join(format!("{label}.log"));
    let log = fs::File::create(&log_path).unwrap();
    // GNU timeout creates a process group and terminates cc and its children.
    let status = Command::new("timeout")
        .current_dir(dir)
        .args([
            "--kill-after=5s",
            "60s",
            "cc",
            "-std=c11",
            "-pedantic-errors",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-c",
            &format!("{label}.c"),
            "-o",
            &format!("{label}.o"),
        ])
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("C11 compiler and GNU timeout required");
    fs::write(dir.join(format!("{label}.exit")), format!("{status}\n")).unwrap();
    fs::write(dir.join(format!("{label}.command")), format!("timeout --kill-after=5s 60s cc -std=c11 -pedantic-errors -Wall -Wextra -Werror -c {label}.c -o {label}.o\n")).unwrap();
    let diagnostics = fs::read_to_string(&log_path).unwrap();
    assert!(
        !matches!(status.code(), Some(124 | 127 | 137)),
        "C compiler unavailable/timed out: {diagnostics}"
    );
    assert_eq!(
        status.success(),
        success,
        "C consumer {label}: {status}: {diagnostics}"
    );
    if !success {
        assert!(
            diagnostics.contains("Bitloom CSR macro collision: A_B_C_MASK"),
            "expected explicit collision diagnosis: {diagnostics}"
        );
    }
}

#[test]
fn p0_header_has_handwritten_golden_constants_and_compiles_single_and_two_blocks() {
    let b = bank();
    let header = b.emit_c_header().unwrap();
    assert!(header.contains("BITLOOM_PROBE_CSR_H"));
    assert!(header.to_lowercase().contains("local"));
    assert!(header.to_lowercase().contains("base"));
    let md = b.emit_markdown().unwrap();
    assert!(md.to_lowercase().contains("local"));
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-csr-c")
        .join(std::process::id().to_string());
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("probe.h"), &header).unwrap();
    let mut source = String::from("#include \"probe.h\"\n#include \"probe.h\"\n");
    // Oracle is deliberately written independently of the bank and emitters.
    for (reg, offset, mask, access) in [
        ("CONTROL", 0u32, 0x80ff00ffu32, "RW"),
        ("STATUS", 4, 0xff, "RO"),
        ("TX", 8, 0xff, "WO"),
        ("EVENTS", 12, 0x8000000f, "W1C"),
        ("COUNTER", 16, 0xffffffff, "RW"),
    ] {
        for (suffix, value) in [
            ("OFFSET", offset),
            ("MASK", mask),
            ("RESET", 0),
            ("BITS_MASK", mask),
            ("BITS_RESET", 0),
        ] {
            let name = format!("PROBE_{reg}_{suffix}");
            assert!(
                header
                    .lines()
                    .any(|line| line.trim() == format!("#define {name} UINT32_C(0x{value:08x})")),
                "missing golden {name}"
            );
            source.push_str(&format!(
                "_Static_assert({name} == UINT32_C(0x{value:08x}), \"{name}\");\n"
            ));
        }
        for suffix in ["ACCESS", "BITS_ACCESS"] {
            assert!(header.lines().any(|line| line.trim() == format!("#define PROBE_{reg}_{suffix} \"{access}\"")));
            source.push_str(&format!(
                "const char *golden_{reg}_{suffix} = PROBE_{reg}_{suffix};\n"
            ));
        }
        // Markdown uses hexadecimal local offsets, masks and resets. Ignore
        // padding/case/table layout, but require each register row to carry its
        // own independent golden values (not values found elsewhere in the file).
        let register_row = md
            .lines()
            .find(|line| {
                line.split(|c: char| !c.is_ascii_alphanumeric())
                    .any(|token| token.eq_ignore_ascii_case(reg))
                    && line.to_ascii_lowercase().contains("0x")
            })
            .unwrap_or_else(|| panic!("missing Markdown register row {reg}"));
        let hex_values: Vec<u32> = register_row
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter_map(|token| {
                token
                    .strip_prefix("0x")
                    .or_else(|| token.strip_prefix("0X"))
            })
            .map(|digits| u32::from_str_radix(digits, 16).expect("valid Markdown hex constant"))
            .collect();
        assert_eq!(
            hex_values,
            [offset, mask, 0],
            "{reg}: offset/mask/reset golden Markdown values"
        );
        assert!(
            register_row
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|token| token == access),
            "{reg}: Markdown access"
        );
    }
    compile(&dir, &source, "single");
    let mut other = b.clone();
    other.name = "Aux".into();
    other.registers[0].offset = 24;
    fs::write(dir.join("aux.h"), other.emit_c_header().unwrap()).unwrap();
    source.push_str("#include \"aux.h\"\n_Static_assert(AUX_CONTROL_OFFSET == UINT32_C(0x00000018), \"independent namespace\");\n_Static_assert(PROBE_CONTROL_OFFSET == 0, \"first header survived\");\n_Static_assert(AUX_COUNTER_MASK == UINT32_C(0xffffffff), \"high mask\");\n");
    compile(&dir, &source, "two_blocks");
    writeln!(
        fs::File::create(dir.join("commands.log")).unwrap(),
        "cc -std=c11 -pedantic-errors -Wall -Wextra -Werror -c {{single,two_blocks}}.c"
    )
    .unwrap();
}

#[test]
fn p0_distinct_block_names_with_identical_macro_sets_fail_joint_c11_include() {
    let mut first = bank();
    first.name = "A_B".into();
    first.registers.truncate(1);
    first.registers[0].name = "C".into();
    let mut second = first.clone();
    second.name = "A".into();
    second.registers[0].name = "B_C".into();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-csr-c")
        .join(format!("collision-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("first.h"), first.emit_c_header().unwrap()).unwrap();
    fs::write(dir.join("second.h"), second.emit_c_header().unwrap()).unwrap();
    // Each alone is valid; identical macro values used to silently compile.
    compile(&dir, "#include \"first.h\"\nint consumer;\n", "first");
    compile(&dir, "#include \"second.h\"\nint consumer;\n", "second");
    compile_expected(
        &dir,
        "#include \"first.h\"\n#include \"second.h\"\nint consumer;\n",
        "joint",
        false,
    );
    compile_expected(
        &dir,
        "#include \"second.h\"\n#include \"first.h\"\nint consumer;\n",
        "reverse_joint",
        false,
    );
}

#[test]
fn p1_markdown_reports_reject_switches_and_independent_multifield_constants() {
    let mut b = bank();
    b.registers[0].fields = vec![
        CsrField {
            name: "enable".into(),
            mask: 1,
            reset: 0,
            access: CsrAccess::Rw,
        },
        CsrField {
            name: "mode".into(),
            mask: 0xf0,
            reset: 0,
            access: CsrAccess::Rw,
        },
        CsrField {
            name: "high".into(),
            mask: 0x80000000,
            reset: 0,
            access: CsrAccess::Rw,
        },
    ];
    let header = b.emit_c_header().unwrap();
    let md = b.emit_markdown().unwrap();
    assert!(md.contains("| Read reject | Write reject |"));
    for (name, rd, wr) in [
        ("control", false, false),
        ("status", true, false),
        ("tx", false, true),
        ("events", false, false),
        ("counter", false, true),
    ] {
        let row = md
            .lines()
            .find(|line| line.starts_with(&format!("| {name} |")))
            .unwrap();
        assert!(row.ends_with(&format!("| {rd} | {wr} |")), "{row}");
    }
    for (name, mask) in [
        ("enable", 0x00000001u32),
        ("mode", 0x000000f0),
        ("high", 0x80000000),
    ] {
        let base = format!("PROBE_CONTROL_{}", name.to_ascii_uppercase());
        for (suffix, value) in [("MASK", mask), ("RESET", 0)] {
            assert!(
                header
                    .lines()
                    .any(|line| line == format!("#define {base}_{suffix} UINT32_C(0x{value:08x})"))
            );
        }
        assert!(
            header
                .lines()
                .any(|line| line == format!("#define {base}_ACCESS \"RW\""))
        );
        assert!(
            md.lines()
                .any(|line| line == format!("| control.{name} | 0x{mask:08x} | 0x00000000 | RW |"))
        );
    }
    assert!(header.contains("#define PROBE_CONTROL_MASK UINT32_C(0x800000f1)"));
    assert!(
        md.lines()
            .any(|line| line.starts_with("| control | 0x0000 | 0x800000f1 | 0x00000000 | RW |"))
    );
    let mut changed = b.clone();
    changed.registers[0].read_reject = true;
    changed.registers[0].write_reject = true;
    assert!(
        changed
            .emit_markdown()
            .unwrap()
            .lines()
            .find(|line| line.starts_with("| control |"))
            .unwrap()
            .ends_with("| true | true |")
    );
}

#[test]
fn p1_documented_c_byte_offset_consumer_compiles_without_mmio_execution() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let document = fs::read_to_string(root.join("docs/ip/csr.md")).unwrap();
    let blocks: Vec<_> = document.split("```c\n").collect();
    assert_eq!(blocks.len(), 2, "one complete C consumer example");
    let source = blocks[1].split("```").next().unwrap();
    assert!(source.contains("base + (uintptr_t)EXAMPLE_CONTROL_OFFSET"));
    let mut block = bank();
    block.name = "Example".into();
    block.registers[0].fields[0].mask = 0xff;
    let dir = root
        .join("target/fr196-csr-c")
        .join(format!("documentation-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("example.h"), block.emit_c_header().unwrap()).unwrap();
    compile(&dir, source, "documented_consumer"); // compile -c only, never execute MMIO
}

//! Story128.5 UART public API, shared definitions and prelude-only consumers.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{CsrAccess, CsrOwner, Gpio, UartCsr},
};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Stdio},
};
fn ports() -> BTreeMap<String, (PortDirection, GroundType)> {
    use PortDirection::{Input, Output};
    let mut p = BTreeMap::from([
        ("clk".into(), (Input, GroundType::Clock)),
        ("rst".into(), (Input, GroundType::Reset)),
    ]);
    for (n, w, d) in [
        ("req_valid", 1, Input),
        ("write", 1, Input),
        ("addr", 16, Input),
        ("wdata", 32, Input),
        ("wstrb", 4, Input),
        ("rsp_ready", 1, Input),
        ("rx", 1, Input),
        ("req_ready", 1, Output),
        ("rsp_valid", 1, Output),
        ("rdata", 32, Output),
        ("error", 2, Output),
        ("tx", 1, Output),
        ("raw_events", 4, Output),
    ] {
        p.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    p
}
#[test]
fn p0_exact_fifteen_ports_and_shared_standalone_equivalence() {
    let hir = UartCsr::elaborate().unwrap();
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "UartCsr")
        .unwrap();
    assert_eq!(top.ports.len(), 15);
    assert_eq!(
        top.ports
            .iter()
            .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
            .collect::<BTreeMap<_, _>>(),
        ports()
    );
    let mut s = ElaborateSession::new("UartCsr");
    assert_eq!(
        UartCsr::define_module(&mut s, String::from("UartCsr")).unwrap(),
        "UartCsr"
    );
    UartCsr::define_module(&mut s, "UartCsr").unwrap();
    assert_eq!(hir, s.finish().unwrap());
}

#[test]
fn p0_descriptor_exact_addresses_masks_owners_and_dynamic_rejects() {
    use CsrAccess::{Ro, Rw, W1c, Wo};
    use CsrOwner::{External, Leaf};
    let block = UartCsr::registers();
    block.validate().unwrap();
    assert_eq!(block.name, "UartCsr");
    assert_eq!(block.registers.len(), 6);
    for (name, offset, mask, access, owner, event, rd, wr) in [
        ("ctrl", 0, 1, Rw, Leaf, None, false, true),
        ("baud_div", 4, 0xffffffff, Rw, Leaf, None, false, true),
        ("status", 8, 15, Ro, External, None, false, false),
        ("tx_data", 12, 255, Wo, CsrOwner::None, None, false, true),
        ("rx_data", 16, 255, Ro, External, None, true, false),
        ("EVENT", 20, 15, W1c, Leaf, Some("event_bits"), false, false),
    ] {
        let r = block.registers.iter().find(|r| r.name == name).unwrap();
        assert_eq!(
            (
                r.offset,
                r.reset,
                r.fields
                    .iter()
                    .fold(0u32, |mask, field| mask | field.mask as u32),
                r.access,
                r.owner
            ),
            (offset, 0, mask, access, owner)
        );
        assert_eq!(r.event.as_deref(), event);
        assert_eq!((r.read_reject, r.write_reject), (rd, wr), "{name}");
        assert_eq!(r.fields.len(), 1);
        let f = &r.fields[0];
        assert_eq!(f.name, "bits");
        assert_eq!((f.mask, f.reset, f.access), (u64::from(mask), 0, access));
    }
}
#[test]
fn p1_software_products_match_literal_local_address_golden() {
    let c = UartCsr::registers().emit_c_header().unwrap();
    let md = UartCsr::registers().emit_markdown().unwrap();
    assert_eq!(c, UartCsr::registers().emit_c_header().unwrap());
    assert_eq!(md, UartCsr::registers().emit_markdown().unwrap());
    for (name, offset, mask, access) in [
        ("CTRL", 0, 1, "RW"),
        ("BAUD_DIV", 4, 0xffffffffu32, "RW"),
        ("STATUS", 8, 15, "RO"),
        ("TX_DATA", 12, 255, "WO"),
        ("RX_DATA", 16, 255, "RO"),
        ("EVENT", 20, 15, "W1C"),
    ] {
        for line in [
            format!("#define UARTCSR_{name}_OFFSET UINT32_C(0x{offset:08x})"),
            format!("#define UARTCSR_{name}_MASK UINT32_C(0x{mask:08x})"),
            format!("#define UARTCSR_{name}_BITS_MASK UINT32_C(0x{mask:08x})"),
            format!("#define UARTCSR_{name}_RESET UINT32_C(0x00000000)"),
            format!("#define UARTCSR_{name}_ACCESS \"{access}\""),
        ] {
            assert!(c.lines().any(|l| l == line), "missing {line}");
        }
    }
    for line in [
        "| ctrl | 0x0000 | 0x00000001 | 0x00000000 | RW | Leaf | — | false | true |",
        "| baud_div | 0x0004 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | true |",
        "| status | 0x0008 | 0x0000000f | 0x00000000 | RO | External | — | false | false |",
        "| tx_data | 0x000c | 0x000000ff | 0x00000000 | WO | None | — | false | true |",
        "| rx_data | 0x0010 | 0x000000ff | 0x00000000 | RO | External | — | true | false |",
        "| EVENT | 0x0014 | 0x0000000f | 0x00000000 | W1C | Leaf | event_bits | false | false |",
    ] {
        assert!(md.lines().any(|l| l == line), "missing {line}");
    }
    assert!(md.contains("Local byte offsets; caller supplies the base address."));
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(
        c,
        fs::read_to_string(root.join("docs/ip/uart-csr-registers.h")).unwrap()
    );
    assert_eq!(
        md,
        fs::read_to_string(root.join("docs/ip/uart-csr-registers.md")).unwrap()
    );
}
#[test]
fn p0_invalid_names_and_conflicting_bodies_are_rejected() {
    for name in ["", "not a module", "module"] {
        let mut s = ElaborateSession::new("Top");
        assert_diag(
            &UartCsr::define_module(&mut s, name).unwrap_err(),
            "rhdl::E0241",
            &["module name", "identifier"],
        );
        assert_diag(&s.finish().unwrap_err(), "rhdl::E0241", &["module name"]);
    }
    let mut s = ElaborateSession::new("UartCsr");
    UartCsr::define_module(&mut s, "UartCsr").unwrap();
    assert_diag(
        &Gpio::define_module(&mut s, "UartCsr").unwrap_err(),
        "rhdl::E0244",
        &["module 'UartCsr'", "conflicts"],
    );
    assert_diag(
        &s.finish().unwrap_err(),
        "rhdl::E0244",
        &["module 'UartCsr'"],
    );
}
#[test]
fn p0_shared_and_renamed_definitions_and_wiring_diagnostics() {
    for (bad, code, port, net) in [
        ("width", "rhdl::E0203", "uart0.addr", "t0_addr"),
        ("type", "rhdl::E0256", "uart0.clk", "bad_clk"),
        ("direction", "rhdl::E0257", "uart0.tx", "t0_tx"),
    ] {
        assert_diag(&pair(Some(bad)).unwrap_err(), code, &[port, net]);
    }
    for same in [true, false] {
        let h = pair_kind(None, same).unwrap();
        assert_eq!(
            h.circuit()
                .modules
                .iter()
                .filter(|m| m.name == "SharedUart")
                .count(),
            1
        );
        assert_eq!(
            h.circuit()
                .modules
                .iter()
                .filter(|m| m.name == "OtherUart")
                .count(),
            usize::from(!same)
        );
        if !same {
            assert_eq!(
                h.circuit().modules.len(),
                pair_kind(None, true).unwrap().circuit().modules.len() + 1
            );
        }
    }
}
#[test]
fn p0_two_fifos_reuse_exact_param_sync_fifo_eight_by_four_body() {
    // Compare public FIFO elaboration, never private implementation names.
    use bitloom_hir::Stmt;
    use bitloom_prelude::ip::ParamSyncFifo;
    let reference = ParamSyncFifo::<8, 4>::elaborate().unwrap();
    let expected = &reference.circuit().modules[0];
    let hir = UartCsr::elaborate().unwrap();
    let fifo_defs: Vec<_> = hir
        .circuit()
        .modules
        .iter()
        .filter(|m| m.ports == expected.ports && m.body == expected.body)
        .collect();
    assert_eq!(
        fifo_defs.len(),
        1,
        "one shared ParamSyncFifo<8,4> definition"
    );
    let instances: Vec<_> = hir
        .circuit()
        .modules
        .iter()
        .flat_map(|m| &m.body)
        .filter_map(|s| match s {
            Stmt::Instance(i) if i.module == fifo_defs[0].name => Some(i),
            _ => None,
        })
        .collect();
    assert_eq!(instances.len(), 2, "actual TX and RX FIFO instances");
    assert_ne!(instances[0].name, instances[1].name);
}
#[test]
fn p1_hierarchy_native_and_generated_reject_explicitly() {
    use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
    for hir in [
        UartCsr::elaborate().unwrap(),
        pair(None).unwrap(),
        pair_kind(None, true).unwrap(),
    ] {
        let mut errors = vec![];
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            errors.push(
                std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine))
                    .err()
                    .expect("hierarchy must reject"),
            );
        }
        errors.push(
            std::panic::catch_unwind(|| GeneratedFunctional::from_hir(&hir))
                .err()
                .expect("hierarchy must reject"),
        );
        for payload in errors {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("");
            assert!(
                message.contains("hierarchical simulation is unsupported"),
                "unexpected panic: {message}"
            );
        }
    }
}
fn artifact_dir(label: &str) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart-api")
        .join(format!(
            "{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    dir
}
fn command(dir: &Path, tool: &str, args: &[&str], label: &str) {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required consumer compiler must be installed");
    fs::write(dir.join(format!("{label}.json")), serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"start_unix_ms":start,"end_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    assert!(
        status.success(),
        "{tool} {status}: {}",
        fs::read_to_string(dir.join(format!("{label}.log"))).unwrap()
    );
}
#[test]
fn p1_generated_header_is_real_c11_consumer_with_independent_constants() {
    let dir = artifact_dir("c11");
    fs::write(
        dir.join("uart.h"),
        UartCsr::registers().emit_c_header().unwrap(),
    )
    .unwrap();
    fs::write(dir.join("consumer.c"), r#"#include "uart.h"
#include "uart.h"
#include <string.h>
_Static_assert(UARTCSR_CTRL_OFFSET == 0 && UARTCSR_CTRL_MASK == 1, "ctrl");
_Static_assert(UARTCSR_BAUD_DIV_OFFSET == 4 && UARTCSR_BAUD_DIV_MASK == UINT32_MAX, "div");
_Static_assert(UARTCSR_STATUS_OFFSET == 8 && UARTCSR_STATUS_MASK == 15, "status");
_Static_assert(UARTCSR_TX_DATA_OFFSET == 12 && UARTCSR_TX_DATA_MASK == 255, "tx");
_Static_assert(UARTCSR_RX_DATA_OFFSET == 16 && UARTCSR_RX_DATA_MASK == 255, "rx");
_Static_assert(UARTCSR_EVENT_OFFSET == 20 && UARTCSR_EVENT_MASK == 15, "event");
int main(void) { return strcmp(UARTCSR_EVENT_ACCESS, "W1C") || strcmp(UARTCSR_TX_DATA_ACCESS, "WO") || strcmp(UARTCSR_RX_DATA_ACCESS, "RO"); }
"#).unwrap();
    command(
        &dir,
        "cc",
        &[
            "-std=c11",
            "-Wall",
            "-Wextra",
            "-Werror",
            "consumer.c",
            "-o",
            "consumer",
        ],
        "compile",
    );
    command(&dir, "./consumer", &[], "execute");
}
#[test]
fn p1_documented_example_compiles_runs_with_only_prelude_and_reproduces_products() {
    let dir = artifact_dir("prelude-example").canonicalize().unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::copy(
        root.join("docs/ip/uart-csr-example.rs"),
        dir.join("src/main.rs"),
    )
    .unwrap();
    let prelude = root.join("crates/bitloom-prelude");
    fs::write(dir.join("Cargo.toml"), format!("[package]\nname = \"uart-csr-doc-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[dependencies]\nbitloom-prelude = {{ path = {:?} }}\n", prelude.to_str().unwrap())).unwrap();
    command(
        &dir,
        "cargo",
        &["run", "--offline", "--quiet", "--", "software"],
        "prelude-run",
    );
    assert_eq!(
        fs::read_to_string(dir.join("software/uart-csr-registers.h")).unwrap(),
        UartCsr::registers().emit_c_header().unwrap()
    );
    assert_eq!(
        fs::read_to_string(dir.join("software/uart-csr-registers.md")).unwrap(),
        UartCsr::registers().emit_markdown().unwrap()
    );
}
fn assert_diag(err: &bitloom_prelude::Diagnostics, code: &str, details: &[&str]) {
    assert!(
        err.0
            .iter()
            .any(|d| d.code == code && details.iter().all(|text| d.en.contains(text))),
        "expected {code} with {details:?}, got {err:?}"
    );
}

fn pair(bad: Option<&str>) -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    pair_kind(bad, false)
}

fn pair_kind(
    bad: Option<&str>,
    same_definition: bool,
) -> Result<FrozenHir, bitloom_prelude::Diagnostics> {
    let mut s = ElaborateSession::new("TwoUarts");
    UartCsr::define_module(&mut s, "SharedUart")?;
    UartCsr::define_module(&mut s, "SharedUart")?;
    if !same_definition {
        UartCsr::define_module(&mut s, "OtherUart")?;
    }
    let sp = Span::default();
    s.begin_module("TwoUarts", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut connects = vec![];
        for (name, (direction, mut ty)) in ports() {
            let net = if name == "clk" || name == "rst" {
                name.clone()
            } else {
                format!("t{lane}_{name}")
            };
            if lane == 0 && bad == Some("width") && name == "addr" {
                ty = GroundType::UInt { width: 8 };
            }
            if lane == 0 && bad == Some("type") && name == "clk" {
                s.add_input("bad_clk", GroundType::UInt { width: 1 }, sp);
                connects.push((name, "bad_clk".into()));
                continue;
            }
            if name != "clk" && name != "rst" {
                if direction == PortDirection::Input
                    || (lane == 0 && bad == Some("direction") && name == "tx")
                {
                    s.add_input(&net, ty, sp);
                } else {
                    s.add_output(&net, ty, sp);
                }
            }
            connects.push((name, net));
        }
        s.add_instance(
            format!("uart{lane}"),
            if lane == 0 || same_definition {
                "SharedUart"
            } else {
                "OtherUart"
            },
            connects,
            vec![],
            sp,
        );
    }
    s.end_module();
    s.finish()
}

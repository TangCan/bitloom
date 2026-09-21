//! Story127.4: actual emitted bridge/decoder/four-CsrBlock hierarchy.
//! External values/events/rejections are test peers, not Epic128 peripherals.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, Span,
    ip::{AxiLiteCsrBridge, CsrAccess, CsrBlock, CsrDecoder, CsrField, CsrOwner, CsrRegister},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
pub const LEAVES: [&str; 4] = ["uart", "gpio", "timer", "irq"];
const BUS: [&str; 10] = [
    "req_valid",
    "write",
    "addr",
    "wdata",
    "wstrb",
    "rsp_ready",
    "req_ready",
    "rsp_valid",
    "rdata",
    "error",
];
pub fn banks() -> Vec<CsrBlock> {
    use CsrAccess::*;
    let layouts: Vec<Vec<(&str, u32, CsrAccess, u32)>> = vec![
        vec![
            ("ctrl", 0, Rw, 1),
            ("baud_div", 4, Rw, 0xffffffff),
            ("status", 8, Ro, 15),
            ("tx_data", 12, Wo, 255),
            ("rx_data", 16, Ro, 255),
            ("EVENT", 20, W1c, 15),
        ],
        vec![
            ("dir", 0, Rw, 0xffffffff),
            ("out", 4, Rw, 0xffffffff),
            ("in", 8, Ro, 0xffffffff),
            ("set", 12, Wo, 0xffffffff),
            ("clear", 16, Wo, 0xffffffff),
            ("rise_event", 20, W1c, 0xffffffff),
        ],
        vec![
            ("ctrl", 0, Rw, 3),
            ("count", 4, Rw, 0xffffffff),
            ("compare", 8, Rw, 0xffffffff),
            ("EVENT", 12, W1c, 1),
        ],
        vec![
            ("pending", 0, W1c, 31),
            ("enable", 4, Rw, 31),
            ("test", 8, Wo, 31),
            ("raw", 12, Ro, 31),
        ],
    ];
    layouts
        .into_iter()
        .enumerate()
        .map(|(j, layout)| CsrBlock {
            name: LEAVES[j].into(),
            registers: layout
                .into_iter()
                .map(|(name, offset, access, mask)| CsrRegister {
                    name: name.into(),
                    offset,
                    reset: 0,
                    access,
                    owner: match access {
                        Wo => CsrOwner::None,
                        W1c => CsrOwner::Leaf,
                        _ => CsrOwner::External,
                    },
                    event: if access == W1c {
                        Some(format!("{name}_events"))
                    } else {
                        None
                    },
                    read_reject: access == Ro,
                    write_reject: access == Wo,
                    fields: vec![CsrField {
                        name: "bits".into(),
                        mask: mask as u64,
                        reset: 0,
                        access,
                    }],
                })
                .collect(),
        })
        .collect()
}
pub fn hierarchy() -> FrozenHir {
    let bridge = AxiLiteCsrBridge::elaborate().unwrap();
    let decoder = CsrDecoder::elaborate().unwrap();
    let banks = banks();
    let leaves: Vec<_> = banks
        .iter()
        .enumerate()
        .map(|(j, b)| b.elaborate(format!("Leaf{j}")).unwrap())
        .collect();
    let mut s = ElaborateSession::new("DecoderSystem");
    AxiLiteCsrBridge::define_module(&mut s, "Bridge").unwrap();
    CsrDecoder::define_module(&mut s, "Decoder").unwrap();
    for (j, b) in banks.iter().enumerate() {
        b.define_module(&mut s, format!("Leaf{j}")).unwrap();
    }
    let sp = Span::default();
    s.begin_module("DecoderSystem", sp);
    let mut bc = vec![];
    for p in &bridge.circuit().modules[0].ports {
        if p.name.starts_with("csr_") || p.direction != PortDirection::Input {
            s.add_output(&p.name, p.ty.clone(), sp);
        } else {
            s.add_input(&p.name, p.ty.clone(), sp);
        }
        bc.push((p.name.clone(), p.name.clone()));
    }
    let mut dc = vec![];
    for p in &decoder.circuit().modules[0].ports {
        let n = if BUS.contains(&p.name.as_str()) {
            format!("csr_{}", p.name)
        } else {
            p.name.clone()
        };
        if p.name != "clk" && p.name != "rst" && !BUS.contains(&p.name.as_str()) {
            s.add_output(&n, p.ty.clone(), sp);
        }
        dc.push((p.name.clone(), n));
    }
    s.add_instance("bridge", "Bridge", bc, vec![], sp);
    s.add_instance("decoder", "Decoder", dc, vec![], sp);
    for (j, leaf) in leaves.iter().enumerate() {
        let mut lc = vec![];
        for p in &leaf.circuit().modules[0].ports {
            let n = if p.name == "clk" || p.name == "rst" {
                p.name.clone()
            } else {
                format!("{}_{}", LEAVES[j], p.name)
            };
            if p.name != "clk" && p.name != "rst" && !BUS.contains(&p.name.as_str()) {
                if p.direction == PortDirection::Input {
                    s.add_input(&n, p.ty.clone(), sp);
                } else {
                    s.add_output(&n, p.ty.clone(), sp);
                }
            }
            lc.push((p.name.clone(), n));
        }
        s.add_instance(LEAVES[j], format!("Leaf{j}"), lc, vec![], sp);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 7);
    hir
}
pub fn dir(label: &str) -> PathBuf {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-decoder-integration")
        .join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&p).unwrap();
    p
}
pub fn run(dir: &Path, tool: &str, args: &[&str], label: &str) -> String {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start = Instant::now();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "60s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required real tool");
    fs::write(dir.join(format!("{label}.json")),serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"elapsed_seconds":start.elapsed().as_secs_f64()}).to_string()).unwrap();
    let output = fs::read_to_string(dir.join(format!("{label}.log"))).unwrap();
    assert!(
        status.success(),
        "{tool} {status}: {output}; {}",
        dir.display()
    );
    output
}

/// Test-only port declarations and external RW owner peers; no peripheral model.
pub fn bench(hir: &FrozenHir) -> String {
    use std::fmt::Write as _;
    let mut tb = String::from("module tb;\n");
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "DecoderSystem")
        .unwrap();
    for p in &top.ports {
        let width = match p.ty {
            bitloom_prelude::GroundType::UInt { width } => width,
            _ => 1,
        };
        let input = p.direction == PortDirection::Input;
        writeln!(
            tb,
            "{} [{}:0] {}{};",
            if input { "reg" } else { "wire" },
            width - 1,
            p.name,
            if input { "=0" } else { "" }
        )
        .unwrap();
    }
    tb.push_str("DecoderSystem dut(.*);\nalways @(posedge clk) begin\n");
    for b in banks() {
        for r in b.registers {
            if r.access == CsrAccess::Rw {
                writeln!(tb,"if(rst) {0}_{1}_value<=0; else if({0}_{1}_write_commit) {0}_{1}_value<={0}_{1}_candidate;",b.name,r.name).unwrap();
            }
        }
    }
    tb.push_str("end\n");
    tb
}

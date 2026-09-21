use super::*;
use bitloom_prelude::{
    ElaborateSession, Span,
    ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister},
};
// One SV source serves the real hierarchy and the monitor sensitivity probes.
const PRODUCER_MONITOR: &str = include_str!("producer_monitor.sv");

fn monitor_probe_tb(stimulus: &str) -> String {
    format!(
        r#"module tb;
reg clk=0, rst=0;
reg s_axi_awvalid=0,s_axi_awready=0,s_axi_wvalid=0,s_axi_wready=0;
reg s_axi_arvalid=0,s_axi_arready=0,csr_rsp_valid=0,csr_rsp_ready=0;
reg [15:0] s_axi_awaddr=0,s_axi_araddr=0;
reg [2:0] s_axi_awprot=0,s_axi_arprot=0;
reg [31:0] s_axi_wdata=0,csr_rdata=0;
reg [3:0] s_axi_wstrb=0;
reg [1:0] csr_error=0;
{PRODUCER_MONITOR}
task tick; begin #1;monitor_producers();clk=1;#1;clk=0;end endtask
initial begin
 rst=1;tick();rst=0;
 {stimulus}
 $display("FR196 PASS monitor probe");$finish;
end
initial begin #1000;$fatal(1,"monitor probe watchdog");end
endmodule
"#
    )
}

#[test]
fn p0_sv_producer_monitor_rejects_each_hold_violation_and_accepts_release() {
    // [P0] Validate the actual integration monitor under Icarus, independently
    // of the Rust monitor. These are monitor probes, not product-DUT mutants.
    let start = Instant::now();
    let mut negative = 0;
    let mut positive = 0;
    for (channel, valid, ready, fields, diagnostic) in [
        (
            "aw",
            "s_axi_awvalid",
            "s_axi_awready",
            ["s_axi_awaddr", "s_axi_awprot"],
            "AW producer hold violation",
        ),
        (
            "w",
            "s_axi_wvalid",
            "s_axi_wready",
            ["s_axi_wdata", "s_axi_wstrb"],
            "W producer hold violation",
        ),
        (
            "ar",
            "s_axi_arvalid",
            "s_axi_arready",
            ["s_axi_araddr", "s_axi_arprot"],
            "AR producer hold violation",
        ),
        (
            "csr-rsp",
            "csr_rsp_valid",
            "csr_rsp_ready",
            ["csr_rdata", "csr_error"],
            "CSR responder hold violation",
        ),
    ] {
        // Nonzero initial payload, then alter one field at a time. The other
        // producers remain idle, making the expected fatal unambiguous.
        let held = format!(
            "{}=2;{}=2;{valid}=1;{ready}=0;repeat(3)tick();",
            fields[0], fields[1]
        );
        for accepting in [0, 1] {
            for mutation in [valid, fields[0], fields[1]] {
                let value = if mutation == valid { 0 } else { 3 };
                let tb = monitor_probe_tb(&format!(
                    "{held}{ready}={accepting};{mutation}={value};tick();"
                ));
                let label = format!("sv-monitor-{channel}-ready{accepting}-{mutation}");
                tools::run_monitor_probe(&label, &tb, Some(diagnostic));
                negative += 1;
                println!(
                    "monitor negative accepted: channel={channel} ready={accepting} mutation={mutation} exact_fatal={diagnostic}"
                );
            }
        }
        let accepted = monitor_probe_tb(&format!(
            "{held}{ready}=1;tick();{valid}=0;{}=3;{}=3;tick();",
            fields[0], fields[1]
        ));
        tools::run_monitor_probe(
            &format!("sv-monitor-{channel}-stable-acceptance"),
            &accepted,
            None,
        );
        positive += 1;
        let canceled = monitor_probe_tb(&format!(
            "{held}rst=1;{valid}=0;{}=3;{}=3;tick();rst=0;tick();{valid}=1;tick();{ready}=1;tick();{valid}=0;tick();",
            fields[0], fields[1]
        ));
        tools::run_monitor_probe(
            &format!("sv-monitor-{channel}-reset-cancellation"),
            &canceled,
            None,
        );
        positive += 1;
    }
    assert_eq!((negative, positive), (24, 8));
    println!(
        "SV monitor probes: native_test_entries=1 negative={negative} positive={positive} elapsed={:?}",
        start.elapsed()
    );
}

fn bank() -> CsrBlock {
    CsrBlock {
        name: "BridgeProbe".into(),
        registers: [
            (
                "control",
                0,
                CsrAccess::Rw,
                CsrOwner::Leaf,
                0x80ff00ff,
                None,
                false,
                false,
            ),
            (
                "status",
                4,
                CsrAccess::Ro,
                CsrOwner::External,
                0xff,
                None,
                true,
                false,
            ),
            (
                "tx",
                8,
                CsrAccess::Wo,
                CsrOwner::None,
                0xff,
                None,
                false,
                true,
            ),
            (
                "events",
                12,
                CsrAccess::W1c,
                CsrOwner::Leaf,
                0x8000000f,
                Some("hw_events"),
                false,
                false,
            ),
        ]
        .into_iter()
        .map(
            |(name, offset, access, owner, mask, event, read_reject, write_reject)| CsrRegister {
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
fn hierarchy() -> FrozenHir {
    let bridge = AxiLiteCsrBridge::elaborate().unwrap();
    let bank = bank();
    let leaf = bank.elaborate("BridgeLeaf").unwrap();
    let mut s = ElaborateSession::new("BridgeSystem");
    assert_eq!(
        AxiLiteCsrBridge::define_module(&mut s, "Bridge").unwrap(),
        "Bridge"
    );
    assert_eq!(
        AxiLiteCsrBridge::define_module(&mut s, "Bridge").unwrap(),
        "Bridge"
    );
    bank.define_module(&mut s, "BridgeLeaf").unwrap();
    let sp = Span::default();
    s.begin_module("BridgeSystem", sp);
    let mut bc = vec![];
    for p in &bridge.circuit().modules[0].ports {
        if p.name.starts_with("csr_") {
            s.add_output(p.name.clone(), p.ty.clone(), sp);
        } else if p.direction == PortDirection::Input {
            s.add_input(p.name.clone(), p.ty.clone(), sp);
        } else {
            s.add_output(p.name.clone(), p.ty.clone(), sp);
        }
        bc.push((p.name.clone(), p.name.clone()));
    }
    let mut lc = vec![];
    for p in &leaf.circuit().modules[0].ports {
        let name = if [
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
        ]
        .contains(&p.name.as_str())
        {
            format!("csr_{}", p.name)
        } else {
            if p.name != "clk" && p.name != "rst" {
                if p.direction == PortDirection::Input {
                    s.add_input(p.name.clone(), p.ty.clone(), sp);
                } else {
                    s.add_output(p.name.clone(), p.ty.clone(), sp);
                }
            }
            p.name.clone()
        };
        lc.push((p.name.clone(), name));
    }
    s.add_instance("bridge", "Bridge", bc, vec![], sp);
    s.add_instance("leaf", "BridgeLeaf", lc, vec![], sp);
    s.end_module();
    s.finish().unwrap()
}
#[test]
fn p0_same_session_bridge_real_csr_accesses_snapshots_and_single_side_effects() {
    let hir = hierarchy();
    // Real hierarchy remains outside both native tick engines and generated FL.
    // The RTL run below, not a flattened substitute, supplies behavior evidence.
    let reject = |payload: Box<dyn std::any::Any + Send>| {
        let message = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("hierarchical simulation is unsupported"),
            "{message}"
        );
    };
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        reject(
            std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine))
                .err()
                .expect("hierarchy must reject"),
        );
    }
    reject(
        std::panic::catch_unwind(|| bitloom_sim::GeneratedFunctional::from_hir(&hir))
            .err()
            .expect("generated hierarchy must reject"),
    );
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "BridgeSystem")
        .unwrap();
    let mut tb = String::from("module tb;\n");
    for p in &top.ports {
        let w = match p.ty {
            GroundType::UInt { width } => width,
            GroundType::Clock | GroundType::Reset => 1,
            _ => panic!(),
        };
        writeln!(
            tb,
            "{} [{}:0] {}{};",
            if p.direction == PortDirection::Input {
                "reg"
            } else {
                "wire"
            },
            w - 1,
            p.name,
            if p.direction == PortDirection::Input {
                "=0"
            } else {
                ""
            }
        )
        .unwrap();
    }
    tb.push_str(PRODUCER_MONITOR);
    tb.push_str(r#"
BridgeSystem dut(.*);
integer submits=0, consumes=0, bs=0, rs=0, cw=0, tw=0, ew=0, cr=0, sr=0, er=0;
reg [31:0] golden=0, mask;
integer st, k, phase, before_s, before_c, before_b, before_r, before_w;
reg snapshot_change=0, collide=0; integer collisions=0;
task tick; begin
 #1;
 // Inject a natural event only on the actual successful clear commit.
 if(collide && csr_req_valid && csr_req_ready && csr_write && csr_addr==12) begin
   hw_events=32'h00000003;collisions=collisions+1;
 end
 #1;
 monitor_producers();
 if (!rst) begin
  if(csr_req_valid && csr_req_ready) submits=submits+1;
  if(csr_rsp_valid && csr_rsp_ready) consumes=consumes+1;
  if(s_axi_bvalid && s_axi_bready) bs=bs+1;
  if(s_axi_rvalid && s_axi_rready) rs=rs+1;
  cw=cw+control_write_commit;tw=tw+tx_write_commit;ew=ew+events_write_commit;
  cr=cr+control_read_commit;sr=sr+status_read_commit;er=er+events_read_commit;
 end
 clk=1;#1;clk=0;#1;
 if(collide)hw_events=0;
end endtask
task wr(input[15:0] a,input[31:0] d,input[3:0] strobe,input[1:0] err,input integer effects);
 integer age,start_effects,start_submits,start_bs; reg ah,wh; begin
 start_effects=cw+tw+ew;start_submits=submits;start_bs=bs;
 s_axi_awaddr=a;s_axi_awprot=7;s_axi_wdata=d;s_axi_wstrb=strobe;s_axi_awvalid=1;s_axi_wvalid=1;ah=0;wh=0;age=0;
 while(!ah||!wh) begin
  #1;ah=ah||(s_axi_awvalid&&s_axi_awready);wh=wh||(s_axi_wvalid&&s_axi_wready);tick();
  if(ah)s_axi_awvalid=0;if(wh)s_axi_wvalid=0;age=age+1;if(age>128)$fatal(1,"write acceptance timeout");
 end
 age=0;while(!s_axi_bvalid)begin tick();age=age+1;if(age>128)$fatal(1,"B timeout");end
 if(s_axi_bresp!==err)$fatal(1,"BRESP addr=%h",a);
 repeat(31)begin if(!s_axi_bvalid||s_axi_bresp!==err)$fatal(1,"B hold");tick();end
 if(cw+tw+ew-start_effects!=effects)$fatal(1,"write side effect addr=%h",a);
 if(submits-start_submits!=1)$fatal(1,"write duplicate/drop");
 s_axi_bready=1;tick();s_axi_bready=0;if(bs-start_bs!=1)$fatal(1,"B count");
 end endtask
task rd(input[15:0] a,input[31:0] data,input[1:0] err,input integer effects);
 integer age,start_effects,start_submits,start_rs;reg accepted;begin
 start_effects=cr+sr+er;start_submits=submits;start_rs=rs;
 s_axi_araddr=a;s_axi_arprot=5;s_axi_arvalid=1;accepted=0;age=0;
 while(!accepted)begin #1;accepted=s_axi_arready;tick();age=age+1;if(age>128)$fatal(1,"AR timeout");end
 s_axi_arvalid=0;
 if(snapshot_change)begin
  if(submits!=start_submits)$fatal(1,"snapshot stimulus missed precommit window");
  status_value=32'h0000005a;
 end
 age=0;while(!s_axi_rvalid)begin tick();age=age+1;if(age>128)$fatal(1,"R timeout");end
 if(s_axi_rresp!==err||s_axi_rdata!==data)$fatal(1,"R addr=%h got=%h want=%h",a,s_axi_rdata,data);
 status_value=32'h12345678;status_read_reject=1;
 repeat(31)begin if(!s_axi_rvalid||s_axi_rresp!==err||s_axi_rdata!==data)$fatal(1,"snapshot changed");tick();end
 if(cr+sr+er-start_effects!=effects)$fatal(1,"read side effect addr=%h",a);
 if(submits-start_submits!=1)$fatal(1,"read duplicate/drop");
 s_axi_rready=1;tick();s_axi_rready=0;status_read_reject=0;if(rs-start_rs!=1)$fatal(1,"R count");
 end endtask
initial begin
 rst=1;tick();rst=0;
 for(st=0;st<16;st=st+1) begin
  mask=0;for(k=0;k<4;k=k+1)if((st>>k)&1)mask=mask|(32'hff<<(8*k));mask=mask&32'h80ff00ff;
  // Each mask starts with contrasting data, then exercises both polarities.
  wr(0,32'h80ff00ff,15,0,1);
  golden=32'h80ff00ff&~mask;
  wr(0,0,st,0,mask!=0);rd(0,golden,0,1);
  wr(0,0,15,0,1);
  golden=mask;
  wr(0,32'hffffffff,st,0,mask!=0);rd(0,golden,0,1);
  wr(4,32'hffffffff,st,2,0);
  wr(8,32'h000000a5,st,0,(st&1)!=0);
  rd(8,0,2,0);
 end
 status_value=32'hdeadbea5;snapshot_change=1;rd(4,32'h5a,0,1);snapshot_change=0;
 status_read_reject=1;rd(4,0,2,0);
 tx_write_reject=1;wr(8,32'hff,15,2,0);wr(8,32'hff,0,0,0);tx_write_reject=0;
 hw_events=32'h8000000f;tick();hw_events=0;rd(12,32'h8000000f,0,1);
 collide=1;wr(12,32'h00000003,15,0,1);collide=0;
 if(collisions!=1)$fatal(1,"clear/event commit collision missing");
 rd(12,32'h8000000f,0,1);
 wr(1,1,15,2,0);wr(2,1,15,2,0);wr(3,1,15,2,0);wr(16,1,15,2,0);
 wr(16'h8000,1,15,2,0);wr(16'hfffc,1,15,2,0);wr(16'hffff,1,15,2,0);
 rd(1,0,2,0);rd(2,0,2,0);rd(3,0,2,0);rd(16,0,2,0);rd(16'h8000,0,2,0);rd(16'hfffc,0,2,0);rd(16'hffff,0,2,0);
 rd(0,golden,0,1);
 if(submits!=consumes||consumes!=bs+rs)$fatal(1,"transaction accounting");
 // Common resets at real offer, committed side effect, held B and held R.
 for(phase=0;phase<4;phase=phase+1)begin
  before_s=submits;before_c=consumes;before_b=bs;before_r=rs;before_w=cw;
  if(phase<3)begin
   s_axi_awaddr=0;s_axi_wdata=32'h80ff00ff;s_axi_wstrb=15;
   s_axi_awvalid=1;s_axi_wvalid=1;#1;
   if(!s_axi_awready||!s_axi_wready)$fatal(1,"reset setup capture");
   tick();s_axi_awvalid=0;s_axi_wvalid=0;
  end else begin s_axi_araddr=0;s_axi_arvalid=1;tick();s_axi_arvalid=0;end
  k=0;while(!csr_req_valid)begin tick();k=k+1;if(k>32)$fatal(1,"offer watchdog");end
  if(phase>0)begin
   tick(); // This edge commits and updates the real leaf state.
   if(submits!=before_s+1)$fatal(1,"expected actual commit");
   if(phase<3 && (cw!=before_w+1 || control_value!==32'h80ff00ff))$fatal(1,"side effect before reset");
  end
  if(phase>=2)begin
   k=0;while(!(s_axi_bvalid||s_axi_rvalid))begin tick();k=k+1;if(k>32)$fatal(1,"held response watchdog");end
   if((phase==2 && (!s_axi_bvalid||s_axi_rvalid))||(phase==3&&(!s_axi_rvalid||s_axi_bvalid)))$fatal(1,"wrong held response class");
   repeat(7)tick();
   if(consumes!=before_c+1)$fatal(1,"CSR response not consumed");
  end
  rst=1;tick();rst=0;
  if(control_value!==0||events_value!==0||s_axi_bvalid||s_axi_rvalid)$fatal(1,"common reset state");
  s_axi_bready=1;s_axi_rready=1;repeat(12)tick();s_axi_bready=0;s_axi_rready=0;
  if(submits!=before_s+(phase!=0)||cw!=before_w+(phase>0&&phase<3)||
     consumes!=before_c+(phase>=2)||bs!=before_b||rs!=before_r)$fatal(1,"cancellation/no replay accounting");
  $display("common reset phase=%0d canceled_submit=%0d canceled_CSR_response=%0d no_AXI_replay=1",phase,submits-before_s,consumes-before_c);
  before_s=submits;before_c=consumes;before_b=bs;before_r=rs;before_w=cw;
  wr(0,32'h55,15,0,1);rd(0,32'h55,0,1);
  if(submits!=before_s+2||consumes!=before_c+2||bs!=before_b+1||rs!=before_r+1||cw!=before_w+1)$fatal(1,"recovery counts");
 end
 $display("FR196 PASS integration submits=%0d consumes=%0d B=%0d R=%0d CW=%0d TW=%0d EW=%0d",submits,consumes,bs,rs,cw,tw,ew);$finish;
end
initial begin #1000000;$fatal(1,"integration watchdog");end
endmodule
"#);
    tools::run_rtl("real-csr-integration", &hir, &tb);
}
#[test]
fn p0_public_ports_and_two_same_session_bridge_instances_have_private_state() {
    let standalone = AxiLiteCsrBridge::elaborate().unwrap();
    let ports = &standalone.circuit().modules[0].ports;
    let expected = [
        ("clk", true, 1),
        ("rst", true, 1),
        ("s_axi_awaddr", true, 16),
        ("s_axi_awprot", true, 3),
        ("s_axi_awvalid", true, 1),
        ("s_axi_wdata", true, 32),
        ("s_axi_wstrb", true, 4),
        ("s_axi_wvalid", true, 1),
        ("s_axi_bready", true, 1),
        ("s_axi_araddr", true, 16),
        ("s_axi_arprot", true, 3),
        ("s_axi_arvalid", true, 1),
        ("s_axi_rready", true, 1),
        ("s_axi_awready", false, 1),
        ("s_axi_wready", false, 1),
        ("s_axi_bvalid", false, 1),
        ("s_axi_bresp", false, 2),
        ("s_axi_arready", false, 1),
        ("s_axi_rvalid", false, 1),
        ("s_axi_rdata", false, 32),
        ("s_axi_rresp", false, 2),
        ("csr_req_valid", false, 1),
        ("csr_write", false, 1),
        ("csr_addr", false, 16),
        ("csr_wdata", false, 32),
        ("csr_wstrb", false, 4),
        ("csr_rsp_ready", false, 1),
        ("csr_req_ready", true, 1),
        ("csr_rsp_valid", true, 1),
        ("csr_rdata", true, 32),
        ("csr_error", true, 2),
    ];
    assert_eq!(ports.len(), expected.len());
    for (n, input, width) in expected {
        let p = ports.iter().find(|p| p.name == n).unwrap();
        assert_eq!(p.direction == PortDirection::Input, input, "{n}");
        assert_eq!(
            p.ty,
            if n == "clk" {
                GroundType::Clock
            } else if n == "rst" {
                GroundType::Reset
            } else {
                GroundType::UInt { width }
            },
            "{n}"
        );
    }
    let mut s = ElaborateSession::new("DualBridge");
    AxiLiteCsrBridge::define_module(&mut s, "SharedBridge").unwrap();
    AxiLiteCsrBridge::define_module(&mut s, "SharedBridge").unwrap();
    let sp = Span::default();
    s.begin_module("DualBridge", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for prefix in ["a", "b"] {
        let mut connections = vec![];
        for p in ports {
            let name = if p.name == "clk" || p.name == "rst" {
                p.name.clone()
            } else {
                format!("{prefix}_{}", p.name)
            };
            if p.name != "clk" && p.name != "rst" {
                if p.direction == PortDirection::Input {
                    s.add_input(name.clone(), p.ty.clone(), sp);
                } else {
                    s.add_output(name.clone(), p.ty.clone(), sp);
                }
            }
            connections.push((p.name.clone(), name));
        }
        s.add_instance(prefix, "SharedBridge", connections, vec![], sp);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    let mut a = Harness::new(TickEngine::Compiled);
    let mut b = Harness::new(TickEngine::Compiled);
    a.issue(true, 0x8000, 0x11223344, 15, 1, 31, true);
    a.issue(false, 3, 0, 0, 0, 0, false);
    a.drain();
    b.issue(false, 6, 0, 0, 7, 0, false);
    b.issue(true, 0xffff, 0xaabbccdd, 5, 4, 7, false);
    b.drain();
    while a.frames.len() < b.frames.len() {
        a.idle(1, true, true)
    }
    while b.frames.len() < a.frames.len() {
        b.idle(1, true, true)
    }
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "DualBridge")
        .unwrap();
    let mut tb = String::from("module tb;\n");
    for p in &top.ports {
        let w = match p.ty {
            GroundType::UInt { width } => width,
            _ => 1,
        };
        writeln!(
            tb,
            "{} [{}:0] {}{};",
            if p.direction == PortDirection::Input {
                "reg"
            } else {
                "wire"
            },
            w - 1,
            p.name,
            if p.direction == PortDirection::Input {
                "=0"
            } else {
                ""
            }
        )
        .unwrap();
    }
    tb.push_str("DualBridge dut(.*);initial begin\n");
    for cycle in 0..a.frames.len() {
        for (prefix, h) in [("a", &a), ("b", &b)] {
            for (n, v) in &h.frames[cycle].0 {
                let name = if n == "clk" || n == "rst" {
                    n.clone()
                } else {
                    format!("{prefix}_{n}")
                };
                writeln!(tb, "{name}=64'h{v:x};").unwrap();
            }
        }
        tb.push_str("#1;\n");
        if cycle > 0 {
            for (prefix, h) in [("a", &a), ("b", &b)] {
                for (n, v) in &h.frames[cycle].1 {
                    writeln!(tb,"if({prefix}_{n} !== 64'h{v:x}) $fatal(1,\"dual {prefix} cycle{cycle} {n}\");").unwrap();
                }
            }
        }
        tb.push_str("clk=1;#1;clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
    tools::run_rtl("dual-state", &hir, &tb);
}

#[test]
fn p0_bridge_definition_and_connection_diagnostics() {
    let check = |d: bitloom_prelude::Diagnostics, code: &str| {
        assert!(d.0.iter().any(|e| e.code == code), "{d:?}");
    };
    let mut invalid = ElaborateSession::new("Top");
    check(
        AxiLiteCsrBridge::define_module(&mut invalid, "bad-name").unwrap_err(),
        "rhdl::E0241",
    );
    check(invalid.finish().unwrap_err(), "rhdl::E0241");
    let mut conflict = ElaborateSession::new("Top");
    bitloom_prelude::ip::RvRegSlice::<32>::define_module(&mut conflict, "Shared").unwrap();
    check(
        AxiLiteCsrBridge::define_module(&mut conflict, "Shared").unwrap_err(),
        "rhdl::E0244",
    );
    check(conflict.finish().unwrap_err(), "rhdl::E0244");
    for direction in [false, true] {
        let mut s = ElaborateSession::new("Top");
        AxiLiteCsrBridge::define_module(&mut s, "Bridge").unwrap();
        s.begin_module("Top", Span::default());
        let mut connections = vec![];
        let hir = AxiLiteCsrBridge::elaborate().unwrap();
        for p in &hir.circuit().modules[0].ports {
            let ty = if !direction && p.name == "s_axi_awaddr" {
                GroundType::UInt { width: 8 }
            } else {
                p.ty.clone()
            };
            if p.direction == PortDirection::Input || (direction && p.name == "s_axi_bvalid") {
                s.add_input(&p.name, ty, Span::default());
            } else {
                s.add_output(&p.name, ty, Span::default());
            }
            connections.push((p.name.clone(), p.name.clone()));
        }
        s.add_instance("bridge", "Bridge", connections, vec![], Span::default());
        s.end_module();
        check(
            s.finish().unwrap_err(),
            if direction {
                "rhdl::E0257"
            } else {
                "rhdl::E0203"
            },
        );
    }
}

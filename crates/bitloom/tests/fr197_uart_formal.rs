//! Story128.5: independent UART public CSR safety subset, reachability, and raw synthesis.
use bitloom_prelude::{Elaboratable, ip::UartCsr};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
fn run(dir: &Path, tool: &str, args: &[&str], label: &str) {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
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
        .unwrap();
    fs::write(dir.join(format!("{label}.exit")), format!("{status}\n")).unwrap();
    fs::write(dir.join(format!("{label}-command.json")),serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; {}",
        dir.display()
    );
}
fn design(stage: &str) -> (PathBuf, String) {
    let hir = UartCsr::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart-formal")
        .join(format!(
            "{stage}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("design.v"), &source).unwrap();
    (dir, source)
}
// Independent response ghost plus local full-width countdown properties.
// Internal timer assertions supplement public bus safety, not complete serial
// sampling/FIFO/configuration equivalence. No DUT/ghost agreement is assumed.
const OBSERVER: &str = r#"
reg f_started=0, f_pending=0, f_cancelled=0;
reg [3:0] f_stall=0;
reg f_zero_response=0, f_static_error=0, f_static_ok=0;
wire f_submit=req_valid && !f_pending && !rst;
wire f_known=(addr==0 || addr==4 || addr==8 || addr==12 || addr==16 || addr==20);
wire f_bad_access=!f_known || (write && (addr==8 || addr==16)) || (!write && addr==12);
always @(posedge clk) begin
 f_started<=1;
 if(!f_started) assume(rst);
 if(f_started) begin
  uart_ready: assert(req_ready==(!f_pending && !rst));
  uart_pending: assert(rsp_valid==f_pending);
  uart_raw_reset: assert(!rst || raw_events==0);
  if(f_pending) begin
   uart_zero_response: assert(!f_zero_response || rdata==0);
   uart_static_error: assert(!f_static_error || error==2);
   uart_static_ok: assert(!f_static_ok || error==0);
  end
  if($past(rst)) begin
   uart_reset_response: assert(!rsp_valid && rdata==0 && error==0);
   uart_reset_tx: assert(tx);
  end
  if($past(rsp_valid && !rsp_ready && !rst)) begin
   uart_stalled_valid: assert(rsp_valid);
   uart_stalled_data: assert(rdata==$past(rdata));
   uart_stalled_error: assert(error==$past(error));
  end
  if(!$past(rst)) begin
   if($past(!tx_busy) && !tx_busy) begin
    uart_tx_idle_timer_hold: assert(tx_timer==$past(tx_timer));
   end
   if($past(rx_state==0) && rx_state==0) begin
    uart_rx_idle_timer_hold: assert(rx_timer==$past(rx_timer));
   end
   if($past(tx_busy && tx_timer!=0)) begin
    uart_tx_countdown: assert(tx_timer==$past(tx_timer)-32'd1);
   end
   if($past(tx_busy && tx_timer==0)) begin
    uart_tx_reload_full_width: assert(tx_timer==$past(tx_div));
   end
   if($past(rx_state!=0 && rx_timer!=0)) begin
    uart_rx_countdown: assert(rx_timer==$past(rx_timer)-32'd1);
   end
   if($past(rx_state!=0 && rx_timer==0)) begin
    uart_rx_reload_full_width: assert(rx_timer==$past(rx_div));
   end
  end
  // Legal-master stability only. No DUT/ghost correspondence is assumed.
  if($past(req_valid && !req_ready && !rst) && !rst) begin
   assume(req_valid); assume(write==$past(write)); assume(addr==$past(addr));
   assume(wdata==$past(wdata)); assume(wstrb==$past(wstrb));
  end
  cover(!rst && f_pending && f_stall>=5);
  cover(!rst && f_cancelled && f_submit);
  cover(!rst && f_pending && f_static_error && error==2);
 end
 if(rst) begin
  f_pending<=0; f_stall<=0;
  f_zero_response<=0; f_static_error<=0; f_static_ok<=0;
  if(f_started && f_pending) f_cancelled<=1;
 end else begin
  if(f_pending && !rsp_ready && f_stall<15) f_stall<=f_stall+1;
  if(f_pending && rsp_ready) begin f_pending<=0; f_stall<=0; end
  if(f_submit) begin
   f_pending<=1;
   f_zero_response<=write || f_bad_access;
   f_static_error<=f_bad_access;
   f_static_ok<=!write && (addr==0 || addr==4 || addr==8 || addr==20);
  end
 end
end
"#;
const OWNERSHIP_OBSERVER: &str = r#"// Independent register/configuration/occupancy accounting. Input transactions
// decide software side effects; raw serial events decide engine side effects.
reg [2:0] f_tx_count=0, f_rx_count=0;
reg f_enable=0;
reg [31:0] f_div=0, f_events=0;
wire [31:0] f_mask={{8{wstrb[3]}},{8{wstrb[2]}},{8{wstrb[1]}},{8{wstrb[0]}}};
wire [31:0] f_div_candidate=(f_div & ~f_mask)|(wdata & f_mask);
wire f_busy=tx_busy || rx_state!=0;
wire f_ctrl_reject=(f_busy && wdata[0]!=f_enable)||(wdata[0] && f_div<3);
wire f_div_reject=f_busy||(f_enable && f_div_candidate<3);
wire f_ctrl_write=f_submit && write && addr==0 && wstrb[0] && !f_ctrl_reject;
wire f_div_write=f_submit && write && addr==4 && wstrb!=0 && !f_div_reject;
wire f_tx_push=f_submit && write && addr==12 && wstrb[0] && f_tx_count<4;
wire f_rx_pop=f_submit && !write && addr==16 && f_rx_count!=0;
wire [31:0] f_clear=(f_submit && write && addr==20)?wdata & f_mask & 15:0;
always @(posedge clk) begin
 if(f_started) begin
  uart_tx_occupancy: assert(f_tx_count==f_actual_tx_count && f_tx_count<=4 && txq_input_ready==(f_tx_count<4) && txq_output_valid==(f_tx_count!=0));
  uart_rx_occupancy: assert(f_rx_count==f_actual_rx_count && f_rx_count<=4 && rxq_input_ready==(f_rx_count<4) && rxq_output_valid==(f_rx_count!=0));
  uart_tx_unique_push: assert(tx_data_write_commit==f_tx_push);
  uart_rx_unique_pop: assert(rx_data_read_commit==f_rx_pop);
  uart_ctrl_configuration: assert(ctrl_value=={31'b0,f_enable});
  uart_div_configuration: assert(baud_div_value==f_div);
  uart_event_set_priority: assert(EVENT_value==f_events);
  if(!rst) begin
   uart_no_empty_dequeue: assert(!raw_events[1] || f_tx_count!=0);
   uart_no_full_enqueue: assert(!raw_events[0] || f_rx_count<4);
   uart_no_rx_on_error: assert(!(raw_events[0] && (raw_events[2] || raw_events[3])));
  end
 end
 if(rst) begin
  f_tx_count<=0;f_rx_count<=0;f_enable<=0;f_div<=0;f_events<=0;
 end else begin
  case({f_tx_push,raw_events[1]})
   2'b10:f_tx_count<=f_tx_count+1;2'b01:f_tx_count<=f_tx_count-1;
  endcase
  case({raw_events[0],f_rx_pop})
   2'b10:f_rx_count<=f_rx_count+1;2'b01:f_rx_count<=f_rx_count-1;
  endcase
  if(f_ctrl_write)f_enable<=wdata[0];
  if(f_div_write)f_div<=f_div_candidate;
  f_events<=(f_events & ~f_clear)|{28'b0,raw_events};
 end
end
"#;
fn instrument(source: &str) -> String {
    // Read-only proof probes: exposing the actual count strengthens the
    // inductive invariant in the middle occupancy states. Equality is asserted,
    // never assumed. Only the formal copy gains these observer ports.
    let fifo = "module BitloomUartCsrFifo8x4 (";
    assert_eq!(source.matches(fifo).count(), 1);
    let mut formal = source.replace(
        fifo,
        &format!("{fifo}\n  output wire [2:0] observed_count,"),
    );
    let start = formal.find(fifo).unwrap();
    let end = start + formal[start..].find("endmodule").unwrap();
    formal.insert_str(end, "assign observed_count=count;\n");
    for lane in ["tx", "rx"] {
        let instance = format!("BitloomUartCsrFifo8x4 {lane}q (");
        assert_eq!(formal.matches(&instance).count(), 1);
        formal = formal.replace(
            &instance,
            &format!("{instance}\n .observed_count(f_actual_{lane}_count),"),
        );
    }
    let start = formal.find("module UartCsr (").expect("frozen top module");
    let body = start + formal[start..].find(");").unwrap() + 2;
    formal.insert_str(body, "\nwire [2:0] f_actual_tx_count,f_actual_rx_count;\n");
    let end = start + formal[start..].find("endmodule").unwrap();
    formal.insert_str(end, &format!("{OBSERVER}\n{OWNERSHIP_OBSERVER}"));
    formal
}

fn sby_file(dir: &Path, name: &str, source: &str, tasks: bool) {
    fs::write(dir.join(format!("{name}.v")), instrument(source)).unwrap();
    let options = if tasks {
        "[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 16\ncover: depth 24\n"
    } else {
        "[options]\nmode prove\ndepth 16\n"
    };
    fs::write(dir.join(format!("{name}.sby")),format!("{options}[engines]\nsmtbmc --unroll z3\n[script]\nread -formal {name}.v\nprep -top UartCsr -flatten\n[files]\n{name}.v\n")).unwrap();
}
#[test]
#[ignore = "dedicated actual SBY public-bus safety subset and cover"]
fn p0_uart_public_bus_safety_and_cover() {
    let (dir, source) = design("proof");
    fs::write(
        dir.join("observer.sv"),
        format!("{OBSERVER}\n{OWNERSHIP_OBSERVER}"),
    )
    .unwrap();
    sby_file(&dir, "uart", &source, true);
    for (tool, arg) in [("sby", "--version"), ("yosys", "-V"), ("z3", "-version")] {
        run(&dir, tool, &[arg], tool);
    }
    for task in ["prove", "cover"] {
        run(&dir, "sby", &["-f", "uart.sby", task], task);
        assert_eq!(
            fs::read_to_string(dir.join(format!("uart_{task}/status")))
                .unwrap()
                .split_whitespace()
                .next(),
            Some("PASS")
        );
    }
    println!(
        "UartCsr bus/config/event/occupancy/timer safety subset/three covers {}",
        dir.display()
    );
}
#[test]
#[ignore = "dedicated original, uninstrumented Yosys synthesis"]
fn p0_uart_original_synthesis() {
    let (dir, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "version");
    run(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog design.v; hierarchy -check -top UartCsr; proc; check -assert; synth -top UartCsr -flatten; check -assert; stat; write_json synthesis.json",
        ],
        "synthesis",
    );
    let net: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = net["modules"]["UartCsr"]["cells"].as_object().unwrap();
    assert!(
        cells
            .values()
            .any(|v| v["type"].as_str().unwrap().contains("DFF"))
    );
    assert!(
        !cells
            .values()
            .any(|v| v["type"].as_str().unwrap().contains("LATCH"))
    );
    println!(
        "UartCsr original synthesis cells={} {}",
        cells.len(),
        dir.display()
    );
}
// Keep the ANSI public port, disconnect its original driver in the top body,
// and supply a wrong constant. Child port labels remain unchanged. This changes
// actual DUT RTL without depending on any private register/net name.
fn ready_fault(source: &str) -> String {
    let top = source.find("module UartCsr (").unwrap();
    let body = top + source[top..].find(");").unwrap() + 2;
    let end = body + source[body..].find("endmodule").unwrap();
    let input = &source[body..end];
    let mut replaced = String::new();
    let mut count = 0;
    let mut chars = input.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        if c.is_ascii_alphabetic() || c == '_' {
            let mut finish = start + c.len_utf8();
            while let Some(&(i, n)) = chars.peek() {
                if n.is_ascii_alphanumeric() || n == '_' {
                    chars.next();
                    finish = i + n.len_utf8();
                } else {
                    break;
                }
            }
            let token = &input[start..finish];
            if token == "req_ready" && !input[..start].trim_end().ends_with('.') {
                replaced.push_str("mutation_original_req_ready");
                count += 1;
            } else {
                replaced.push_str(token);
            }
        } else {
            replaced.push(c);
        }
    }
    assert!(count > 0, "public ready must have a driver/use");
    format!(
        "{}\nwire mutation_original_req_ready;\n{}\nassign req_ready = 1'b0;\n{}",
        &source[..body],
        replaced,
        &source[end..]
    )
}
#[test]
#[ignore = "actual SBY ready-driver fault must fail uart_ready with VCD"]
fn p0_uart_observer_rejects_ready_driver_fault() {
    let (dir, source) = design("mutation");
    fs::write(
        dir.join("observer.sv"),
        format!("{OBSERVER}\n{OWNERSHIP_OBSERVER}"),
    )
    .unwrap();
    sby_file(&dir, "control", &source, false);
    run(&dir, "sby", &["-f", "control.sby"], "control");
    assert_eq!(
        fs::read_to_string(dir.join("control/status"))
            .unwrap()
            .split_whitespace()
            .next(),
        Some("PASS")
    );
    let mutant = ready_fault(&source);
    assert_ne!(mutant, source);
    fs::write(dir.join("ready-stuck-low-dut.v"), &mutant).unwrap();
    sby_file(&dir, "ready-stuck-low", &mutant, false);
    let log = fs::File::create(dir.join("ready-stuck-low.log")).unwrap();
    let start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args([
            "--kill-after=5s",
            "180s",
            "sby",
            "-f",
            "ready-stuck-low.sby",
        ])
        .current_dir(&dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(dir.join("ready-stuck-low.exit"), format!("{status}\n")).unwrap();
    fs::write(dir.join("ready-stuck-low-command.json"),serde_json::json!({"command":["timeout","--kill-after=5s","180s","sby","-f","ready-stuck-low.sby"],"exit_code":status.code(),"start_utc_unix_ms":start,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),"expected_property":"uart_ready"}).to_string()).unwrap();
    assert_eq!(status.code(), Some(2), "not ERROR/UNKNOWN/timeout");
    assert_eq!(
        fs::read_to_string(dir.join("ready-stuck-low/status"))
            .unwrap()
            .split_whitespace()
            .next(),
        Some("FAIL")
    );
    let base =
        fs::read_to_string(dir.join("ready-stuck-low/engine_0/logfile_basecase.txt")).unwrap();
    assert!(
        base.lines()
            .any(|line| line.contains("Assert failed") && line.contains("uart_ready")),
        "{base}"
    );
    let trace = fs::read_to_string(dir.join("ready-stuck-low/engine_0/trace.vcd")).unwrap();
    assert!(trace.contains("$enddefinitions") && trace.contains("#0"));
    println!(
        "UartCsr original PASS and ready fault specified counterexample PASS {}",
        dir.display()
    );
}

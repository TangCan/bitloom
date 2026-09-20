//! FR195 / 126.3：实际生成RTL的安全归纳证明与非空cover；缺工具严格失败。
use bitloom_prelude::{Elaboratable, ip::RvRegSlice};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    process::{Command, Stdio},
};

fn run(dir: &Path, tool: &str, args: &[&str], log_name: &str) {
    let log = fs::File::create(dir.join(log_name)).unwrap();
    // timeout退出124同样视为失败，保留完整工具日志；不可skip/UNKNOWN冒充证明。
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required formal tool runner");
    assert!(
        status.success(),
        "{tool} {args:?} failed: {status}; see {}",
        dir.join(log_name).display()
    );
}

fn harness(width: u32) -> String {
    format!(
        r#"  reg past_valid = 0;
  reg [1:0] pending = 0;
  reg [{last}:0] q0, q1;
  reg seen_full = 0;
  wire ghost_push = input_valid && input_ready;
  wire ghost_pop = output_valid && output_ready;
  always @(posedge clk) begin
    past_valid <= 1;
    if (!past_valid) assume(rst);
    if (past_valid) begin
      // Strengthening assertions, never assumptions: relate hidden storage to
      // the independent port-transaction queue so arbitrary full stalls induct.
      assert(pending == count);
      if (pending == 2) assert(back == q1);
      assert(pending <= 2);
      assert(input_ready == (pending < 2));
      assert(output_valid == (pending != 0));
      if (output_valid) assert(output_data == q0);
      assert(!ghost_push || pending < 2);
      assert(!ghost_pop || pending != 0);
      if ($past(output_valid && !output_ready && !rst && !flush)) begin
        assert(output_valid);
        assert(output_data == $past(output_data));
      end
      // 合法生产者受阻时保持；reset/flush显式取消该epoch。
      if ($past(input_valid && !input_ready && !rst && !flush) && !rst && !flush) begin
        assume(input_valid);
        assume(input_data == $past(input_data));
      end
      cover(seen_full && pending == 0);
      cover(!rst && !flush && pending == 1 && ghost_push && ghost_pop);
      // [P1] Full-slot reset cancellation can accept a fresh transaction next cycle.
      cover($past(pending == 2 && rst) && pending == 0 && !rst && !flush && ghost_push);
      // [P1] Flush alone cancels a full slot and permits next-cycle recovery.
      cover($past(pending == 2 && !rst && flush) && pending == 0 && !rst && !flush && ghost_push);
    end
    if (rst || flush) begin
      pending <= 0;
      seen_full <= 0;
    end else begin
      if (pending == 2) seen_full <= 1;
      // 独立端口事务队列：同时握手先交付队首，再接受新payload。
      case ({{ghost_push,ghost_pop}})
        2'b10: begin
          if (pending == 0) q0 <= input_data;
          else q1 <= input_data;
          pending <= pending + 1;
        end
        2'b01: begin q0 <= q1; pending <= pending - 1; end
        2'b11: begin q0 <= input_data; end
      endcase
    end
  end
"#,
        last = width - 1
    )
}

fn input_path(
    bit: u64,
    inputs: &HashSet<u64>,
    drivers: &HashMap<u64, Vec<u64>>,
    seen: &mut HashSet<u64>,
) -> bool {
    if inputs.contains(&bit) {
        return true;
    }
    assert!(seen.insert(bit), "combinational cycle at bit {bit}");
    let sources = drivers
        .get(&bit)
        .unwrap_or_else(|| panic!("undriven bit {bit}"));
    let result = sources
        .iter()
        .any(|source| input_path(*source, inputs, drivers, seen));
    seen.remove(&bit);
    result
}

// Yosys string literals 0/1/x/z are constants, including optimization don't-cares.
// Every numeric net must resolve to a driver (or fail as an input path).
fn signal_bit(value: &Value) -> Option<u64> {
    if let Some(bit) = value.as_u64() {
        Some(bit)
    } else {
        assert!(
            matches!(value.as_str(), Some("0" | "1" | "x" | "z")),
            "unresolved bit {value}"
        );
        None
    }
}

fn assert_registered_outputs(json: &str, top: &str) {
    let data: Value = serde_json::from_str(json).unwrap();
    let module = &data["modules"][top];
    let ports = module["ports"].as_object().unwrap();
    let mut input_bits = HashSet::new();
    for port in ports.values().filter(|port| port["direction"] == "input") {
        input_bits.extend(
            port["bits"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(signal_bit),
        );
    }
    let mut drivers = HashMap::new();
    for cell in module["cells"].as_object().unwrap().values() {
        let kind = cell["type"].as_str().unwrap();
        // proc/opt之后的edge-triggered FF切断组合追踪；latch绝不视作合法寄存边界。
        let ff = matches!(kind, "$dff" | "$dffe" | "$sdff" | "$sdffe" | "$sdffce");
        let directions = cell["port_directions"].as_object().unwrap();
        let connections = cell["connections"].as_object().unwrap();
        let sources: Vec<u64> = if ff {
            vec![]
        } else {
            connections
                .iter()
                .filter(|(name, _)| directions[*name] == "input")
                .flat_map(|(_, bits)| bits.as_array().unwrap().iter().filter_map(signal_bit))
                .collect()
        };
        for (_, bits) in connections
            .iter()
            .filter(|(name, _)| directions[*name] == "output")
        {
            for bit in bits.as_array().unwrap().iter().filter_map(signal_bit) {
                assert!(
                    drivers.insert(bit, sources.clone()).is_none(),
                    "multiple drivers for bit {bit}"
                );
            }
        }
    }
    for name in ["input_ready", "output_valid", "output_data"] {
        for bit in ports[name]["bits"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(signal_bit)
        {
            assert!(
                !input_path(bit, &input_bits, &drivers, &mut HashSet::new()),
                "input combinational path to {name} bit {bit}"
            );
        }
    }
}

fn formal<const WIDTH: u32>() {
    let hir = RvRegSlice::<WIDTH>::elaborate().unwrap();
    let top = &hir.circuit().modules[0].name;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr195-formal")
        .join(format!("width{WIDTH}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|file| file.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), &source).unwrap();
    let assertions = harness(WIDTH);
    fs::write(dir.join("harness.sv"), &assertions).unwrap();
    // Append only observer logic to the emitted module; original RTL is retained
    // separately and used for structural isolation. No hierarchical implicit nets.
    assert_eq!(source.matches("endmodule").count(), 1);
    let instrumented = source.replace("endmodule", &format!("{assertions}\nendmodule"));
    fs::write(dir.join("design_formal.v"), instrumented).unwrap();
    let config = format!(
        "[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 4\ncover: depth 12\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal design_formal.v\nprep -top {top}\n[files]\ndesign_formal.v\n"
    );
    fs::write(dir.join("slice.sby"), config).unwrap();
    for (tool, args) in [
        ("yosys", vec!["-V"]),
        ("sby", vec!["--version"]),
        ("z3", vec!["-version"]),
    ] {
        run(&dir, tool, &args, &format!("{tool}-version.log"));
    }
    let script = format!(
        "read_verilog design.v; hierarchy -top {top}; proc; flatten; opt; write_json structure.json"
    );
    fs::write(
        dir.join("commands.log"),
        format!("yosys -p {script}\nsby -f slice.sby prove\nsby -f slice.sby cover\n"),
    )
    .unwrap();
    run(&dir, "yosys", &["-p", &script], "structure.log");
    assert_registered_outputs(
        &fs::read_to_string(dir.join("structure.json")).unwrap(),
        top,
    );
    for task in ["prove", "cover"] {
        run(
            &dir,
            "sby",
            &["-f", "slice.sby", task],
            &format!("{task}.log"),
        );
        let status = fs::read_to_string(dir.join(format!("slice_{task}/status"))).unwrap();
        assert_eq!(
            status.split_whitespace().next(),
            Some("PASS"),
            "{task} cannot claim PASS from {status}"
        );
    }
    println!(
        "FR195 formal width={WIDTH} prove+cover PASS artifacts={}",
        dir.display()
    );
}

#[test]
#[ignore = "requires real Yosys/SymbiYosys/Z3; run this dedicated target with --ignored; ordinary workspace tests do not prove FR195"]
fn p0_generated_rtl_width1_safety_induction_cover_and_registered_outputs() {
    formal::<1>();
}
#[test]
#[ignore = "requires real Yosys/SymbiYosys/Z3; run this dedicated target with --ignored; ordinary workspace tests do not prove FR195"]
fn p0_generated_rtl_width8_safety_induction_cover_and_registered_outputs() {
    formal::<8>();
}

#[test]
fn structural_checker_rejects_input_paths_and_unresolved_bits() {
    use serde_json::json;
    fn circuit(output: Value, cells: Value) -> String {
        json!({"modules":{"Test":{"ports":{
            "din":{"direction":"input","bits":[1]},
            "input_ready":{"direction":"output","bits":[output]},
            "output_valid":{"direction":"output","bits":["1"]},
            "output_data":{"direction":"output","bits":["0"]}
        },"cells":cells}}})
        .to_string()
    }
    fn cell(kind: &str, input: Value, output: u64) -> Value {
        json!({"type":kind,"port_directions":{"A":"input","Y":"output"},
            "connections":{"A":[input],"Y":[output]}})
    }
    let direct = circuit(json!(1), json!({}));
    let chain = circuit(
        json!(3),
        json!({"a":cell("$not",json!(1),2),"b":cell("$not",json!(2),3)}),
    );
    let missing = circuit(json!(3), json!({"a":cell("$not",json!(2),3)}));
    let unknown = circuit(json!("unresolved"), json!({}));
    for invalid in [direct, chain, missing, unknown] {
        assert!(std::panic::catch_unwind(|| assert_registered_outputs(&invalid, "Test")).is_err());
    }
    // A real edge-register boundary cuts the path; ordinary combinational logic
    // following the register must still be traversed. Constants need no driver.
    let registered = circuit(
        json!(3),
        json!({"ff":cell("$dff",json!(1),2),"out":cell("$not",json!(2),3)}),
    );
    assert_registered_outputs(&registered, "Test");
    let constant = circuit(json!(3), json!({"out":cell("$not",json!("0"),3)}));
    assert_registered_outputs(&constant, "Test");
}

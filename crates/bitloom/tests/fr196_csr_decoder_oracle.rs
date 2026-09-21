//! [P0] Story127.4: prove the independent WO observer detects corrupted emitted
//! leaf payloads after real AXI requests traverse the seven-module hierarchy.
#[path = "fr196_csr_decoder/common.rs"]
mod common;

use common::{bench, dir, hierarchy, run};
use std::{
    fs,
    process::{Command, Stdio},
    time::Instant,
};

// Golden addresses and expected masked payloads are requirements, not extracted
// from the CsrBlock descriptors or emitted expressions. Reuse the existing bench
// builder and transaction tasks; no product or shared fixture mutation is needed.
fn exercise_wo_observer(
    register: &str,
    address: u16,
    data: u32,
    strobe: u8,
    expected_mask: u32,
    expected_candidate: u32,
) {
    assert_ne!(expected_mask, 0, "control must exercise a real write mask");
    assert_ne!(expected_candidate, 0, "zero mutation must be observable");
    let hir = hierarchy();
    let rtl = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut tb = bench(&hir);
    tb.push_str(include_str!("fr196_csr_decoder/scoreboard.sv"));
    tb.push_str(&format!(
        r#"
initial begin
 for(j=0;j<5;j=j+1)windows[j]=0;
 for(j=0;j<4;j=j+1)errors[j]=0;
 for(j=0;j<16;j=j+1)strobes[j]=0;
 reset_all();
 transact(1,16'h{address:04x},32'h{data:08x},4'h{strobe:x},3,1,7);
 if(accepted_aw!==1||accepted_w!==1||accepted_ar!==0||commits!==1||leaf_commits!==1||csr_responses!==1||consumed_b!==1||consumed_r!==0||wo_effects!==1||side_effects!==1||errors[0]!==1)
  $fatal(1,"WO control transaction ledger");
 if(byte_mask!==32'h{expected_mask:08x}||(transaction_data&byte_mask)!==32'h{expected_candidate:08x})
  $fatal(1,"WO control independent golden");
 $display("WO CONTROL PASS {register} mask=%h candidate=%h",byte_mask,transaction_data&byte_mask);
 $finish;
end
initial begin #1000000;$fatal(1,"WO watchdog");end
endmodule
"#
    ));

    // A passing original control plus two independent single-output mutants per
    // register. Each mutation starts with the original RTL, never another mutant.
    for suffix in [None, Some("candidate"), Some("write_mask")] {
        let label = suffix.unwrap_or("control");
        let dir = dir(&format!("wo-oracle-{register}-{label}"));
        let design = if let Some(suffix) = suffix {
            let output = format!("{register}_{suffix}");
            let prefix = format!("assign {output} = ");
            let assignments: Vec<_> = rtl
                .lines()
                .filter(|line| line.trim_start().starts_with(&prefix))
                .collect();
            assert_eq!(assignments.len(), 1, "unique emitted assignment: {output}");
            let original = assignments[0];
            assert!(original.trim_end().ends_with(';'));
            assert_eq!(rtl.matches(original).count(), 1);
            let replacement = format!("  assign {output} = 32'h00000000;");
            assert_ne!(original, replacement);
            let mutated = rtl.replacen(original, &replacement, 1);
            assert_eq!(mutated.matches(&replacement).count(), 1);
            fs::write(
                dir.join("mutation.json"),
                serde_json::json!({"output": output, "original": original, "replacement": replacement, "replacements": 1}).to_string(),
            )
            .unwrap();
            mutated
        } else {
            rtl.clone()
        };
        fs::write(dir.join("design.v"), design).unwrap();
        fs::write(dir.join("tb.sv"), &tb).unwrap();
        // Compilation must succeed even for mutants: syntax/tool failures never
        // count as evidence that the payload scoreboard detects wrong hardware.
        run(
            &dir,
            "iverilog",
            &[
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
            "compile",
        );
        if suffix.is_none() {
            let output = run(&dir, "vvp", &["simulation"], "run");
            assert!(
                output.contains(&format!("WO CONTROL PASS {register}")),
                "{output}"
            );
            continue;
        }
        let log = fs::File::create(dir.join("run.log")).unwrap();
        let started = Instant::now();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "60s", "vvp", "simulation"])
            .current_dir(&dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .expect("required vvp mutation simulation");
        let output = fs::read_to_string(dir.join("run.log")).unwrap();
        let expected = format!("WO payload {address:08x}");
        fs::write(
            dir.join("run.json"),
            serde_json::json!({
                "command": ["timeout", "--kill-after=5s", "60s", "vvp", "simulation"],
                "exit_code": status.code(),
                "elapsed_seconds": started.elapsed().as_secs_f64(),
                "expected_rejection": expected,
                "register": register,
                "mutated_output": label
            })
            .to_string(),
        )
        .unwrap();
        assert_eq!(status.code(), Some(1), "{register}/{label}: {output}");
        assert!(
            output
                .lines()
                .any(|line| line.starts_with("FATAL: tb.sv:") && line.contains(&expected)),
            "{register}/{label}: expected actual scoreboard fatal, got {output}"
        );
        assert!(
            !output.contains("WO CONTROL PASS"),
            "mutant escaped: {output}"
        );
    }
}

#[test]
fn p0_uart_tx_data_observer_kills_candidate_and_mask_rtl_mutants() {
    exercise_wo_observer(
        "tx_data",
        0x000c,
        0xa5c3_7e96,
        0b0101,
        0x0000_00ff,
        0x0000_0096,
    );
}

#[test]
fn p0_gpio_set_observer_kills_candidate_and_mask_rtl_mutants() {
    exercise_wo_observer("set", 0x010c, 0xa5c3_7e96, 0b0101, 0x00ff_00ff, 0x00c3_0096);
}

#[test]
fn p0_gpio_clear_observer_kills_candidate_and_mask_rtl_mutants() {
    exercise_wo_observer(
        "clear",
        0x0110,
        0x5a3c_8169,
        0b1010,
        0xff00_ff00,
        0x5a00_8100,
    );
}

#[test]
fn p0_irq_test_observer_kills_candidate_and_mask_rtl_mutants() {
    exercise_wo_observer(
        "test",
        0x0308,
        0xd36a_95b5,
        0b1001,
        0x0000_001f,
        0x0000_0015,
    );
}

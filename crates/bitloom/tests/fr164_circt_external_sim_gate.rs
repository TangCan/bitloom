//! ATDD Story 96.2 / FR164 — external CIRCT simulation gate.
//!
//! ```text
//! cargo test -p bitloom --test fr164_circt_external_sim_gate
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr164_docs_contract_forbid_fr137_alone() {
    let docs = read("docs/fr164-circt-external-sim-gate.md");
    assert!(docs.contains("FR164") && docs.contains("Bitloom"));
    assert!(
        docs.contains("sim") || docs.contains("仿真") || docs.contains("execution"),
        "must name sim/execution gate"
    );
    assert!(
        docs.contains("FR137")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR137 distinct"
    );
    assert!(docs.contains("1.159.0") || docs.contains("firtool-1.159.0"));
    assert!(
        docs.contains("circt-external-sim-check") || docs.contains("circt-external-sim"),
        "must name sim-check path / CI job"
    );
    assert!(
        (docs.contains("MLIR") || docs.contains("NFR71") || docs.contains("全家桶"))
            && (docs.contains("Deferred") || docs.contains("不做") || docs.contains("新合同")),
        "must defer broader MLIR"
    );
}

#[test]
fn fr164_script_and_just_path() {
    let script = read("scripts/circt-external-sim-check.sh");
    assert!(script.contains("1.159.0"));
    assert!(script.contains("FR164"));
    assert!(
        script.contains("RHDL_FIRTOOL_PATH") || script.contains("firtool ensure"),
        "must resolve via AD-9"
    );
    assert!(
        script.contains("fr164_circt_sim_gate") || script.contains("cycle-sim"),
        "must run sim/execution predicate beyond compile"
    );
    assert!(
        script.contains("BITLOOM_CIRCT_SIM_FORCE_MISSING"),
        "must honor FORCE_MISSING"
    );
    let just = read("Justfile");
    assert!(just.contains("circt-external-sim-check"));
    assert!(just.contains("scripts/circt-external-sim-check.sh"));
}

#[test]
fn fr164_ci_required_job_no_continue_on_error() {
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-external-sim:"),
        "must define required circt-external-sim job"
    );
    let idx = ci.find("circt-external-sim:").expect("circt-external-sim");
    let block: String = ci[idx..].lines().take(40).collect::<Vec<_>>().join("\n");
    assert!(
        !block.contains("continue-on-error"),
        "FR164 required job must not continue-on-error"
    );
    assert!(
        block.contains("circt-external-sim-check"),
        "CI must run circt-external-sim-check"
    );
}

#[test]
fn fr164_force_missing_nonzero_readable() {
    let script = root().join("scripts/circt-external-sim-check.sh");
    assert!(script.is_file());
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_CIRCT_SIM_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("spawn");
    assert!(!out.status.success(), "FORCE_MISSING must fail non-zero");
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        err.contains("FORCE_MISSING")
            || err.contains("unavailable")
            || err.contains("missing")
            || err.contains("refusing silent"),
        "failure must be readable: {err}"
    );
}

#[test]
fn fr164_not_satisfied_by_fr137_compile_script_alone() {
    let fr137 = read("scripts/circt-external-check.sh");
    let fr164 = read("scripts/circt-external-sim-check.sh");
    assert!(
        fr137.contains("compile") || fr137.contains("FR137"),
        "FR137 script remains compile gate"
    );
    assert!(
        fr164.contains("FR164")
            && (fr164.contains("sim")
                || fr164.contains("cycle-sim")
                || fr164.contains("execution")),
        "FR164 script must deepen beyond compile naming"
    );
    assert_ne!(
        fr137, fr164,
        "FR164 must not be a copy-only rename of FR137 script"
    );
}

#[test]
fn fr164_sim_fixture_and_example_exist() {
    assert!(
        root()
            .join("crates/rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir")
            .is_file()
    );
    assert!(
        root()
            .join("crates/bitloom/examples/fr164_circt_sim_gate.rs")
            .is_file()
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md",
    );
    assert!(nfr.contains("仿真") || nfr.contains("sim"));
}

fn execute_sim_fixture(text: &str) -> std::process::Output {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("bitloom-fr164-{}-{unique}.fir", std::process::id()));
    fs::write(&path, text).expect("写入夹具");
    let result = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args([
            "run",
            "--offline",
            "--quiet",
            "-p",
            "bitloom",
            "--example",
            "fr164_circt_sim_gate",
            "--",
        ])
        .arg(&path)
        .current_dir(root())
        .output();
    fs::remove_file(path).expect("清理夹具");
    result.expect("执行真实仿真门禁示例")
}

#[test]
fn fr164_default_fixture_executes_and_constant_outputs_are_detected() {
    let text = read("crates/rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir");
    let good = execute_sim_fixture(&text);
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    assert!(String::from_utf8_lossy(&good.stdout).contains("sequence=00,a5,5a,ff,00"));
    for value in [0, 165] {
        let wrong = text.replace("y <= x", &format!("y <= UInt<8>({value})"));
        assert_ne!(wrong, text, "故障注入必须命中");
        let output = execute_sim_fixture(&wrong);
        assert!(!output.status.success(), "恒定输出必须使真实门禁失败");
        assert!(String::from_utf8_lossy(&output.stderr).contains("FR164 sim gate: expected y="));
    }
}

#[test]
#[should_panic(expected = "hierarchical simulation is unsupported")]
fn fr164_does_not_relax_hierarchy_rejection() {
    let text = "FIRRTL version 6.0.0\ncircuit Top :\n  module Child :\n    input clk: Clock\n    input rst: Reset\n    input x: UInt<8>\n    output y: UInt<8>\n    y <= x\n  module Top :\n    input clk: Clock\n    input rst: Reset\n    input x: UInt<8>\n    output y: UInt<8>\n    inst u0 of Child\n    u0.clk <= clk\n    u0.rst <= rst\n    u0.x <= x\n    y <= u0.y\n";
    let hir = bitloom_firrtl::import(text).expect("合法层级 HIR");
    let _ = bitloom_sim::Sim::new(hir);
}

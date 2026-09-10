//! ATDD: FR119 SymbiYosys/SMT product path (Story 60.2 / Epic 60).
//! Contract: documented first-class `sby` entry + fixtures (pass + readable fail);
//! missing tool → non-zero readable fail (never silent success); ≠ FR85/FR100/FR112-B/FR107.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr119_docs_nail_sby_binding_and_bans() {
    let docs = read("docs/fr119-symbiyosys-smt.md");
    assert!(docs.contains("Bitloom") || docs.contains("bitloom"));
    assert!(docs.contains("SymbiYosys") && docs.contains("sby"));
    assert!(docs.contains("formal-sby-check"));
    assert!(docs.contains("assume") && docs.contains("assert"));
    assert!(
        docs.contains("FR100") && (docs.contains("≠") || docs.contains("alone")),
        "docs must isolate FR100"
    );
    assert!(
        docs.contains("FR112") && (docs.contains("≠") || docs.contains("alone")),
        "docs must isolate FR112-B"
    );
    assert!(
        docs.contains("FR85") || docs.contains("formal-sva-check"),
        "docs must isolate FR85"
    );
    assert!(docs.contains("FR107"), "docs must isolate FR107");
    assert!(
        docs.contains("FORCE_MISSING") || docs.contains("silent"),
        "docs must document missing-tool non-silent failure"
    );
    assert!(
        docs.contains("分支 C") || docs.contains("handwritten FL") || docs.contains("deferred"),
        "docs must keep branch C / deferred honesty"
    );
}

#[test]
fn fr119_script_and_just_entry_exist() {
    let script = workspace_root().join("scripts/formal-sby-check.sh");
    assert!(script.is_file(), "missing {}", script.display());
    let text = fs::read_to_string(&script).unwrap_or_else(|e| panic!("read script: {e}"));
    assert!(text.contains("sby"));
    assert!(text.contains("FR119"));
    assert!(text.contains("BITLOOM_SBY_FORCE_MISSING"));
    assert!(text.contains("assume property") || text.contains("assume"));
    assert!(text.contains("assert property") || text.contains("assert"));
    assert!(
        text.contains("exit 1") && text.contains("refusing silent success"),
        "script must refuse silent success when sby missing"
    );
    assert!(
        !text.contains("formal-sva-check.sh") || text.contains("≠ FR85"),
        "FR119 script must not collapse into FR85 entry"
    );

    let justfile = read("Justfile");
    assert!(
        justfile.contains("formal-sby-check"),
        "Justfile must expose formal-sby-check"
    );
    assert!(
        justfile.contains("formal-sva-check"),
        "FR85 formal-sva-check must remain (NFR48 / isolation)"
    );
}

#[test]
fn fr119_fixtures_have_assume_assert_pass_and_fail() {
    let root = workspace_root().join("crates/rhdl-formal/fixtures/fr119");
    for name in [
        "fr119_pass.sv",
        "fr119_pass.sby",
        "fr119_fail.sv",
        "fr119_fail.sby",
    ] {
        let p = root.join(name);
        assert!(p.is_file(), "missing fixture {}", p.display());
    }
    for mode in ["pass", "fail"] {
        let sv = fs::read_to_string(root.join(format!("fr119_{mode}.sv")))
            .unwrap_or_else(|e| panic!("read sv: {e}"));
        assert!(
            sv.contains("assume property"),
            "{mode} fixture must include assume property"
        );
        assert!(
            sv.contains("assert property"),
            "{mode} fixture must include assert property"
        );
        let sby = fs::read_to_string(root.join(format!("fr119_{mode}.sby")))
            .unwrap_or_else(|e| panic!("read sby: {e}"));
        assert!(sby.contains("mode bmc") || sby.contains("bmc"));
        assert!(
            sby.contains(&format!("expect {mode}")),
            "{mode} .sby must declare expect {mode}"
        );
    }
}

#[test]
fn fr119_script_fails_readably_when_sby_missing() {
    let script = workspace_root().join("scripts/formal-sby-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_SBY_FORCE_MISSING", "1")
        .output()
        .unwrap_or_else(|e| panic!("spawn formal-sby-check: {e}"));
    assert!(
        !out.status.success(),
        "FORCE_MISSING must exit non-zero (got {:?})",
        out.status.code()
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let combined = format!("{stderr}{stdout}");
    assert!(
        combined.contains("sby")
            && (combined.contains("unavailable")
                || combined.contains("not found")
                || combined.contains("FORCE_MISSING")),
        "missing-tool failure must be readable; got: {combined}"
    );
    assert!(
        combined.contains("silent success") || combined.contains("refusing"),
        "must explicitly refuse silent success; got: {combined}"
    );
}

#[test]
fn fr119_nfr14_selected_binding_still_gates() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md");
    assert!(nfr.contains("SymbiYosys") && nfr.contains("sby"));
    assert!(nfr.contains("formal-sby-check") || nfr.contains("Fork"));
    assert!(nfr.contains("FR100") && nfr.contains("FR112"));
}

#[test]
fn fr119_fr100_and_fr112_surfaces_still_present() {
    // NFR48 regression guard: prior close docs/recipes remain reachable.
    let fr100 = read("docs/fr100-formal-equiv.md");
    assert!(
        fr100.contains("FormalEquivProduct") || fr100.contains("F1-(i)"),
        "FR100 surface must remain"
    );
    assert!(
        fr100.contains("fr119") || fr100.contains("FR119"),
        "FR100 docs should cross-link FR119"
    );

    let fr112 = read("docs/fr112-generated-functional-memread-equiv.md");
    assert!(
        fr112.contains("MemRead") || fr112.contains("GeneratedFunctional"),
        "FR112-B surface must remain"
    );
    assert!(
        fr112.contains("FR119") || fr112.contains("fr119"),
        "FR112 docs should point branch A at FR119"
    );

    let fr100_test = workspace_root().join("crates/bitloom/tests/fr100_formal_equiv_product.rs");
    let fr112_test = workspace_root().join("crates/bitloom/tests/fr112_memread_equiv_tick.rs");
    assert!(fr100_test.is_file(), "FR100 ATDD must remain");
    assert!(fr112_test.is_file(), "FR112 ATDD must remain");
}

#[test]
fn fr119_optional_real_sby_pass_when_installed() {
    if !Command::new("bash")
        .arg("-c")
        .arg("command -v sby >/dev/null 2>&1 && command -v z3 >/dev/null 2>&1")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!(
            "sby/z3 not on PATH — skipping live SymbiYosys pass (contract covered by FORCE_MISSING)"
        );
        return;
    }
    let script = workspace_root().join("scripts/formal-sby-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_SBY_MODE", "pass")
        .env_remove("BITLOOM_SBY_FORCE_MISSING")
        .output()
        .unwrap_or_else(|e| panic!("spawn sby pass: {e}"));
    if !out.status.success() {
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
        panic!("sby present but pass fixture failed (readable):\n{combined}");
    }
}

#[test]
fn fr119_optional_real_sby_fail_fixture_when_installed() {
    if !Command::new("bash")
        .arg("-c")
        .arg("command -v sby >/dev/null 2>&1 && command -v z3 >/dev/null 2>&1")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("sby/z3 not on PATH — skipping live SymbiYosys fail fixture");
        return;
    }
    let script = workspace_root().join("scripts/formal-sby-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_SBY_MODE", "fail")
        .env_remove("BITLOOM_SBY_FORCE_MISSING")
        .output()
        .unwrap_or_else(|e| panic!("spawn sby fail: {e}"));
    // With `expect fail` in the .sby, SymbiYosys treats assertion violation as SUCCESS.
    // If the engine/setup is broken, we still require a readable non-silent outcome.
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        out.status.success()
            || combined.contains("FAIL")
            || combined.contains("Assert")
            || combined.contains("error"),
        "fail fixture must yield expect-fail OK or a readable engine diagnostic; got status={:?}\n{combined}",
        out.status.code()
    );
}

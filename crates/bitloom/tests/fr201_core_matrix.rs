//! Story 129.3 ATDD red scaffold for the FR198/FR201 complete-system matrix.
use serde_json::Value;
use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn evidence() -> Value {
    let path = root().join("_agile-output/test-artifacts/129-3-latest-results.json");
    serde_json::from_str(&fs::read_to_string(path).expect("129.3 build evidence must exist"))
        .expect("129.3 evidence must be valid JSON")
}

#[test]
fn p0_three_backends_execute_the_complete_four_peripheral_system() {
    let value = evidence();
    for backend in ["direct", "firrtl", "chisel"] {
        let row = &value["backends"][backend];
        assert_eq!(row["exit_code"].as_i64(), Some(0), "{backend} must execute");
        assert!(
            row["transactions"].as_u64().unwrap_or(0) >= 16_000,
            "{backend} frozen transaction budget"
        );
        assert!(
            row["assertions"].as_u64().unwrap_or(0) > 0,
            "{backend} assertions"
        );
        assert!(
            row["vcd_bytes"].as_u64().unwrap_or(0) > 0,
            "{backend} non-empty VCD"
        );
        for feature in [
            "slverr",
            "decerr",
            "wstrb",
            "backpressure",
            "reset_cancel",
            "timer",
            "gpio",
            "uart_tx",
            "uart_rx",
            "irq_0_4",
        ] {
            assert_eq!(
                row["coverage"][feature].as_bool(),
                Some(true),
                "{backend} missing {feature}"
            );
        }
    }
}

#[test]
fn p0_formal_cover_negative_control_and_raw_synthesis_are_real() {
    let value = evidence();
    assert_eq!(value["formal"]["prove"]["status"], "PASS");
    assert_eq!(value["formal"]["cover"]["status"], "PASS");
    assert_eq!(value["formal"]["response_reset_bmc"]["status"], "PASS");
    assert_eq!(value["formal"]["prove"]["depth"].as_u64(), Some(4));
    assert_eq!(
        value["formal"]["response_reset_bmc"]["depth"].as_u64(),
        Some(8)
    );
    assert_eq!(value["formal"]["cover"]["depth"].as_u64(), Some(2));
    assert_eq!(
        value["formal"]["negative_control"]["status"],
        "EXPECTED_FAIL"
    );
    assert_eq!(
        value["formal"]["negative_control"]["counterexample_vcd"].as_bool(),
        Some(true)
    );
    assert!(value["formal"]["prove"]["depth"].as_u64().unwrap_or(0) > 0);
    for backend in ["direct", "firrtl", "chisel"] {
        assert_eq!(value["synthesis"][backend]["check_assert"], true);
        assert!(value["synthesis"][backend]["cells"].as_u64().unwrap_or(0) > 0);
    }
}

#[test]
fn p0_missing_tools_fail_and_isolated_checkout_replays() {
    let value = evidence();
    for tool in [
        "iverilog", "vvp", "yosys", "firtool", "java", "sbt", "sby", "solver",
    ] {
        assert_eq!(
            value["missing_tool_negative"][tool]["exit_code"].as_i64(),
            Some(4),
            "{tool} absence must use the required-tool failure code"
        );
    }
    assert_eq!(value["isolated_replay"]["exit_code"].as_i64(), Some(0));
    assert_eq!(
        value["isolated_replay"]["used_main_target"].as_bool(),
        Some(false)
    );
    assert_eq!(
        value["isolated_replay"]["used_hidden_tmp_rtl"].as_bool(),
        Some(false)
    );
}

#[test]
fn p0_evidence_producer_validated_owned_paths_and_rtl_hashes() {
    let value = evidence();
    assert_eq!(
        value["provenance"]["source_paths_validated"].as_bool(),
        Some(true)
    );
    assert_eq!(
        value["provenance"]["rtl_sha256_recomputed"].as_bool(),
        Some(true)
    );
}

#[test]
fn p1_semver_and_public_surface_claims_are_explicit() {
    let value = evidence();
    assert_eq!(value["semver"]["harness"], "PASS");
    assert_eq!(value["semver"]["registry"], "PASS");
    assert_eq!(
        value["semver"]["packages"],
        serde_json::json!(["bitloom-prelude", "bitloom-sim", "bitloom-firrtl"])
    );
    assert_eq!(value["public_api_added"], serde_json::json!([]));
    assert_eq!(value["package_versions_changed"].as_bool(), Some(false));
}

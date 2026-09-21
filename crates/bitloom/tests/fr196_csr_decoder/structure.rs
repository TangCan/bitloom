use serde_json::Value;
use std::collections::{HashMap, HashSet};
fn bit(v: &Value) -> Option<u64> {
    if let Some(b) = v.as_u64() {
        Some(b)
    } else {
        assert!(
            matches!(v.as_str(), Some("0" | "1")),
            "unknown/nonbinary structural constant {v}"
        );
        None
    }
}
fn path_to_input(
    b: u64,
    inputs: &HashSet<u64>,
    drivers: &HashMap<u64, Vec<u64>>,
    seen: &mut HashSet<u64>,
) -> bool {
    if inputs.contains(&b) {
        return true;
    }
    assert!(seen.insert(b), "combinational cycle {b}");
    let sources = drivers
        .get(&b)
        .unwrap_or_else(|| panic!("undriven bit {b}"));
    let found = sources
        .iter()
        .any(|s| path_to_input(*s, inputs, drivers, seen));
    seen.remove(&b);
    found
}
pub fn registered(json_text: &str, top: &str, boundary: &[&str]) -> usize {
    let value: Value = serde_json::from_str(json_text).unwrap();
    let module = &value["modules"][top];
    let ports = module["ports"].as_object().unwrap();
    let inputs: HashSet<u64> = ports
        .values()
        .filter(|p| p["direction"] == "input")
        .flat_map(|p| p["bits"].as_array().unwrap().iter().filter_map(bit))
        .collect();
    let clock = &ports["clk"];
    assert_eq!(clock["direction"], "input", "dedicated clk must be input");
    let clock_bits = clock["bits"].as_array().expect("dedicated clk bits");
    assert_eq!(clock_bits.len(), 1, "dedicated clk must be one bit");
    let clock_bit = bit(&clock_bits[0]).expect("clk cannot be constant");
    let mut drivers = HashMap::new();
    let mut ff_count = 0;
    for cell in module["cells"].as_object().unwrap().values() {
        let kind = cell["type"].as_str().unwrap();
        // Exact list: only rising-edge FFs; async reset, latches and unknown variants fail closed.
        let ff = matches!(
            kind,
            "$_DFF_P_"
                | "$_DFFE_PP_"
                | "$_DFFE_PN_"
                | "$_SDFF_PP0_"
                | "$_SDFF_PP1_"
                | "$_SDFF_PN0_"
                | "$_SDFF_PN1_"
                | "$_SDFFE_PP0P_"
                | "$_SDFFE_PP0N_"
                | "$_SDFFE_PP1P_"
                | "$_SDFFE_PP1N_"
                | "$_SDFFCE_PP0P_"
                | "$_SDFFCE_PP0N_"
                | "$_SDFFCE_PP1P_"
                | "$_SDFFCE_PP1N_"
        );
        let comb = matches!(
            kind,
            "$_BUF_"
                | "$_NOT_"
                | "$_AND_"
                | "$_NAND_"
                | "$_OR_"
                | "$_NOR_"
                | "$_XOR_"
                | "$_XNOR_"
                | "$_ANDNOT_"
                | "$_ORNOT_"
                | "$_MUX_"
                | "$_NMUX_"
        );
        assert!(ff || comb, "unsupported/latch/unknown cell {kind}");
        ff_count += usize::from(ff);
        let connections = cell["connections"].as_object().unwrap();
        let sources = if ff {
            assert_eq!(
                cell["port_directions"]["C"], "input",
                "FF missing input clock port"
            );
            let clocks = connections
                .get("C")
                .and_then(Value::as_array)
                .expect("FF missing C connection");
            assert_eq!(clocks.len(), 1, "FF clock must have one bit");
            assert_eq!(
                bit(&clocks[0]),
                Some(clock_bit),
                "FF clock must be the dedicated clk input"
            );
            vec![]
        } else {
            connections
                .iter()
                .filter(|(p, _)| cell["port_directions"][*p] == "input")
                .flat_map(|(_, b)| b.as_array().unwrap().iter().filter_map(bit))
                .collect()
        };
        for (_, bits) in connections
            .iter()
            .filter(|(p, _)| cell["port_directions"][*p] == "output")
        {
            for b in bits.as_array().unwrap().iter().filter_map(bit) {
                assert!(
                    drivers.insert(b, sources.clone()).is_none(),
                    "multiple driver {b}"
                );
            }
        }
    }
    assert!(ff_count > 0, "no sequential state");
    for name in boundary {
        for b in ports[*name]["bits"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(bit)
        {
            assert!(
                !path_to_input(b, &inputs, &drivers, &mut HashSet::new()),
                "input (including rst) path to {name}"
            );
        }
    }
    ff_count
}

use serde_json::json;
const BOUNDARY: [&str; 5] = [
    "s_axi_awready",
    "s_axi_wready",
    "s_axi_arready",
    "s_axi_bvalid",
    "s_axi_rvalid",
];
#[test]
fn p1_structural_observer_rejects_reset_paths_latches_unknowns_and_missing_drivers() {
    fn fixture(kind: &str, output: Value) -> String {
        json!({"modules":{"T":{"ports":{
            "rst":{"direction":"input","bits":[1]},
            "clk":{"direction":"input","bits":[3]},
            "s_axi_awready":{"direction":"output","bits":[output]},
            "s_axi_wready":{"direction":"output","bits":[2]},
            "s_axi_arready":{"direction":"output","bits":[2]},
            "s_axi_bvalid":{"direction":"output","bits":[2]},
            "s_axi_rvalid":{"direction":"output","bits":[2]}},
            "cells":{"ff":{"type":kind,"port_directions":{"C":"input","D":"input","Q":"output"},"connections":{"C":[3],"D":[1],"Q":[2]}}}}}}).to_string()
    }
    for (kind, output) in [
        ("$_DFF_P_", json!(1)),
        ("$_DLATCH_P_", json!(2)),
        ("$_DFF_N_", json!(2)),
        ("$_DFFE_NP_", json!(2)),
        ("$_DFFE_NN_", json!(2)),
        ("mystery", json!(2)),
        ("$_DFF_FAKE_", json!(2)),
        ("$_DFF_P_", json!(9)),
        ("$_DFF_P_", json!("x")),
    ] {
        assert!(
            std::panic::catch_unwind(|| registered(&fixture(kind, output), "T", &BOUNDARY))
                .is_err()
        );
    }
    for connection in [Some(json!([1])), None] {
        let mut bad: Value = serde_json::from_str(&fixture("$_DFF_P_", json!(2))).unwrap();
        let ports = bad["modules"]["T"]["cells"]["ff"]["connections"]
            .as_object_mut()
            .unwrap();
        if let Some(connection) = connection {
            ports.insert("C".into(), connection);
        } else {
            ports.remove("C");
        }
        let failure =
            std::panic::catch_unwind(|| registered(&bad.to_string(), "T", &BOUNDARY)).unwrap_err();
        let message = failure
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| failure.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("FF clock must be the dedicated clk input")
                || message.contains("FF missing C connection"),
            "{message}"
        );
    }
    assert_eq!(
        registered(&fixture("$_DFF_P_", json!(2)), "T", &BOUNDARY),
        1
    );
}

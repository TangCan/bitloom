//! ATDD matrix: FR76 + FR77 closure transparency (Story 29.4 / Cap-R-64 / NFR36).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix
//! ```
//!
//! Consolidates emit/viz spot-checks after closure dissolve. Deep fixtures:
//! - `fr76_hls_dataflow_closure` — dissolve-before-schedule + missing-backend
//! - `fr77_ip_generator_closure` — Crc8Lut customize emit/tick + violations

use std::path::PathBuf;

use bitloom::hls::{HlsDataflowOp, dissolve_dataflow_transform, run_hls_dataflow_with_backend};
use bitloom_prelude::ip::{Crc8Lut, crc8_table_byte};

/// NFR36 / Cap-R-64: backends and viz must not retain closure/callback IR tokens.
fn assert_no_closure_ir(label: &str, text: &str) {
    let lower = text.to_lowercase();
    let has_fn_ir = ["Fn(", "Fn (", "FnOnce", "FnMut", "dyn Fn"]
        .iter()
        .any(|needle| {
            let mut start = 0;
            while let Some(rel) = text[start..].find(needle) {
                let abs = start + rel;
                let prev_ok = abs == 0 || !text.as_bytes()[abs - 1].is_ascii_alphanumeric();
                if prev_ok {
                    return true;
                }
                start = abs + 1;
            }
            false
        });
    let bad =
        lower.contains("closure") || lower.contains("callback") || has_fn_ir || text.contains("||");
    assert!(
        !bad,
        "{label}: must not contain closure/callback IR (NFR36 / Cap-R-64):\n{text}"
    );
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn matrix_sibling_fixtures_present() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    for name in [
        "fr76_hls_dataflow_closure.rs",
        "fr77_ip_generator_closure.rs",
    ] {
        assert!(
            root.join(name).is_file(),
            "matrix consolidates sibling fixture {name}"
        );
    }
}

#[test]
fn matrix_fr76_hls_dissolve_c_and_rtl_no_closure_ir() {
    let dissolved =
        dissolve_dataflow_transform("map_xor", &[], || HlsDataflowOp::XorConst(0xa5)).unwrap();
    assert_no_closure_ir("matrix/fr76/dissolved.c_source", &dissolved.c_source);
    assert!(
        dissolved.c_source.contains("^ 165u"),
        "expected xor const: {}",
        dissolved.c_source
    );

    let root = workspace_root();
    let stub = root.join("scripts/fixtures/bambu-ci-stub.sh");
    assert!(stub.is_file(), "missing {}", stub.display());
    let out = root.join("target/fr76-fr77-matrix-hls-stub");
    let _ = std::fs::remove_dir_all(&out);

    let rtl = run_hls_dataflow_with_backend("map_xor", "xor_a5", &out, false, Some(&stub))
        .expect("stub HLS");
    let v = std::fs::read_to_string(&rtl).unwrap();
    assert_no_closure_ir("matrix/fr76/stub.v", &v);
    let c = std::fs::read_to_string(out.join("map_xor.c")).unwrap();
    assert_no_closure_ir("matrix/fr76/pre-schedule.c", &c);
}

#[test]
fn matrix_fr77_ip_viz_verilog_firrtl_transparent() {
    let poly = 0x1du8;
    let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| crc8_table_byte(i as u8, poly) as u64)
        .expect("custom table");

    let v = bitloom_vlog::emit(&hir).files[0].contents.clone();
    assert_no_closure_ir("matrix/fr77/verilog", &v);

    let fir = rhdl_firrtl::emit(&hir).files[0].contents.clone();
    assert_no_closure_ir("matrix/fr77/firrtl", &fir);

    // Cap-R-64: product viz (FR38 hierarchy HTML) must stay closure-opaque.
    let html = rhdl_viz::to_html(&hir);
    assert!(
        html.contains("Crc8Lut") || html.contains("Bitloom"),
        "viz should render hierarchy for Crc8Lut"
    );
    assert_no_closure_ir("matrix/fr77/viz.html", &html);

    let dbg = format!("{hir:?}");
    assert!(
        !dbg.to_lowercase().contains("closure")
            && !dbg.contains("FnOnce")
            && !dbg.contains("FnMut"),
        "FrozenHir debug must not retain closure types:\n{dbg}"
    );
}

//! ATDD / guardrail: AD-2 publish identity must be `bitloom` (Story 11.1 / FR43).
//! Red before AD-2 edit; green after spine + AGENTS align.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn ad2_publish_identity_is_bitloom_not_rhdl_rs() {
    let spine = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let text =
        fs::read_to_string(&spine).unwrap_or_else(|e| panic!("read {}: {e}", spine.display()));

    let ad2 = text
        .split("### AD-2")
        .nth(1)
        .and_then(|rest| rest.split("### AD-3").next())
        .expect("AD-2 section present");

    assert!(
        ad2.contains("`bitloom`") || ad2.contains("**`bitloom`**"),
        "AD-2 must name crates.io publish identity bitloom"
    );
    assert!(
        ad2.contains("发布名是 **`bitloom`**") || ad2.contains("crates.io 发布名是 **`bitloom`**"),
        "AD-2 Rule must state crates.io publish name is bitloom"
    );
    assert!(
        !ad2.contains("发布名是 **`rhdl-rs`**"),
        "AD-2 must not keep rhdl-rs as the positive publish identity"
    );
    assert!(
        ad2.contains("`rhdl`") && ad2.contains("`rhdl-bits`"),
        "AD-2 must still forbid crates.io rhdl / rhdl-bits"
    );

    let agents = workspace_root().join("AGENTS.md");
    let agents_text =
        fs::read_to_string(&agents).unwrap_or_else(|e| panic!("read {}: {e}", agents.display()));
    assert!(
        agents_text.contains("Bitloom") && agents_text.contains("`bitloom`"),
        "AGENTS.md must lock Bitloom / bitloom"
    );
    assert!(
        agents_text.contains("不用 `rhdl-rs`") || !agents_text.contains("CLI 发布名用 `rhdl-rs`"),
        "AGENTS.md must not prescribe rhdl-rs as publish name"
    );
}

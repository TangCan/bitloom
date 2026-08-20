//! ATDD: AD-2/AD-6 design dependency is `bitloom-prelude` (Story 13.1 / FR49).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn ad2_and_ad6_require_bitloom_prelude_not_bitloom_prelude() {
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
        ad2.contains("`bitloom-prelude`") || ad2.contains("**`bitloom-prelude`**"),
        "AD-2 must require design crates to depend on bitloom-prelude"
    );
    assert!(
        !ad2.contains("只依赖 **`rhdl-prelude`**"),
        "AD-2 must not keep rhdl-prelude as the sole allowed design dependency"
    );

    let ad6 = text
        .split("### AD-6")
        .nth(1)
        .and_then(|rest| rest.split("### AD-7").next())
        .expect("AD-6 section present");
    assert!(
        ad6.contains("`bitloom-prelude`") || ad6.contains("[bitloom-prelude]"),
        "AD-6 must name bitloom-prelude as the design dependency"
    );
    assert!(
        ad6.contains("[dependencies]` 只能是 `bitloom-prelude`")
            || ad6.contains("[dependencies]` 唯一允许 **`bitloom-prelude`**")
            || ad6.contains("只能是 `bitloom-prelude`"),
        "AD-6 Rule must restrict design [dependencies] to bitloom-prelude"
    );
    assert!(
        ad6.contains("不得依赖 CLI") || ad6.contains("不得依赖 CLI 包"),
        "AD-6 must forbid design crates depending on the CLI"
    );

    let agents = workspace_root().join("AGENTS.md");
    let agents_text =
        fs::read_to_string(&agents).unwrap_or_else(|e| panic!("read {}: {e}", agents.display()));
    assert!(
        agents_text.contains("bitloom-prelude"),
        "AGENTS.md must mention bitloom-prelude as the user-facing design dependency"
    );
}

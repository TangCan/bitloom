//! ATDD Story 24.4: README / HLS docs list HLS as a supported product feature (FR50).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn readme_and_fr35_list_hls_as_supported() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    let fr35 = fs::read_to_string(root.join("docs/fr35-hls.md")).expect("fr35");

    assert!(
        readme.contains("HLS") && (readme.contains("支持") || readme.contains("supported")),
        "README must list HLS as supported"
    );
    assert!(
        readme.contains("docs/fr35-hls.md"),
        "README must link HLS chapter"
    );
    assert!(
        readme.contains("hls-smoke") || readme.contains("scripts/hls-smoke.sh"),
        "README must link smoke/fixture"
    );
    assert!(
        readme.contains("Bambu") || readme.contains("bambu"),
        "README must name pinned backend"
    );
    assert!(
        !readme.contains("永久 unsupported")
            && !readme.contains("仅实验且无路径")
            && !readme
                .to_lowercase()
                .contains("hls permanently unsupported"),
        "README must not describe HLS as permanently unsupported / experiment-only"
    );

    assert!(
        fr35.contains("supports") || fr35.contains("支持"),
        "fr35 chapter must declare support"
    );
    assert!(
        fr35.contains("2024.10"),
        "fr35 must cite pinned Bambu version"
    );
    assert!(
        fr35.contains("hls-smoke") || fr35.contains("scripts/hls-smoke"),
        "fr35 must link smoke"
    );
    assert!(
        fr35.contains("调度") || fr35.contains("scheduler") || fr35.contains("scheduling"),
        "fr35 must discuss scheduling (in-tree FR95 and/or external)"
    );
    // After Epic 41: public README must not forbid in-tree scheduler as current AD-25.
    assert!(
        !readme.contains("不实现树内调度器"),
        "README must not claim Bitloom does not implement an in-tree scheduler"
    );
    assert!(
        readme.contains("FR95")
            && (readme.contains("树内")
                || readme.contains("--in-tree")
                || readme.contains("in-tree")),
        "README must document FR95 in-tree path after Epic 41"
    );
    assert!(
        !fr35.contains("永久 unsupported") && !fr35.contains("仅实验且无路径"),
        "fr35 must not use permanent-unsupported / experiment-only framing"
    );
}

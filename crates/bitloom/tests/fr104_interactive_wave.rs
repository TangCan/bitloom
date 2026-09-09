//! ATDD — Story 47.2 / FR104: interactive rich waveform product path.
//!
//! ```text
//! cargo test -p bitloom --test fr104_interactive_wave
//! ```

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

fn tempfile_dir(tag: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "bitloom-{tag}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn fr104_docs_interactive_contract() {
    let text = read("docs/fr104-interactive-wave.md");
    assert!(text.contains("FR104"), "must name FR104");
    assert!(text.contains("Bitloom"), "must brand Bitloom");
    assert!(
        (text.contains("I1") || text.contains("浏览") || text.contains("browse"))
            && (text.contains("I2") || text.contains("缩放") || text.contains("zoom"))
            && (text.contains("I3") || text.contains("检索") || text.contains("search")),
        "must nail I1–I3 interactive acceptance"
    );
    assert!(
        text.contains("interactive.html")
            || text.contains("interactive_wave")
            || text.contains("交互"),
        "must name interactive product artifact / path"
    );
    assert!(
        (text.contains("timing.html") || text.contains("静态"))
            && (text.contains("≠")
                || text.contains("not")
                || text.contains("Not")
                || text.contains("不是")
                || text.contains("alone")
                || text.contains("不足以")),
        "must contrast ≠ static timing.html alone"
    );
    assert!(
        text.contains("VCD")
            || text.contains("wave.vcd")
            || text.contains("AD-5")
            || text.contains("AD-24"),
        "must retain / cross-link default VCD path (AD-5/24)"
    );
    assert!(
        !text.contains("Epic 47 已关闭") && !text.contains("epic-47: done"),
        "must not claim Epic 47 closed in FR104 product doc"
    );
}

#[test]
fn fr104_docs_repro_steps() {
    let text = read("docs/fr104-interactive-wave.md");
    assert!(
        text.contains("cargo bitloom wave")
            && (text.contains("interactive.html") || text.contains("interactive")),
        "must document reproducible wave → interactive.html steps"
    );
}

#[test]
fn fr104_wave_emits_interactive_html_i1_i3() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("fr104-wave");
    let out = Command::new(bin)
        .args([
            "wave",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--ticks",
            "4",
        ])
        .output()
        .expect("run wave");
    assert!(
        out.status.success(),
        "wave failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let interactive = out_dir.join("interactive.html");
    assert!(
        interactive.is_file(),
        "must emit interactive.html (FR104); stdout={}",
        String::from_utf8_lossy(&out.stdout)
    );
    let html = fs::read_to_string(&interactive).expect("interactive.html");
    assert!(
        html.contains("Bitloom"),
        "interactive viewer must brand Bitloom"
    );
    assert!(
        html.contains("data-bitloom-interactive-wave") || html.contains("bitloom-interactive-wave"),
        "must mark product interactive wave root for ATDD"
    );
    // I1 browse — canvas / svg timeline
    assert!(
        html.contains("<canvas")
            || html.contains("<svg")
            || html.contains("timeline")
            || html.contains("wave-canvas"),
        "I1 browse: must include timeline/canvas surface:\n{html}"
    );
    // I2 zoom / pan
    let lower = html.to_lowercase();
    assert!(
        lower.contains("zoom") || lower.contains("pan") || html.contains("viewport"),
        "I2 zoom/pan: must include zoom/pan/viewport controls"
    );
    // I3 search / filter
    assert!(
        lower.contains("search")
            || lower.contains("filter")
            || html.contains("id=\"signal-search\"")
            || html.contains("signalSearch"),
        "I3 search: must include signal search/filter control"
    );
    // Must not be a GTKWave-only stub
    assert!(
        !lower.contains("please open gtkwave")
            || html.contains("not the sole")
            || html.contains("not</strong> the sole"),
        "must not be GTKWave-only instructions"
    );

    let _ = fs::remove_dir_all(&out_dir);
}

#[test]
fn fr104_vcd_and_timing_still_emitted() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("fr104-vcd");
    let out = Command::new(bin)
        .args([
            "wave",
            "--input",
            fir,
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--ticks",
            "2",
        ])
        .output()
        .expect("run wave");
    assert!(out.status.success());
    let vcd = fs::read_to_string(out_dir.join("wave.vcd")).expect("wave.vcd");
    assert!(!vcd.trim().is_empty(), "AD-5/24 VCD path must remain");
    let timing = fs::read_to_string(out_dir.join("timing.html")).expect("timing.html");
    assert!(
        timing.contains("Value table"),
        "FR38/49 timing.html must remain"
    );
    assert!(out_dir.join("interactive.html").is_file());
    let _ = fs::remove_dir_all(&out_dir);
}

#[test]
fn fr104_sprint_47_2_done_47_3_backlog_epic_in_progress() {
    let text = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        text.contains("47-2-交互式富波形-fr104: done")
            || text.contains("47-2-交互式富波形-fr104:done"),
        "47-2 must be done"
    );
    assert!(
        text.contains("47-3-仿真覆盖率扩展-fr105-收口: backlog")
            || text.contains("47-3-仿真覆盖率扩展-fr105-收口:backlog"),
        "47-3 must stay backlog"
    );
    assert!(
        text.contains("epic-47: in-progress") || text.contains("epic-47:in-progress"),
        "epic-47 must stay in-progress (not done)"
    );
    assert!(
        !text.contains("epic-47: done") && !text.contains("epic-47:done"),
        "must not close epic-47 in this story"
    );
}

#[test]
fn fr104_nfr14_47_2_checkable_not_epic_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic47-waveform-coverage.md");
    assert!(
        text.contains("47.2") && (text.contains("FR104") || text.contains("交互")),
        "NFR14 must still reference 47.2 / FR104"
    );
    // 47.2 close item should be checkable (checked after implementation)
    assert!(
        text.contains("[x] **47.2") || text.contains("[x] 47.2") || text.contains("- [x] **47.2"),
        "NFR14 47.2 / FR104 close item must be checked after this story"
    );
    assert!(
        text.contains("[ ] **47.3") || text.contains("[ ] 47.3") || text.contains("- [ ] **47.3"),
        "47.3 / FR105 must remain unchecked"
    );
    assert!(
        !text.contains("状态 | **closed**") && !text.contains("状态 | closed"),
        "must not mark Epic 47 NFR14 record closed yet"
    );
}

#[test]
fn fr104_fr38_cross_links_interactive() {
    let text = read("docs/fr38-wave.md");
    assert!(
        text.contains("fr104") || text.contains("FR104") || text.contains("interactive.html"),
        "fr38-wave must cross-link FR104 interactive path"
    );
    assert!(
        !(text.contains("Deferred — richer interactive wave")
            && text.contains("future epic")
            && !text.contains("FR104")),
        "must not leave interactive wave as future-only defer without FR104 pointer"
    );
}

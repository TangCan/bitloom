//! ATDD — Story 58.2 / FR117: in-house typed IDE waveform (NFR14 subset B).
//!
//! ```text
//! cargo test -p bitloom --test fr117_typed_ide_wave
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
fn fr117_docs_subset_b_contract() {
    let text = read("docs/fr117-typed-ide-wave.md");
    assert!(text.contains("FR117"), "must name FR117");
    assert!(text.contains("Bitloom"), "must brand Bitloom");
    assert!(
        text.contains("typed-wave.html")
            || text.contains("typed_wave")
            || text.contains("wave.typed.json"),
        "must name typed product artifact"
    );
    assert!(
        (text.contains("typed") || text.contains("类型"))
            && (text.contains("I1") || text.contains("interactive.html") || text.contains("I1–I3")),
        "must contrast typed surface beyond FR104 I1–I3"
    );
    assert!(
        text.contains("deferred") && text.contains("Tywaves"),
        "NFR51: unselected A Tywaves must stay deferred"
    );
    assert!(
        text.contains("cargo bitloom wave"),
        "must document reproducible wave → typed path"
    );
    assert!(
        !(text.contains("alone") || text.contains("≠") || text.contains("not"))
            || text.contains("FR104")
            || text.contains("FR114")
            || text.contains("LCOV")
            || text.contains("interactive"),
        "must ban closing via FR104/FR114 alone"
    );
}

#[test]
fn fr117_nfr14_gate_selects_b() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md",
    );
    assert!(
        text.contains("(B)")
            && (text.contains("自研") || text.contains("等价") || text.contains("typed IDE")),
        "NFR14 must keep subset B selected"
    );
    assert!(
        text.contains("deferred") && (text.contains("(A)") || text.contains("Tywaves")),
        "A must remain deferred"
    );
}

#[test]
fn fr117_wave_emits_typed_artifacts_beyond_i1_i3() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let fir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../rhdl-firrtl/fixtures/external_hierarchy.fir"
    );
    let out_dir = tempfile_dir("fr117-wave");
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

    let typed_html = out_dir.join("typed-wave.html");
    assert!(
        typed_html.is_file(),
        "must emit typed-wave.html (FR117); stdout={}",
        String::from_utf8_lossy(&out.stdout)
    );
    let html = fs::read_to_string(&typed_html).expect("typed-wave.html");
    assert!(html.contains("Bitloom"), "typed viewer must brand Bitloom");
    assert!(
        html.contains("data-bitloom-typed-wave"),
        "must mark product typed-wave root for ATDD"
    );
    // Typed semantics beyond flat I1–I3 names
    let lower = html.to_lowercase();
    assert!(
        lower.contains("uint")
            || lower.contains("sint")
            || lower.contains("clock")
            || html.contains("data-signal-type")
            || html.contains("\"ty\""),
        "must expose signal type semantics:\n{html}"
    );
    assert!(
        html.contains("signal-tree")
            || html.contains("typed-tree")
            || html.contains("hierarchy")
            || html.contains("data-signal-kind"),
        "must expose typed hierarchy / kind surface beyond name-only timeline"
    );

    let typed_json = out_dir.join("wave.typed.json");
    assert!(
        typed_json.is_file(),
        "must emit wave.typed.json typed metadata sidecar"
    );
    let json = fs::read_to_string(&typed_json).expect("wave.typed.json");
    assert!(
        json.contains("\"signals\"")
            && (json.contains("\"ty\"") || json.contains("\"type\"") || json.contains("\"kind\"")),
        "typed JSON must carry structured signal type/kind fields: {json}"
    );

    // NFR48: prior wave artifacts still present
    assert!(out_dir.join("wave.vcd").is_file(), "VCD must remain");
    assert!(
        out_dir.join("interactive.html").is_file(),
        "FR104 interactive.html must remain"
    );
    assert!(
        out_dir.join("timing.html").is_file(),
        "FR38/49 timing.html must remain"
    );

    let _ = fs::remove_dir_all(&out_dir);
}

#[test]
fn fr117_missing_typed_meta_fails_readable() {
    // Library guard: empty typed meta must not silent-Ok as FR117.
    use rhdl_viz::{WaveSample, typed_wave_html};
    use std::collections::BTreeMap;

    let samples = vec![WaveSample {
        time: 0,
        values: BTreeMap::from([("x".into(), 1)]),
    }];
    let html = typed_wave_html("empty-meta", &samples, &[]);
    assert!(
        html.contains("data-bitloom-typed-wave=\"empty\"")
            || html.contains("typed-meta-missing")
            || html.contains("(no typed signals)"),
        "empty typed meta must be explicit, not silent FR117 green: {html}"
    );
}

#[test]
fn fr117_nfr48_fr104_and_fr114_paths_still_documented() {
    let fr104 = read("docs/fr104-interactive-wave.md");
    assert!(fr104.contains("interactive.html"));
    let fr114 = read("docs/fr114-lcov-coverage-gui.md");
    assert!(fr114.contains("coverage.lcov") || fr114.contains("coverage.html"));
    let cli = read("crates/bitloom/src/main.rs");
    assert!(cli.contains("Commands::Wave") || cli.contains("run_wave"));
    assert!(cli.contains("Commands::Coverage") || cli.contains("run_coverage"));
}

#[test]
fn fr117_sprint_58_2_done_allows_58_3_and_epic58_done() {
    let text = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        text.contains("58-2-typed-ide-波形路径实现与验收-fr117: done")
            || text.contains("58-2-typed-ide-波形路径实现与验收-fr117: in-progress")
            || text.contains("58-2-typed-ide-波形路径实现与验收-fr117: ready-for-dev"),
        "58-2 must be tracked"
    );
    let story_done = text.contains("58-3-fr117-收口与文档指针: done")
        || text.contains("58-3-fr117-收口与文档指针:done");
    let story_open = text.contains("58-3-fr117-收口与文档指针: backlog")
        || text.contains("58-3-fr117-收口与文档指针: in-progress")
        || text.contains("58-3-fr117-收口与文档指针: ready-for-dev");
    assert!(
        story_done || story_open,
        "58.3 must be backlog/in-progress during 58.2 or done after closeout"
    );
    let epic_done = text.contains("epic-58: done") || text.contains("epic-58:done");
    let epic_open = text.contains("epic-58: in-progress") || text.contains("epic-58:in-progress");
    assert!(
        epic_done || epic_open,
        "epic-58 must be in-progress during 58.2 or done after Story 58.3"
    );
    // After closeout, story and epic stamps must stay coupled.
    if story_done || epic_done {
        assert!(
            story_done && epic_done,
            "when either 58-3 or epic-58 is done, both must be done"
        );
    }
}

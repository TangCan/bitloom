//! ATDD / guardrail: Story 37.2 / FR88 — firtool↔Chisel pinned ops + mechanical≠idiomatic honesty.
//!
//! Red until `docs/fr28-chisel-compilable.md` documents the pinned version pair,
//! cache/override env, and an explicit compilable≠idiomatic statement; README
//! cross-links; FR46 does not claim handwritten idiomatic maintainability.
//!
//! ```text
//! cargo test -p bitloom --test fr88_firtool_chisel_ops_honesty
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn fr28() -> String {
    read("docs/fr28-chisel-compilable.md")
}

fn fr46() -> String {
    read("docs/fr46-chisel-import.md")
}

fn readme() -> String {
    read("README.md")
}

#[test]
fn fr88_fr28_documents_pinned_chisel_firtool_pair() {
    let text = fr28();
    assert!(
        text.contains("7.15.0") && text.contains("1.158.0"),
        "FR28 doc must pin Chisel 7.15.0 ↔ firtool 1.158.0 (AD-9 / NFR12)"
    );
    assert!(
        (text.contains("Chisel") || text.contains("chisel"))
            && (text.contains("firtool") || text.contains("FIRTOOL")),
        "FR28 doc must name both Chisel and firtool in the pin"
    );
}

#[test]
fn fr88_fr28_documents_firtool_override_and_cache() {
    let text = fr28();
    assert!(
        text.contains("RHDL_FIRTOOL_PATH"),
        "FR28 ops doc must document RHDL_FIRTOOL_PATH override (NFR3 / FR88)"
    );
    assert!(
        text.contains("RHDL_FIRTOOL_CACHE")
            || text.contains("缓存")
            || text.contains("firtool ensure"),
        "FR28 ops doc must document cache root and/or `firtool ensure` path"
    );
    assert!(
        text.contains("PATH")
            && (text.contains("不信任")
                || text.contains("默认不")
                || text.contains("不得")
                || text.contains("禁止")
                || text.contains("不信任")),
        "FR28 ops doc must warn that bare PATH firtool must not impersonate the pin"
    );
}

#[test]
fn fr88_fr28_states_compilable_not_idiomatic() {
    let text = fr28();
    let honesty = (text.contains("idiomatic") || text.contains("惯用"))
        && (text.contains("不")
            || text.contains("≠")
            || text.contains("!=")
            || text.contains("非"))
        && (text.contains("可编译")
            || text.contains("compilable")
            || text.contains("emit_chisel")
            || text.contains("机械"));
    let explicit_ne = text.contains("可编译 ≠ idiomatic")
        || text.contains("可编译≠idiomatic")
        || text.contains("compilable ≠ idiomatic")
        || text.contains("≠ idiomatic");
    assert!(
        honesty || explicit_ne,
        "FR28 must publicly state emit_chisel / roundtrip = compilable ≠ idiomatic (FR88)"
    );
    assert!(
        text.contains("端口") || text.contains("层次") || text.contains("谓词"),
        "FR28 honesty statement must retain ports/hierarchy predicate acceptance"
    );
}

#[test]
fn fr88_readme_cross_links_fr28_ops_honesty() {
    let text = readme();
    assert!(
        text.contains("fr28-chisel-compilable") || text.contains("docs/fr28-chisel-compilable.md"),
        "README must cross-link docs/fr28-chisel-compilable.md"
    );
    // FR88 surface: either mention FR88 near Chisel/firtool, or link into honesty/ops framing.
    let fr88_or_honesty = text.contains("FR88")
        || text.contains("可编译 ≠ idiomatic")
        || text.contains("≠ idiomatic")
        || (text.contains("firtool")
            && text.contains("fr28-chisel-compilable")
            && (text.contains("运维")
                || text.contains("RHDL_FIRTOOL_PATH")
                || text.contains("钉死")));
    assert!(
        fr88_or_honesty,
        "README must cross-link FR28 ops/honesty (FR88) — firtool pin + fr28 doc and/or explicit FR88"
    );
}

#[test]
fn fr88_fr46_no_handwritten_maintainable_claim() {
    let text = fr46();
    let misleading = (text.contains("可维护手写")
        || text.contains("手写可维护")
        || text.contains("手写风格")
        || text.contains("idiomatic 手写")
        || (text.contains("idiomatic")
            && text.contains("可维护")
            && !text.contains("不")
            && !text.contains("≠")
            && !text.contains("非目标")))
        && !text.contains("不要求")
        && !text.contains("≠");
    // Strong ban: phrases that claim handwritten maintainable style as the contract.
    let banned = text.contains("可维护手写风格")
        || text.contains("手写可维护风格")
        || text.contains("idiomatic 可维护");
    assert!(
        !banned && !misleading,
        "FR46 must not claim handwritten / idiomatic maintainable Chisel (NFR39)"
    );
}

#[test]
fn fr88_fr28_no_handwritten_maintainable_claim() {
    let text = fr28();
    let banned = text.contains("可维护手写风格")
        || text.contains("手写可维护风格")
        || text.contains("idiomatic 可维护")
        || (text.contains("可维护")
            && text.contains("手写")
            && !text.contains("不")
            && !text.contains("≠"));
    assert!(
        !banned,
        "FR28 must not claim handwritten maintainable Chisel style (NFR39 / FR88)"
    );
}

#[test]
fn fr88_brand_remains_bitloom() {
    let text = format!("{}\n{}", fr28(), readme());
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "public docs must keep Bitloom / bitloom brand"
    );
}

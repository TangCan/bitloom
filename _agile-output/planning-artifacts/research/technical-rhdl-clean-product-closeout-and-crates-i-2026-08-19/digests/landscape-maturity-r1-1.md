## Findings

- **Claim:** crates.io “release readiness” for a first publish is defined by *permanence + required package metadata + successful package verification*, not by an empty feature/sprint backlog: publishes are permanent (versions cannot be overwritten/deleted); required discoverability fields include license (or license-file), description, homepage, repository, and readme; `cargo publish --dry-run` / `cargo package` is recommended before upload; SemVer governs later version bumps.  
  **Source:** https://doc.rust-lang.org/stable/cargo/reference/publishing.html  
  **Publisher:** Rust Project (The Cargo Book)  
  **pub_date:** living docs (stable Cargo Book; no fixed article date)  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** Mature release practice (product closeout hygiene) is framed as an *automated release process* with curated changelog + git tag at the published commit—orthogonal to “backlog empty.”  
  **Source:** https://doc.rust-lang.org/stable/cargo/reference/publishing.html  
  **Publisher:** Rust Project (The Cargo Book)  
  **pub_date:** living docs  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** Crossing into “stable product” (`>=1.0.0`) is an *API-stability gate*: a crate cannot be stable unless all of its *public* dependencies are also stable (C-STABLE). This is a maturity definition distinct from finishing sprint work.  
  **Source:** https://rust-lang.github.io/api-guidelines/necessities.html  
  **Publisher:** Rust API Guidelines (rust-lang)  
  **pub_date:** living docs  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** Dominant MSRV maturity signals are: declare support via `package.rust-version`; document a support/update policy; verify on supported toolchains (CI); Cargo documents that changing `rust-version` is assumed a *minor* incompatibility; policies like “N-2” / calendar-year windows are explicit ecosystem examples.  
  **Source:** https://doc.rust-lang.org/stable/cargo/reference/rust-version.html  
  **Publisher:** Rust Project (The Cargo Book)  
  **pub_date:** living docs (MSRV respected as of Rust 1.56)  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** SemVer compatibility remains the primary ongoing API-stability contract after publish; Cargo’s publishing docs explicitly point maintainers to SemVer rules for compatible vs breaking changes when bumping versions.  
  **Source:** https://doc.rust-lang.org/stable/cargo/reference/publishing.html  
  **Publisher:** Rust Project (The Cargo Book)  
  **pub_date:** living docs  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** `cargo-semver-checks` is a current-generation pre-`cargo publish` maturity tool that lints a candidate version for SemVer API breakage; it does not require the crate to already be on crates.io if a baseline is supplied (`--baseline-rev` / `--baseline-root` / etc.); crates.io listing shows active releases through **0.50.0 (2026-08-01)**.  
  **Source:** https://github.com/obi1kenobi/cargo-semver-checks ; https://crates.io/crates/cargo-semver-checks  
  **Publisher:** Predrag Gruevski et al. / crates.io registry metadata  
  **pub_date:** tool README living; registry update **2026-08-01** for 0.50.0  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=version**

- **Claim:** docs.rs is an automatic post-publish documentation maturity signal: all libraries published to crates.io are documented; builds may be queued; docs are built in a sandbox on nightly rustc; crate authors can configure builds via `[package.metadata.docs.rs]`.  
  **Source:** https://docs.rs/about ; https://docs.rs/about/builds  
  **Publisher:** Docs.rs team / Rust Project  
  **pub_date:** about page reports Docs.rs build/version context including **2026-07** nightly/build notes in live site metadata  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=landscape**

- **Claim:** Crater is primarily a *compiler/Cargo release-team* ecosystem regression instrument (build/test large slices of crates.io to detect toolchain breakage), not a standard crate-author “product release” gate; it has known coverage limits (Linux x86_64-centric; not all crates tested; not all real-world code on crates.io).  
  **Source:** https://rustc-dev-guide.rust-lang.org/tests/crater.html  
  **Publisher:** Rust Project (rustc-dev-guide)  
  **pub_date:** living docs  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=landscape**

- **Claim:** Security-advisory maturity for published crates is a *maintainer process*, not a crates.io built-in: crate owners are expected to publish security policy (commonly `SECURITY.md`); RustSec is the ecosystem advisory DB for crates.io crates; coordinated upstream disclosure precedes advisory filing; yank affected versions is an optional remediation step; Foundation/Project offer help for high-impact crate-author incidents.  
  **Source:** https://crates.io/security ; https://rustsec.org/contributing.html  
  **Publisher:** crates.io / RustSec (Secure Code WG ecosystem)  
  **pub_date:** living docs  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=pattern**

- **Claim:** Current-generation crates.io packaging makes *tokenless CI publish* newly practical via Trusted Publishing (OIDC): short-lived (~30 min) tokens; GitHub Actions supported; GitLab CI public beta; **first publish still requires an API token**; subsequent publishes can use `rust-lang/crates-io-auth-action` without long-lived registry secrets. Announced as implemented in crates.io development update.  
  **Source:** https://crates.io/docs/trusted-publishing ; https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/  
  **Publisher:** crates.io / Rust Blog  
  **pub_date:** **2025-07-11** (blog); trusted-publishing docs living (GitLab noted as public beta; example image pin `rust:1.91.0-alpine`)  
  **accessed=2026-08-19**  
  **confidence=high**  
  **class=version**

- **Claim (synthesis for A/B decision):** For a Rust CLI toolchain crate, “maturity-debt-free product closeout” in 2024–2026 practice maps to *release contracts + verification surfaces* (documented MSRV/`rust-version`, SemVer policy, docs.rs-green docs, semver-checks in CI, security contact path, changelog+tag+automated publish), while “sprint backlog empty” is neither necessary nor sufficient for crates.io readiness; first `rhdl-rs` publish is gated by uniqueness/metadata/permanence, then Trusted Publishing becomes available for repeatable closeout.  
  **Source:** composed from Cargo publishing + rust-version + API Guidelines C-STABLE + Trusted Publishing + docs.rs + RustSec (URLs above)  
  **Publisher:** multi-source synthesis  
  **pub_date:** n/a  
  **accessed=2026-08-19**  
  **confidence=medium**  
  **class=other**

## Leads

- Cargo SemVer compatibility reference (full breaking/compatible change matrix): https://doc.rust-lang.org/stable/cargo/reference/semver.html  
- RFC 3691 (Trusted Publishing for crates.io) linked from trusted-publishing docs.  
- API Guidelines discussion on MSRV-as-(non)breaking policy consensus: https://github.com/rust-lang/api-guidelines/discussions/231  
- `cargo docs-rs` (local docs.rs-like build checks) mentioned on docs.rs builds guidance.  
- Third-party release automation named in Cargo publishing chapter as representative workflows (investigate current names/versions before adopting).

## Gaps (looked for, not found)

- No authoritative Rust-Project checklist that equates “product release done” with “sprint backlog empty” (not found as a primary normative definition).  
- No crates.io requirement that a crate be `1.0`, MSRV-tested, or `cargo-semver-checks`-clean before first publish (beyond metadata/verification permanence).  
- No evidence that Crater is routinely runnable/required by ordinary crate maintainers as a release gate for their own CLI/library products.  
- Fresh ≤1 month primary “landscape survey” paper specifically on Rust *toolchain* product closeout criteria: not found; relied on living standards docs + 2025 Trusted Publishing announcement + 2026 tooling versions.
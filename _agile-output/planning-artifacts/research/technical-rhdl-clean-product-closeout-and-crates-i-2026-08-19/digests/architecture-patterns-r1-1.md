# Digest: architecture-patterns (crates.io release) — r1

**Assistant:** crates.io release patterns  
**Accessed:** 2026-08-19  
**Dimension:** Architecture patterns in practice for crates.io release

## Findings

1. **Publish is permanent** — versions cannot be overwritten; yank does not delete code. Prefer `cargo publish --dry-run` / `cargo package` first.  
   source: https://doc.rust-lang.org/cargo/reference/publishing.html · publisher: Rust Project (Cargo Book) · pub_date: living · accessed: 2026-08-19 · confidence: high · class: policy

2. **Crate names are FCFS** — once taken, another crate cannot use that name.  
   source: same · confidence: high · class: policy

3. **Cargo Book lists release tools** — cargo-release, cargo-smart-release, cargo-workspaces, release-plz, etc.; recommend automating changelog + git tag.  
   source: same · confidence: high · class: pattern

4. **Yank** blocks new deps; existing Cargo.lock keep working; does not delete secrets. Prefer yank after compatible fix.  
   source: https://doc.rust-lang.org/cargo/commands/cargo-yank.html · publisher: Rust Project · pub_date: living · accessed: 2026-08-19 · confidence: high · class: policy

5. **Rename** only by republishing under a new name. Self-serve deletion (RFC 3660) is narrow (&lt;72h, or single owner + no reverse deps + low downloads).  
   source: https://rust-lang.github.io/rfcs/3660-crates-io-crate-deletions.html · publisher: Rust RFCs · pub_date: 2024 · accessed: 2026-08-19 · confidence: high · class: policy

6. **Name squatting** violates crates.io usage policy (RFC 3463; policy ~2023-11). Trademark → Rust Foundation.  
   source: https://rust-lang.github.io/rfcs/3463-crates-io-policy-update.html ; https://blog.rust-lang.org/2023/09/22/crates-io-usage-policy-rfc/ · publisher: crates.io team / Rust Blog · pub_date: 2023-09-22 · accessed: 2026-08-19 · confidence: high · class: policy

7. **crates.io no longer mediates ownership transfers** (RFC 3646) — contact owner or pick another name.  
   source: https://rust-lang.github.io/rfcs/3646-remove-crate-transfer-mediation-policy.html · confidence: high · class: policy

8. **`[package].name`** is crates.io identity; default bin = package name; override with `[[bin]] name`. Artifact filename ≠ registry name.  
   source: https://doc.rust-lang.org/cargo/reference/manifest.html ; https://doc.rust-lang.org/cargo/reference/cargo-targets.html · confidence: high · class: pattern

9. **`rust-version`** documents MSRV; Cargo errors on older toolchains; should be verified in CI; changing MSRV treated as minor incompatibility (not necessarily major). Separate from edition.  
   source: https://doc.rust-lang.org/cargo/reference/rust-version.html · confidence: high · class: version

10. **0.y.z SemVer** — treat y as major / z as minor; leftmost non-zero for incompatibility. Raising MSRV often non-major by convention (document policy).  
    source: https://doc.rust-lang.org/cargo/reference/semver.html · confidence: high · class: version

11. **release-plz** — Release PR from CI → merge → cargo publish + tags/changelogs; workspace-aware; optional cargo-semver-checks.  
    source: https://release-plz.dev/docs · publisher: Marco Ieni / release-plz · pub_date: living · accessed: 2026-08-19 · confidence: high · class: pattern

12. **cargo-dist** — tag-triggered binary Plan/Build/Host/Publish/Announce; does **not** replace crates.io versioning—pair with cargo-release or release-plz. Tag shape gotcha for workspaces.  
    source: https://axodotdev.github.io/cargo-dist/ ; https://axodotdev.github.io/cargo-dist/book/workspaces/cargo-release-guide.html · publisher: Axo · confidence: high · class: pattern

13. **cargo-release + dist** — topological publish + registry wait; shared-version/tag config for lockstep; dry-run imperfect.  
    source: same as #12 · confidence: high · class: pattern

14. **Dominant split** — registry publish (release-plz | cargo-release | bare GHA) vs CLI binaries (cargo-dist). Combining release-plz + cargo-dist common; token/permissions for tag workflows is a gotcha.  
    source: synthesis of #11–13 + community · confidence: medium · class: pattern

### Architecture table (from digest)

| Architecture | crates.io | CLI binaries | Failure modes |
|---|---|---|---|
| Bare GHA cargo publish | Yes | No | Topo/index lag; no dry-run; token leaks |
| cargo-release (+ dist) | Yes | Via tag→dist | Tag mismatch; root-only release |
| release-plz (+ dist) | Yes | Via tag→dist | Conventional-commit noise; GITHUB_TOKEN not firing tags |
| cargo-dist alone | No | Yes | Forgotten crates publish |

**Versioning for immature toolchain:** stay on **0.x** until public surface intentionally stable; 1.0 only when ready for full major/minor/patch discipline.

## Leads

- Cargo Book Rust version CI guide
- crates.io/policies (fetch timed out in pass)
- release-plz GitHub Action docs (token scopes)
- cargo-semver-checks for lib crates in workspace
- Trusted Publishing / OIDC as alternative to long-lived tokens

## Gaps

- Live crates.io HTML policy page not retrieved
- No authoritative “most used” pipeline ranking
- Trusted Publishing details not re-verified this pass
- Trademark risk for similar names is legal, not FCFS uniqueness

# Digest: trusted-publishing lead follow-up — r1

**Lead from:** architecture-patterns-r1-1  
**Accessed:** 2026-08-19

## Findings

1. **Trusted Publishing shipped on crates.io (mid-2025)** — OIDC from GitHub Actions; short-lived tokens; no long-lived API token in secrets for subsequent publishes.  
   source: https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/ · publisher: Rust Blog · pub_date: 2025-07-11 · accessed: 2026-08-19 · confidence: high · class: version/policy

2. **First publish must be manual** — then configure Trusted Publisher (repo + workflow filename + optional environment) on crates.io; use `rust-lang/crates-io-auth-action` with `id-token: write`.  
   source: https://crates.io/docs/trusted-publishing · publisher: crates.io · pub_date: living · accessed: 2026-08-19 · confidence: high · class: pattern

3. **Tokens expire ~30 minutes**; repository/workflow verification; designed for future GitLab etc.  
   source: same · confidence: high · class: policy

4. **RFC 3691** is the design basis.  
   source: https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html · publisher: Rust RFCs · confidence: high · class: policy

## Leads
- Pair Trusted Publishing with release-plz or cargo-dist tag workflows

## Gaps
- None material for decision (primary docs retrieved)

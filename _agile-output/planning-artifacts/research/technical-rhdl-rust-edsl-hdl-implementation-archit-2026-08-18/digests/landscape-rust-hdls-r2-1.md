# Digest: landscape-rust-hdls r2-1

## Findings

1. **claim:** `samitbasu/rhdl` is public, not archived, default branch `main`, `topics` empty, `homepage` null, `language` Verilog, `pushed_at` `2026-04-21T15:56:36Z`, `updated_at` `2026-08-14T22:49:37Z`, `created_at` `2023-09-02T18:00:16Z`, owner type User `samitbasu`, license MIT. **source:** https://api.github.com/repos/samitbasu/rhdl **publisher:** GitHub **pub_date:** API fields as listed (response retrieved this run) **accessed** 2026-08-18 **confidence:** high **class:** primary-api

2. **claim:** Latest commit on default `main` is `c99d5cc53269a247bbc675d0fbd766991d409f56`, author/committer date `2026-02-03T06:26:37Z`, message “Merge pull request #21 … Doc update”. This is **not** equal to `pushed_at`. **source:** https://api.github.com/repos/samitbasu/rhdl/commits?per_page=1 **publisher:** GitHub **pub_date:** 2026-02-03 **accessed** 2026-08-18 **confidence:** high **class:** primary-api

3. **claim:** Date contradiction is two different GitHub clocks: default-branch HEAD commit date 2026-02-03 vs repository `pushed_at` 2026-04-21. The repo has 40 branches; `main` tip SHA matches that HEAD commit; other branch tip SHAs differ. This run did **not** date those other tips, so the 2026-04-21 push is unexplained by `main` HEAD. Public events returned only `WatchEvent`s (latest 2026-08-06), no `PushEvent` for 2026-04-21. **source:** https://api.github.com/repos/samitbasu/rhdl ; https://api.github.com/repos/samitbasu/rhdl/commits?per_page=1 ; https://api.github.com/repos/samitbasu/rhdl/branches?per_page=100 ; https://api.github.com/repos/samitbasu/rhdl/events?per_page=5 **publisher:** GitHub **pub_date:** field dates as above **accessed** 2026-08-18 **confidence:** high on the split; low on *why* `pushed_at` moved **class:** primary-api / unresolved-mechanism

4. **claim:** GitHub language bytes: Verilog 6,999,415; Rust 2,671,549; Tcl 1,681; Just 1,269. **source:** https://api.github.com/repos/samitbasu/rhdl/languages **publisher:** GitHub **pub_date:** n/a (live API) **accessed** 2026-08-18 **confidence:** high **class:** primary-api

5. **claim:** README describes RHDL as a rewrite of `rust-hdl`; plan items include “hardware-compatible intermediate representation (RHIF)”, “RHIF -> Verilog assembler”, Verilog AST, and comparing Verilog with Rust results. README text retrieved this run contains **no** tokens `FIRRTL`, `firtool`, or `Chisel`. It does contain `RHIF` and `Verilog`. **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md **publisher:** Samit Basu / GitHub raw **pub_date:** file as of `main` commit 2026-02-03 **accessed** 2026-08-18 **confidence:** high for README contents; medium for whole-repo emission (no authenticated code search) **class:** primary-docs

6. **claim:** LATTE ’25 paper (author Samit Basu) states RHDL lowers a subset of Rust to synthesizable Verilog; compiler steps named are RHIF (typed SSA VM) then RTL then Verilog; Verilog is treated “like a machine-code target”. Chisel appears as an embedding analogy (Scala), not as an RHDL output. Paper URL cited GitHub `samitbasu/rhdl` (accessed Jan 25, 2025 in the paper’s bibliography snippet), not a live `rhdl.org` doc set. Direct PDF fetch timed out; claims rest on retrieved snippets. **source:** https://capra.cs.cornell.edu/latte25/paper/2.pdf (snippets via WebSearch this run) **publisher:** LATTE ’25 / Cornell CAPRA hosting **pub_date:** LATTE ’25, 2025-03-30 (venue line in snippet) **accessed** 2026-08-18 **confidence:** medium (snippets only) **class:** paper

7. **claim:** Workspace `Cargo.toml` lists members `crates/rhdl`, `rhdl-bits`, `rhdl-bsp`, `rhdl-core`, `rhdl-fpga`, `rhdl-macro`, `rhdl-macro-core`, `rhdl-surfer-plugin`, `rhdl-span`, `rhdl-toolchains`, `rhdl-trace-type`, `rhdl-vlog`, plus doc crates. In-tree package `crates/rhdl` is named `rhdl` `0.1.0` edition `2024` and depends on path crates `rhdl-bits`, `rhdl-core`, `rhdl-macro`, `rhdl-trace-type`, `rhdl-vlog`. **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/Cargo.toml ; https://raw.githubusercontent.com/samitbasu/rhdl/main/crates/rhdl/Cargo.toml **publisher:** Samit Basu / GitHub raw **pub_date:** as of `main` 2026-02-03 **accessed** 2026-08-18 **confidence:** high **class:** primary-source

8. **claim:** crates.io crate `rhdl` `0.1.0` is **not** that workspace compiler. It was published `2023-09-02T18:04:18Z` by `samitbasu` (Samit Basu); `crate_size` 1963 bytes; `linecounts` Rust 3 code lines / 1 file; `has_lib` false; `bin_names` `["rhdl"]`; `newest_version`/`max_version` `0.1.0`; downloads 1679 / recent 10; homepage `https://rhdl.org`; repository `https://github.com/samitbasu/rhdl`; keywords hardware/hdl/verilog/fpga; `documentation` null. **source:** https://crates.io/api/v1/crates/rhdl ; https://crates.io/api/v1/crates/rhdl/0.1.0 **publisher:** crates.io **pub_date:** 2023-09-02 **accessed** 2026-08-18 **confidence:** high **class:** primary-api

9. **claim:** Exact crate name `rhdl-core` is **not** published (HTTP 404). crates.io search `q=rhdl` `meta.total` 18; exact match `rhdl`; also `rhdl-bits` (samitbasu, 2023-09-09, homepage rhdl.org); `csa-rhdl` is a different crate (MavenRain, 2026-04-06, hdl-cat backend). **source:** https://crates.io/api/v1/crates/rhdl-core (404) ; https://crates.io/api/v1/crates?q=rhdl **publisher:** crates.io **pub_date:** crate `updated_at` fields as returned **accessed** 2026-08-18 **confidence:** high **class:** primary-api / negative-result

10. **claim:** `yupferris/kaze` GitHub API `archived` is **true** (not false). Also `pushed_at` `2023-11-15T21:54:24Z`, `updated_at` `2026-04-05T19:08:31Z`, default branch `master`, language Rust, topics `digital-logic-design`, `hdl`, `rust`. **source:** https://api.github.com/repos/yupferris/kaze **publisher:** GitHub **pub_date:** API fields as listed **accessed** 2026-08-18 **confidence:** high **class:** primary-api

11. **claim:** GitHub HTML code search for `firrtl OR firtool OR chisel` in `samitbasu/rhdl` did **not** return hits; it returned a sign-in wall (“Before you can access our code search functionality please sign in”). **source:** https://github.com/search?q=repo%3Asamitbasu%2Frhdl+firrtl+OR+firtool+OR+chisel&type=code **publisher:** GitHub **pub_date:** n/a **accessed** 2026-08-18 **confidence:** high that search was blocked; **no** repo-wide token census **class:** access-blocked

12. **claim:** Unrelated commercial use of letters “RHDL”: RHD Limited / “RHDL India” BPO (rhdlimited.com) and GST listing “RHDL Textile Unit” / legal name Ratan Housing Development Ltd. No USPTO or software-trademark registration for this HDL was retrieved this run. GitHub owner is a User, not an org. **source:** https://rhdlimited.com/ ; https://findgst.in/gst/09AACCR6099R3Z9 ; GitHub repo owner object **publisher:** RHD Limited; findgst.in; GitHub **pub_date:** pages retrieved 2026-08-18; GST registered 2017-07-16 on that listing **accessed** 2026-08-18 **confidence:** medium (unrelated-name collision only; trademark legal status not searched at USPTO) **class:** adjacent-name / not-IP-clearance

## Leads worth chasing

- Date non-main branch tips (and tags) to find which ref moved `pushed_at` to 2026-04-21.
- Authenticated GitHub code search or a local clone grep for `firrtl`, `firtool`, `chisel` under `crates/` (esp. `rhdl-core`, `rhdl-vlog`, `rhdl-toolchains`).
- Retry `https://rhdl.org` (timeout this run); also docs.rs, GitHub Pages (`has_pages`: false), and `doc/book` in the repo.
- Publish-status of remaining workspace crates (`rhdl-vlog`, `rhdl-macro`, `rhdl-fpga`, …) beyond `rhdl-core` 404.
- Full LATTE ’25 PDF (fetch timed out) for any FIRRTL/CIRCT sentence not in snippets.

## Looked for and did not find

- Live `rhdl.org` documentation (fetch timed out; GitHub `homepage` is null; crates.io still lists that URL).
- Repo-wide code-search hits for `firrtl` / `firtool` / `chisel` (GitHub code search required login).
- FIRRTL or `firtool` named as an RHDL emission target in the retrieved README or LATTE snippets (those sources name RHIF → RTL → Verilog).
- crates.io crate `rhdl-core` (404).
- Evidence that crates.io `rhdl` 0.1.0 is the current compiler (3-line stub from 2023 vs 2024-edition workspace crate with path deps).
- USPTO / trademark registration for the HDL name “RHDL”.
- GitHub `PushEvent` explaining `pushed_at` 2026-04-21 (events payload this run had only watches).
- GitHub topics on `samitbasu/rhdl` (empty array).
- `yupferris/kaze` as not-archived (`archived`: true).

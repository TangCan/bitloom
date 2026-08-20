# Digest: Landscape & maturity — round 1

**Assistant:** lead (inline; parallel agent still running)  
**Accessed:** 2026-08-20  
**Dimension questions:** cargo-install distribution patterns; monorepo toolchains; cargo subcommand model

## Claims

1. **claim:** `cargo install` installs only packages that have executable `[[bin]]` or `[[example]]` targets into the install root `bin` folder; it is not a way to install library crates for use as dependencies.  
   **source:** https://doc.rust-lang.org/cargo/commands/cargo-install.html  
   **publisher:** Rust / Cargo Book  
   **pub_date:** living docs (fetched 2026-08-20)  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** process

2. **claim:** Cargo discovers custom subcommands by mapping `cargo <command>` to an executable named `cargo-<command>` on `$PATH` (with `$CARGO_HOME/bin` prioritized by default).  
   **source:** https://doc.rust-lang.org/cargo/reference/external-tools.html  
   **publisher:** Rust / Cargo Book  
   **pub_date:** living docs  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** pattern

3. **claim:** mdBook’s official install path is `cargo install mdbook` (or prebuilt binary / `--git`); after install, users run `mdbook` against *their own* book content without cloning the mdBook repository.  
   **source:** https://rust-lang.github.io/mdBook/guide/installation.html ; https://rust-lang.github.io/mdBook/cli/index.html  
   **publisher:** rust-lang / mdBook  
   **pub_date:** living docs  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** landscape

4. **claim:** Dominant mature pattern for “install then use” CLI tools is: published binary crate + user-owned inputs (project files), not “install binary then keep using the tool’s monorepo as the only design workspace.”  
   **source:** mdBook installation + CLI docs above; Cargo install semantics  
   **publisher:** rust-lang  
   **accessed:** 2026-08-20  
   **confidence:** medium-high (pattern synthesis from primary docs; not a single normative sentence)  
   **class:** pattern

5. **claim:** Alternative acquisition paths include `cargo install --git <url>` and `cargo install --path`, used when crates.io lags or for local development of the tool itself.  
   **source:** https://doc.rust-lang.org/cargo/commands/cargo-install.html ; mdBook “latest master” section  
   **publisher:** Rust / mdBook  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** process

## Leads
- cargo-expand / wasm-pack README “after install” wording for contrast with monorepo compilers
- Whether any published Rust HDL toolchains ship CLI-only vs library suite (rhdl on crates.io is a different product)

## Looked for / not found
- A Cargo Book chapter that requires cloning the tool’s own repo after `cargo install` — not found; docs assume the installed binary operates on user projects.

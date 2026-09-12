//! `cargo bitloom` CLI — published as crate `bitloom` (AD-2). Never publish as `rhdl` / `rhdl-bits`.

mod firtool;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use bitloom::hls;
use clap::{Parser, Subcommand};
use serde::Deserialize;

const BITLOOM_BACKEND_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "cargo-bitloom",
    about = "Bitloom elaborate and emit tools (crates.io: bitloom). Unrelated to samitbasu/rhdl."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Elaborate a design package's `rhdl_elaborate()` and write Yosys-friendly Verilog.
    Build {
        /// Cargo package that exports `rhdl_elaborate()` (design crate / example).
        #[arg(long)]
        package: String,
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Workspace / repo root containing the root `Cargo.toml`.
        #[arg(long, default_value = ".")]
        manifest_dir: PathBuf,
    },
    /// Scaffold a minimal design crate depending only on `bitloom-prelude`.
    New {
        /// Package / directory name (e.g. `blink`).
        name: String,
        /// Parent directory for the new crate (default: `.`).
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    /// Manage the pinned CIRCT firtool binary (AD-9 / NFR3).
    Firtool {
        #[command(subcommand)]
        cmd: FirtoolCmd,
    },
    /// List `bitloom-sim` tick engines (FR32). Simulation itself lives in tests / bitloom-sim.
    SimEngines,
    /// Product HLS: external Bambu (FR35) or in-tree schedule MVP (FR95 / AD-25).
    Hls {
        /// Top function name (also used for the emitted C stub / schedule artifact).
        #[arg(long, default_value = "add")]
        function: String,
        /// Output directory for C stub, schedule IR, and RTL artifacts.
        #[arg(long, default_value = "target/bitloom-hls")]
        out_dir: PathBuf,
        /// Write the C stub only; do not invoke Bambu (not a successful RTL run).
        /// Ignored when `--in-tree` (in-tree never calls Bambu).
        #[arg(long, default_value_t = false)]
        emit_only: bool,
        /// Dissolve a documented dataflow transform before emit / schedule:
        /// `add` | `identity` | `add1` | `xor_a5`.
        /// With `--in-tree`, this always runs the FR96 dissolve path
        /// (`schedule_in_tree_from_transform`); success prints `fr96=true`.
        /// Bare FR95 (no dissolve) → library `schedule_in_tree` / `run_hls_in_tree`.
        #[arg(long, default_value = "add")]
        dataflow: String,
        /// FR95: run in-tree schedule MVP (no Bambu). Optional `--unroll N` (default 4).
        /// Note: CLI `--in-tree` always dissolves `--dataflow` first (FR96); bare FR95
        /// without dissolve is library-only (`schedule_in_tree`).
        #[arg(long, default_value_t = false)]
        in_tree: bool,
        /// Loop-unroll trip count for `--in-tree` (FR95 documented subset).
        #[arg(long, default_value_t = 4)]
        unroll: u32,
        /// FR110: with `--in-tree`, use pipeline schedule (commercial depth) instead of loop-unroll.
        #[arg(long, default_value_t = false)]
        pipeline: bool,
        /// Initiation interval for `--in-tree --pipeline` (FR110 / Q2).
        #[arg(long, default_value_t = 1)]
        ii: u32,
        /// Pipeline stage count for `--in-tree --pipeline` (FR110 / Q1; require ≥2 for `fr110`).
        #[arg(long, default_value_t = 2)]
        stages: u32,
        /// FR121: Handshake / dynamic-DF default synthesizable path (ready/valid).
        /// Implies in-tree; takes precedence over `--pipeline` / loop-unroll.
        #[arg(long, default_value_t = false)]
        handshake: bool,
        /// Handshake channel stages for `--handshake` / `--circt-handshake` (require ≥1).
        #[arg(long, default_value_t = 1)]
        channels: u32,
        /// FR129: CIRCT Handshake dialect + multi-clock elastic buffers.
        /// Implies in-tree; takes precedence over `--handshake` / `--pipeline`.
        #[arg(long, default_value_t = false)]
        circt_handshake: bool,
        /// Clock domains for `--circt-handshake` (FR129; require ≥2).
        #[arg(long, default_value_t = 2)]
        clock_domains: u32,
        /// Elastic buffer depth for `--circt-handshake` (FR129; require ≥1).
        #[arg(long, default_value_t = 1)]
        elastic_depth: u32,
    },
    /// Import FIRRTL 6.0.0 `.fir` (Chisel→firtool output ok) into the same emit path as `build` (FR40 / FR46).
    Import {
        /// Path to a `.fir` file with `FIRRTL version 6.0.0` header.
        #[arg(long)]
        input: PathBuf,
        /// Directory for emitted Yosys-friendly `.v` (and optional `.fir` / Chisel Scala).
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Also write re-emitted FIRRTL text next to Verilog.
        #[arg(long, default_value_t = false)]
        also_fir: bool,
        /// Also write Chisel Scala via `emit_chisel` (FR28) next to Verilog.
        #[arg(long, default_value_t = false)]
        also_chisel: bool,
    },
    /// Generate a Rust functional-sim crate from a design package's FrozenHir (FR47 leg 1).
    GenFunc {
        /// Cargo package that exports `rhdl_elaborate()`.
        #[arg(long)]
        package: String,
        #[arg(long, default_value = "target/bitloom-func-sim")]
        out_dir: PathBuf,
        #[arg(long, default_value = ".")]
        manifest_dir: PathBuf,
    },
    /// Generate a cycle-accurate tick-wrapper crate from FrozenHir (FR47 leg 2).
    GenCycle {
        #[arg(long)]
        package: String,
        #[arg(long, default_value = "target/bitloom-cycle-sim")]
        out_dir: PathBuf,
        #[arg(long, default_value = ".")]
        manifest_dir: PathBuf,
    },
    /// Generate SystemC TLM-2.0 LT C++ fixture from FrozenHir (FR101). Not Rust FL.
    GenTlm {
        #[arg(long)]
        package: String,
        #[arg(long, default_value = "target/bitloom-systemc-tlm")]
        out_dir: PathBuf,
        #[arg(long, default_value = ".")]
        manifest_dir: PathBuf,
    },
    /// Generate SystemC TLM-2.0 AT-subset C++ fixture from FrozenHir (FR107). Parallel to gen-tlm.
    GenTlmAt {
        #[arg(long)]
        package: String,
        #[arg(long, default_value = "target/bitloom-systemc-tlm-at")]
        out_dir: PathBuf,
        #[arg(long, default_value = ".")]
        manifest_dir: PathBuf,
    },
    /// Emit module hierarchy HTML from a FIRRTL 6.0.0 `.fir` (FR38 / FR49 / FR40).
    Visualize {
        /// Path to a `.fir` file with `FIRRTL version 6.0.0` header.
        #[arg(long)]
        input: PathBuf,
        /// Directory for `hierarchy.html`.
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Alias of `visualize` — hierarchy HTML docs from FrozenHir (FR40 `doc`).
    Doc {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Tick a `.fir` design, dump VCD, timing HTML (FR38/49), interactive wave (FR104),
    /// typed IDE wave (FR117 subset B), and **by default** FR162 primary Tywaves GUI depth
    /// (FR134-level `tywaves.gui.*` + FR125 sidecar). Opt out with `--no-tywaves-gui`.
    Wave {
        /// Path to a `.fir` file with `FIRRTL version 6.0.0` header.
        #[arg(long)]
        input: PathBuf,
        /// Directory for `wave.vcd` + `timing.html` + `interactive.html` + `typed-wave.html`
        /// (+ FR162 default `tywaves.gui.*` / FR125 sidecar unless `--no-tywaves-gui`).
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Number of ticks after reset.
        #[arg(long, default_value_t = 8)]
        ticks: u64,
        /// Also attempt FST via `vcd2fst` (optional; VCD+HTML always written).
        #[arg(long, default_value_t = false)]
        fst: bool,
        /// Also emit FR125 upstream Tywaves sidecar (`wave.tywaves.json` + `tywaves.launch.sh`).
        /// Redundant when FR162 GUI primary is on (default); useful with `--no-tywaves-gui`.
        #[arg(long, default_value_t = false)]
        tywaves: bool,
        /// Emit FR134/FR162 GUI/IDE depth (`tywaves.gui.manifest.json` + install descriptor).
        /// **Default on** (FR162 primary surface). Implies FR125 sidecar emission.
        /// Kept for FR134 ATDD / docs compatibility.
        #[arg(long, default_value_t = false)]
        tywaves_gui: bool,
        /// FR162: disable default GUI primary surface (legacy typed-wave / VCD-focused path).
        /// With this flag, FR125 requires `--tywaves`; GUI manifests are not emitted.
        #[arg(long, default_value_t = false)]
        no_tywaves_gui: bool,
        /// FR167 (a): emit ChiselSim coupling artifacts (`chiselsim.*`) beyond FR162 GUI primary.
        #[arg(long, default_value_t = false)]
        chiselsim: bool,
        /// FR167 (b): emit multi IDE-store descriptors (Open VSX + JetBrains beyond FR134 G1).
        #[arg(long, default_value_t = false)]
        ide_stores: bool,
    },
    /// Tick a Mux demo (or `.fir`) and write FR114 `coverage.lcov` + `coverage.html`.
    ///
    /// Optional `--genhtml` runs third-party `genhtml` on the LCOV (FR158).
    Coverage {
        /// Optional `.fir` input; when omitted, uses the built-in Mux coverage demo DUT.
        #[arg(long)]
        input: Option<PathBuf>,
        /// Directory for `coverage.lcov` + `coverage.html`.
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Number of ticks after reset (Mux demo).
        #[arg(long, default_value_t = 4)]
        ticks: u64,
        /// FR158: after writing LCOV, run third-party `genhtml` (fails if not on PATH).
        #[arg(long, default_value_t = false)]
        genhtml: bool,
        /// FR158: directory for `genhtml` HTML output (default: `<out-dir>/genhtml`).
        #[arg(long)]
        genhtml_out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum FirtoolCmd {
    /// Download (if needed), verify sha256, and print the firtool binary path.
    Ensure,
    /// Print configured version and asset names (no download).
    Info,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<MetaPackage>,
}

#[derive(Debug, Deserialize)]
struct MetaPackage {
    name: String,
    manifest_path: String,
}

/// Resolve package directory via `cargo metadata` (FR51).
fn resolve_package_dir(manifest_dir: &Path, package: &str) -> Result<PathBuf, String> {
    let manifest = if manifest_dir.join("Cargo.toml").is_file() {
        manifest_dir.join("Cargo.toml")
    } else {
        return Err(format!(
            "no Cargo.toml under {} — pass --manifest-dir to a Cargo workspace/package root",
            manifest_dir.display()
        ));
    };
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1")
        .arg("--manifest-path")
        .arg(&manifest)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("spawn cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let meta: Metadata =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("parse cargo metadata: {e}"))?;
    let pkg = meta
        .packages
        .into_iter()
        .find(|p| p.name == package)
        .ok_or_else(|| {
            format!(
                "package `{package}` not found in cargo metadata for {} \
                 (use a package name from that workspace, not only examples/<name>)",
                manifest.display()
            )
        })?;
    let manifest_path = PathBuf::from(pkg.manifest_path);
    Ok(manifest_path
        .parent()
        .ok_or_else(|| "package manifest has no parent".to_string())?
        .to_path_buf())
}

fn use_dev_path_backends(workspace: &Path) -> bool {
    // Monorepo checkout: path backends avoid duplicate bitloom-hir (path design + registry vlog).
    // True standalone (no crates/bitloom-vlog): use crates.io versions matching this CLI.
    // Override: BITLOOM_FORCE_REGISTRY=1 always uses crates.io; BITLOOM_DEV_PATH=1 forces path.
    if std::env::var_os("BITLOOM_FORCE_REGISTRY").is_some() {
        return false;
    }
    if std::env::var_os("BITLOOM_DEV_PATH").is_some() {
        return workspace.join("crates/bitloom-vlog/Cargo.toml").is_file();
    }
    workspace.join("crates/bitloom-vlog/Cargo.toml").is_file()
}

fn build_host_cargo(workspace: &Path, package: &str, pkg_path: &Path) -> String {
    let crate_name = package.replace('-', "_");
    let backends = if use_dev_path_backends(workspace) {
        format!(
            r#"bitloom-vlog = {{ path = "{vlog}" }}
bitloom-hir = {{ path = "{hir}" }}
"#,
            vlog = workspace.join("crates/bitloom-vlog").display(),
            hir = workspace.join("crates/bitloom-hir").display(),
        )
    } else {
        format!(
            r#"bitloom-vlog = "{ver}"
bitloom-hir = "{ver}"
"#,
            ver = BITLOOM_BACKEND_VERSION,
        )
    };
    format!(
        r#"[package]
name = "bitloom-host-shim"
version = "0.0.0"
edition = "2024"
publish = false

# Keep this shim out of the parent workspace.
[workspace]

[dependencies]
{crate_name} = {{ path = "{pkg}" }}
{backends}"#,
        crate_name = crate_name,
        pkg = pkg_path.display(),
        backends = backends,
    )
}

fn build_host_main(package: &str, out_dir: &Path) -> String {
    let crate_name = package.replace('-', "_");
    format!(
        r#"fn main() {{
    let frozen = {crate_name}::rhdl_elaborate().expect("rhdl_elaborate");
    let art = bitloom_vlog::emit(&frozen);
    let out_dir = std::path::PathBuf::from({out_dir:?});
    std::fs::create_dir_all(&out_dir).expect("out_dir");
    for f in &art.files {{
        let path = out_dir.join(&f.path);
        std::fs::write(&path, &f.contents).expect("write");
        println!("wrote {{}}", path.display());
    }}
}}
"#,
        crate_name = crate_name,
        out_dir = out_dir,
    )
}

fn build_gen_func_host_cargo(workspace: &Path, package: &str, pkg_path: &Path) -> String {
    let crate_name = package.replace('-', "_");
    let backends = if use_dev_path_backends(workspace) {
        format!(
            r#"bitloom-sim = {{ path = "{sim}" }}
bitloom-hir = {{ path = "{hir}" }}
"#,
            sim = workspace.join("crates/bitloom-sim").display(),
            hir = workspace.join("crates/bitloom-hir").display(),
        )
    } else {
        format!(
            r#"bitloom-sim = "{ver}"
bitloom-hir = "{ver}"
"#,
            ver = BITLOOM_BACKEND_VERSION,
        )
    };
    format!(
        r#"[package]
name = "bitloom-gen-func-shim"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
{crate_name} = {{ path = "{pkg}" }}
{backends}"#,
        crate_name = crate_name,
        pkg = pkg_path.display(),
        backends = backends,
    )
}

fn build_gen_func_host_main(package: &str, out_dir: &Path) -> String {
    let crate_name = package.replace('-', "_");
    format!(
        r#"fn main() {{
    let frozen = {crate_name}::rhdl_elaborate().expect("rhdl_elaborate");
    let out_dir = std::path::PathBuf::from({out_dir:?});
    let written = bitloom_sim::generate_functional_sim(&frozen, &out_dir).expect("generate");
    println!("wrote functional-sim crate {{}}", written.display());
}}
"#,
        crate_name = crate_name,
        out_dir = out_dir,
    )
}

fn build_gen_cycle_host_main(package: &str, out_dir: &Path) -> String {
    let crate_name = package.replace('-', "_");
    format!(
        r#"fn main() {{
    let frozen = {crate_name}::rhdl_elaborate().expect("rhdl_elaborate");
    let out_dir = std::path::PathBuf::from({out_dir:?});
    let written = bitloom_sim::generate_cycle_accurate_sim(&frozen, &out_dir).expect("generate");
    println!("wrote cycle-accurate crate {{}}", written.display());
}}
"#,
        crate_name = crate_name,
        out_dir = out_dir,
    )
}

fn build_gen_tlm_host_main(package: &str, out_dir: &Path) -> String {
    let crate_name = package.replace('-', "_");
    format!(
        r#"fn main() {{
    let frozen = {crate_name}::rhdl_elaborate().expect("rhdl_elaborate");
    let out_dir = std::path::PathBuf::from({out_dir:?});
    let written = bitloom_sim::emit_systemc_tlm_lt(&frozen, &out_dir).expect("emit_systemc_tlm_lt");
    println!("wrote SystemC TLM-2.0 LT fixture {{}}", written.display());
}}
"#,
        crate_name = crate_name,
        out_dir = out_dir,
    )
}

fn build_gen_tlm_at_host_main(package: &str, out_dir: &Path) -> String {
    let crate_name = package.replace('-', "_");
    format!(
        r#"fn main() {{
    let frozen = {crate_name}::rhdl_elaborate().expect("rhdl_elaborate");
    let out_dir = std::path::PathBuf::from({out_dir:?});
    let written = bitloom_sim::emit_systemc_tlm_at(&frozen, &out_dir).expect("emit_systemc_tlm_at");
    println!("wrote SystemC TLM-2.0 AT fixture {{}}", written.display());
}}
"#,
        crate_name = crate_name,
        out_dir = out_dir,
    )
}

fn scaffold_new(name: &str, parent: &Path) -> Result<PathBuf, String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err("name must be non-empty ASCII alphanumeric / _ / -".into());
    }
    let dir = parent.join(name);
    if dir.exists() {
        return Err(format!("{} already exists", dir.display()));
    }
    fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
rust-version = "1.97.1"
publish = false

[dependencies]
bitloom-prelude = "{ver}"
"#,
        name = name,
        ver = BITLOOM_BACKEND_VERSION,
    );
    let struct_name = {
        let mut s = String::new();
        for p in name.split(|c| c == '-' || c == '_') {
            if p.is_empty() {
                continue;
            }
            let mut ch = p.chars();
            if let Some(f) = ch.next() {
                s.push(f.to_ascii_uppercase());
                s.extend(ch);
            }
        }
        if s.is_empty() { "Design".into() } else { s }
    };
    let lib_rs = format!(
        r#"//! Bitloom design crate (scaffolded by `cargo bitloom new`).

use bitloom_prelude::rhdl::module;
use bitloom_prelude::{{Clock, Elaboratable, Input, Output, Reset, UInt}};

#[module]
pub struct {struct_name} {{
    pub clk: Input<Clock>,
    pub rst: Input<Reset>,
    pub data_in: Input<UInt<8>>,
    pub data_out: Output<UInt<8>>,
}}

/// Entry for `cargo bitloom build --package {name}`.
pub fn rhdl_elaborate() -> Result<bitloom_prelude::FrozenHir, bitloom_prelude::Diagnostics> {{
    {struct_name}::elaborate()
}}
"#,
        struct_name = struct_name,
        name = name,
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml).map_err(|e| e.to_string())?;
    fs::write(dir.join("src/lib.rs"), lib_rs).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if matches!(
        args.get(1).map(|s| s.as_str()),
        Some("bitloom") | Some("rhdl")
    ) {
        args.remove(1);
    }
    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Build {
            package,
            out_dir,
            manifest_dir,
        } => {
            let workspace = fs::canonicalize(&manifest_dir).unwrap_or(manifest_dir);
            let pkg_path = match resolve_package_dir(&workspace, &package) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            let host_dir = workspace.join("target/rhdl-host").join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            let host_toml = build_host_cargo(&workspace, &package, &pkg_path);
            fs::write(host_dir.join("Cargo.toml"), &host_toml).expect("host Cargo.toml");
            fs::write(
                host_dir.join("src/main.rs"),
                build_host_main(&package, &abs_out),
            )
            .expect("host main");
            let status = Command::new("cargo")
                .arg("+1.97.1")
                .arg("run")
                .arg("--manifest-path")
                .arg(host_dir.join("Cargo.toml"))
                .status()
                .expect("spawn cargo");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::New { name, path } => match scaffold_new(&name, &path) {
            Ok(dir) => {
                println!("created {}", dir.display());
                println!(
                    "next: cargo bitloom build --package {name} --manifest-dir {name} --out-dir out"
                );
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
        Commands::Firtool { cmd } => match cmd {
            FirtoolCmd::Info => {
                println!("version={}", firtool::FIRTOOL_VERSION);
                match firtool::firtool_asset_for_host() {
                    Ok(a) => {
                        println!("asset={a}");
                        println!("sha_asset={a}.sha256");
                    }
                    Err(e) => println!("asset_error={e}"),
                }
                println!(
                    "note=HIR→RHDL source regen is debug-only (NFR10); see docs/hir-to-source-debug-only.md"
                );
            }
            FirtoolCmd::Ensure => match firtool::ensure_firtool() {
                Ok(p) => {
                    println!("{}", p.display());
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
        },
        Commands::SimEngines => {
            println!("interpreter  # default: walk FrozenHir AST each tick");
            println!("compiled     # linearized assign schedule compiled at Sim construction");
            println!("select=bitloom_sim::Sim::with_engine(hir, TickEngine::from_name(..))");
        }
        Commands::Hls {
            function,
            out_dir,
            emit_only,
            dataflow,
            in_tree,
            unroll,
            pipeline,
            ii,
            stages,
            handshake,
            channels,
            circt_handshake,
            clock_domains,
            elastic_depth,
        } => {
            if circt_handshake || handshake || in_tree {
                if circt_handshake {
                    println!(
                        "path=in-tree-circt-handshake fr129=true fr96=true circt_handshake=true channels={channels} clock_domains={clock_domains} elastic_depth={elastic_depth} dataflow={dataflow}"
                    );
                    match hls::parse_dataflow_alias(&dataflow).and_then(|op| {
                        let art = hls::schedule_circt_handshake_from_transform(
                            &function,
                            &[],
                            channels,
                            clock_domains,
                            elastic_depth,
                            move || op,
                        )?;
                        hls::emit_in_tree_schedule(&art, &out_dir)
                    }) {
                        Ok((sched, rtl)) => {
                            println!("ok_schedule={}", sched.display());
                            println!("ok_rtl={}", rtl.display());
                        }
                        Err(e) => {
                            eprintln!("error: {e}");
                            std::process::exit(1);
                        }
                    }
                } else if handshake {
                    println!(
                        "path=in-tree-handshake fr121=true fr96=true handshake=true channels={channels} dataflow={dataflow}"
                    );
                    match hls::parse_dataflow_alias(&dataflow).and_then(|op| {
                        let art = hls::schedule_handshake_from_transform(
                            &function,
                            &[],
                            channels,
                            move || op,
                        )?;
                        hls::emit_in_tree_schedule(&art, &out_dir)
                    }) {
                        Ok((sched, rtl)) => {
                            println!("ok_schedule={}", sched.display());
                            println!("ok_rtl={}", rtl.display());
                        }
                        Err(e) => {
                            eprintln!("error: {e}");
                            std::process::exit(1);
                        }
                    }
                } else {
                    let kind = if pipeline {
                        hls::InTreeScheduleKind::Pipeline {
                            initiation_interval: ii,
                            stages,
                        }
                    } else {
                        hls::InTreeScheduleKind::LoopUnroll { trip_count: unroll }
                    };
                    let fr110 = hls::meets_fr110_commercial_depth(&kind);
                    if pipeline {
                        println!(
                            "path=in-tree fr95=true fr96=true fr110={fr110} kind=pipeline ii={ii} pipeline_stages={stages} dataflow={dataflow}"
                        );
                    } else {
                        println!(
                            "path=in-tree fr95=true fr96=true kind=loop-unroll trip_count={unroll} dataflow={dataflow}"
                        );
                    }
                    // FR96: CLI `--in-tree --dataflow` dissolves the transform alias as a
                    // closure unit before entering the FR95/FR110 schedule path (no Bambu).
                    match hls::parse_dataflow_alias(&dataflow).and_then(|op| {
                        hls::run_hls_in_tree_from_transform(
                            &function,
                            &[],
                            kind,
                            move || op,
                            &out_dir,
                        )
                    }) {
                        Ok((sched, rtl)) => {
                            println!("ok_schedule={}", sched.display());
                            println!("ok_rtl={}", rtl.display());
                        }
                        Err(e) => {
                            eprintln!("error: {e}");
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                println!(
                    "backend={} version={} dataflow={}",
                    hls::HLS_BACKEND,
                    hls::HLS_BACKEND_VERSION,
                    dataflow
                );
                match hls::run_hls_dataflow(&function, &dataflow, &out_dir, emit_only) {
                    Ok(p) => {
                        if emit_only {
                            println!("emit_only={}", p.display());
                        } else {
                            println!("ok={}", p.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Import {
            input,
            out_dir,
            also_fir,
            also_chisel,
        } => match run_import(&input, &out_dir, also_fir, also_chisel) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
        Commands::GenFunc {
            package,
            out_dir,
            manifest_dir,
        } => {
            let workspace = fs::canonicalize(&manifest_dir).unwrap_or(manifest_dir);
            let pkg_path = match resolve_package_dir(&workspace, &package) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            let host_dir = workspace.join("target/rhdl-gen-func-host").join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            fs::write(
                host_dir.join("Cargo.toml"),
                build_gen_func_host_cargo(&workspace, &package, &pkg_path),
            )
            .expect("host Cargo.toml");
            fs::write(
                host_dir.join("src/main.rs"),
                build_gen_func_host_main(&package, &abs_out),
            )
            .expect("host main");
            let status = Command::new("cargo")
                .arg("+1.97.1")
                .arg("run")
                .arg("--manifest-path")
                .arg(host_dir.join("Cargo.toml"))
                .arg("--quiet")
                .status();
            match status {
                Ok(s) if s.success() => {}
                Ok(s) => {
                    eprintln!("error: gen-func host failed ({s})");
                    std::process::exit(s.code().unwrap_or(1));
                }
                Err(e) => {
                    eprintln!("error: spawn cargo: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::GenCycle {
            package,
            out_dir,
            manifest_dir,
        } => {
            let workspace = fs::canonicalize(&manifest_dir).unwrap_or(manifest_dir);
            let pkg_path = match resolve_package_dir(&workspace, &package) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            let host_dir = workspace.join("target/rhdl-gen-cycle-host").join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            fs::write(
                host_dir.join("Cargo.toml"),
                build_gen_func_host_cargo(&workspace, &package, &pkg_path)
                    .replace("bitloom-gen-func-shim", "bitloom-gen-cycle-shim"),
            )
            .expect("host Cargo.toml");
            fs::write(
                host_dir.join("src/main.rs"),
                build_gen_cycle_host_main(&package, &abs_out),
            )
            .expect("host main");
            let status = Command::new("cargo")
                .arg("+1.97.1")
                .arg("run")
                .arg("--manifest-path")
                .arg(host_dir.join("Cargo.toml"))
                .arg("--quiet")
                .status();
            match status {
                Ok(s) if s.success() => {}
                Ok(s) => {
                    eprintln!("error: gen-cycle host failed ({s})");
                    std::process::exit(s.code().unwrap_or(1));
                }
                Err(e) => {
                    eprintln!("error: spawn cargo: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::GenTlm {
            package,
            out_dir,
            manifest_dir,
        } => {
            let workspace = fs::canonicalize(&manifest_dir).unwrap_or(manifest_dir);
            let pkg_path = match resolve_package_dir(&workspace, &package) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            let host_dir = workspace.join("target/bitloom-gen-tlm-host").join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            fs::write(
                host_dir.join("Cargo.toml"),
                build_gen_func_host_cargo(&workspace, &package, &pkg_path)
                    .replace("bitloom-gen-func-shim", "bitloom-gen-tlm-shim"),
            )
            .expect("host Cargo.toml");
            fs::write(
                host_dir.join("src/main.rs"),
                build_gen_tlm_host_main(&package, &abs_out),
            )
            .expect("host main");
            let status = Command::new("cargo")
                .arg("+1.97.1")
                .arg("run")
                .arg("--manifest-path")
                .arg(host_dir.join("Cargo.toml"))
                .arg("--quiet")
                .status();
            match status {
                Ok(s) if s.success() => {}
                Ok(s) => {
                    eprintln!("error: gen-tlm host failed ({s})");
                    std::process::exit(s.code().unwrap_or(1));
                }
                Err(e) => {
                    eprintln!("error: spawn cargo: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::GenTlmAt {
            package,
            out_dir,
            manifest_dir,
        } => {
            let workspace = fs::canonicalize(&manifest_dir).unwrap_or(manifest_dir);
            let pkg_path = match resolve_package_dir(&workspace, &package) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            let host_dir = workspace
                .join("target/bitloom-gen-tlm-at-host")
                .join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            fs::write(
                host_dir.join("Cargo.toml"),
                build_gen_func_host_cargo(&workspace, &package, &pkg_path)
                    .replace("bitloom-gen-func-shim", "bitloom-gen-tlm-at-shim"),
            )
            .expect("host Cargo.toml");
            fs::write(
                host_dir.join("src/main.rs"),
                build_gen_tlm_at_host_main(&package, &abs_out),
            )
            .expect("host main");
            let status = Command::new("cargo")
                .arg("+1.97.1")
                .arg("run")
                .arg("--manifest-path")
                .arg(host_dir.join("Cargo.toml"))
                .arg("--quiet")
                .status();
            match status {
                Ok(s) if s.success() => {}
                Ok(s) => {
                    eprintln!("error: gen-tlm-at host failed ({s})");
                    std::process::exit(s.code().unwrap_or(1));
                }
                Err(e) => {
                    eprintln!("error: spawn cargo: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Visualize { input, out_dir } | Commands::Doc { input, out_dir } => {
            match run_visualize(&input, &out_dir) {
                Ok(path) => println!("wrote {}", path.display()),
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Wave {
            input,
            out_dir,
            ticks,
            fst,
            tywaves,
            tywaves_gui: _,
            no_tywaves_gui,
            chiselsim,
            ide_stores,
        } => {
            // FR162: GUI depth is the default primary wave surface unless opted out.
            // `--tywaves-gui` remains accepted (FR134 ATDD / docs); default path already on.
            let emit_gui = !no_tywaves_gui;
            match run_wave(
                &input, &out_dir, ticks, fst, tywaves, emit_gui, chiselsim, ide_stores,
            ) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Coverage {
            input,
            out_dir,
            ticks,
            genhtml,
            genhtml_out,
        } => match run_coverage(
            input.as_deref(),
            &out_dir,
            ticks,
            genhtml,
            genhtml_out.as_deref(),
        ) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
    }
}

fn run_import(
    input: &Path,
    out_dir: &Path,
    also_fir: bool,
    also_chisel: bool,
) -> Result<(), String> {
    let text = fs::read_to_string(input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let hir = bitloom_firrtl::import(&text).map_err(|d| d.to_string())?;
    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;
    let art = bitloom_vlog::emit(&hir);
    for f in &art.files {
        let path = out_dir.join(&f.path);
        fs::write(&path, &f.contents).map_err(|e| format!("write {}: {e}", path.display()))?;
        println!("wrote {}", path.display());
    }
    if also_fir {
        let fir = bitloom_firrtl::emit(&hir);
        for f in &fir.files {
            let path = out_dir.join(&f.path);
            fs::write(&path, &f.contents).map_err(|e| format!("write {}: {e}", path.display()))?;
            println!("wrote {}", path.display());
        }
    }
    if also_chisel {
        let chisel = bitloom_firrtl::emit_chisel(&hir).map_err(|e| e.to_string())?;
        for f in &chisel.files {
            let path = out_dir.join(&f.path);
            fs::write(&path, &f.contents).map_err(|e| format!("write {}: {e}", path.display()))?;
            println!("wrote {}", path.display());
        }
    }
    Ok(())
}

/// Product entry: FrozenHir → hierarchy HTML (`hierarchy.html`).
fn run_visualize(input: &Path, out_dir: &Path) -> Result<PathBuf, String> {
    let text = fs::read_to_string(input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let hir = bitloom_firrtl::import(&text).map_err(|d| d.to_string())?;
    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;
    let html = bitloom_viz::to_html(&hir);
    if !html.contains("Instance hierarchy") || !html.contains("Modules and ports") {
        return Err("hierarchy HTML missing required sections".into());
    }
    let path = out_dir.join("hierarchy.html");
    fs::write(&path, &html).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

/// Product entry: tick → VCD + timing.html + interactive.html (FR38/49 + FR104)
/// + typed-wave.html / wave.typed.json (FR117 subset B; secondary under FR162)
/// + **default** tywaves.gui.* FR134/FR162 GUI primary (`want_tywaves_gui`; opt out via CLI)
/// + FR125 sidecar when GUI primary or `--tywaves`.
/// + optional FR167 `--chiselsim` / `--ide-stores` deepen.
fn run_wave(
    input: &Path,
    out_dir: &Path,
    ticks: u64,
    want_fst: bool,
    want_tywaves: bool,
    want_tywaves_gui: bool,
    want_chiselsim: bool,
    want_ide_stores: bool,
) -> Result<(), String> {
    use bitloom_hir::PortValues;
    use bitloom_sim::Sim;

    let want_tywaves = want_tywaves || want_tywaves_gui;

    let text = fs::read_to_string(input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let hir = bitloom_firrtl::import(&text).map_err(|d| d.to_string())?;
    let title = hir.abi_name.clone();
    let typed = bitloom_viz::typed_signals_from_hir(&hir);
    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;

    let vcd_path = out_dir.join("wave.vcd");
    let mut sim = Sim::new(hir);
    if want_fst {
        let fst_path = out_dir.join("wave.fst");
        match sim.enable_fst(&fst_path) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("note: FST unavailable ({e}); continuing with VCD + timing HTML");
                sim.enable_vcd(&vcd_path)
                    .map_err(|e| format!("enable_vcd: {e}"))?;
            }
        }
    } else {
        sim.enable_vcd(&vcd_path)
            .map_err(|e| format!("enable_vcd: {e}"))?;
    }

    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("clk", 0);
    pv.set("x", 0);
    sim.set_inputs(pv.clone());
    sim.tick();

    pv.set("rst", 0);
    for i in 0..ticks {
        pv.set("x", i.wrapping_add(1));
        pv.set("clk", i & 1);
        sim.set_inputs(pv.clone());
        sim.tick();
    }

    if let Err(e) = sim.finish_waves() {
        eprintln!("note: finish_waves: {e}");
    }

    let vcd_text =
        fs::read_to_string(&vcd_path).map_err(|e| format!("read {}: {e}", vcd_path.display()))?;
    if vcd_text.trim().is_empty() {
        return Err(format!("VCD empty at {}", vcd_path.display()));
    }
    let samples = bitloom_viz::samples_from_vcd(&vcd_text)?;
    let html = bitloom_viz::timing_html(&title, &samples);
    if !html.contains("Value table") {
        return Err("timing HTML missing Value table".into());
    }
    let timing_path = out_dir.join("timing.html");
    fs::write(&timing_path, &html).map_err(|e| format!("write {}: {e}", timing_path.display()))?;

    let interactive = bitloom_viz::interactive_wave_html(&title, &samples);
    if !interactive.contains("data-bitloom-interactive-wave") {
        return Err("interactive HTML missing FR104 marker".into());
    }
    let interactive_path = out_dir.join("interactive.html");
    fs::write(&interactive_path, &interactive)
        .map_err(|e| format!("write {}: {e}", interactive_path.display()))?;

    if typed.is_empty() {
        return Err(
            "bitloom.typed-wave-empty: no typed signal metadata from HIR; cannot claim FR117"
                .into(),
        );
    }
    let typed_html = bitloom_viz::typed_wave_html(&title, &samples, &typed);
    if !typed_html.contains("data-bitloom-typed-wave=\"1\"") {
        return Err("typed-wave.html missing FR117 marker".into());
    }
    let typed_path = out_dir.join("typed-wave.html");
    fs::write(&typed_path, &typed_html)
        .map_err(|e| format!("write {}: {e}", typed_path.display()))?;

    let typed_json = bitloom_viz::typed_wave_json(&title, &samples, &typed);
    if !typed_json.contains("\"ty\"") || !typed_json.contains("\"signals\"") {
        return Err("wave.typed.json missing typed signal fields".into());
    }
    let typed_json_path = out_dir.join("wave.typed.json");
    fs::write(&typed_json_path, &typed_json)
        .map_err(|e| format!("write {}: {e}", typed_json_path.display()))?;

    println!("wrote {}", vcd_path.display());
    println!("wrote {}", timing_path.display());
    println!("wrote {}", interactive_path.display());
    println!("wrote {}", typed_path.display());
    println!("wrote {}", typed_json_path.display());

    if want_tywaves {
        let tywaves_json = bitloom_viz::tywaves_wave_json(&title, &samples, &typed);
        if !tywaves_json.contains("data-bitloom-tywaves")
            || !tywaves_json.contains("\"fr\": \"FR125\"")
            || !tywaves_json.contains("schemaVersion")
        {
            return Err("wave.tywaves.json missing FR125 Tywaves contract fields".into());
        }
        let tywaves_path = out_dir.join("wave.tywaves.json");
        fs::write(&tywaves_path, &tywaves_json)
            .map_err(|e| format!("write {}: {e}", tywaves_path.display()))?;
        let launch = bitloom_viz::tywaves_launch_sh("wave.tywaves.json");
        let launch_path = out_dir.join("tywaves.launch.sh");
        fs::write(&launch_path, &launch)
            .map_err(|e| format!("write {}: {e}", launch_path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&launch_path)
                .map_err(|e| format!("stat {}: {e}", launch_path.display()))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&launch_path, perms)
                .map_err(|e| format!("chmod {}: {e}", launch_path.display()))?;
        }
        println!("wrote {}", tywaves_path.display());
        println!("wrote {}", launch_path.display());

        if want_tywaves_gui {
            let manifest = bitloom_viz::tywaves_gui_manifest(&title);
            if !manifest.contains("data-bitloom-tywaves-gui")
                || !manifest.contains("\"fr\": \"FR134\"")
                || !manifest.contains("schemaVersion")
                || !manifest.contains("tywaves.gui")
                || !manifest.contains("tywaves.ide-plugin")
            {
                return Err("tywaves.gui.manifest.json missing FR134 contract fields".into());
            }
            let manifest_path = out_dir.join("tywaves.gui.manifest.json");
            fs::write(&manifest_path, &manifest)
                .map_err(|e| format!("write {}: {e}", manifest_path.display()))?;
            let install = bitloom_viz::tywaves_gui_install_json();
            if !install.contains("version")
                || !install.contains("channel")
                || !install.contains("tywaves.gui")
                || !install.contains("tywaves.ide-plugin")
            {
                return Err("tywaves.gui.install.json missing FR134 G1 fields".into());
            }
            let install_path = out_dir.join("tywaves.gui.install.json");
            fs::write(&install_path, &install)
                .map_err(|e| format!("write {}: {e}", install_path.display()))?;
            let gui_sh = bitloom_viz::tywaves_gui_install_sh();
            let gui_sh_path = out_dir.join("tywaves.gui.install.sh");
            fs::write(&gui_sh_path, &gui_sh)
                .map_err(|e| format!("write {}: {e}", gui_sh_path.display()))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&gui_sh_path)
                    .map_err(|e| format!("stat {}: {e}", gui_sh_path.display()))?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&gui_sh_path, perms)
                    .map_err(|e| format!("chmod {}: {e}", gui_sh_path.display()))?;
            }
            println!("wrote {}", manifest_path.display());
            println!("wrote {}", install_path.display());
            println!("wrote {}", gui_sh_path.display());

            let gui_force = std::env::var_os("BITLOOM_TYWAVES_GUI_FORCE_MISSING").is_some();
            let gui_root = std::env::var_os("BITLOOM_TYWAVES_GUI_ROOT");
            let gui_ok = gui_root.as_ref().map(|r| {
                let p = Path::new(r);
                p.is_dir() && p.join("BITLOOM_TYWAVES_GUI_OK").is_file()
            });
            if gui_force || gui_ok == Some(false) {
                return Err(
                    "bitloom.tywaves-missing: upstream Tywaves GUI install / IDE plugin \
                     depth unavailable (set BITLOOM_TYWAVES_GUI_ROOT to a directory with \
                     BITLOOM_TYWAVES_GUI_OK marker, or unset BITLOOM_TYWAVES_GUI_FORCE_MISSING); \
                     FR134 manifests written but must not silent-succeed"
                        .into(),
                );
            }
            if let Some(root) = gui_root {
                println!(
                    "FR134: validated upstream Tywaves GUI root via BITLOOM_TYWAVES_GUI_ROOT ({})",
                    Path::new(&root).display()
                );
            } else {
                // No root and no force: write artifacts but do not claim live GUI green.
                // Require explicit root OR force for a decisive outcome — missing root without
                // force is a soft advisory (sidecar/manifests written) only when not claiming
                // live open. For ATDD honesty, treat missing root like FR125 missing BIN:
                // soft OK with advisory (FORCE_MISSING is the hard fail path).
                println!(
                    "FR162/FR134: wrote GUI/IDE primary surface {}; set BITLOOM_TYWAVES_GUI_ROOT \
                     (or exec {}) to validate upstream GUI install; IDE plugin channel in \
                     tywaves.gui.install.json (typed-wave.html remains secondary)",
                    manifest_path.display(),
                    gui_sh_path.display()
                );
            }
        }

        // FR125 live-open semantics (unchanged when GUI path also runs).
        let force_missing = std::env::var_os("BITLOOM_TYWAVES_FORCE_MISSING").is_some();
        let bin = std::env::var_os("BITLOOM_TYWAVES_BIN");
        if force_missing
            || bin
                .as_ref()
                .map(|b| !Path::new(b).is_file())
                .unwrap_or(false)
        {
            return Err(
                "bitloom.tywaves-missing: upstream Tywaves viewer unavailable \
                 (set BITLOOM_TYWAVES_BIN to an executable, or unset \
                 BITLOOM_TYWAVES_FORCE_MISSING); sidecar written but FR125 live \
                 open must not silent-succeed"
                    .into(),
            );
        }
        if let Some(bin) = bin {
            let status = Command::new(&bin)
                .arg(&tywaves_path)
                .status()
                .map_err(|e| format!("bitloom.tywaves-missing: spawn {:?}: {e}", bin))?;
            if !status.success() {
                return Err(format!(
                    "bitloom.tywaves: upstream viewer exited non-zero ({status})"
                ));
            }
            println!(
                "FR125: launched upstream Tywaves via BITLOOM_TYWAVES_BIN ({})",
                Path::new(&bin).display()
            );
        } else if !want_tywaves_gui {
            println!(
                "FR125: wrote Tywaves sidecar {}; set BITLOOM_TYWAVES_BIN and re-run \
                 --tywaves (or exec {}) to live-open upstream viewer",
                tywaves_path.display(),
                launch_path.display()
            );
        }
    } else {
        println!(
            "open {} in a browser for FR117 typed IDE wave (interactive.html is FR104 I1–I3; \
             GTKWave optional for {}; pass --tywaves for FR125 upstream Tywaves sidecar; \
             omit --no-tywaves-gui for FR162/FR134 GUI primary depth)",
            typed_path.display(),
            vcd_path.display()
        );
    }

    if want_chiselsim {
        let manifest = bitloom::fr167::chiselsim_manifest(&title);
        if !manifest.contains("data-bitloom-chiselsim")
            || !manifest.contains("\"fr\": \"FR167\"")
            || !manifest.contains("chiselsim")
        {
            return Err("chiselsim.manifest.json missing FR167 contract fields".into());
        }
        let manifest_path = out_dir.join("chiselsim.manifest.json");
        fs::write(&manifest_path, &manifest)
            .map_err(|e| format!("write {}: {e}", manifest_path.display()))?;
        let install = bitloom::fr167::chiselsim_install_json();
        let install_path = out_dir.join("chiselsim.install.json");
        fs::write(&install_path, &install)
            .map_err(|e| format!("write {}: {e}", install_path.display()))?;
        let check_sh = bitloom::fr167::chiselsim_check_sh();
        let check_path = out_dir.join("chiselsim.check.sh");
        fs::write(&check_path, &check_sh)
            .map_err(|e| format!("write {}: {e}", check_path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&check_path)
                .map_err(|e| format!("stat {}: {e}", check_path.display()))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&check_path, perms)
                .map_err(|e| format!("chmod {}: {e}", check_path.display()))?;
        }
        println!("wrote {}", manifest_path.display());
        println!("wrote {}", install_path.display());
        println!("wrote {}", check_path.display());

        let force = std::env::var_os("BITLOOM_CHISELSIM_FORCE_MISSING").is_some();
        let root = std::env::var_os("BITLOOM_CHISELSIM_ROOT");
        let root_ok = root.as_ref().is_some_and(|r| {
            let p = Path::new(r);
            p.is_dir() && p.join("BITLOOM_CHISELSIM_OK").is_file()
        });
        if force || !root_ok {
            return Err("bitloom.chiselsim-missing: ChiselSim coupling unavailable \
                 (set BITLOOM_CHISELSIM_ROOT to a directory with BITLOOM_CHISELSIM_OK, \
                 or unset BITLOOM_CHISELSIM_FORCE_MISSING); FR167 manifests written \
                 but must not silent-succeed"
                .into());
        }
        println!(
            "FR167: validated ChiselSim root via BITLOOM_CHISELSIM_ROOT ({})",
            Path::new(root.as_ref().unwrap()).display()
        );
    }

    if want_ide_stores {
        let manifest = bitloom::fr167::ide_stores_manifest(&title);
        if !manifest.contains("data-bitloom-ide-stores")
            || !manifest.contains("\"fr\": \"FR167\"")
            || !manifest.contains("open-vsx")
            || !manifest.contains("jetbrains")
        {
            return Err("tywaves.ide-stores.manifest.json missing FR167 multi-store fields".into());
        }
        let manifest_path = out_dir.join("tywaves.ide-stores.manifest.json");
        fs::write(&manifest_path, &manifest)
            .map_err(|e| format!("write {}: {e}", manifest_path.display()))?;
        let install = bitloom::fr167::ide_stores_install_json();
        if !install.contains("OVSX_PAT") || !install.contains("JETBRAINS_TOKEN") {
            return Err("tywaves.ide-stores.install.json missing token env contract".into());
        }
        let install_path = out_dir.join("tywaves.ide-stores.install.json");
        fs::write(&install_path, &install)
            .map_err(|e| format!("write {}: {e}", install_path.display()))?;
        println!("wrote {}", manifest_path.display());
        println!("wrote {}", install_path.display());

        let force = std::env::var_os("BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING").is_some();
        let ovsx = std::env::var_os("OVSX_PAT").filter(|v| !v.is_empty());
        let jb = std::env::var_os("JETBRAINS_TOKEN").filter(|v| !v.is_empty());
        if force || ovsx.is_none() || jb.is_none() {
            return Err(
                "bitloom.ide-store-missing-token: Open VSX / JetBrains publish tokens \
                 unavailable (set OVSX_PAT and JETBRAINS_TOKEN for live publish, or unset \
                 BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING); FR167 store descriptors written \
                 but must not silent-succeed as published"
                    .into(),
            );
        }
        println!(
            "FR167: IDE store tokens present (OVSX_PAT + JETBRAINS_TOKEN); \
             live publish via scripts/publish-tywaves-ide-stores.sh (NFR75)"
        );
    }

    Ok(())
}

/// Product entry: tick → `coverage.lcov` + `coverage.html` (FR114).
/// Optional `--genhtml` → third-party HTML via `genhtml` (FR158).
fn run_coverage(
    input: Option<&Path>,
    out_dir: &Path,
    ticks: u64,
    want_genhtml: bool,
    genhtml_out: Option<&Path>,
) -> Result<(), String> {
    use bitloom::lcov_gui::run_genhtml;
    use bitloom_builder::{ElaborateSession, GroundType, Span};
    use bitloom_hir::PortValues;
    use bitloom_sim::Sim;

    let hir = if let Some(path) = input {
        let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        bitloom_firrtl::import(&text).map_err(|d| d.to_string())?
    } else {
        let mut s = ElaborateSession::new("Fr114Mux");
        s.begin_module("Fr114Mux", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("sel", GroundType::Bool, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_mux("y", "sel", "a", "b", Span::default());
        s.end_process();
        s.end_module();
        s.finish().map_err(|d| d.to_string())?
    };

    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;
    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("clk", 0);
    pv.set("sel", 0);
    pv.set("a", 1);
    pv.set("b", 2);
    sim.set_inputs(pv.clone());
    sim.tick();

    pv.set("rst", 0);
    for i in 0..ticks {
        pv.set("sel", 0);
        pv.set("a", i.wrapping_add(1));
        pv.set("b", 2);
        pv.set("clk", i & 1);
        sim.set_inputs(pv.clone());
        sim.tick();
    }

    let (lcov, html) = sim
        .write_coverage_artifacts(out_dir)
        .map_err(|e| format!("FR114 coverage artifacts: {e}"))?;
    let lcov_text =
        fs::read_to_string(&lcov).map_err(|e| format!("read {}: {e}", lcov.display()))?;
    if !lcov_text.contains("end_of_record") || !lcov_text.contains("DA:") {
        return Err("coverage.lcov missing LCOV markers".into());
    }
    let html_text =
        fs::read_to_string(&html).map_err(|e| format!("read {}: {e}", html.display()))?;
    if !html_text.contains("data-bitloom-coverage-gui") {
        return Err("coverage.html missing FR114 GUI marker".into());
    }
    println!("wrote {}", lcov.display());
    println!("wrote {}", html.display());
    println!(
        "open {} in a browser for FR114 coverage GUI (LCOV at {}; ≠ Tywaves; ≠ FR104 wave)",
        html.display(),
        lcov.display()
    );

    if want_genhtml {
        let gen_out = genhtml_out
            .map(PathBuf::from)
            .unwrap_or_else(|| out_dir.join("genhtml"));
        let entry = run_genhtml(&lcov, &gen_out).map_err(|e| e.to_string())?;
        println!(
            "FR158: wrote third-party genhtml HTML at {} (≠ FR114 in-tree coverage.html)",
            entry.display()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn host_cargo_uses_registry_backends_outside_monorepo() {
        let ws = Path::new("/tmp/fake-ws-no-toolchain");
        let pkg = Path::new("/tmp/fake-ws-no-toolchain/my_design");
        let toml = build_host_cargo(ws, "my_design", pkg);
        assert!(
            toml.contains(&format!("bitloom-vlog = \"{BITLOOM_BACKEND_VERSION}\"")),
            "expected version-pinned bitloom-vlog, got:\n{toml}"
        );
        assert!(
            !toml.contains("crates/rhdl-vlog") && !toml.contains("crates/bitloom-vlog"),
            "must not path-depend monorepo vlog outside monorepo"
        );
    }
}

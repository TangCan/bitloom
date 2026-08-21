//! `cargo bitloom` CLI — published as crate `bitloom` (AD-2). Never publish as `rhdl` / `rhdl-bits`.

mod firtool;
mod hls;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
    /// Product HLS path: emit C and invoke pinned Bambu (FR35 / FR50 / AD-25).
    Hls {
        /// Top function name (also used for the emitted C stub).
        #[arg(long, default_value = "add")]
        function: String,
        /// Output directory for C stub and backend RTL artifacts.
        #[arg(long, default_value = "target/bitloom-hls")]
        out_dir: PathBuf,
        /// Write the C stub only; do not invoke Bambu (not a successful RTL run).
        #[arg(long, default_value_t = false)]
        emit_only: bool,
    },
    /// Import FIRRTL 6.0.0 `.fir` (Chisel→firtool output ok) into the same emit path as `build` (FR40 / FR46).
    Import {
        /// Path to a `.fir` file with `FIRRTL version 6.0.0` header.
        #[arg(long)]
        input: PathBuf,
        /// Directory for emitted Yosys-friendly `.v` (and optional `.fir` re-emit).
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Also write re-emitted FIRRTL text next to Verilog.
        #[arg(long, default_value_t = false)]
        also_fir: bool,
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
    /// Tick a `.fir` design, dump VCD, and emit browsable timing HTML (FR38 / FR49 / FR40).
    Wave {
        /// Path to a `.fir` file with `FIRRTL version 6.0.0` header.
        #[arg(long)]
        input: PathBuf,
        /// Directory for `wave.vcd` + `timing.html`.
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
        /// Number of ticks after reset.
        #[arg(long, default_value_t = 8)]
        ticks: u64,
        /// Also attempt FST via `vcd2fst` (optional; VCD+HTML always written).
        #[arg(long, default_value_t = false)]
        fst: bool,
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
        } => {
            println!(
                "backend={} version={}",
                hls::HLS_BACKEND,
                hls::HLS_BACKEND_VERSION
            );
            match hls::run_hls(&function, &out_dir, emit_only) {
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
        Commands::Import {
            input,
            out_dir,
            also_fir,
        } => match run_import(&input, &out_dir, also_fir) {
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
        } => match run_wave(&input, &out_dir, ticks, fst) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
    }
}

fn run_import(input: &Path, out_dir: &Path, also_fir: bool) -> Result<(), String> {
    let text = fs::read_to_string(input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let hir = rhdl_firrtl::import(&text).map_err(|d| d.to_string())?;
    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;
    let art = bitloom_vlog::emit(&hir);
    for f in &art.files {
        let path = out_dir.join(&f.path);
        fs::write(&path, &f.contents).map_err(|e| format!("write {}: {e}", path.display()))?;
        println!("wrote {}", path.display());
    }
    if also_fir {
        let fir = rhdl_firrtl::emit(&hir);
        for f in &fir.files {
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
    let hir = rhdl_firrtl::import(&text).map_err(|d| d.to_string())?;
    fs::create_dir_all(out_dir).map_err(|e| format!("create out_dir: {e}"))?;
    let html = rhdl_viz::to_html(&hir);
    if !html.contains("Instance hierarchy") || !html.contains("Modules and ports") {
        return Err("hierarchy HTML missing required sections".into());
    }
    let path = out_dir.join("hierarchy.html");
    fs::write(&path, &html).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

/// Product entry: tick → VCD + browsable `timing.html` (not GTKWave-only).
fn run_wave(input: &Path, out_dir: &Path, ticks: u64, want_fst: bool) -> Result<(), String> {
    use bitloom_hir::PortValues;
    use bitloom_sim::Sim;

    let text = fs::read_to_string(input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let hir = rhdl_firrtl::import(&text).map_err(|d| d.to_string())?;
    let title = hir.abi_name.clone();
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
    let samples = rhdl_viz::samples_from_vcd(&vcd_text)?;
    let html = rhdl_viz::timing_html(&title, &samples);
    if !html.contains("Value table") {
        return Err("timing HTML missing Value table".into());
    }
    let timing_path = out_dir.join("timing.html");
    fs::write(&timing_path, &html).map_err(|e| format!("write {}: {e}", timing_path.display()))?;
    println!("wrote {}", vcd_path.display());
    println!("wrote {}", timing_path.display());
    println!(
        "open {} in a browser (GTKWave optional for {})",
        timing_path.display(),
        vcd_path.display()
    );
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

//! `cargo bitloom` CLI — published as crate `bitloom` (AD-2). Never publish as `rhdl` / `rhdl-bits`.

mod firtool;
mod hls;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{Parser, Subcommand};

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
    /// Manage the pinned CIRCT firtool binary (AD-9 / NFR3).
    Firtool {
        #[command(subcommand)]
        cmd: FirtoolCmd,
    },
    /// List `rhdl-sim` tick engines (FR32). Simulation itself lives in tests / rhdl-sim.
    SimEngines,
    /// Optional HLS front-end status / run (FR35 / AD-25). Default: unsupported.
    Hls {
        #[arg(long, default_value = "add")]
        function: String,
        #[arg(long, default_value = "target/rhdl-hls")]
        out_dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum FirtoolCmd {
    /// Download (if needed), verify sha256, and print the firtool binary path.
    Ensure,
    /// Print configured version and asset names (no download).
    Info,
}

fn package_path(workspace: &Path, package: &str) -> PathBuf {
    let candidates = [
        workspace.join("examples").join(package),
        workspace.join("crates").join(package),
        workspace.join(package),
    ];
    candidates
        .into_iter()
        .find(|p| p.join("Cargo.toml").exists())
        .unwrap_or_else(|| workspace.join("examples").join(package))
}

fn build_host_cargo(workspace: &Path, package: &str) -> String {
    let pkg_path = package_path(workspace, package);
    let crate_name = package.replace('-', "_");
    format!(
        r#"[package]
name = "rhdl-host-shim"
version = "0.0.0"
edition = "2024"
publish = false

# Keep this shim out of the parent rhdl workspace.
[workspace]

[dependencies]
{crate_name} = {{ path = "{pkg}" }}
bitloom-vlog = {{ path = "{vlog}" }}
bitloom-hir = {{ path = "{hir}" }}
"#,
        crate_name = crate_name,
        pkg = pkg_path.display(),
        vlog = workspace.join("crates/rhdl-vlog").display(),
        hir = workspace.join("crates/rhdl-hir").display(),
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
            let host_dir = workspace.join("target/rhdl-host").join(&package);
            fs::create_dir_all(host_dir.join("src")).expect("host dir");
            let abs_out = if out_dir.is_absolute() {
                out_dir
            } else {
                workspace.join(out_dir)
            };
            fs::create_dir_all(&abs_out).expect("out_dir");
            fs::write(
                host_dir.join("Cargo.toml"),
                build_host_cargo(&workspace, &package),
            )
            .expect("host Cargo.toml");
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
            println!("select=rhdl_sim::Sim::with_engine(hir, TickEngine::from_name(..))");
        }
        Commands::Hls { function, out_dir } => {
            println!("backend={}", hls::HLS_BACKEND);
            match hls::run_hls(&function, &out_dir) {
                Ok(p) => println!("ok={}", p.display()),
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

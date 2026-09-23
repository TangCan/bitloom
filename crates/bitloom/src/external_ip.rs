use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: u32 = 1;
const ADAPTER_NAME: &str = "bitloom-yosys-sv-compat";
const ADAPTER_VERSION: u32 = 1;

#[cfg(test)]
#[path = "external_ip_tests.rs"]
mod tests;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Manifest {
    schema_version: u32,
    name: String,
    owner: String,
    support_level: String,
    sources: Vec<SourceIntent>,
    dependencies: Vec<DependencyIntent>,
    compile: CompileIntent,
    maintenance: MaintenanceIntent,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SourceIntent {
    name: String,
    source_type: String,
    url: String,
    r#ref: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DependencyIntent {
    parent: String,
    name: String,
    declared_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CompileIntent {
    top: String,
    files: Vec<String>,
    include_dirs: Vec<String>,
    defines: Vec<String>,
    parameters: BTreeMap<String, String>,
    ports: Vec<PortIntent>,
    clock_reset: ClockResetIntent,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PortIntent {
    name: String,
    direction: String,
    width: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ClockResetIntent {
    clock_port: String,
    reset_port: String,
    reset_polarity: String,
    reset_kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MaintenanceIntent {
    upgrade_policy: String,
    deprecation_policy: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SourceLock {
    name: String,
    source_type: String,
    url: String,
    r#ref: String,
    tag_type: String,
    tag_object: String,
    commit: String,
    closure_digest: String,
    tree_digest: String,
    cache_path: String,
    files: Vec<FileLock>,
    license: LicenseLock,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FileLock {
    path: String,
    sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LicenseLock {
    source_path: String,
    sha256: String,
    archive_path: String,
    notice_paths: Vec<String>,
    copyright_attribution_facts: String,
    redistribution: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolLock {
    bitloom: String,
    bitloom_path: String,
    bitloom_sha256: String,
    git: String,
    git_path: String,
    git_sha256: String,
    yosys: String,
    yosys_path: String,
    yosys_sha256: String,
    timeout: String,
    timeout_path: String,
    timeout_sha256: String,
    #[serde(default)]
    iverilog: String,
    #[serde(default)]
    iverilog_path: String,
    #[serde(default)]
    iverilog_sha256: String,
    #[serde(default)]
    iverilog_base: String,
    #[serde(default)]
    iverilog_runtime_sha256: BTreeMap<String, String>,
    #[serde(default)]
    vvp: String,
    #[serde(default)]
    vvp_path: String,
    #[serde(default)]
    vvp_sha256: String,
    adapter: String,
    adapter_version: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LockFile {
    schema_version: u32,
    manifest_sha256: String,
    name: String,
    owner: String,
    support_level: String,
    sources: Vec<SourceLock>,
    dependencies: Vec<DependencyIntent>,
    compile: CompileIntent,
    maintenance: MaintenanceIntent,
    binding: BindingLock,
    generator: GeneratorLock,
    evidence_paths: Vec<String>,
    last_passed_utc_location: String,
    tools: ToolLock,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BindingLock {
    kind: String,
    version: String,
    owner: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GeneratorLock {
    kind: String,
    version: String,
    inputs: Vec<String>,
}

fn fail(class: &str, message: impl AsRef<str>) -> String {
    format!("bitloom.external-ip.{class}: {}", message.as_ref())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, kind: &str) -> Result<T, String> {
    let bytes = fs::read(path).map_err(|e| fail("io", format!("read {}: {e}", path.display())))?;
    serde_json::from_slice(&bytes)
        .map_err(|e| fail("schema", format!("invalid {kind} {}: {e}", path.display())))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    fs::read(path)
        .map(|bytes| sha256(&bytes))
        .map_err(|e| fail("io", format!("read {}: {e}", path.display())))
}

fn validate_relative(value: &str, kind: &str) -> Result<(), String> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(fail("path", format!("unsafe {kind} path {value:?}")));
    }
    Ok(())
}

fn validate_manifest(manifest: &Manifest) -> Result<(), String> {
    if manifest.schema_version != SCHEMA_VERSION {
        return Err(fail(
            "schema",
            format!("manifest schemaVersion must be {SCHEMA_VERSION}"),
        ));
    }
    if manifest.sources.is_empty() || manifest.owner.trim().is_empty() {
        return Err(fail("schema", "sources and owner must be non-empty"));
    }
    if manifest.support_level != "catalogued" {
        return Err(fail(
            "scope",
            "Story 130.2 manifest supportLevel must remain catalogued",
        ));
    }
    let mut names = BTreeSet::new();
    for source in &manifest.sources {
        validate_relative(&source.name, "source name")?;
        if Path::new(&source.name).components().count() != 1 {
            return Err(fail(
                "path",
                format!("source name must be one path component: {}", source.name),
            ));
        }
        if !names.insert(&source.name) {
            return Err(fail("schema", format!("duplicate source {}", source.name)));
        }
        if !source.url.starts_with("https://") || !source.url.ends_with(".git") {
            return Err(fail(
                "source",
                format!("{} URL must be explicit HTTPS Git", source.name),
            ));
        }
        if source.source_type != "git" {
            return Err(fail("source", "Story 130.2 only accepts sourceType=git"));
        }
        let version = source.r#ref.strip_prefix('v').unwrap_or(&source.r#ref);
        let pieces: Vec<_> = version.split('.').collect();
        if pieces.len() != 3
            || pieces
                .iter()
                .any(|piece| piece.is_empty() || !piece.bytes().all(|b| b.is_ascii_digit()))
        {
            return Err(fail(
                "floating-ref",
                format!(
                    "{} ref {:?} is not an exact semantic version tag",
                    source.name, source.r#ref
                ),
            ));
        }
    }
    // This story admits one concrete, three-repository pilot.  Requiring the
    // complete source set here prevents a shortened manifest from silently
    // turning the lock into a different (and unreviewed) dependency closure.
    let expected_sources = [
        (
            "common_cells",
            "https://github.com/pulp-platform/common_cells.git",
            "v1.40.0",
        ),
        (
            "common_verification",
            "https://github.com/pulp-platform/common_verification.git",
            "v0.2.0",
        ),
        (
            "tech_cells_generic",
            "https://github.com/pulp-platform/tech_cells_generic.git",
            "v0.2.11",
        ),
    ];
    if manifest.sources.len() != expected_sources.len()
        || expected_sources.iter().any(|(name, url, r#ref)| {
            !manifest
                .sources
                .iter()
                .any(|source| source.name == *name && source.url == *url && source.r#ref == *r#ref)
        })
    {
        return Err(fail(
            "dependency",
            "candidate must contain the complete common_cells v1.40.0 source closure",
        ));
    }
    let expected_dependencies = [
        ("common_cells", "common_verification", "0.2.0"),
        ("common_cells", "tech_cells_generic", "0.2.11"),
        ("tech_cells_generic", "common_verification", "0.2.0"),
    ];
    if manifest.dependencies.len() != expected_dependencies.len()
        || expected_dependencies.iter().any(|expected| {
            !manifest.dependencies.iter().any(|dependency| {
                (
                    dependency.parent.as_str(),
                    dependency.name.as_str(),
                    dependency.declared_version.as_str(),
                ) == *expected
            })
        })
    {
        return Err(fail(
            "dependency",
            "candidate must declare all three reviewed Bender dependency edges",
        ));
    }
    for dependency in &manifest.dependencies {
        if !names.contains(&dependency.parent) || !names.contains(&dependency.name) {
            return Err(fail(
                "dependency",
                format!(
                    "undeclared dependency endpoint {} -> {}",
                    dependency.parent, dependency.name
                ),
            ));
        }
    }
    for value in manifest
        .compile
        .files
        .iter()
        .chain(manifest.compile.include_dirs.iter())
    {
        let (_, path) = value.split_once(':').ok_or_else(|| {
            fail(
                "schema",
                format!("compile entry lacks source prefix: {value}"),
            )
        })?;
        validate_relative(path, "compile")?;
    }
    let ports: BTreeSet<_> = manifest
        .compile
        .ports
        .iter()
        .map(|port| port.name.as_str())
        .collect();
    if ports.len() != manifest.compile.ports.len()
        || manifest.compile.ports.iter().any(|port| {
            !matches!(port.direction.as_str(), "input" | "output") || port.width.trim().is_empty()
        })
        || !ports.contains(manifest.compile.clock_reset.clock_port.as_str())
        || !ports.contains(manifest.compile.clock_reset.reset_port.as_str())
        || manifest.maintenance.upgrade_policy.trim().is_empty()
        || manifest.maintenance.deprecation_policy.trim().is_empty()
    {
        return Err(fail(
            "schema",
            "port widths, clock/reset mapping, and maintenance policies must be complete",
        ));
    }
    Ok(())
}

// GNU timeout owns the process group, including compiler/simulator children.
fn bounded_output(command: &Command, label: &str) -> Result<Output, String> {
    let mut bounded = Command::new("timeout");
    bounded
        .args(["--kill-after=5s", "60s"])
        .arg(command.get_program())
        .args(command.get_args());
    if let Some(directory) = command.get_current_dir() {
        bounded.current_dir(directory);
    }
    for (key, value) in command.get_envs() {
        if let Some(value) = value {
            bounded.env(key, value);
        } else {
            bounded.env_remove(key);
        }
    }
    let output = bounded
        .output()
        .map_err(|e| fail("tool", format!("spawn {label}: {e}")))?;
    if matches!(output.status.code(), Some(124 | 137)) {
        return Err(fail(
            "timeout",
            format!(
                "{label} exceeded 60 seconds: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }
    Ok(output)
}

fn command_output(command: &mut Command, label: &str) -> Result<Output, String> {
    let output = bounded_output(command, label)?;
    if !output.status.success() {
        return Err(fail(
            "tool",
            format!(
                "{label} exited {}: {}{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }
    Ok(output)
}

fn simulator_identity(name: &str) -> Result<(String, String, String), String> {
    let path = tool_path(name)?;
    let output = command_output(Command::new(&path).arg("-V"), name)?;
    let version = String::from_utf8_lossy(&output.stdout)
        .lines()
        .chain(String::from_utf8_lossy(&output.stderr).lines())
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned();
    if version.is_empty() {
        return Err(fail("tool", format!("{name} returned an empty version")));
    }
    let digest = sha256_file(Path::new(&path))?;
    Ok((version, path, digest))
}

// Pin the compiler behind the Icarus driver as well as the driver itself.
// The selected base is passed explicitly with -B; ambiguous installations
// fail rather than hashing one installation and executing another.
fn iverilog_runtime(driver: &str) -> Result<(String, BTreeMap<String, String>), String> {
    let prefix = Path::new(driver)
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| fail("tool", "cannot locate Icarus installation prefix"))?;
    let mut candidates = BTreeSet::new();
    for library in [prefix.join("lib"), prefix.join("lib64")] {
        candidates.insert(library.join("ivl"));
        if let Ok(entries) = fs::read_dir(&library) {
            for entry in entries {
                let entry =
                    entry.map_err(|e| fail("tool", format!("scan Icarus installation: {e}")))?;
                if entry.file_name().to_string_lossy().contains("linux-gnu") {
                    candidates.insert(entry.path().join("ivl"));
                }
            }
        }
    }
    let required = ["ivl", "ivlpp", "vvp.tgt", "vvp.conf"];
    let mut bases = BTreeSet::new();
    for candidate in candidates {
        if required.iter().all(|file| candidate.join(file).is_file()) {
            bases.insert(
                fs::canonicalize(candidate)
                    .map_err(|e| fail("tool", format!("Icarus base: {e}")))?,
            );
        }
    }
    if bases.len() != 1 {
        return Err(fail(
            "tool",
            "expected exactly one complete Icarus ivl installation",
        ));
    }
    let base = bases.into_iter().next().unwrap();
    let mut hashes = BTreeMap::new();
    for name in required {
        let file = base.join(name);
        hashes.insert(file.to_string_lossy().into_owned(), sha256_file(&file)?);
    }
    for entry in fs::read_dir(&base).map_err(|e| fail("tool", format!("Icarus runtime: {e}")))? {
        let file = entry
            .map_err(|e| fail("tool", format!("Icarus runtime: {e}")))?
            .path();
        if file.extension() == Some(OsStr::new("vpi")) {
            hashes.insert(file.to_string_lossy().into_owned(), sha256_file(&file)?);
        }
    }
    Ok((base.to_string_lossy().into_owned(), hashes))
}

fn line(output: Output) -> String {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned()
}

fn tool_path(name: &str) -> Result<String, String> {
    let path = std::env::var_os("PATH").ok_or_else(|| fail("tool", "PATH is unset"))?;
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(name);
        if candidate.is_file() {
            return fs::canonicalize(&candidate)
                .map(|path| path.to_string_lossy().into_owned())
                .map_err(|e| fail("tool", format!("canonicalize {}: {e}", candidate.display())));
        }
    }
    Err(fail("tool", format!("{name} is not on PATH")))
}

fn tool_lock() -> Result<ToolLock, String> {
    let current_exe = fs::canonicalize(
        std::env::current_exe().map_err(|e| fail("tool", format!("locate Bitloom CLI: {e}")))?,
    )
    .map_err(|e| fail("tool", format!("canonicalize Bitloom CLI: {e}")))?;
    // Paths are host-specific and cannot identify a copied replay binary.
    // Lock the logical tool identity; the executable's content hash remains
    // the immutable identity check.
    let bitloom_path = "bitloom".to_owned();
    let git = line(command_output(
        Command::new("git").arg("--version"),
        "git --version",
    )?);
    let yosys = line(command_output(Command::new("yosys").arg("-V"), "yosys -V")?);
    let timeout = line(command_output(
        Command::new("timeout").arg("--version"),
        "timeout --version",
    )?);
    let git_path = tool_path("git")?;
    let yosys_path = tool_path("yosys")?;
    let timeout_path = tool_path("timeout")?;
    let (iverilog, iverilog_path, iverilog_sha256) = simulator_identity("iverilog")?;
    let (vvp, vvp_path, vvp_sha256) = simulator_identity("vvp")?;
    let (iverilog_base, iverilog_runtime_sha256) = iverilog_runtime(&iverilog_path)?;
    Ok(ToolLock {
        bitloom: format!("bitloom {}", env!("CARGO_PKG_VERSION")),
        bitloom_path,
        bitloom_sha256: sha256_file(&current_exe)?,
        git,
        git_sha256: sha256_file(Path::new(&git_path))?,
        git_path,
        yosys,
        yosys_sha256: sha256_file(Path::new(&yosys_path))?,
        yosys_path,
        timeout,
        timeout_sha256: sha256_file(Path::new(&timeout_path))?,
        timeout_path,
        iverilog,
        iverilog_path,
        iverilog_sha256,
        iverilog_base,
        iverilog_runtime_sha256,
        vvp,
        vvp_path,
        vvp_sha256,
        adapter: ADAPTER_NAME.into(),
        adapter_version: ADAPTER_VERSION,
    })
}

fn temp_path(parent: &Path, name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    parent.join(format!(".tmp-{name}-{}-{nonce}", std::process::id()))
}

fn git_text(repo: &Path, args: &[&str], label: &str) -> Result<String, String> {
    let mut command = Command::new("git");
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .arg("-C")
        .arg(repo)
        .args(args);
    Ok(line(command_output(&mut command, label)?))
}

fn tracked_files(repo: &Path) -> Result<Vec<String>, String> {
    let output = command_output(
        Command::new("git")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .arg("-C")
            .arg(repo)
            .args(["ls-files", "-z"]),
        "git ls-files",
    )?;
    let mut files = Vec::new();
    for raw in output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
    {
        let path = std::str::from_utf8(raw)
            .map_err(|_| fail("path", "non-UTF-8 tracked path is unsupported"))?
            .to_owned();
        validate_relative(&path, "tracked")?;
        let metadata = fs::symlink_metadata(repo.join(&path))
            .map_err(|e| fail("io", format!("stat {path}: {e}")))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(fail(
                "path",
                format!("tracked entry must be a regular file: {path}"),
            ));
        }
        files.push(path);
    }
    files.sort();
    Ok(files)
}

fn tree_digest(files: &[FileLock]) -> String {
    let mut digest = Sha256::new();
    for file in files {
        digest.update(file.path.as_bytes());
        digest.update([0]);
        digest.update(file.sha256.as_bytes());
        digest.update([0]);
    }
    format!("{:x}", digest.finalize())
}

fn closure_digest(manifest: &Manifest, sources: &[SourceLock]) -> String {
    let mut digest = Sha256::new();
    digest.update(SCHEMA_VERSION.to_be_bytes());
    for source in sources {
        for value in [
            source.name.as_str(),
            source.source_type.as_str(),
            source.url.as_str(),
            source.r#ref.as_str(),
            source.tag_type.as_str(),
            source.tag_object.as_str(),
            source.commit.as_str(),
            source.tree_digest.as_str(),
        ] {
            digest.update(value.as_bytes());
            digest.update([0]);
        }
    }
    for dependency in &manifest.dependencies {
        for value in [
            dependency.parent.as_str(),
            dependency.name.as_str(),
            dependency.declared_version.as_str(),
        ] {
            digest.update(value.as_bytes());
            digest.update([0]);
        }
    }
    format!("{:x}", digest.finalize())
}

fn prepare_closure_metadata(
    manifest: &Manifest,
    sources: &mut [SourceLock],
) -> (String, Vec<String>) {
    let closure = closure_digest(manifest, sources);
    let closure_relative = format!("closures/v{SCHEMA_VERSION}-{closure}");
    let staging_paths = sources
        .iter()
        .map(|source| source.cache_path.clone())
        .collect();
    for source in sources {
        let name = format!("{}-{}-{}", source.name, source.commit, source.tree_digest);
        source.closure_digest = closure.clone();
        source.cache_path = format!("{closure_relative}/{name}");
    }
    (closure_relative, staging_paths)
}

fn publish_closure_cache(
    sources: &[SourceLock],
    staging_paths: &[String],
    cache: &Path,
    closure_relative: &str,
) -> Result<bool, String> {
    let final_root = cache_directory(cache, closure_relative, true)?;
    let created = !final_root.exists();
    if !created {
        for staging in staging_paths {
            fs::remove_dir_all(cache.join(staging))
                .map_err(|e| fail("io", format!("remove redundant source cache: {e}")))?;
        }
    } else {
        let staging_root = temp_path(cache, "closure");
        let result = (|| {
            fs::create_dir_all(&staging_root)
                .map_err(|e| fail("io", format!("create closure transaction: {e}")))?;
            for (source, staging_path) in sources.iter().zip(staging_paths) {
                let name = format!("{}-{}-{}", source.name, source.commit, source.tree_digest);
                fs::rename(cache.join(staging_path), staging_root.join(name))
                    .map_err(|e| fail("io", format!("stage closure cache: {e}")))?;
            }
            let parent = final_root
                .parent()
                .ok_or_else(|| fail("path", "closure cache has no parent"))?;
            fs::create_dir_all(parent)
                .map_err(|e| fail("io", format!("create closure cache parent: {e}")))?;
            fs::rename(&staging_root, &final_root)
                .map_err(|e| fail("io", format!("publish complete closure atomically: {e}")))
        })();
        if let Err(error) = result {
            let _ = fs::remove_dir_all(&staging_root);
            for staging in staging_paths {
                let _ = fs::remove_dir_all(cache.join(staging));
            }
            return Err(error);
        }
    }
    Ok(created)
}

fn archive_path(lock_path: &Path, source: &str) -> Result<(PathBuf, String), String> {
    let parent = lock_path
        .parent()
        .ok_or_else(|| fail("path", "lock path has no parent"))?;
    let relative = format!("licenses/{source}-LICENSE");
    Ok((parent.join(&relative), relative))
}

fn fetch_source(
    source: &SourceIntent,
    cache: &Path,
    lock_path: &Path,
) -> Result<SourceLock, String> {
    fs::create_dir_all(cache)
        .map_err(|e| fail("io", format!("create cache {}: {e}", cache.display())))?;
    let transaction = temp_path(cache, &source.name);
    fs::create_dir_all(&transaction).map_err(|e| fail("io", format!("create transaction: {e}")))?;
    let repo = transaction.join("repo");
    let result = (|| {
        let mut last_error = None;
        for attempt in 1..=3 {
            let _ = fs::remove_dir_all(&repo);
            command_output(
                Command::new("git").args(["init", "--quiet"]).arg(&repo),
                "git init fetch transaction",
            )?;
            command_output(
                Command::new("git").arg("-C").arg(&repo).args([
                    "remote",
                    "add",
                    "origin",
                    &source.url,
                ]),
                "git add immutable origin",
            )?;
            let fetch = command_output(
                Command::new("timeout")
                    .args(["--kill-after=5s", "30s", "git"])
                    .env("GIT_CONFIG_NOSYSTEM", "1")
                    .env("GIT_CONFIG_GLOBAL", "/dev/null")
                    .env_remove("GIT_DIR")
                    .env_remove("GIT_WORK_TREE")
                    .args([
                        "-c",
                        "http.lowSpeedLimit=1024",
                        "-c",
                        "http.lowSpeedTime=20",
                        "-C",
                    ])
                    .arg(&repo)
                    .args([
                        "fetch",
                        "--depth",
                        "1",
                        "--",
                        "origin",
                        &format!("refs/tags/{tag}:refs/tags/{tag}", tag = source.r#ref),
                    ]),
                &format!("git fetch {} attempt {attempt}/3", source.name),
            );
            match fetch {
                Ok(_) => {
                    command_output(
                        Command::new("git").arg("-C").arg(&repo).args([
                            "checkout",
                            "--quiet",
                            "--detach",
                            &format!("refs/tags/{}^{{commit}}", source.r#ref),
                        ]),
                        "git checkout immutable commit",
                    )?;
                    last_error = None;
                    break;
                }
                Err(error) => {
                    last_error = Some(error);
                    if attempt < 3 {
                        thread::sleep(Duration::from_secs(attempt));
                    }
                }
            }
        }
        if let Some(error) = last_error {
            return Err(error);
        }
        let tag_type = git_text(
            &repo,
            &["cat-file", "-t", &format!("refs/tags/{}", source.r#ref)],
            "git tag type",
        )?;
        if tag_type != "tag" && tag_type != "commit" {
            return Err(fail(
                "identity",
                format!("unsupported tag object type {tag_type}"),
            ));
        }
        let tag_object = git_text(
            &repo,
            &["rev-parse", &format!("refs/tags/{}", source.r#ref)],
            "git tag object",
        )?;
        let commit = git_text(
            &repo,
            &[
                "rev-parse",
                &format!("refs/tags/{}^{{commit}}", source.r#ref),
            ],
            "git peeled commit",
        )?;
        let paths = tracked_files(&repo)?;
        let files_root = transaction.join("files");
        let mut files = Vec::with_capacity(paths.len());
        for relative in paths {
            let from = repo.join(&relative);
            let to = files_root.join(&relative);
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| fail("io", format!("create {}: {e}", parent.display())))?;
            }
            fs::copy(&from, &to)
                .map_err(|e| fail("io", format!("copy {} to cache: {e}", from.display())))?;
            files.push(FileLock {
                path: relative,
                sha256: sha256_file(&from)?,
            });
        }
        let license_sha256 = files
            .iter()
            .find(|file| file.path == "LICENSE")
            .ok_or_else(|| fail("license", format!("{} has no root LICENSE", source.name)))?
            .sha256
            .clone();
        let notice_paths: Vec<_> = files
            .iter()
            .filter(|file| {
                Path::new(&file.path)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .is_some_and(|name| name.to_ascii_uppercase().starts_with("NOTICE"))
            })
            .map(|file| file.path.clone())
            .collect();
        let digest = tree_digest(&files);
        // Per-repository fetches remain in a hidden staging namespace. Only
        // `publish_closure_cache` publishes the complete recursive closure.
        let cache_relative = format!(".staging-sources/{}-{commit}-{digest}", source.name);
        let final_path = cache.join(&cache_relative);
        let staged_files = transaction.join("files");
        let (_, archive_relative) = archive_path(lock_path, &source.name)?;
        let staged_license = cache
            .join(".staging-licenses")
            .join(format!("{}-LICENSE", source.name));
        if let Some(parent) = staged_license.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| fail("io", format!("create license staging: {e}")))?;
        }
        fs::copy(repo.join("LICENSE"), &staged_license)
            .map_err(|e| fail("io", format!("stage LICENSE: {e}")))?;
        if final_path.exists() {
            fs::remove_dir_all(&staged_files)
                .map_err(|e| fail("io", format!("remove staged cache: {e}")))?;
        } else {
            if let Some(parent) = final_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| fail("io", format!("create cache parent: {e}")))?;
            }
            fs::rename(&staged_files, &final_path)
                .map_err(|e| fail("io", format!("publish cache staging: {e}")))?;
        }
        Ok(SourceLock {
            name: source.name.clone(),
            source_type: source.source_type.clone(),
            url: source.url.clone(),
            r#ref: source.r#ref.clone(),
            tag_type: if tag_type == "tag" {
                "annotated".into()
            } else {
                "lightweight".into()
            },
            tag_object,
            commit,
            closure_digest: String::new(),
            tree_digest: digest,
            cache_path: cache_relative,
            files,
            license: LicenseLock {
                source_path: "LICENSE".into(),
                sha256: license_sha256,
                archive_path: archive_relative,
                notice_paths,
                copyright_attribution_facts: "No repository-specific copyright line appears in the root LICENSE; preserve all source and license notices under SHL-0.51 clauses 3 and 4."
                    .into(),
                redistribution: "Solderpad Hardware License v0.51; retain license and notices"
                    .into(),
            },
        })
    })();
    let _ = fs::remove_dir_all(&transaction);
    result
}

fn publish_license_archives(
    sources: &[SourceLock],
    cache: &Path,
    lock_path: &Path,
) -> Result<(), String> {
    let staged = cache.join(".staging-licenses");
    for source in sources {
        validate_relative(&source.license.archive_path, "license archive")?;
        let destination = lock_path
            .parent()
            .unwrap()
            .join(&source.license.archive_path);
        if destination.exists() {
            if sha256_file(&destination)? != source.license.sha256 {
                return Err(fail(
                    "license-drift",
                    format!("refusing to replace {}", destination.display()),
                ));
            }
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| fail("io", format!("create license archive: {e}")))?;
        }
        fs::rename(
            staged.join(format!("{}-LICENSE", source.name)),
            &destination,
        )
        .map_err(|e| fail("io", format!("publish license archive: {e}")))?;
    }
    let _ = fs::remove_dir_all(staged);
    Ok(())
}

// Deliberately small, fail-closed parser for the admitted Bender dependency
// mapping. This is not a general YAML loader: aliases, tags and nested maps
// are rejected rather than guessed. Both upstream inline maps and block maps
// are supported; scalar values are compared in full.
fn dependency_scalar(value: &str) -> Result<String, String> {
    let value = value.trim();
    let scalar = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        &value[1..value.len() - 1]
    } else if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
        &value[1..value.len() - 1]
    } else {
        value
    };
    if scalar.is_empty()
        || scalar
            .chars()
            .any(|c| c.is_whitespace() || "{}[],&*!#\\\"'".contains(c))
    {
        return Err(fail("dependency", "unsupported dependency scalar"));
    }
    Ok(scalar.to_owned())
}

fn dependency_line(line: &str) -> Result<&str, String> {
    let mut quote = None;
    for (i, c) in line.char_indices() {
        if c == '\'' || c == '"' {
            if quote == Some(c) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(c);
            }
        } else if c == '#' && quote.is_none() {
            return Ok(line[..i].trim_end());
        }
    }
    if quote.is_some() {
        return Err(fail("dependency", "unterminated dependency quote"));
    }
    Ok(line.trim_end())
}

fn parse_dependencies(text: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>, String> {
    let mut result = BTreeMap::<String, BTreeMap<String, String>>::new();
    let mut active = false;
    let mut seen = false;
    let mut current: Option<String> = None;
    for raw in text.lines() {
        // Ignore unrelated YAML, whose syntax is outside this narrow parser.
        if !active && !raw.starts_with("dependencies:") {
            continue;
        }
        let line = dependency_line(raw)?;
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(' ') {
            active = false;
            if line == "dependencies:" || line == "dependencies: {}" {
                if seen {
                    return Err(fail("dependency", "duplicate dependencies section"));
                }
                seen = true;
                active = line == "dependencies:";
            } else if line.starts_with("dependencies:") {
                return Err(fail("dependency", "unsupported dependencies mapping"));
            }
            continue;
        }
        if !active {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        let (key, value) = line
            .trim()
            .split_once(':')
            .ok_or_else(|| fail("dependency", "invalid dependency entry"))?;
        let key = dependency_scalar(key)?;
        let value = value.trim();
        if indent == 2 {
            if result.contains_key(&key) {
                return Err(fail("dependency", "duplicate dependency name"));
            }
            let mut fields = BTreeMap::new();
            if !value.is_empty() {
                let body = value
                    .strip_prefix('{')
                    .and_then(|v| v.strip_suffix('}'))
                    .ok_or_else(|| fail("dependency", "expected dependency map"))?;
                for entry in body.split(',') {
                    let (field, scalar) = entry
                        .split_once(':')
                        .ok_or_else(|| fail("dependency", "invalid inline field"))?;
                    let field = dependency_scalar(field)?;
                    if fields.insert(field, dependency_scalar(scalar)?).is_some() {
                        return Err(fail("dependency", "duplicate dependency field"));
                    }
                }
                current = None;
            } else {
                current = Some(key.clone());
            }
            result.insert(key, fields);
        } else if indent == 4 {
            let name = current
                .as_ref()
                .ok_or_else(|| fail("dependency", "unexpected nested dependency field"))?;
            if result
                .get_mut(name)
                .unwrap()
                .insert(key, dependency_scalar(value)?)
                .is_some()
            {
                return Err(fail("dependency", "duplicate dependency field"));
            }
        } else {
            return Err(fail("dependency", "unsupported dependency indentation"));
        }
    }
    for fields in result.values() {
        if fields.len() != 2 || !fields.contains_key("git") || !fields.contains_key("version") {
            return Err(fail(
                "dependency",
                "only exact git/version dependency fields are admitted",
            ));
        }
    }
    Ok(result)
}

fn verify_dependencies(
    manifest: &Manifest,
    locks: &[SourceLock],
    cache: &Path,
) -> Result<(), String> {
    for parent in locks {
        let bender = cache.join(&parent.cache_path).join("Bender.yml");
        let text = fs::read_to_string(&bender)
            .map_err(|e| fail("dependency", format!("read {}: {e}", bender.display())))?;
        let actual = parse_dependencies(&text)?;
        let mut expected = BTreeMap::new();
        for dependency in manifest
            .dependencies
            .iter()
            .filter(|d| d.parent == parent.name)
        {
            let child = manifest
                .sources
                .iter()
                .find(|s| s.name == dependency.name)
                .ok_or_else(|| fail("dependency", "child intent missing"))?;
            expected.insert(
                dependency.name.clone(),
                BTreeMap::from([
                    ("git".to_owned(), child.url.clone()),
                    ("version".to_owned(), dependency.declared_version.clone()),
                ]),
            );
        }
        if actual != expected {
            return Err(fail(
                "dependency",
                format!("{} exact dependency closure differs", bender.display()),
            ));
        }
    }
    Ok(())
}

fn validate_pilot_compile(compile: &CompileIntent) -> Result<(), String> {
    if compile.top != "fifo_v3"
        || compile.files != ["common_cells:src/fifo_v3.sv"]
        || compile.include_dirs != ["common_cells:include"]
        || compile.defines != ["SYNTHESIS"]
    {
        return Err(fail(
            "compile-contract",
            "ordered filelist, include path, define, or top differs from the admitted pilot",
        ));
    }
    let expected_parameters = BTreeMap::from([
        (
            "ADDR_DEPTH".to_owned(),
            "(DEPTH > 1) ? $clog2(DEPTH) : 1".to_owned(),
        ),
        ("DATA_WIDTH".to_owned(), "32".to_owned()),
        ("DEPTH".to_owned(), "8".to_owned()),
        ("FALL_THROUGH".to_owned(), "0".to_owned()),
        ("dtype".to_owned(), "logic [DATA_WIDTH-1:0]".to_owned()),
    ]);
    if compile.parameters != expected_parameters {
        return Err(fail(
            "compile-contract",
            "pilot parameters differ from the reviewed fifo_v3 parameter set",
        ));
    }
    let expected_ports = [
        ("clk_i", "input", "1"),
        ("rst_ni", "input", "1"),
        ("flush_i", "input", "1"),
        ("testmode_i", "input", "1"),
        ("full_o", "output", "1"),
        ("empty_o", "output", "1"),
        ("usage_o", "output", "ADDR_DEPTH"),
        ("data_i", "input", "DATA_WIDTH"),
        ("push_i", "input", "1"),
        ("data_o", "output", "DATA_WIDTH"),
        ("pop_i", "input", "1"),
    ];
    if compile.ports.len() != expected_ports.len()
        || compile
            .ports
            .iter()
            .zip(expected_ports)
            .any(|(port, expected)| {
                (
                    port.name.as_str(),
                    port.direction.as_str(),
                    port.width.as_str(),
                ) != expected
            })
        || compile.clock_reset.clock_port != "clk_i"
        || compile.clock_reset.reset_port != "rst_ni"
        || compile.clock_reset.reset_polarity != "active-low"
        || compile.clock_reset.reset_kind != "asynchronous"
    {
        return Err(fail(
            "compile-contract",
            "pilot fifo_v3 port ordering or clock/reset mapping differs",
        ));
    }
    Ok(())
}

fn verify_compile_contract(
    manifest: &Manifest,
    locks: &[SourceLock],
    cache: &Path,
) -> Result<(), String> {
    validate_pilot_compile(&manifest.compile)?;
    let source_path = compile_path(&manifest.compile.files[0], locks, cache)?;
    let source = fs::read_to_string(&source_path).map_err(|e| {
        fail(
            "compile-contract",
            format!("read {}: {e}", source_path.display()),
        )
    })?;
    verify_pilot_source(&source)?;
    let include = compile_path(&manifest.compile.include_dirs[0], locks, cache)?;
    if !include.join("common_cells/assertions.svh").is_file() {
        return Err(fail(
            "compile-contract",
            "locked assertion include is missing",
        ));
    }
    Ok(())
}

fn verify_pilot_source(source: &str) -> Result<(), String> {
    for expected in [
        "`include \"common_cells/assertions.svh\"",
        "module fifo_v3 #(",
        "parameter bit          FALL_THROUGH = 1'b0,",
        "parameter int unsigned DATA_WIDTH   = 32,",
        "parameter int unsigned DEPTH        = 8,",
        "parameter type dtype                = logic [DATA_WIDTH-1:0],",
        "parameter int unsigned ADDR_DEPTH   = (DEPTH > 1) ? $clog2(DEPTH) : 1",
        "input  logic  clk_i",
        "input  logic  rst_ni",
        "input  logic  flush_i",
        "input  logic  testmode_i",
        "output logic  full_o",
        "output logic  empty_o",
        "output logic  [ADDR_DEPTH-1:0] usage_o",
        "input  dtype  data_i",
        "input  logic  push_i",
        "output dtype  data_o",
        "input  logic  pop_i",
    ] {
        if source.matches(expected).count() != 1 {
            return Err(fail(
                "compile-contract",
                format!("locked fifo_v3 must contain exactly one {expected:?}"),
            ));
        }
    }
    Ok(())
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|e| fail("schema", format!("serialize lock: {e}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn ensure_lock_compatible(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    if path.exists() {
        let existing = fs::read(path)
            .map_err(|e| fail("io", format!("read existing {}: {e}", path.display())))?;
        if existing == bytes {
            return Ok(true);
        }
        return Err(fail(
            "identity-drift",
            format!("refusing to rewrite immutable lock {}", path.display()),
        ));
    }
    Ok(false)
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|e| fail("io", format!("create {}: {e}", parent.display())))?;
    let temp = temp_path(parent, "lock");
    fs::write(&temp, bytes).map_err(|e| fail("io", format!("write {}: {e}", temp.display())))?;
    fs::rename(&temp, path).map_err(|e| fail("io", format!("publish {}: {e}", path.display())))
}

pub fn lock(manifest_path: &Path, lock_path: &Path, cache: &Path) -> Result<(), String> {
    let manifest_bytes = fs::read(manifest_path)
        .map_err(|e| fail("io", format!("read {}: {e}", manifest_path.display())))?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| fail("schema", format!("invalid manifest: {e}")))?;
    validate_manifest(&manifest)?;
    // Reject static cache redirects before fetch can write through the anchor.
    cache_directory(cache, "closures", true)?;
    let mut sources = Vec::new();
    for source in &manifest.sources {
        match fetch_source(source, cache, lock_path) {
            Ok(resolved) => sources.push(resolved),
            Err(error) => {
                for resolved in &sources {
                    let _ = fs::remove_dir_all(cache.join(&resolved.cache_path));
                }
                let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
                return Err(error);
            }
        }
    }
    let preflight = (|| {
        verify_dependencies(&manifest, &sources, cache)?;
        verify_compile_contract(&manifest, &sources, cache)?;
        for source in &sources {
            verify_source(source, cache, lock_path, Some(&source.cache_path))?;
        }
        Ok(())
    })();
    if let Err(error) = preflight {
        for source in &sources {
            let _ = fs::remove_dir_all(cache.join(&source.cache_path));
        }
        let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
        return Err(error);
    }
    let tools = match tool_lock() {
        Ok(tools) => tools,
        Err(error) => {
            for source in &sources {
                let _ = fs::remove_dir_all(cache.join(&source.cache_path));
            }
            let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
            return Err(error);
        }
    };
    let (closure_relative, staging_paths) = prepare_closure_metadata(&manifest, &mut sources);
    let owner = manifest.owner.clone();
    let lock = LockFile {
        schema_version: SCHEMA_VERSION,
        manifest_sha256: sha256(&manifest_bytes),
        name: manifest.name,
        owner: owner.clone(),
        support_level: "locked".into(),
        sources,
        dependencies: manifest.dependencies,
        compile: manifest.compile,
        maintenance: manifest.maintenance,
        binding: BindingLock {
            kind: "source-only; no Bitloom wrapper in Story 130.2".into(),
            version: "none".into(),
            owner,
        },
        generator: GeneratorLock {
            kind: "none; upstream tracked SystemVerilog is consumed directly".into(),
            version: "none".into(),
            inputs: Vec::new(),
        },
        evidence_paths: vec![
            "_agile-output/test-artifacts/130-2-online-fetch.json".into(),
            "_agile-output/test-artifacts/130-2-offline-replay.json".into(),
        ],
        last_passed_utc_location: "evidence JSON startedUtc; excluded from deterministic lock"
            .into(),
        tools,
    };
    let bytes = json_bytes(&lock)?;
    let already_locked = match ensure_lock_compatible(lock_path, &bytes) {
        Ok(value) => value,
        Err(error) => {
            for staging in &staging_paths {
                let _ = fs::remove_dir_all(cache.join(staging));
            }
            let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
            return Err(error);
        }
    };
    finalize_lock(
        &lock,
        &staging_paths,
        cache,
        &closure_relative,
        lock_path,
        &bytes,
        already_locked,
    )?;
    println!(
        "locked={} sources={}",
        lock_path.display(),
        lock.sources.len()
    );
    Ok(())
}

fn finalize_lock(
    lock: &LockFile,
    staging_paths: &[String],
    cache: &Path,
    closure_relative: &str,
    lock_path: &Path,
    bytes: &[u8],
    already_locked: bool,
) -> Result<(), String> {
    let created = match publish_closure_cache(&lock.sources, staging_paths, cache, closure_relative)
    {
        Ok(created) => created,
        Err(error) => {
            for staging in staging_paths {
                let _ = fs::remove_dir_all(cache.join(staging));
            }
            let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
            return Err(error);
        }
    };
    let result = (|| {
        publish_license_archives(&lock.sources, cache, lock_path)?;
        for source in &lock.sources {
            verify_source(source, cache, lock_path, None)?;
        }
        if !already_locked {
            write_bytes_atomic(lock_path, bytes)?;
        }
        Ok(())
    })();
    if result.is_err() {
        // Lock-file existence says nothing about ownership of a shared cache.
        if created {
            let _ = fs::remove_dir_all(cache.join(closure_relative));
        }
        let _ = fs::remove_dir_all(cache.join(".staging-licenses"));
    }
    result
}

// Validate every directory below the caller-selected cache anchor, before any
// traversal or publication. This rejects static symlink escapes; it does not
// provide a capability-based filesystem transaction against concurrent mutation.
fn cache_directory(cache: &Path, relative: &str, allow_missing: bool) -> Result<PathBuf, String> {
    validate_relative(relative, "cache")?;
    let mut path = cache.to_path_buf();
    for component in std::iter::once(None).chain(Path::new(relative).components().map(Some)) {
        if let Some(component) = component {
            path.push(component.as_os_str());
        }
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
            Ok(_) => {
                return Err(fail(
                    "path",
                    format!(
                        "cache path must contain only real directories: {}",
                        path.display()
                    ),
                ));
            }
            Err(error) if allow_missing && error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(fail("cache", format!("stat {}: {error}", path.display()))),
        }
    }
    Ok(path)
}

fn collect_files(root: &Path, current: &Path, out: &mut Vec<String>) -> Result<(), String> {
    for entry in fs::read_dir(current)
        .map_err(|e| fail("cache", format!("read {}: {e}", current.display())))?
    {
        let entry = entry.map_err(|e| fail("cache", format!("read entry: {e}")))?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|e| fail("cache", format!("stat {}: {e}", entry.path().display())))?;
        if metadata.file_type().is_symlink() {
            return Err(fail(
                "path",
                format!("cache symlink is forbidden: {}", entry.path().display()),
            ));
        }
        #[cfg(unix)]
        if metadata.is_file() && metadata.nlink() != 1 {
            return Err(fail(
                "path",
                format!("cache hardlink is forbidden: {}", entry.path().display()),
            ));
        }
        if metadata.is_dir() {
            collect_files(root, &entry.path(), out)?;
        } else if metadata.is_file() {
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|e| fail("path", format!("cache escape: {e}")))?
                .to_string_lossy()
                .replace('\\', "/");
            validate_relative(&relative, "cache")?;
            out.push(relative);
        } else {
            return Err(fail("path", "cache contains a non-regular entry"));
        }
    }
    Ok(())
}

fn verify_source(
    source: &SourceLock,
    cache: &Path,
    lock_path: &Path,
    staging_path: Option<&str>,
) -> Result<(), String> {
    validate_relative(&source.cache_path, "cache")?;
    let hex40 =
        |value: &str| value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
    let expected_cache = format!(
        "closures/v{SCHEMA_VERSION}-{}/{}-{}-{}",
        source.closure_digest, source.name, source.commit, source.tree_digest
    );
    if !hex40(&source.commit)
        || !hex40(&source.tag_object)
        || (staging_path.is_none() && source.cache_path != expected_cache)
        || (source.tag_type == "lightweight" && source.tag_object != source.commit)
        || (source.tag_type == "annotated" && source.tag_object == source.commit)
        || !matches!(source.tag_type.as_str(), "lightweight" | "annotated")
    {
        return Err(fail(
            "identity-drift",
            format!("{} tag/commit/cache identity is inconsistent", source.name),
        ));
    }
    let root = cache_directory(cache, staging_path.unwrap_or(&source.cache_path), false)?;
    let expected: BTreeMap<_, _> = source
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.sha256.as_str()))
        .collect();
    if expected.len() != source.files.len() {
        return Err(fail(
            "lock",
            format!("{} has duplicate file entries", source.name),
        ));
    }
    let mut actual = Vec::new();
    collect_files(&root, &root, &mut actual)?;
    actual.sort();
    let ordered: Vec<_> = source.files.iter().map(|file| file.path.clone()).collect();
    let mut sorted = ordered.clone();
    sorted.sort();
    if ordered != sorted {
        return Err(fail(
            "order",
            format!("{} lock filelist is not ordered", source.name),
        ));
    }
    if actual != sorted {
        return Err(fail(
            "undeclared-file",
            format!("{} cache file set differs from lock", source.name),
        ));
    }
    for (path, expected_hash) in expected {
        validate_relative(path, "locked")?;
        let actual_hash = sha256_file(&root.join(path))?;
        if actual_hash != expected_hash {
            return Err(fail(
                "content-drift",
                format!(
                    "{}:{path} hash {actual_hash} != {expected_hash}",
                    source.name
                ),
            ));
        }
    }
    if tree_digest(&source.files) != source.tree_digest {
        return Err(fail(
            "content-drift",
            format!("{} tree digest drift", source.name),
        ));
    }
    validate_relative(&source.license.source_path, "license source")?;
    if source.license.source_path != "LICENSE" {
        return Err(fail(
            "license-drift",
            format!("{} license source must be LICENSE", source.name),
        ));
    }
    let (archive, relative) = archive_path(lock_path, &source.name)?;
    let archive_to_check = if staging_path.is_some() {
        cache
            .join(".staging-licenses")
            .join(format!("{}-LICENSE", source.name))
    } else {
        archive.clone()
    };
    let archive_meta = fs::symlink_metadata(&archive_to_check)
        .map_err(|e| fail("license-drift", format!("stat {}: {e}", archive.display())))?;
    #[cfg(unix)]
    if archive_meta.nlink() != 1 {
        return Err(fail(
            "license-drift",
            format!(
                "license archive is a hardlink: {}",
                archive_to_check.display()
            ),
        ));
    }
    if archive_meta.file_type().is_symlink()
        || !archive_meta.is_file()
        || relative != source.license.archive_path
        || sha256_file(&archive_to_check)? != source.license.sha256
        || sha256_file(&root.join(&source.license.source_path))? != source.license.sha256
    {
        return Err(fail(
            "license-drift",
            format!("{} LICENSE drift", source.name),
        ));
    }
    let actual_notices: Vec<_> = source
        .files
        .iter()
        .filter(|file| {
            Path::new(&file.path)
                .file_name()
                .and_then(OsStr::to_str)
                .is_some_and(|name| name.to_ascii_uppercase().starts_with("NOTICE"))
        })
        .map(|file| file.path.clone())
        .collect();
    if actual_notices != source.license.notice_paths
        || source.license.copyright_attribution_facts
            != "No repository-specific copyright line appears in the root LICENSE; preserve all source and license notices under SHL-0.51 clauses 3 and 4."
        || source.license.redistribution
            != "Solderpad Hardware License v0.51; retain license and notices"
    {
        return Err(fail(
            "license-drift",
            format!("{} NOTICE or redistribution facts drift", source.name),
        ));
    }
    Ok(())
}

fn load_verified(
    manifest_path: &Path,
    lock_path: &Path,
    cache: &Path,
) -> Result<(Manifest, LockFile), String> {
    let manifest_bytes =
        fs::read(manifest_path).map_err(|e| fail("io", format!("read manifest: {e}")))?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| fail("schema", format!("invalid manifest: {e}")))?;
    validate_manifest(&manifest)?;
    let lock: LockFile = read_json(lock_path, "lock")?;
    if lock.schema_version != SCHEMA_VERSION
        || lock.manifest_sha256 != sha256(&manifest_bytes)
        || lock.name != manifest.name
        || lock.owner != manifest.owner
        || lock.support_level != "locked"
        || lock.binding.kind != "source-only; no Bitloom wrapper in Story 130.2"
        || lock.binding.version != "none"
        || lock.binding.owner != manifest.owner
        || lock.generator.kind != "none; upstream tracked SystemVerilog is consumed directly"
        || lock.generator.version != "none"
        || !lock.generator.inputs.is_empty()
        || lock.evidence_paths
            != [
                "_agile-output/test-artifacts/130-2-online-fetch.json",
                "_agile-output/test-artifacts/130-2-offline-replay.json",
            ]
        || lock.last_passed_utc_location
            != "evidence JSON startedUtc; excluded from deterministic lock"
    {
        return Err(fail("identity-drift", "manifest and lock identity differ"));
    }
    if serde_json::to_value(&lock.dependencies).ok()
        != serde_json::to_value(&manifest.dependencies).ok()
        || serde_json::to_value(&lock.compile).ok() != serde_json::to_value(&manifest.compile).ok()
        || serde_json::to_value(&lock.maintenance).ok()
            != serde_json::to_value(&manifest.maintenance).ok()
    {
        return Err(fail("identity-drift", "dependency or compile intent drift"));
    }
    let tools = tool_lock()?;
    if serde_json::to_value(&tools).ok() != serde_json::to_value(&lock.tools).ok() {
        return Err(fail(
            "tool-drift",
            "Git, Yosys, simulator or adapter identity differs from lock",
        ));
    }
    if lock.sources.len() != manifest.sources.len() {
        return Err(fail("identity-drift", "source count differs"));
    }
    let expected_closure = closure_digest(&manifest, &lock.sources);
    if lock
        .sources
        .iter()
        .any(|source| source.closure_digest != expected_closure)
    {
        return Err(fail(
            "identity-drift",
            "schema/URL/dependency/source closure digest differs",
        ));
    }
    for (intent, source) in manifest.sources.iter().zip(&lock.sources) {
        if intent.name != source.name
            || intent.source_type != source.source_type
            || intent.url != source.url
            || intent.r#ref != source.r#ref
        {
            return Err(fail(
                "identity-drift",
                format!("{} source intent differs", intent.name),
            ));
        }
        verify_source(source, cache, lock_path, None)?;
    }
    verify_dependencies(&manifest, &lock.sources, cache)?;
    verify_compile_contract(&manifest, &lock.sources, cache)?;
    Ok((manifest, lock))
}

pub fn verify(
    manifest_path: &Path,
    lock_path: &Path,
    cache: &Path,
    offline: bool,
) -> Result<(), String> {
    if !offline {
        return Err(fail(
            "scope",
            "read-only verification requires explicit --offline",
        ));
    }
    let (_, lock) = load_verified(manifest_path, lock_path, cache)?;
    println!(
        "verified={} sources={}",
        lock_path.display(),
        lock.sources.len()
    );
    Ok(())
}

const FIFO_WRAPPER: &str = "BitloomExternalFifo";

fn pilot_port_type(port: &PortIntent) -> Result<bitloom_hir::GroundType, String> {
    use bitloom_hir::GroundType;
    match (port.name.as_str(), port.width.as_str()) {
        ("clk_i", "1") => Ok(GroundType::Clock),
        ("rst_ni", "1") => Ok(GroundType::Reset),
        (_, "1") => Ok(GroundType::UInt { width: 1 }),
        (_, "DATA_WIDTH") => Ok(GroundType::UInt { width: 32 }),
        (_, "ADDR_DEPTH") => Ok(GroundType::UInt { width: 3 }),
        _ => Err(fail("binding", "unsupported pilot port width")),
    }
}

fn fifo_wrapper_hir(compile: &CompileIntent) -> Result<bitloom_hir::FrozenHir, String> {
    use bitloom_builder::{ElaborateSession, Span};
    validate_pilot_compile(compile)?;
    let span = Span::default();
    let mut session = ElaborateSession::new(FIFO_WRAPPER);
    // Existing port-only external declaration, with no simulated zero body.
    for name in [compile.top.as_str(), FIFO_WRAPPER] {
        session.begin_module(name, span);
        for port in &compile.ports {
            let ty = pilot_port_type(port)?;
            if port.direction == "input" {
                session.add_input(&port.name, ty, span);
            } else {
                session.add_output(&port.name, ty, span);
            }
        }
        if name == FIFO_WRAPPER {
            session.add_instance(
                "fifo",
                &compile.top,
                compile
                    .ports
                    .iter()
                    .map(|p| (p.name.clone(), p.name.clone()))
                    .collect(),
                vec![
                    ("FALL_THROUGH".into(), 0),
                    ("DATA_WIDTH".into(), 32),
                    ("DEPTH".into(), 8),
                ],
                span,
            );
        }
        session.end_module();
    }
    session
        .finish()
        .map_err(|e| fail("binding", format!("freeze wrapper: {e}")))
}

// CLI link boundary only: keep the backend's parent byte-for-byte. The
// backend presently emits external declarations; remove precisely the one
// verified port-only declaration that the locked RTL will supply instead.
fn link_fifo_wrapper(
    hir: &bitloom_hir::FrozenHir,
    emitted: &str,
    compile: &CompileIntent,
) -> Result<String, String> {
    use bitloom_hir::{PortDirection, Stmt};
    validate_pilot_compile(compile)?;
    let modules = &hir.circuit().modules;
    let opaque = modules
        .iter()
        .find(|m| m.name == compile.top)
        .ok_or_else(|| fail("binding", "external declaration missing"))?;
    let parent = modules
        .iter()
        .find(|m| m.name == FIFO_WRAPPER)
        .ok_or_else(|| fail("binding", "parent missing"))?;
    if modules.len() != 2
        || !opaque.body.is_empty()
        || opaque.ports.len() != compile.ports.len()
        || parent.ports != opaque.ports
        || parent.body.len() != 1
    {
        return Err(fail(
            "binding",
            "wrapper or opaque declaration shape/body differs",
        ));
    }
    for (actual, expected) in opaque.ports.iter().zip(&compile.ports) {
        let direction = if expected.direction == "input" {
            PortDirection::Input
        } else {
            PortDirection::Output
        };
        if actual.name != expected.name
            || actual.direction != direction
            || actual.ty != pilot_port_type(expected)?
        {
            return Err(fail("binding", "opaque declaration port shape differs"));
        }
    }
    let Stmt::Instance(instance) = &parent.body[0] else {
        return Err(fail("binding", "parent is not a composed instance"));
    };
    let expected_params = vec![
        ("FALL_THROUGH".into(), 0),
        ("DATA_WIDTH".into(), 32),
        ("DEPTH".into(), 8),
    ];
    if instance.name != "fifo"
        || instance.module != compile.top
        || instance.params != expected_params
        || instance.connects.len() != compile.ports.len()
        || instance
            .connects
            .iter()
            .zip(&compile.ports)
            .any(|(c, p)| c.dangling || c.child_port != p.name || c.parent_net != p.name)
    {
        return Err(fail("binding", "parent instance mapping differs"));
    }
    let regenerated = bitloom_vlog::emit(hir);
    if regenerated.files.len() != 1 || regenerated.files[0].contents != emitted {
        return Err(fail(
            "binding",
            "emitted wrapper changed after backend generation",
        ));
    }
    let mut declaration = format!("module {} (\n", compile.top);
    let ports = compile
        .ports
        .iter()
        .map(|p| {
            let width = match p.width.as_str() {
                "DATA_WIDTH" => "[31:0] ",
                "ADDR_DEPTH" => "[2:0] ",
                _ => "",
            };
            format!("  {} {width}{}", p.direction, p.name)
        })
        .collect::<Vec<_>>()
        .join(",\n");
    declaration.push_str(&ports);
    declaration.push_str("\n);\nendmodule\n");
    if emitted.matches(&declaration).count() != 1 {
        return Err(fail(
            "binding",
            "emitted external declaration differs from exact port-only shape",
        ));
    }
    let linked = emitted.replacen(&declaration, "", 1);
    if linked.matches("endmodule").count() != 1
        || !linked.contains(&format!("module {FIFO_WRAPPER} ("))
        || !linked.contains("  fifo_v3 fifo (\n")
    {
        return Err(fail(
            "binding",
            "composed parent missing after external link",
        ));
    }
    Ok(linked)
}

fn emit_fifo_wrapper(compile: &CompileIntent) -> Result<String, String> {
    let hir = fifo_wrapper_hir(compile)?;
    let artifact = bitloom_vlog::emit(&hir);
    if artifact.files.len() != 1 {
        return Err(fail("binding", "unexpected backend artifact count"));
    }
    link_fifo_wrapper(&hir, &artifact.files[0].contents, compile)
}

fn guard_binding_output(
    output: &Path,
    manifest: &Path,
    lock: &Path,
    cache: &Path,
) -> Result<(), String> {
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let destination = fs::canonicalize(parent)
        .map_err(|e| fail("path", format!("binding output directory: {e}")))?
        .join(
            output
                .file_name()
                .ok_or_else(|| fail("path", "binding output has no file name"))?,
        );
    let cache = fs::canonicalize(cache).map_err(|e| fail("path", format!("cache: {e}")))?;
    if destination.starts_with(cache)
        || [manifest, lock]
            .iter()
            .any(|p| fs::canonicalize(p).ok().as_ref() == Some(&destination))
    {
        return Err(fail(
            "path",
            "binding output must not overwrite source inputs or cache",
        ));
    }
    if let Ok(metadata) = fs::symlink_metadata(&destination) {
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(fail("path", "binding output must be a regular file"));
        }
    }
    Ok(())
}

/// Produce an auditable wrapper binding from the verified manifest and lock.
/// This intentionally records the upstream identity separately from the
/// Bitloom adapter version and carries the ordered shape verbatim from lock.
pub fn binding(
    manifest_path: &Path,
    lock_path: &Path,
    cache: &Path,
    output: &Path,
) -> Result<(), String> {
    let (manifest, lock) = load_verified(manifest_path, lock_path, cache)?;
    verify_compile_contract(&manifest, &lock.sources, cache)?;
    let source_path = compile_path(&manifest.compile.files[0], &lock.sources, cache)?;
    guard_binding_output(output, manifest_path, lock_path, cache)?;
    let upstream = lock
        .sources
        .iter()
        .find(|source| source.name == "common_cells")
        .ok_or_else(|| fail("binding", "common_cells source identity missing"))?;
    let wrapper = emit_fifo_wrapper(&manifest.compile)?;
    let descriptor = serde_json::json!({
        "schemaVersion": 1,
        "upstream": {
            "name": manifest.name,
            "url": upstream.url,
            "ref": upstream.r#ref,
            "commit": upstream.commit,
            "sourcePath": manifest.compile.files[0],
            "sourceSha256": sha256_file(&source_path)?,
        },
        "wrapper": { "name": FIFO_WRAPPER, "version": 1, "verilog": wrapper,
            "sha256": sha256(wrapper.as_bytes()),
            "parameterBinding": "fixed upstream defaults; no backend parameter overrides" },
        "adapter": { "name": ADAPTER_NAME, "version": ADAPTER_VERSION },
        "module": manifest.compile.top,
        "parameters": manifest.compile.parameters,
        "ports": manifest.compile.ports.iter().map(|p| serde_json::json!({
            "name": p.name, "direction": p.direction, "width": p.width
        })).collect::<Vec<_>>(),
        "clockReset": manifest.compile.clock_reset,
        "supportLevel": "locked"
    });
    let bytes = serde_json::to_vec_pretty(&descriptor)
        .map_err(|e| fail("schema", format!("serialize binding: {e}")))?;
    let mut bytes = bytes;
    bytes.push(b'\n');
    write_bytes_atomic(output, &bytes)?;
    println!(
        "binding={} module={} source_sha256={}",
        output.display(),
        manifest.compile.top,
        sha256_file(&source_path)?
    );
    Ok(())
}

/// Simulate the actual locked RTL under Icarus and compare every sampled
/// boundary with a separate software queue embedded in the testbench.
pub fn behavior(manifest_path: &Path, lock_path: &Path, cache: &Path) -> Result<(), String> {
    let (manifest, lock) = load_verified(manifest_path, lock_path, cache)?;
    verify_compile_contract(&manifest, &lock.sources, cache)?;
    let source_path = compile_path(&manifest.compile.files[0], &lock.sources, cache)?;
    let source = fs::read_to_string(&source_path)
        .map_err(|e| fail("behavior", format!("read locked RTL: {e}")))?;
    let adapted = adapt_fifo_for_yosys(&source)?;
    let wrapper = emit_fifo_wrapper(&manifest.compile)?;
    let include = compile_path(&manifest.compile.include_dirs[0], &lock.sources, cache)?;
    let stdout = simulate_fifo(
        &adapted,
        &wrapper,
        &include,
        &lock.tools.iverilog_path,
        &lock.tools.iverilog_base,
        &lock.tools.vvp_path,
    )?;
    println!(
        "behavior=passed rtl_sha256={} wrapper_sha256={} simulator=iverilog",
        sha256_file(&source_path)?,
        sha256(wrapper.as_bytes())
    );
    print!("{stdout}");
    Ok(())
}

fn simulate_fifo(
    adapted: &str,
    wrapper: &str,
    include: &Path,
    iverilog: &str,
    iverilog_base: &str,
    vvp: &str,
) -> Result<String, String> {
    let work = temp_path(&std::env::temp_dir(), "behavior");
    fs::create_dir_all(&work).map_err(|e| fail("behavior", format!("create work dir: {e}")))?;
    let result = (|| {
        let rtl = work.join("fifo_v3.sv");
        let parent = work.join("BitloomExternalFifo.v");
        let tb = work.join("fifo_v3_tb.sv");
        let exe = work.join("fifo_v3.vvp");
        for (path, contents) in [(&rtl, adapted), (&parent, wrapper), (&tb, FIFO_BEHAVIOR_TB)] {
            fs::write(path, contents)
                .map_err(|e| fail("behavior", format!("write {}: {e}", path.display())))?;
        }
        let result = bounded_output(
            Command::new(iverilog)
                .arg("-B")
                .arg(iverilog_base)
                .env("TMPDIR", &work)
                .args([
                    "-g2012",
                    "-DSYNTHESIS",
                    "-DCOMMON_CELLS_ASSERTS_OFF",
                    "-s",
                    "fifo_v3_tb",
                    "-o",
                ])
                .arg(&exe)
                .arg("-I")
                .arg(include)
                .arg(&rtl)
                .arg(&parent)
                .arg(&tb),
            "iverilog behavior",
        )?;
        if !result.status.success() {
            return Err(fail(
                "behavior",
                format!(
                    "iverilog failed: {}",
                    String::from_utf8_lossy(&result.stderr)
                ),
            ));
        }
        let result = bounded_output(Command::new(vvp).arg(&exe), "vvp behavior")?;
        let stdout = String::from_utf8_lossy(&result.stdout);
        if !result.status.success() || !stdout.contains("FIFO_MODEL_PASS") {
            return Err(fail(
                "behavior",
                format!(
                    "simulation failed: {stdout}{}",
                    String::from_utf8_lossy(&result.stderr)
                ),
            ));
        }
        Ok(stdout.into_owned())
    })();
    let _ = fs::remove_dir_all(&work);
    result
}

const FIFO_BEHAVIOR_TB: &str = r#"
module fifo_v3_tb;
  reg clk_i=0, rst_ni=1, flush_i=0, testmode_i=0, push_i=0, pop_i=0;
  reg [31:0] data_i=0;
  wire full_o, empty_o;
  wire [2:0] usage_o;
  wire [31:0] data_o;
  reg [31:0] model [0:7];
  integer count=0, i, accepted_push, accepted_pop, cycles=0;
  BitloomExternalFifo dut (.*);
  always #5 clk_i=~clk_i;
  task automatic step(input reg p, input reg q, input reg [31:0] d);
    begin
      cycles=cycles+1;
      @(negedge clk_i); push_i=p; pop_i=q; data_i=d; #1;
      if (full_o !== (count==8) || empty_o !== (count==0) || usage_o !== count[2:0]) $fatal(1,"pre-edge status mismatch count=%0d",count);
      if (count>0 && data_o !== model[0]) $fatal(1,"head mismatch expected=%h got=%h",model[0],data_o);
      accepted_push=p && count<8; accepted_pop=q && count>0;
      @(posedge clk_i);
      if (accepted_pop) begin for (integer j=0;j<7;j=j+1) model[j]=model[j+1]; count=count-1; end
      if (accepted_push) begin model[count]=d; count=count+1; end
      #1;
      if (full_o !== (count==8) || empty_o !== (count==0) || usage_o !== count[2:0]) $fatal(1,"post-edge status mismatch count=%0d",count);
      if (count>0 && data_o !== model[0]) $fatal(1,"post-edge head mismatch");
    end
  endtask
  initial begin
    #1 rst_ni=0; #1;
    if (empty_o !== 1'b1 || full_o !== 1'b0 || usage_o !== 0) $fatal(1,"async reset mismatch");
    @(negedge clk_i); rst_ni=1;
    step(0,0,0);
    for(i=0;i<8;i=i+1) step(1,0,32'h1000+i);
    step(1,0,32'hffff); // full rejects a push
    step(1,1,32'h2000); // full simultaneous push/pop accepts only pop
    step(0,1,0);
    step(1,0,32'h3000);
    step(1,1,32'h4000);
    step(0,1,0); step(0,1,0); step(0,1,0); step(0,1,0); step(0,1,0); step(0,1,0); step(0,1,0);
    step(0,0,0);
    for(i=0;i<64;i=i+1) begin
      step((i*17+3)%5<3, (i*11+1)%4<2, 32'h5a000000+i);
    end
    // Fill with nonzero data, then assert reset between active edges.
    while(count>0) step(0,1,0);
    step(1,0,32'hfeed1234); step(1,0,32'hfeed5678);
    @(negedge clk_i); push_i=0; pop_i=0; #2 rst_ni=0; count=0; #1;
    if (empty_o !== 1'b1 || full_o !== 1'b0 || usage_o !== 0) $fatal(1,"populated asynchronous reset mismatch");
    @(negedge clk_i); rst_ni=1;
    step(0,0,0); step(1,0,32'hdeadcafe); step(0,1,0); step(0,1,0);
    // Flush is an exposed input and must clear a populated queue.
    step(1,0,32'habcdef01); step(1,0,32'habcdef02);
    @(negedge clk_i); push_i=0; pop_i=0; flush_i=1;
    @(posedge clk_i); count=0; #1;
    if (empty_o !== 1'b1 || full_o !== 1'b0 || usage_o !== 0) $fatal(1,"flush mismatch");
    @(negedge clk_i); flush_i=0;
    step(1,0,32'hffffeeee); step(0,1,0);
    $display("FIFO_MODEL_PASS cycles=%0d depth=8 width=32 populated_reset=1 flush=1 wrapper=BitloomExternalFifo",cycles); $finish;
  end
endmodule
"#;

fn compile_path<'a>(
    value: &'a str,
    locks: &'a [SourceLock],
    cache: &Path,
) -> Result<PathBuf, String> {
    let (source_name, relative) = value
        .split_once(':')
        .ok_or_else(|| fail("schema", format!("invalid compile path {value}")))?;
    validate_relative(relative, "compile")?;
    let source = locks
        .iter()
        .find(|source| source.name == source_name)
        .ok_or_else(|| fail("schema", format!("unknown compile source {source_name}")))?;
    Ok(cache.join(&source.cache_path).join(relative))
}

fn yosys_path(path: &Path) -> Result<String, String> {
    let value = path
        .to_str()
        .ok_or_else(|| fail("compile", "Yosys path is not UTF-8"))?;
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
    {
        return Err(fail(
            "compile",
            format!("unsafe character in Yosys path {value:?}"),
        ));
    }
    Ok(value.to_owned())
}

fn adapt_fifo_for_yosys(source: &str) -> Result<String, String> {
    let parameter = "parameter type dtype                = logic [DATA_WIDTH-1:0],";
    if source.matches(parameter).count() != 1 {
        return Err(fail(
            "adapter",
            "expected fifo_v3 dtype parameter exactly once",
        ));
    }
    let mut adapted = source.replace(parameter, "// dtype fixed by bitloom-yosys-sv-compat v1");
    let replacements = [
        (
            "input  dtype  data_i,",
            "input  logic [DATA_WIDTH-1:0] data_i,",
        ),
        (
            "output dtype  data_o,",
            "output logic [DATA_WIDTH-1:0] data_o,",
        ),
        (
            "dtype [FifoDepth - 1:0] mem_n, mem_q;",
            "logic [DATA_WIDTH-1:0] mem_n [FifoDepth - 1:0], mem_q [FifoDepth - 1:0];",
        ),
        (
            "mem_n           = mem_q;",
            "for (int unsigned i = 0; i < FifoDepth; i++) mem_n[i] = mem_q[i];",
        ),
        (
            "mem_q <= {FifoDepth{dtype'('0)}};",
            "for (int unsigned i = 0; i < FifoDepth; i++) mem_q[i] <= '0;",
        ),
        (
            "mem_q <= mem_n;",
            "for (int unsigned i = 0; i < FifoDepth; i++) mem_q[i] <= mem_n[i];",
        ),
    ];
    for (from, to) in replacements {
        if adapted.matches(from).count() != 1 {
            return Err(fail(
                "adapter",
                format!("expected source fragment {from:?}"),
            ));
        }
        adapted = adapted.replace(from, to);
    }
    Ok(adapted)
}

pub fn replay(
    manifest_path: &Path,
    lock_path: &Path,
    cache: &Path,
    compile: bool,
) -> Result<(), String> {
    let (manifest, lock) = load_verified(manifest_path, lock_path, cache)?;
    if !compile {
        return Err(fail("scope", "replay requires --compile"));
    }
    if std::env::var("BITLOOM_NETWORK_ISOLATED").as_deref() != Ok("1") {
        return Err(fail(
            "network",
            "replay requires BITLOOM_NETWORK_ISOLATED=1 from the isolation runner",
        ));
    }
    let host_netns = std::env::var("BITLOOM_HOST_NETNS").map_err(|_| {
        fail(
            "network",
            "isolation runner did not record the host network namespace",
        )
    })?;
    let current_netns = fs::read_link("/proc/self/ns/net")
        .map_err(|e| fail("network", format!("read current network namespace: {e}")))?
        .to_string_lossy()
        .into_owned();
    if host_netns == current_netns {
        return Err(fail(
            "network",
            "replay is still in the host network namespace",
        ));
    }
    if manifest.compile.files.len() != 1 {
        return Err(fail(
            "compile",
            "Story 130.2 expects one FIFO compilation unit",
        ));
    }
    let source_path = compile_path(&manifest.compile.files[0], &lock.sources, cache)?;
    let source = fs::read_to_string(&source_path)
        .map_err(|e| fail("compile", format!("read {}: {e}", source_path.display())))?;
    let adapted = adapt_fifo_for_yosys(&source)?;
    let work = temp_path(&std::env::temp_dir(), "compile");
    fs::create_dir_all(&work)
        .map_err(|e| fail("compile", format!("create work directory: {e}")))?;
    let adapted_path = work.join("fifo_v3.yosys.sv");
    fs::write(&adapted_path, adapted.as_bytes())
        .map_err(|e| fail("compile", format!("write adapted source: {e}")))?;
    let includes: Result<Vec<_>, _> = manifest
        .compile
        .include_dirs
        .iter()
        .map(|value| compile_path(value, &lock.sources, cache))
        .collect();
    let mut script = String::from("read_verilog -sv");
    for define in &manifest.compile.defines {
        if !define
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(fail("compile", format!("unsafe define {define:?}")));
        }
        script.push_str(&format!(" -D{define}"));
    }
    for include in includes? {
        script.push_str(&format!(" -I{}", yosys_path(&include)?));
    }
    script.push_str(&format!(
        " {}; hierarchy -check -top {}; proc; check -assert",
        yosys_path(&adapted_path)?,
        manifest.compile.top
    ));
    let result = command_output(
        Command::new("yosys").args(["-q", "-p", &script]),
        "yosys compile",
    );
    let _ = fs::remove_dir_all(&work);
    result?;
    println!("network=denied");
    println!("network_namespace={host_netns}->{current_netns}");
    println!("hdl_compile=passed");
    println!("adapter={ADAPTER_NAME}@{ADAPTER_VERSION}");
    println!("adapter_input_sha256={}", sha256(source.as_bytes()));
    println!("adapter_output_sha256={}", sha256(adapted.as_bytes()));
    Ok(())
}

//! Ensure pinned firtool-1.155.0 is available (AD-9 / NFR3).
//! Never trust PATH by default. Override with `RHDL_FIRTOOL_PATH` (directory containing `firtool`).

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

pub const FIRTOOL_VERSION: &str = "1.155.0";

/// Host triple → CIRCT asset basename (NFR11).
pub fn firtool_asset_for_host() -> Result<&'static str, FirtoolError> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok("firrtl-bin-linux-x64.tar.gz"),
        ("linux", "aarch64") => Ok("firrtl-bin-linux-arm64.tar.gz"),
        ("macos", "x86_64") => Ok("firrtl-bin-macos-x64.tar.gz"),
        ("macos", "aarch64") => Ok("firrtl-bin-macos-arm64.tar.gz"),
        ("windows", "x86_64") => Ok("firrtl-bin-windows-x64.tar.gz"),
        (os, arch) => Err(FirtoolError::Message(format!(
            "no firtool asset for {os}-{arch}; set RHDL_FIRTOOL_PATH (NFR11)"
        ))),
    }
}

pub fn firtool_sha_asset_for_host() -> Result<String, FirtoolError> {
    Ok(format!("{}.sha256", firtool_asset_for_host()?))
}

const GITHUB_RELEASE_BASE: &str = "https://github.com/llvm/circt/releases/download/firtool-1.155.0";

#[derive(Debug)]
pub enum FirtoolError {
    Message(String),
}

impl std::fmt::Display for FirtoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirtoolError::Message(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for FirtoolError {}

fn cache_root() -> PathBuf {
    if let Ok(p) = std::env::var("RHDL_FIRTOOL_CACHE") {
        return PathBuf::from(p);
    }
    if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
        return PathBuf::from(xdg).join("rhdl").join("firtool");
    }
    std::env::var_os("HOME")
        .map(|h| PathBuf::from(h).join(".cache").join("rhdl").join("firtool"))
        .unwrap_or_else(|| PathBuf::from(".rhdl-firtool-cache"))
}

pub fn resolve_override_dir() -> Option<PathBuf> {
    std::env::var_os("RHDL_FIRTOOL_PATH").map(PathBuf::from)
}

pub fn firtool_bin_in(dir: &Path) -> PathBuf {
    dir.join("firtool")
}

/// Ensure firtool is present; return path to the `firtool` binary.
pub fn ensure_firtool() -> Result<PathBuf, FirtoolError> {
    if let Some(dir) = resolve_override_dir() {
        let bin = firtool_bin_in(&dir);
        if bin.is_file() {
            return Ok(bin);
        }
        return Err(FirtoolError::Message(format!(
            "RHDL_FIRTOOL_PATH={dir:?} does not contain a `firtool` binary"
        )));
    }

    let ver_dir = cache_root().join(FIRTOOL_VERSION);
    let bin = ver_dir.join("bin").join("firtool");
    if bin.is_file() {
        return Ok(bin);
    }

    if !cfg!(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64"),
    )) {
        return Err(FirtoolError::Message(format!(
            "automatic firtool download is not implemented for this host; \
             set RHDL_FIRTOOL_PATH to a directory containing firtool {FIRTOOL_VERSION}"
        )));
    }

    download_and_extract(&ver_dir)?;
    if !bin.is_file() {
        return Err(FirtoolError::Message(format!(
            "after extract, missing {bin:?}"
        )));
    }
    Ok(bin)
}

fn download_and_extract(ver_dir: &Path) -> Result<(), FirtoolError> {
    fs::create_dir_all(ver_dir).map_err(|e| FirtoolError::Message(e.to_string()))?;
    let asset = firtool_asset_for_host()?;
    let sha_asset = firtool_sha_asset_for_host()?;
    let tarball = ver_dir.join(asset);
    let sha_file = ver_dir.join(&sha_asset);

    let tar_url = format!("{GITHUB_RELEASE_BASE}/{asset}");
    let sha_url = format!("{GITHUB_RELEASE_BASE}/{sha_asset}");

    eprintln!("rhdl: downloading {tar_url}");
    download_url(&tar_url, &tarball)?;
    eprintln!("rhdl: downloading {sha_url}");
    download_url(&sha_url, &sha_file)?;

    let expected = parse_sha256_file(
        &fs::read_to_string(&sha_file)
            .map_err(|e| FirtoolError::Message(format!("read sha256 file: {e}")))?,
    )?;
    let actual = sha256_file(&tarball)?;
    if actual != expected {
        let _ = fs::remove_file(&tarball);
        return Err(FirtoolError::Message(format!(
            "sha256 mismatch for {asset}: expected {expected}, got {actual}"
        )));
    }

    let status = Command::new("tar")
        .args(["-xzf"])
        .arg(&tarball)
        .arg("-C")
        .arg(ver_dir)
        .status()
        .map_err(|e| FirtoolError::Message(format!("spawn tar: {e}")))?;
    if !status.success() {
        return Err(FirtoolError::Message("tar extract failed".into()));
    }

    let dest = ver_dir.join("bin").join("firtool");
    if dest.is_file() {
        return Ok(());
    }
    let found = find_named(ver_dir, "firtool")?;
    fs::create_dir_all(dest.parent().unwrap()).map_err(|e| FirtoolError::Message(e.to_string()))?;
    fs::copy(&found, &dest).map_err(|e| FirtoolError::Message(e.to_string()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest)
            .map_err(|e| FirtoolError::Message(e.to_string()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest, perms).map_err(|e| FirtoolError::Message(e.to_string()))?;
    }
    Ok(())
}

fn find_named(root: &Path, name: &str) -> Result<PathBuf, FirtoolError> {
    fn walk(dir: &Path, name: &str, out: &mut Option<PathBuf>) -> std::io::Result<()> {
        for e in fs::read_dir(dir)? {
            let e = e?;
            let p = e.path();
            if p.is_dir() {
                walk(&p, name, out)?;
            } else if p.file_name().and_then(|s| s.to_str()) == Some(name) {
                *out = Some(p);
            }
        }
        Ok(())
    }
    let mut found = None;
    walk(root, name, &mut found).map_err(|e| FirtoolError::Message(e.to_string()))?;
    found.ok_or_else(|| FirtoolError::Message(format!("extracted archive has no `{name}`")))
}

fn download_url(url: &str, dest: &Path) -> Result<(), FirtoolError> {
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(dest)
        .arg(url)
        .status()
        .map_err(|e| FirtoolError::Message(format!("spawn curl: {e}")))?;
    if !status.success() {
        return Err(FirtoolError::Message(format!("curl failed for {url}")));
    }
    Ok(())
}

pub fn parse_sha256_file(contents: &str) -> Result<String, FirtoolError> {
    let line = contents
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .ok_or_else(|| FirtoolError::Message("empty sha256 file".into()))?;
    let hex = line.split_whitespace().next().unwrap_or(line);
    if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(FirtoolError::Message(format!(
            "invalid sha256 digest: {hex:?}"
        )));
    }
    Ok(hex.to_ascii_lowercase())
}

pub fn sha256_file(path: &Path) -> Result<String, FirtoolError> {
    let mut f = fs::File::open(path).map_err(|e| FirtoolError::Message(e.to_string()))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| FirtoolError::Message(e.to_string()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bare_digest() {
        let d =
            parse_sha256_file("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n")
                .unwrap();
        assert_eq!(d.len(), 64);
    }

    #[test]
    fn sha256_empty() {
        let dir = std::env::temp_dir().join("rhdl_sha_empty");
        let _ = fs::create_dir_all(&dir);
        let p = dir.join("empty");
        fs::write(&p, b"").unwrap();
        let h = sha256_file(&p).unwrap();
        assert_eq!(
            h,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn host_asset_resolves_on_linux_x64() {
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            assert_eq!(
                firtool_asset_for_host().unwrap(),
                "firrtl-bin-linux-x64.tar.gz"
            );
        }
    }

    #[test]
    fn override_missing_bin_errors() {
        // SAFETY: test-only env mutation, single-threaded test.
        unsafe {
            std::env::set_var("RHDL_FIRTOOL_PATH", "/tmp/rhdl-no-such-firtool-dir");
        }
        let err = ensure_firtool().unwrap_err();
        assert!(err.to_string().contains("RHDL_FIRTOOL_PATH"));
        unsafe {
            std::env::remove_var("RHDL_FIRTOOL_PATH");
        }
    }
}

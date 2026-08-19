//! Optional FST via documented `vcd2fst` (AD-24 / FR31). Default dump remains VCD.

use std::path::{Path, PathBuf};
use std::process::Command;

/// How to obtain FST: never a homegrown writer (AD-24).
pub fn resolve_vcd2fst() -> Result<PathBuf, FstError> {
    resolve_vcd2fst_from(std::env::var_os("RHDL_VCD2FST"), std::env::var_os("PATH"))
}

pub(crate) fn resolve_vcd2fst_from(
    override_bin: Option<std::ffi::OsString>,
    path: Option<std::ffi::OsString>,
) -> Result<PathBuf, FstError> {
    if let Some(p) = override_bin {
        let pb = PathBuf::from(&p);
        if pb.is_file() {
            return Ok(pb);
        }
        return Err(FstError::Message(format!(
            "RHDL_VCD2FST={p:?} is not a file; install gtkwave's vcd2fst or point this env at it"
        )));
    }
    which_in("vcd2fst", path).ok_or_else(|| {
        FstError::Message(
            "FST requested but vcd2fst not found; install gtkwave (vcd2fst) or set RHDL_VCD2FST \
             (AD-24; Verilator --trace-fst is for Verilated C++ models, not native tick)"
                .into(),
        )
    })
}

fn which_in(name: &str, path: Option<std::ffi::OsString>) -> Option<PathBuf> {
    let path = path?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(name);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

#[derive(Debug)]
pub enum FstError {
    Message(String),
    Io(std::io::Error),
}

impl std::fmt::Display for FstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FstError::Message(m) => write!(f, "{m}"),
            FstError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for FstError {}

impl From<std::io::Error> for FstError {
    fn from(e: std::io::Error) -> Self {
        FstError::Io(e)
    }
}

pub(crate) fn convert_vcd_to_fst(converter: &Path, vcd: &Path, fst: &Path) -> Result<(), FstError> {
    let status = Command::new(converter)
        .arg(vcd)
        .arg(fst)
        .status()
        .map_err(|e| FstError::Message(format!("spawn vcd2fst: {e}")))?;
    if !status.success() {
        return Err(FstError::Message(format!(
            "vcd2fst failed converting {vcd:?} -> {fst:?}"
        )));
    }
    if !fst.is_file() {
        return Err(FstError::Message(format!("vcd2fst did not write {fst:?}")));
    }
    Ok(())
}

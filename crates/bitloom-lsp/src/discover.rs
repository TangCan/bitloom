//! FR113 — Cargo-graph + `[package.metadata.bitloom] design_roots` discovery.
//!
//! Beyond FR99 [`crate::DesignFixture`]-only. Full workspace `#[bitloom::top]` syn-scan
//! remains deferred (NFR47).

use std::fs;
use std::path::{Path, PathBuf};

/// One design root discovered from Cargo metadata (not a DesignFixture enum).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredDesignRoot {
    pub package_name: String,
    pub package_dir: PathBuf,
    pub root_id: String,
}

/// Walk `root` as a Cargo workspace or single package; return prelude-dependent
/// packages' `[package.metadata.bitloom] design_roots` entries.
pub fn discover_design_roots(root: impl AsRef<Path>) -> std::io::Result<Vec<DiscoveredDesignRoot>> {
    let root = root.as_ref();
    let cargo = root.join("Cargo.toml");
    let text = fs::read_to_string(&cargo)?;
    let mut out = Vec::new();

    if let Some(members) = workspace_members(&text) {
        for m in members {
            let pkg_dir = root.join(&m);
            collect_package_roots(&pkg_dir, &mut out)?;
        }
    } else {
        collect_package_roots(root, &mut out)?;
    }
    Ok(out)
}

fn collect_package_roots(
    pkg_dir: &Path,
    out: &mut Vec<DiscoveredDesignRoot>,
) -> std::io::Result<()> {
    let cargo = pkg_dir.join("Cargo.toml");
    if !cargo.is_file() {
        return Ok(());
    }
    let text = fs::read_to_string(&cargo)?;
    if !depends_on_bitloom_prelude(&text) {
        return Ok(());
    }
    let package_name = package_name(&text).unwrap_or_else(|| {
        pkg_dir
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into())
    });
    for root_id in design_roots_from_toml(&text) {
        out.push(DiscoveredDesignRoot {
            package_name: package_name.clone(),
            package_dir: pkg_dir.to_path_buf(),
            root_id,
        });
    }
    Ok(())
}

fn depends_on_bitloom_prelude(toml: &str) -> bool {
    toml.lines().any(|l| {
        let t = l.trim();
        t.starts_with("bitloom-prelude")
            || t.contains("bitloom-prelude =")
            || t.contains("\"bitloom-prelude\"")
    })
}

fn package_name(toml: &str) -> Option<String> {
    let section = section_body(toml, "package")?;
    keyed_string(section, "name")
}

fn design_roots_from_toml(toml: &str) -> Vec<String> {
    let Some(section) = section_body(toml, "package.metadata.bitloom") else {
        return Vec::new();
    };
    keyed_string_array(section, "design_roots")
}

fn workspace_members(toml: &str) -> Option<Vec<String>> {
    let section = section_body(toml, "workspace")?;
    let members = keyed_string_array(section, "members");
    if members.is_empty() {
        None
    } else {
        Some(members)
    }
}

fn section_body<'a>(toml: &'a str, header: &str) -> Option<&'a str> {
    let needle = format!("[{header}]");
    let start = toml.find(&needle)? + needle.len();
    let rest = &toml[start..];
    let end = rest.find("\n[").map(|i| i).unwrap_or(rest.len());
    Some(&rest[..end])
}

fn keyed_string(section: &str, key: &str) -> Option<String> {
    for line in section.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        return parse_toml_string(rest.trim());
    }
    None
}

fn keyed_string_array(section: &str, key: &str) -> Vec<String> {
    let mut buf = String::new();
    let mut capturing = false;
    for line in section.lines() {
        let t = line.trim();
        if !capturing {
            let Some(rest) = t.strip_prefix(key) else {
                continue;
            };
            let rest = rest.trim_start();
            let Some(rest) = rest.strip_prefix('=') else {
                continue;
            };
            let rest = rest.trim_start();
            if let Some(bracket) = rest.strip_prefix('[') {
                buf.push_str(bracket);
                capturing = true;
                if bracket.contains(']') {
                    break;
                }
                continue;
            }
        } else {
            buf.push_str(t);
            if t.contains(']') {
                break;
            }
        }
    }
    if !capturing {
        return Vec::new();
    }
    let inner = buf.split(']').next().unwrap_or("");
    inner
        .split(',')
        .filter_map(|p| parse_toml_string(p.trim()))
        .collect()
}

fn parse_toml_string(s: &str) -> Option<String> {
    let s = s.trim().trim_end_matches(',');
    let s = s.strip_prefix('"')?.strip_suffix('"')?;
    Some(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_metadata_design_roots() {
        let toml = r#"
[package]
name = "demo"

[dependencies]
bitloom-prelude = { path = "../bitloom-prelude" }

[package.metadata.bitloom]
design_roots = ["Fr113OkCounter", "Other"]
"#;
        assert!(depends_on_bitloom_prelude(toml));
        assert_eq!(package_name(toml).as_deref(), Some("demo"));
        assert_eq!(
            design_roots_from_toml(toml),
            vec!["Fr113OkCounter".to_string(), "Other".to_string()]
        );
    }
}

//! FR118 — Workspace `#[bitloom::top]` syn-scan + FR113 Cargo metadata discovery.
//!
//! - Packages with `[package.metadata.bitloom] design_roots` → FR113 metadata path.
//! - Prelude packages **without** metadata → scan `src/**/*.rs` for `#[bitloom::top]` /
//!   `#[rhdl::top]` (FR118). DesignFixture-only ≠ FR118.

use std::fs;
use std::path::{Path, PathBuf};

/// One design root discovered from Cargo metadata and/or syn-scan (not a DesignFixture enum).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredDesignRoot {
    pub package_name: String,
    pub package_dir: PathBuf,
    pub root_id: String,
}

/// Walk `root` as a Cargo workspace or single package; return design roots from
/// metadata `design_roots` and/or `#[bitloom::top]` syn-scan (FR113 + FR118).
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
    let meta_roots = design_roots_from_toml(&text);
    if !meta_roots.is_empty() {
        for root_id in meta_roots {
            out.push(DiscoveredDesignRoot {
                package_name: package_name.clone(),
                package_dir: pkg_dir.to_path_buf(),
                root_id,
            });
        }
        return Ok(());
    }
    // FR118: no metadata → syn-scan `#[bitloom::top]` / `#[rhdl::top]`.
    for root_id in syn_scan_top_names(pkg_dir)? {
        out.push(DiscoveredDesignRoot {
            package_name: package_name.clone(),
            package_dir: pkg_dir.to_path_buf(),
            root_id,
        });
    }
    Ok(())
}

/// Scan package `src/` for types annotated with `#[bitloom::top]` or `#[rhdl::top]`.
fn syn_scan_top_names(pkg_dir: &Path) -> std::io::Result<Vec<String>> {
    let src = pkg_dir.join("src");
    if !src.is_dir() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    walk_rs_files(&src, &mut |path| {
        let text = fs::read_to_string(path)?;
        for id in top_type_names_from_source(&text) {
            if !names.contains(&id) {
                names.push(id);
            }
        }
        Ok(())
    })?;
    Ok(names)
}

fn walk_rs_files(
    dir: &Path,
    f: &mut dyn FnMut(&Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_rs_files(&path, f)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            f(&path)?;
        }
    }
    Ok(())
}

/// Recognize `#[bitloom::top]` / `#[rhdl::top]` (optional whitespace) then the next
/// `struct` / `enum` / `type` identifier.
fn top_type_names_from_source(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < src.len() {
        if let Some(after_attr) = match_top_attr(&src[i..]) {
            let rest = &src[i + after_attr..];
            if let Some(name) = next_type_name(rest) {
                if !out.contains(&name) {
                    out.push(name);
                }
            }
            i += after_attr;
            continue;
        }
        i = next_char_boundary(src, i + 1);
    }
    out
}

fn next_char_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

fn match_top_attr(s: &str) -> Option<usize> {
    let trimmed_start = s.len() - s.trim_start().len();
    let s = s.trim_start();
    if !s.starts_with("#[") {
        return None;
    }
    let after = &s[2..];
    let end = after.find(']')?;
    let inner = after[..end].trim();
    // Strip optional spaces: bitloom :: top / rhdl::top
    let compact: String = inner.chars().filter(|c| !c.is_whitespace()).collect();
    if compact == "bitloom::top" || compact == "rhdl::top" {
        Some(trimmed_start + 2 + end + 1)
    } else {
        None
    }
}

fn next_type_name(s: &str) -> Option<String> {
    let mut rest = s.trim_start();
    // Skip other attributes / doc comments between attr and item.
    loop {
        rest = rest.trim_start();
        if rest.starts_with("#[") {
            let end = rest.find(']')?;
            rest = &rest[end + 1..];
            continue;
        }
        if rest.starts_with("///") || rest.starts_with("//!") || rest.starts_with("//") {
            rest = rest.split_once('\n').map(|(_, t)| t).unwrap_or("");
            continue;
        }
        break;
    }
    rest = rest.trim_start();
    if let Some(r) = rest.strip_prefix("pub") {
        rest = r.trim_start();
        if let Some(r) = rest.strip_prefix('(') {
            let end = r.find(')')?;
            rest = r[end + 1..].trim_start();
        }
    }
    let rest = if let Some(r) = rest.strip_prefix("struct") {
        r
    } else if let Some(r) = rest.strip_prefix("enum") {
        r
    } else if let Some(r) = rest.strip_prefix("type") {
        r
    } else {
        return None;
    };
    let rest = rest.trim_start();
    let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    let first = name.chars().next()?;
    if name.is_empty() || !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    Some(name)
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

    #[test]
    fn syn_scan_finds_bitloom_top_struct() {
        let src = r#"
#![allow(dead_code)]

#[bitloom::top]
struct Fr118OkCounter;

fn other() {}
"#;
        assert_eq!(
            top_type_names_from_source(src),
            vec!["Fr118OkCounter".to_string()]
        );
    }

    #[test]
    fn syn_scan_finds_rhdl_top_and_skips_unannotated() {
        let src = r#"
struct NotATop;

#[rhdl::top]
pub enum Fr118Alt {}
"#;
        assert_eq!(
            top_type_names_from_source(src),
            vec!["Fr118Alt".to_string()]
        );
    }

    #[test]
    fn syn_scan_tolerates_utf8_in_comments() {
        let src = "// FR113 negative fixture — prelude dep without metadata.\n#[bitloom::top]\nstruct A;\n";
        assert_eq!(top_type_names_from_source(src), vec!["A".to_string()]);
    }
}

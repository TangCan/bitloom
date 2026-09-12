//! FR157 — automatic FSM state-label extraction from annotated Rust source.
//!
//! Offline / tooling path: scan design source for `#[bitloom::fsm]` /
//! `#[rhdl::fsm]` enums. Compile-time path: the same attributes expand to
//! [`bitloom_prelude::FsmLabels`] (design crates depend only on prelude).

use syn::{Attribute, File, Item, Meta};

/// One FSM label set extracted from source (FR157).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsmLabelSet {
    /// FSM id (`name = "..."` attr, else enum ident).
    pub id: String,
    /// Variant names (unit variants only).
    pub labels: Vec<String>,
}

impl FsmLabelSet {
    /// Machine-readable registry lines: `fsm:<id>:<label>` (FR157 / FR109 key shape).
    pub fn to_registry_lines(&self) -> Vec<String> {
        self.labels
            .iter()
            .map(|l| format!("fsm:{}:{}", self.id, l))
            .collect()
    }
}

/// Explicit extract failure (FR157 — never silent empty success).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsmExtractError {
    Parse(String),
    /// Source has no `#[bitloom::fsm]` / `#[rhdl::fsm]` enum.
    NoAnnotatedFsm,
    /// Annotated enum has zero variants.
    EmptyVariants {
        enum_name: String,
    },
    /// Annotated enum uses non-unit variants (MVP rejection).
    NonUnitVariant {
        enum_name: String,
        variant: String,
    },
}

impl std::fmt::Display for FsmExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "FR157 parse error: {e}"),
            Self::NoAnnotatedFsm => write!(
                f,
                "FR157: no #[bitloom::fsm] / #[rhdl::fsm] enum found in source"
            ),
            Self::EmptyVariants { enum_name } => {
                write!(f, "FR157: enum `{enum_name}` has no variants")
            }
            Self::NonUnitVariant { enum_name, variant } => write!(
                f,
                "FR157: enum `{enum_name}` variant `{variant}` is not unit (MVP)"
            ),
        }
    }
}

impl std::error::Error for FsmExtractError {}

fn is_fsm_attr(attr: &Attribute) -> bool {
    let path = attr.path();
    let segs: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    matches!(
        segs.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        ["fsm"] | ["bitloom", "fsm"] | ["rhdl", "fsm"]
    )
}

fn fsm_name_override(attr: &Attribute) -> Option<String> {
    match &attr.meta {
        Meta::List(list) => {
            // Parse `name = "..."` from the token stream via syn.
            #[derive(Default)]
            struct NameArg {
                name: Option<syn::LitStr>,
            }
            impl syn::parse::Parse for NameArg {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    if input.is_empty() {
                        return Ok(Self::default());
                    }
                    let ident: syn::Ident = input.parse()?;
                    if ident != "name" {
                        return Err(input.error("expected name = \"...\""));
                    }
                    input.parse::<syn::Token![=]>()?;
                    let name: syn::LitStr = input.parse()?;
                    Ok(Self { name: Some(name) })
                }
            }
            syn::parse2::<NameArg>(list.tokens.clone())
                .ok()
                .and_then(|a| a.name.map(|n| n.value()))
        }
        _ => None,
    }
}

/// Extract all `#[bitloom::fsm]` / `#[rhdl::fsm]` label sets from Rust source text.
pub fn extract_fsm_labels_from_source(src: &str) -> Result<Vec<FsmLabelSet>, FsmExtractError> {
    let file: File = syn::parse_file(src).map_err(|e| FsmExtractError::Parse(e.to_string()))?;
    let mut out = Vec::new();
    for item in file.items {
        let Item::Enum(en) = item else {
            continue;
        };
        let Some(fsm_attr) = en.attrs.iter().find(|a| is_fsm_attr(a)) else {
            continue;
        };
        if en.variants.is_empty() {
            return Err(FsmExtractError::EmptyVariants {
                enum_name: en.ident.to_string(),
            });
        }
        let mut labels = Vec::new();
        for v in &en.variants {
            if !matches!(v.fields, syn::Fields::Unit) {
                return Err(FsmExtractError::NonUnitVariant {
                    enum_name: en.ident.to_string(),
                    variant: v.ident.to_string(),
                });
            }
            labels.push(v.ident.to_string());
        }
        let id = fsm_name_override(fsm_attr).unwrap_or_else(|| en.ident.to_string());
        out.push(FsmLabelSet { id, labels });
    }
    if out.is_empty() {
        return Err(FsmExtractError::NoAnnotatedFsm);
    }
    Ok(out)
}

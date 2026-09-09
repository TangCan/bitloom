//! Proc macros expand only to `bitloom_builder` / prelude paths (AD-6). Never depend on rhdl-hir.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, GenericArgument, ItemFn, PathArguments, Type, parse_macro_input,
};

/// Derive [`bitloom_prelude::Bundle`] from named struct fields (FR80 / Story 32.3).
///
/// **Supported:** named-field structs; ground fields `Bool` / `Clock` / `Reset` /
/// `UInt<N>` / `SInt<N>` / `Bits<N>`; other simple path types as **one-level** nested
/// Bundles (`Type::leaves`).
///
/// **Rejected (stable `compile_error!`):** enums, unions, tuple structs, unit structs
/// without named fields, `HwVec<_>`, `Input<_>` / `Output<_>`, references/tuples/arrays,
/// and non-path field types.
///
/// Re-exported from `bitloom-prelude` so design crates never depend on this crate
/// directly (AD-6).
#[proc_macro_derive(Bundle, attributes(bundle))]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_derive_bundle(&input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_derive_bundle(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "rhdl::E0180: #[derive(Bundle)] does not support generic parameters",
        ));
    }

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "rhdl::E0180: #[derive(Bundle)] only supports structs with named fields",
        ));
    };

    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &data.fields,
            "rhdl::E0180: #[derive(Bundle)] requires named fields (tuple/unit structs unsupported)",
        ));
    };

    let name = &input.ident;
    let mut leaf_entries = Vec::new();
    let mut nested_entries = Vec::new();

    for field in &fields.named {
        let Some(ident) = &field.ident else {
            return Err(syn::Error::new_spanned(
                field,
                "rhdl::E0180: #[derive(Bundle)] requires named fields",
            ));
        };
        let field_name = ident.to_string();
        match classify_bundle_field(&field.ty)? {
            BundleFieldKind::Ground { ground_expr } => {
                leaf_entries.push(quote! {
                    (#field_name, #ground_expr)
                });
            }
            BundleFieldKind::Nested { ty } => {
                nested_entries.push(quote! {
                    (#field_name, <#ty as ::bitloom_prelude::Bundle>::leaves)
                });
            }
        }
    }

    Ok(TokenStream::from(quote! {
        impl ::bitloom_prelude::Bundle for #name {
            fn leaves() -> &'static [(&'static str, ::bitloom_prelude::GroundType)] {
                &[#(#leaf_entries),*]
            }

            fn nested_bundles() -> &'static [(
                &'static str,
                fn() -> &'static [(&'static str, ::bitloom_prelude::GroundType)],
            )] {
                &[#(#nested_entries),*]
            }
        }
    }))
}

enum BundleFieldKind {
    Ground {
        ground_expr: proc_macro2::TokenStream,
    },
    Nested {
        ty: Type,
    },
}

fn classify_bundle_field(ty: &Type) -> Result<BundleFieldKind, syn::Error> {
    let Type::Path(type_path) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "rhdl::E0180: #[derive(Bundle)] field types must be simple paths (ground or nested Bundle)",
        ));
    };

    if type_path.qself.is_some() {
        return Err(syn::Error::new_spanned(
            ty,
            "rhdl::E0180: #[derive(Bundle)] does not support qualified self types",
        ));
    }

    let last =
        type_path.path.segments.last().ok_or_else(|| {
            syn::Error::new_spanned(ty, "rhdl::E0180: empty path in Bundle field")
        })?;
    let ident = last.ident.to_string();

    match ident.as_str() {
        "HwVec" => {
            return Err(syn::Error::new_spanned(
                ty,
                "rhdl::E0180: #[derive(Bundle)] does not support HwVec fields; HwVec<Bundle,_> remains OUT OF SCOPE",
            ));
        }
        "Input" | "Output" => {
            return Err(syn::Error::new_spanned(
                ty,
                "rhdl::E0180: #[derive(Bundle)] fields must be bare ground or nested Bundle types, not Input/Output",
            ));
        }
        "Bool" => {
            require_no_args(ty, &last.arguments)?;
            return Ok(BundleFieldKind::Ground {
                ground_expr: quote! { ::bitloom_prelude::GroundType::Bool },
            });
        }
        "Clock" => {
            require_no_args(ty, &last.arguments)?;
            return Ok(BundleFieldKind::Ground {
                ground_expr: quote! { ::bitloom_prelude::GroundType::Clock },
            });
        }
        "Reset" => {
            require_no_args(ty, &last.arguments)?;
            return Ok(BundleFieldKind::Ground {
                ground_expr: quote! { ::bitloom_prelude::GroundType::Reset },
            });
        }
        "UInt" | "Bits" => {
            let width = const_generic_u32(ty, &last.arguments)?;
            return Ok(BundleFieldKind::Ground {
                ground_expr: quote! { ::bitloom_prelude::GroundType::UInt { width: #width } },
            });
        }
        "SInt" => {
            let width = const_generic_u32(ty, &last.arguments)?;
            return Ok(BundleFieldKind::Ground {
                ground_expr: quote! { ::bitloom_prelude::GroundType::SInt { width: #width } },
            });
        }
        _ => {}
    }

    // Nested Bundle: simple path, no type args (Epic 32 one-level contract).
    if !matches!(last.arguments, PathArguments::None) {
        return Err(syn::Error::new_spanned(
            ty,
            "rhdl::E0180: #[derive(Bundle)] nested Bundle fields must be bare type paths without generics",
        ));
    }

    Ok(BundleFieldKind::Nested { ty: ty.clone() })
}

fn require_no_args(ty: &Type, args: &PathArguments) -> Result<(), syn::Error> {
    if matches!(args, PathArguments::None) {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            ty,
            "rhdl::E0180: unexpected type arguments on ground Bundle field",
        ))
    }
}

fn const_generic_u32(ty: &Type, args: &PathArguments) -> Result<u32, syn::Error> {
    let PathArguments::AngleBracketed(ab) = args else {
        return Err(syn::Error::new_spanned(
            ty,
            "rhdl::E0180: width-parameterized ground types require a const generic (e.g. UInt<8>)",
        ));
    };
    let mut width: Option<u32> = None;
    for arg in &ab.args {
        match arg {
            GenericArgument::Const(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit),
                ..
            })) => {
                if width.is_some() {
                    return Err(syn::Error::new_spanned(
                        ty,
                        "rhdl::E0180: expected exactly one const width parameter",
                    ));
                }
                width = Some(lit.base10_parse()?);
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "rhdl::E0180: ground width must be an integer literal const generic",
                ));
            }
        }
    }
    width.ok_or_else(|| {
        syn::Error::new_spanned(
            ty,
            "rhdl::E0180: width-parameterized ground types require a const generic (e.g. UInt<8>)",
        )
    })
}

/// Marks a struct as an RHDL module shell for Story 1.1.
/// Generates `Elaboratable` that records directed ports via the builder session.
#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;

    let syn::Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "rhdl::module only supports structs")
            .to_compile_error()
            .into();
    };

    let mod_name = name.to_string();
    let field_defs = data.fields.iter().map(|f| {
        let id = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let fvis = &f.vis;
        quote! { #fvis #id: #ty }
    });

    let port_stmts = data.fields.iter().map(|field| {
        let Some(ident) = &field.ident else {
            return quote! {
                compile_error!("tuple structs are not supported by rhdl::module");
            };
        };
        let ty = &field.ty;
        let name_str = ident.to_string();
        quote! {
            {
                type __PortTy = #ty;
                for (__leaf, __dir, __gt) in
                    <__PortTy as ::bitloom_prelude::PortField>::flatten(#name_str)
                {
                    match __dir {
                        ::bitloom_prelude::PortDir::Input => {
                            __session.add_input(
                                __leaf,
                                __gt,
                                ::bitloom_prelude::Span::default(),
                            );
                        }
                        ::bitloom_prelude::PortDir::Output => {
                            __session.add_output(
                                __leaf,
                                __gt,
                                ::bitloom_prelude::Span::default(),
                            );
                        }
                    }
                }
            }
        }
    });

    TokenStream::from(quote! {
        #vis struct #name {
            #(#field_defs),*
        }

        impl ::bitloom_prelude::Elaboratable for #name {
            fn elaborate() -> ::core::result::Result<
                ::bitloom_prelude::FrozenHir,
                ::bitloom_prelude::Diagnostics,
            > {
                let mut __session = ::bitloom_prelude::ElaborateSession::new(#mod_name);
                __session.begin_module(#mod_name, ::bitloom_prelude::Span::default());
                #(#port_stmts)*
                __session.end_module();
                __session.finish()
            }
        }
    })
}

/// Marks a hardware process as combinational. Expands to a builder open/close
/// around the function body (must call session helpers for assigns).
#[proc_macro_attribute]
pub fn combinational(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let attrs = &input.attrs;
    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            // Marker retained so unmarked hardware fns are distinguishable.
            const _: () = ();
            let __rhdl_process_kind = ::bitloom_prelude::ProcessKindMark::Combinational;
            let _ = __rhdl_process_kind;
            #block
        }
    })
}

/// Marks a hardware process as sequential.
#[proc_macro_attribute]
pub fn sequential(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let attrs = &input.attrs;
    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            const _: () = ();
            let __rhdl_process_kind = ::bitloom_prelude::ProcessKindMark::Sequential;
            let _ = __rhdl_process_kind;
            #block
        }
    })
}

/// Rejects unmarked hardware process attributes — use `combinational`/`sequential`.
#[proc_macro_attribute]
pub fn process(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    syn::Error::new_spanned(
        input.sig.ident,
        "hardware processes must use #[combinational] or #[sequential]; bare #[process] is forbidden",
    )
    .to_compile_error()
    .into()
}

/// Marks a handwritten functional model (Story 3.3). Does not enter FrozenHir.
#[proc_macro_attribute]
pub fn functional_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    host_only_view(item, "FunctionalModel")
}

/// Handwritten TLM↔pin adapter (FR29). Host-only; never enters FrozenHir / freeze.
/// Does **not** generate TLM from HIR.
#[proc_macro_attribute]
pub fn bridge(_attr: TokenStream, item: TokenStream) -> TokenStream {
    host_only_view(item, "Bridge")
}

/// Handwritten untimed / transaction abstraction (FR29). Host-only.
#[proc_macro_attribute]
pub fn abstraction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    host_only_view(item, "Abstraction")
}

/// Mixed `both` simulation: RTL (`tick`) + handwritten view in one fixture (FR29).
#[proc_macro_attribute]
pub fn both(_attr: TokenStream, item: TokenStream) -> TokenStream {
    host_only_view(item, "Both")
}

/// Marks a function for optional external HLS (FR35 / AD-25). Never schedules in-process.
#[proc_macro_attribute]
pub fn hls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let attrs = &input.attrs;
    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            const _: () = ();
            let __rhdl_hls = ::bitloom_prelude::HlsMark;
            let _ = __rhdl_hls;
            #block
        }
    })
}

fn host_only_view(item: TokenStream, kind: &str) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let kind_ident = syn::Ident::new(kind, name.span());
    TokenStream::from(quote! {
        #input

        impl ::bitloom_prelude::HostView for #name {
            const KIND: ::bitloom_prelude::ViewKind = ::bitloom_prelude::ViewKind::#kind_ident;
        }
        // Host-only; never participates in freeze/HIR. No HIR→TLM lowering.
    })
}

/// Marks the elaboratable top for `cargo rhdl build` (AD-19).
#[proc_macro_attribute]
pub fn top(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    TokenStream::from(quote! {
        #input

        impl #name {
            /// ABI marker used by the RHDL host/CLI.
            pub const RHDL_TOP: bool = true;
        }
    })
}

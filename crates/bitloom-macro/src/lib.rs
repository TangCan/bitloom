//! Proc macros expand only to `bitloom_builder` / prelude paths (AD-6). Never depend on rhdl-hir.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, parse_macro_input};

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

use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, Span as Span2};
use proc_macro_crate::crate_name;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn subtype(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let found = crate_name("subtype_rs")
        .expect("`subtype_rs` must be an explicit dependency in Cargo.toml");
    let crate_ident = match found {
        proc_macro_crate::FoundCrate::Itself => Ident2::new("subtype_rs", Span2::call_site()),
        proc_macro_crate::FoundCrate::Name(name) => Ident2::new(&name, Span2::call_site()),
    };

    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    let mut min = None;
    let mut max = None;
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if let Some(ident) = nv.path.get_ident() {
                match (ident.to_string().as_str(), nv.lit) {
                    ("min", Lit::Int(lit)) => min = Some(lit),
                    ("max", Lit::Int(lit)) => max = Some(lit),
                    _ => {}
                }
            }
        }
    }
    let min = min.expect("`min` attribute missing");
    let max = max.expect("`max` attribute missing");

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let ty = match &input.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
        _ => panic!("#[subtype] only supports tuple structs with a single field"),
    };

    let err_ty = quote!(#crate_ident::SubtypeError<#ty>);

    let expanded = quote! {
        pub struct #name(pub #ty);

        impl #name {
            #[inline(always)]
            pub fn new(value: #ty) -> Result<Self, #err_ty> {
                if value < #min {
                    Err(#crate_ident::SubtypeError::BelowMinimum(value))
                } else if value > #max {
                    Err(#crate_ident::SubtypeError::AboveMaximum(value))
                } else {
                    Ok(Self(value))
                }
            }

            #[inline(always)]
            pub fn into_inner(self) -> #ty {
                self.0
            }
        }

        impl ::core::convert::TryFrom<#ty> for #name {
            type Error = #err_ty;

            #[inline(always)]
            fn try_from(value: #ty) -> Result<Self, Self::Error> {
                #name::new(value)
            }
        }

        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };

    expanded.into()
}

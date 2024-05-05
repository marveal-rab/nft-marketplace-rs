extern crate proc_macro;
use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(QueryParam)]
pub fn derive_query_param(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Build the impl
    let expanded = impl_query_param(&input);

    TokenStream::from(expanded)
}

fn impl_query_param(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let expanded = quote! {
        impl QueryParam for #name {
            fn encode(&self) -> String {
                serde_urlencoded::to_string(self).unwrap()
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(Request, attributes(Path))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Build the impl
    let expanded = impl_request(&input);

    TokenStream::from(expanded)
}

fn impl_request(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let path = match input.attrs.iter().find(|attr| attr.path().is_ident("Path")) {
        Some(attr) => match &attr.meta {
            syn::Meta::NameValue(nv) => {
                let lit = nv.value.clone();
                quote! { #lit }
            }
            _ => quote! { "" },
        },
        None => quote! { "" },
    };
    let expanded = quote! {
        impl Request for #name {
            fn path(&self) -> String {
                #path.to_string()
            }
        }
    };
    expanded.into()
}

//! # cow_struct
//!
//! This crate consists in a procedural macro derive that provides a
//! struct that is Cow and the impl to create one from the target struct

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use std::iter::FromIterator;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(CowStruct)]
pub fn make_cow(items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(items as DeriveInput);

    let cow = format_ident!("Cow{}", input.ident);
    let name = input.ident;
    let fields = if let Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("Only struct supported");
    };

    let set_fields = TokenStream::from_iter(fields.iter().map(|item| {
        let field = &item.ident;
        quote! { #field: std::borrow::Cow::Borrowed(&self.#field), }
    }));
    let cow_fields = TokenStream::from_iter(fields.iter().map(|item| {
        let field = &item.ident;
        let ty = &item.ty;
        quote! { #field: std::borrow::Cow<'a, #ty>, }
    }));

    let ret: TokenStream = quote_spanned! {
        Span::call_site() =>
        impl #name {
            fn to_cow(&self) -> #cow {
                #cow {
                    #set_fields
                }
            }
        }

        #[derive(Debug, Default)]
        struct #cow<'a> {
            #cow_fields
        }
        impl #cow<'_> {
            fn to_cow(&self) -> #cow {
                #cow {
                    #set_fields
                }
            }
        }
    }
    .into();

    ret.into()
}

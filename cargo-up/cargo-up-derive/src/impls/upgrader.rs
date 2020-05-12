use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields};

pub fn upgrader(input: &DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = input;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields.named.clone(),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => Punctuated::<Field, Comma>::new(),
        _ => abort_call_site!("`#[upgrader]` only supports non-tuple structs"),
    };

    let upgrader = quote!(_upgrader: ::cargo_up::UpgraderInner);

    quote! {
        struct #ident {
            #fields,
            #upgrader
        }

        impl ::cargo_up::Upgrader for #ident {
            fn replace(&mut self, range: ::cargo_up::ra_text_edit::TextRange, replace_with: String) {
                self._upgrader.edit.replace(range, replace_with)
            }

            fn delete(&mut self, range: ::cargo_up::ra_text_edit::TextRange) {
                self._upgrader.edit.delete(range)
            }

            fn insert(&mut self, offset: ::cargo_up::ra_text_edit::TextSize, text: String) {
                self._upgrader.edit.insert(offset, text)
            }

            fn finish(&mut self) -> ::cargo_up::ra_text_edit::TextEdit {
                let edit = self._upgrader.edit.clone().finish();
                self._upgrader.edit = ::cargo_up::ra_text_edit::TextEditBuilder::default();
                edit
            }
        }
    }
}

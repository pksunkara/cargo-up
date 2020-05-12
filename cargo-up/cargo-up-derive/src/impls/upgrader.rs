use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    parse_str,
    punctuated::Punctuated,
    token::{Brace, Colon, Comma},
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Visibility,
};

pub fn upgrader(input: &DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = input;

    let mut fields = match input.data {
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

    fields.push(Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(parse_str("_upgrader").unwrap()),
        colon_token: Some(Colon::default()),
        ty: parse_str("::cargo_up::UpgraderInner").unwrap(),
    });

    let mut new = input.clone();

    if let Data::Struct(d) = new.data {
        new.data = Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed {
                brace_token: Brace::default(),
                named: fields,
            }),
            ..d
        });
    }

    quote! {
        #new

        impl ::cargo_up::Upgrader for #ident {
            fn new(version: ::cargo_up::semver::Version) -> Self {
                let mut ret = Self::default();
                ret._upgrader.version = format!("{}", version);
                ret
            }

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

use crate::utils::Options;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use semver::Version;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_str,
    punctuated::Punctuated,
    token::{Brace, Colon, Comma},
    Data, DataStruct, DeriveInput, Expr, Field, Fields, FieldsNamed, Lit, LitStr, Visibility,
};

pub struct Upgrader {
    minimum: Option<LitStr>,
    peers: Vec<LitStr>,
}

impl Parse for Upgrader {
    fn parse(input: ParseStream) -> Result<Self> {
        let options: Options = input.parse()?;

        // TODO: Really think something like serde_token can make this better looking
        let minimum = options.find("minimum").map(|x| {
            if let Expr::Lit(expr_lit) = x.1 {
                if let Lit::Str(lit) = expr_lit.lit.clone() {
                    return lit;
                }
            }

            abort!(x.1, "expected a literal string");
        });

        if let Some(version) = &minimum {
            if let Err(_) = Version::parse(&version.value()) {
                abort!(version, "expected a valid semver version");
            }
        }

        let peers = options.find("peers").map_or(vec![], |x| {
            if let Expr::Array(expr_array) = x.1 {
                return expr_array
                    .elems
                    .iter()
                    .flat_map(|x| {
                        if let Expr::Lit(expr_lit) = x {
                            if let Lit::Str(lit) = expr_lit.lit.clone() {
                                return Some(lit);
                            }
                        }

                        abort!(x, "expected a literal string");
                    })
                    .collect();
            }

            abort!(x.1, "expected an array of literal strings");
        });

        Ok(Upgrader { minimum, peers })
    }
}

pub fn upgrader(attr: Upgrader, input: &DeriveInput) -> TokenStream {
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
            #[inline]
            fn get_upgrader_mut(&mut self) -> &mut ::cargo_up::UpgraderInner {
                &mut self._upgrader
            }

            fn new(version: ::cargo_up::semver::Version) -> Self {
                let mut ret = Self::default();
                ret._upgrader.version = format!("{}", version);
                ret
            }
        }
    }
}

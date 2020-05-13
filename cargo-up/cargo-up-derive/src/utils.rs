use std::collections::BTreeMap as Map;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Comma, Eq},
    Expr, Ident,
};

// TODO: Make serde_token crate?
pub struct Options {
    pub inner: Map<Ident, Expr>,
}

impl Options {
    pub fn find(&self, key: &str) -> Option<(&Ident, &Expr)> {
        self.inner.iter().find(|x| x.0.to_string() == key)
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut inner = Map::new();

        while input.peek(Comma) {
            input.parse::<Comma>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Eq>()?;

            inner.insert(ident, input.parse()?);
        }

        Ok(Options { inner })
    }
}

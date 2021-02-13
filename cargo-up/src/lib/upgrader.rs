use crate::ra_ap_syntax::{
    ast::{Name, NameRef, Path, PathSegment},
    AstNode,
};

use ra_ap_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize};

#[derive(Default, Debug, Clone)]
pub struct Upgrader {
    edit: TextEditBuilder,
}

pub trait ToTextRange {
    fn to(self) -> TextRange;
}

impl ToTextRange for TextRange {
    fn to(self) -> TextRange {
        self
    }
}

impl ToTextRange for Option<Path> {
    fn to(self) -> TextRange {
        self.unwrap().segment().unwrap().name_ref().to()
    }
}

impl ToTextRange for Option<PathSegment> {
    fn to(self) -> TextRange {
        self.unwrap().name_ref().to()
    }
}

impl ToTextRange for Option<NameRef> {
    fn to(self) -> TextRange {
        self.unwrap().syntax().text_range()
    }
}

impl ToTextRange for Option<Name> {
    fn to(self) -> TextRange {
        self.unwrap().syntax().text_range()
    }
}

impl Upgrader {
    pub fn replace<T, S>(&mut self, range: T, replace_with: S)
    where
        T: ToTextRange,
        S: Into<String>,
    {
        self.edit.replace(range.to(), replace_with.into())
    }

    pub fn delete<T>(&mut self, range: T)
    where
        T: ToTextRange,
    {
        self.edit.delete(range.to())
    }

    pub fn insert(&mut self, offset: TextSize, text: String) {
        self.edit.insert(offset, text)
    }

    pub fn add_dep(&mut self) {}

    pub fn add_feature(&mut self) {}

    pub(crate) fn finish(&mut self) -> TextEdit {
        let edit = self.edit.clone().finish();
        self.edit = TextEditBuilder::default();
        edit
    }
}

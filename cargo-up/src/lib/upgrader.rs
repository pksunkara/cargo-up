use crate::{
    ra_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize},
    semver::Version,
};

#[derive(Default, Debug)]
pub struct UpgraderInner {
    pub version: String,
    pub edit: TextEditBuilder,
}

pub trait Upgrader {
    fn new(version: Version) -> Self;
    fn replace(&mut self, range: TextRange, replace_with: String);
    fn delete(&mut self, range: TextRange);
    fn insert(&mut self, offset: TextSize, text: String);
    fn finish(&mut self) -> TextEdit;
}

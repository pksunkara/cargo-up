use ra_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize};

#[derive(Default, Debug)]
pub struct UpgraderInner {
    pub edit: TextEditBuilder,
}

pub trait Upgrader {
    fn replace(&mut self, range: TextRange, replace_with: String);
    fn delete(&mut self, range: TextRange);
    fn insert(&mut self, offset: TextSize, text: String);
    fn finish(&mut self) -> TextEdit;
}

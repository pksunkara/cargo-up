use ra_ap_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize};

#[derive(Default, Debug, Clone)]
pub struct Upgrader {
    edit: TextEditBuilder,
}

impl Upgrader {
    pub fn replace(&mut self, range: TextRange, replace_with: String) {
        self.edit.replace(range, replace_with)
    }

    pub fn delete(&mut self, range: TextRange) {
        self.edit.delete(range)
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

use crate::{
    ra_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize},
    semver::Version,
};

#[derive(Default, Debug)]
pub struct UpgraderInner {
    pub version: String,
    pub edit: TextEditBuilder,
}

pub trait Upgrader: Default {
    fn get_upgrader_mut(&mut self) -> &mut UpgraderInner;

    fn new(version: Version) -> Self {
        let mut ret = Self::default();
        ret.get_upgrader_mut().version = format!("{}", version);
        ret
    }

    fn replace(&mut self, range: TextRange, replace_with: String) {
        self.get_upgrader_mut().edit.replace(range, replace_with)
    }

    fn delete(&mut self, range: TextRange) {
        self.get_upgrader_mut().edit.delete(range)
    }

    fn insert(&mut self, offset: TextSize, text: String) {
        self.get_upgrader_mut().edit.insert(offset, text)
    }

    fn finish(&mut self) -> TextEdit {
        let edit = self.get_upgrader_mut().edit.clone().finish();
        self.get_upgrader_mut().edit = TextEditBuilder::default();
        edit
    }
}

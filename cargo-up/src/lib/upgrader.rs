use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    semver::{SemVerError, Version},
    Visitor,
};
use ra_text_edit::{TextEdit, TextEditBuilder, TextRange, TextSize};

#[derive(Debug, Default)]
pub struct Editor(TextEditBuilder);

impl Editor {
    pub fn replace(&mut self, range: TextRange, replace_with: String) {
        self.0.replace(range, replace_with)
    }

    pub fn delete(&mut self, range: TextRange) {
        self.0.delete(range)
    }

    pub fn insert(&mut self, offset: TextSize, text: String) {
        self.0.insert(offset, text)
    }

    pub(crate) fn finish(&mut self) -> TextEdit {
        let edit = self.0.clone().finish();
        self.0 = TextEditBuilder::default();
        edit
    }
}

#[derive(Debug)]
pub struct Upgrader<T = ()>
where
    T: UpgradeVisitor,
{
    pub(crate) minimum: Option<Version>,
    pub(crate) editor: Editor,
    pub(crate) hook: Option<T>,
}

impl<T> Upgrader<T>
where
    T: UpgradeVisitor,
{
    pub fn new() -> Self {
        Self {
            minimum: None,
            editor: Editor::default(),
            hook: None,
        }
    }

    pub fn hook(mut self, hook: T) -> Self {
        self.hook = Some(hook);
        self
    }

    pub fn minimum(mut self, version: &str) -> Result<Self, SemVerError> {
        self.minimum = Some(Version::parse(version)?);
        Ok(self)
    }
}

impl<T> Visitor for Upgrader<T>
where
    T: UpgradeVisitor,
{
    fn visit_source_file(&mut self, _: ast::SourceFile, _: &Semantics<RootDatabase>) {}

    fn visit_method_call_expr(
        &mut self,
        method_call_expr: ast::MethodCallExpr,
        semantics: &Semantics<RootDatabase>,
    ) {
        if let Some(hook) = self.hook.as_mut() {
            hook.method_call_expr(&mut self.editor, method_call_expr, semantics);
        }
    }
}

pub trait UpgradeVisitor {
    fn method_call_expr(
        &mut self,
        _: &mut Editor,
        _: ast::MethodCallExpr,
        _: &Semantics<RootDatabase>,
    ) {
    }
}

impl UpgradeVisitor for () {}

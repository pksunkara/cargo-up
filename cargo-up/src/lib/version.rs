use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::ast,
    semver::{SemVerError, Version as SemverVersion},
    Upgrader,
};

type MethodCallExpr = dyn Fn(&mut Upgrader, &ast::MethodCallExpr, &Semantics<RootDatabase>);

pub struct Version {
    pub(crate) version: SemverVersion,
    pub(crate) hooks_method_call_expr: Vec<Box<MethodCallExpr>>,
}

impl Version {
    pub fn new(version: &str) -> Result<Self, SemVerError> {
        Ok(Self {
            version: SemverVersion::parse(version)?,
            hooks_method_call_expr: vec![],
        })
    }

    pub fn hook_method_call_expr<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Upgrader, &ast::MethodCallExpr, &Semantics<RootDatabase>) + 'static,
    {
        self.hooks_method_call_expr.push(Box::new(f));
        self
    }
}

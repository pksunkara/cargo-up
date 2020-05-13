use crate::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::ast,
    semver::{SemVerError, Version as SemverVersion},
    Upgrader,
};
use std::collections::BTreeMap as Map;

macro_rules! alias {
    ($node:ident) => {
        type $node = dyn Fn(&mut Upgrader, &ast::$node, &Semantics<RootDatabase>);
    };
}

macro_rules! hook {
    ($method:ident, $node:ident) => {
        pub fn $method<F>(mut self, f: F) -> Self
        where
            F: Fn(&mut Upgrader, &ast::$node, &Semantics<RootDatabase>) + 'static,
        {
            self.$method.push(Box::new(f));
            self
        }
    };
}

alias!(MethodCallExpr);

pub struct Version {
    pub(crate) version: SemverVersion,
    pub(crate) rename_methods: Map<String, Map<String, String>>,
    pub(crate) hook_method_call_expr: Vec<Box<MethodCallExpr>>,
}

impl Version {
    pub fn new(version: &str) -> Result<Self, SemVerError> {
        Ok(Self {
            version: SemverVersion::parse(version)?,
            hook_method_call_expr: vec![],
            rename_methods: Map::new(),
        })
    }

    pub fn rename_methods(mut self, name: &str, map: &[[&str; 2]]) -> Self {
        self.rename_methods.insert(
            name.to_string(),
            map.iter()
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect(),
        );
        self
    }

    hook!(hook_method_call_expr, MethodCallExpr);
}

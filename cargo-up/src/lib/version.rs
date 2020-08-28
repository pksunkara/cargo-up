use crate::{
    ra_ap_hir::Semantics,
    ra_ap_ide_db::RootDatabase,
    ra_ap_syntax::ast,
    semver::{SemVerError, Version as SemverVersion},
    utils::normalize,
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
alias!(CallExpr);
alias!(PathExpr);
alias!(FieldExpr);
alias!(RecordExpr);
alias!(RecordExprField);
alias!(RecordPatField);

pub struct Version {
    pub(crate) version: SemverVersion,
    pub(crate) peers: Vec<String>,
    pub(crate) rename_methods: Map<String, Map<String, String>>,
    pub(crate) rename_members: Map<String, Map<String, String>>,
    pub(crate) rename_variants: Map<String, Map<String, String>>,
    pub(crate) hook_method_call_expr: Vec<Box<MethodCallExpr>>,
    pub(crate) hook_call_expr: Vec<Box<CallExpr>>,
    pub(crate) hook_path_expr: Vec<Box<PathExpr>>,
    pub(crate) hook_field_expr: Vec<Box<FieldExpr>>,
    pub(crate) hook_record_expr: Vec<Box<RecordExpr>>,
    pub(crate) hook_record_expr_field: Vec<Box<RecordExprField>>,
    pub(crate) hook_record_pat_field: Vec<Box<RecordPatField>>,
}

impl Version {
    pub fn new(version: &str) -> Result<Self, SemVerError> {
        Ok(Self {
            version: SemverVersion::parse(version)?,
            peers: vec![],
            rename_methods: Map::new(),
            rename_members: Map::new(),
            rename_variants: Map::new(),
            hook_method_call_expr: vec![],
            hook_call_expr: vec![],
            hook_path_expr: vec![],
            hook_field_expr: vec![],
            hook_record_expr: vec![],
            hook_record_expr_field: vec![],
            hook_record_pat_field: vec![],
        })
    }

    pub fn peers(mut self, peers: &[&str]) -> Self {
        self.peers = peers.to_vec().iter().map(|x| normalize(*x)).collect();
        self
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

    pub fn rename_members(mut self, name: &str, map: &[[&str; 2]]) -> Self {
        self.rename_members.insert(
            name.to_string(),
            map.iter()
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect(),
        );
        self
    }

    pub fn rename_variants(mut self, name: &str, map: &[[&str; 2]]) -> Self {
        self.rename_variants.insert(
            name.to_string(),
            map.iter()
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect(),
        );
        self
    }

    hook!(hook_method_call_expr, MethodCallExpr);
    hook!(hook_call_expr, CallExpr);
    hook!(hook_path_expr, PathExpr);
    hook!(hook_field_expr, FieldExpr);
    hook!(hook_record_expr, RecordExpr);
    hook!(hook_record_expr_field, RecordExprField);
    hook!(hook_record_pat_field, RecordPatField);
}

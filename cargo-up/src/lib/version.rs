use crate::{
    semver::{SemVerError, Version as SemverVersion},
    utils::normalize,
    Semantics, Upgrader,
};

use paste::paste;
use rust_visitor::ra_ap_syntax::ast;

use std::collections::HashMap as Map;

macro_rules! members {
    ($($node:ident,)*) => {
        paste! {
            pub struct Version {
                pub(crate) version: SemverVersion,
                pub(crate) peers: Vec<String>,
                pub(crate) rename_structs: Map<String, Map<String, String>>,
                pub(crate) rename_methods: Map<String, Map<String, String>>,
                pub(crate) rename_members: Map<String, Map<String, String>>,
                pub(crate) rename_variants: Map<String, Map<String, String>>,
                $(pub(crate) [<hook_ $node:snake>]: Vec<Box<dyn Fn(&mut Upgrader, &ast::$node, &Semantics)>>,)*
            }

            impl Version {
                pub fn new(version: &str) -> Result<Self, SemVerError> {
                    Ok(Self {
                        version: SemverVersion::parse(version)?,
                        peers: vec![],
                        rename_structs: Map::new(),
                        rename_methods: Map::new(),
                        rename_members: Map::new(),
                        rename_variants: Map::new(),
                        $([<hook_ $node:snake>]: Vec::new(),)*
                    })
                }
            }
        }
    };
}

macro_rules! methods {
    ($($node:ident,)*) => {
        paste! {
            $(
                pub fn [<hook_ $node:snake>]<F>(mut self, f: F) -> Self
                where
                    F: Fn(&mut Upgrader, &ast::$node, &Semantics) + 'static,
                {
                    self.[<hook_ $node:snake>].push(Box::new(f));
                    self
                }
            )*
        }
    };
}

members!(
    MethodCallExpr,
    CallExpr,
    IdentPat,
    Path,
    PathExpr,
    PathPat,
    FieldExpr,
    RecordPat,
    RecordExpr,
    RecordExprField,
    RecordPatField,
    TupleStructPat,
);

impl Version {
    pub fn peers(mut self, peers: &[&str]) -> Self {
        self.peers = peers.to_vec().iter().map(|x| normalize(*x)).collect();
        self
    }

    pub fn rename_structs(mut self, name: &str, map: &[[&str; 2]]) -> Self {
        self.rename_structs.insert(
            name.to_string(),
            map.iter()
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect(),
        );
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
}

impl Version {
    methods!(
        MethodCallExpr,
        CallExpr,
        IdentPat,
        Path,
        PathExpr,
        PathPat,
        FieldExpr,
        RecordPat,
        RecordExpr,
        RecordExprField,
        RecordPatField,
        TupleStructPat,
    );
}

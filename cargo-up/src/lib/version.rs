use crate::{
    ra_ap_syntax::{
        ast::{self, Expr, NameOrNameRef, NameOwner},
        AstNode,
    },
    semver::{SemVerError, Version as SemverVersion},
    utils::{normalize, INTERNAL_ERR},
    Semantics, Upgrader,
};

use anyhow::Result;
use paste::paste;

use std::{collections::HashMap as Map, ops::Deref};

pub(crate) type Hook<T> = Box<dyn Fn(&mut Upgrader, &T, &Semantics)>;

pub(crate) struct Hooks<T>(Map<String, Map<String, Vec<Hook<T>>>>);

impl<T> Default for Hooks<T> {
    fn default() -> Self {
        Self(Map::new())
    }
}

impl<T> Deref for Hooks<T> {
    type Target = Map<String, Map<String, Vec<Hook<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Hooks<T> {
    pub(crate) fn insert(&mut self, path: &str, name: &str, hook: Hook<T>) {
        let path = path.to_string();
        let name = name.to_string();

        if !self.0.contains_key(&path) {
            self.0.insert(path.clone(), Map::new());
        }

        let hook_map = self.0.get_mut(&path).expect(INTERNAL_ERR);

        if !hook_map.contains_key(&name) {
            hook_map.insert(name.clone(), vec![]);
        }

        hook_map.get_mut(&name).expect(INTERNAL_ERR).push(hook);
    }
}

macro_rules! members {
    ($($node:ident,)*) => {
        paste! {
            pub struct Version {
                pub(crate) version: SemverVersion,
                pub(crate) peers: Vec<String>,
                pub(crate) init: Option<Box<dyn Fn(&mut Upgrader, &SemverVersion) -> Result<()>>>,
                $(
                    pub(crate) [<hook_ $node:snake>]: Vec<Hook<ast::$node>>,
                    pub(crate) [<hook_ $node:snake _on>]: Hooks<ast::$node>,
                )*
            }

            impl Version {
                pub fn new(version: &str) -> Result<Self, SemVerError> {
                    Ok(Self {
                        version: SemverVersion::parse(version)?,
                        peers: vec![],
                        init: None,
                        $(
                            [<hook_ $node:snake>]: Vec::new(),
                            [<hook_ $node:snake _on>]: Hooks::default(),
                        )*
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

                pub fn [<hook_ $node:snake _on>]<F>(mut self, path: &str, name: &str, f: F) -> Self
                where
                    F: Fn(&mut Upgrader, &ast::$node, &Semantics) + 'static
                {
                    self.[<hook_ $node:snake _on>].insert(path, name, Box::new(f));
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

    pub fn init<F>(mut self, init: F) -> Self
    where
        F: Fn(&mut Upgrader, &SemverVersion) -> Result<()> + 'static,
    {
        self.init = Some(Box::new(init));
        self
    }

    pub fn rename_structs(mut self, name: &str, map: &'static [[&str; 2]]) -> Self {
        for rename in map.into_iter() {
            self = self.hook_path_on(name, rename[0], move |u, n, _| {
                u.replace(n.segment(), rename[1]);
            })
        }

        self
    }

    pub fn rename_methods(mut self, name: &str, map: &'static [[&str; 2]]) -> Self {
        for rename in map.into_iter() {
            self = self
                .hook_method_call_expr_on(name, rename[0], move |u, n, _| {
                    u.replace(n.name_ref(), rename[1]);
                })
                .hook_path_expr_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                });
        }

        self
    }

    pub fn rename_members(mut self, name: &str, map: &'static [[&str; 2]]) -> Self {
        for rename in map.into_iter() {
            self = self
                .hook_field_expr_on(name, rename[0], move |u, n, _| {
                    u.replace(n.name_ref(), rename[1]);
                })
                .hook_record_pat_field_on(name, rename[0], move |u, n, _| match n.field_name() {
                    Some(NameOrNameRef::Name(_)) => {
                        u.replace(n.syntax().text_range(), format!("{}: {}", rename[1], n))
                    }
                    Some(NameOrNameRef::NameRef(name_ref)) => {
                        u.replace(name_ref.syntax().text_range(), rename[1])
                    }
                    _ => {}
                })
                .hook_record_expr_field_on(name, rename[0], move |u, n, _| {
                    if let Some(name_ref) = n.name_ref() {
                        u.replace(name_ref.syntax().text_range(), rename[1]);
                    } else if let Some(Expr::PathExpr(path_expr)) = n.expr() {
                        u.replace(
                            path_expr.syntax().text_range(),
                            format!("{}: {}", rename[1], rename[0]),
                        );
                    }
                });
        }

        self
    }

    pub fn rename_variants(mut self, name: &str, map: &'static [[&str; 2]]) -> Self {
        for rename in map.into_iter() {
            self = self
                .hook_path_expr_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                })
                .hook_path_pat_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                })
                .hook_record_pat_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                })
                .hook_record_expr_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                })
                .hook_tuple_struct_pat_on(name, rename[0], move |u, n, _| {
                    u.replace(n.path(), rename[1]);
                })
                .hook_ident_pat_on(name, rename[0], move |u, n, _| {
                    u.replace(n.name(), rename[1]);
                });
        }

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

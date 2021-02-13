use crate::{
    helpers::{get_name, get_name_from_name, get_name_from_name_ref, get_name_from_path},
    ra_ap_syntax::{
        ast::{self, NameOwner},
        AstNode,
    },
    semver::{SemVerError, Version as SemverVersion},
    utils::{normalize, Error, INTERNAL_ERR, TERM_ERR, TERM_OUT},
    Preloader, Semantics, Upgrader, Version,
};

use anyhow::Result as AnyResult;
use ra_ap_base_db::{FileId, SourceDatabaseExt};
use ra_ap_hir::{Adt, AssocItem, Crate, ModuleDef, PathResolution};
use ra_ap_ide_db::symbol_index::SymbolsDatabase;
use ra_ap_rust_analyzer::cli::load_cargo;
use ra_ap_text_edit::TextEdit;
use rust_visitor::{Options, Visitor};

use std::{
    collections::HashMap as Map,
    fs::{read_to_string, write},
    io::Result as IoResult,
    path::Path,
};

pub struct Runner {
    pub(crate) minimum: Option<SemverVersion>,
    pub(crate) versions: Vec<Version>,
    version: SemverVersion,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            minimum: None,
            versions: vec![],
            version: SemverVersion::parse("0.0.0").expect(INTERNAL_ERR),
        }
    }

    pub fn minimum(mut self, version: &str) -> Result<Self, SemVerError> {
        self.minimum = Some(SemverVersion::parse(version)?);
        Ok(self)
    }

    pub fn version(mut self, version: Version) -> Self {
        self.versions.push(version);
        self
    }
}

impl Runner {
    fn get_version(&self) -> Option<&Version> {
        self.versions.iter().find(|x| x.version == self.version)
    }
}

#[doc(hidden)]
pub fn run(
    root: &Path,
    dep: &str,
    mut runner: Runner,
    from: SemverVersion,
    to: SemverVersion,
) -> IoResult<()> {
    if let Some(min) = &runner.minimum {
        if from < *min {
            return Error::NotMinimum(dep.into(), min.to_string()).print_err();
        }
    }

    runner.version = to;
    let version = runner.get_version();

    let peers = if let Some(version) = version {
        let mut peers = vec![dep.to_string()];
        peers.extend(version.peers.clone());
        peers
    } else {
        return Error::NoChanges(dep.into(), runner.version.to_string()).print_out();
    };

    let (host, vfs) = load_cargo(root, true, false).unwrap();
    let db = host.raw_database();

    let mut changes = Map::<FileId, TextEdit>::new();
    let semantics = Semantics::new(db);

    let mut wrapper = RunnerWrapper::new(runner, semantics);

    // Run init hook
    if let Err(err) = wrapper.init(&from) {
        TERM_ERR.write_line(&format!("{:?}", err))?;
        return TERM_ERR.flush();
    };

    // Loop to find and eager load the dep we are upgrading
    for krate in Crate::all(db) {
        let file_id = krate.root_file(db);
        let source_root = db.source_root(db.file_source_root(file_id));

        if !source_root.is_library {
            continue;
        }

        if let Some(name) = krate.display_name(db) {
            if let Some(peer) = peers
                .iter()
                .find(|x| **x == normalize(&format!("{}", name)))
            {
                wrapper.preloader.load(peer, db, &krate);
            }
        }
    }

    // Actual loop to walk through the source code
    for source_root_id in db.local_roots().iter() {
        let source_root = db.source_root(*source_root_id);

        for file_id in source_root.iter() {
            let source_file = wrapper.semantics.parse(file_id);
            // println!("{:#?}", source_file.syntax());

            wrapper.walk(source_file.syntax());

            changes.insert(file_id, wrapper.upgrader.finish());
        }
    }

    // Apply changes
    for (file_id, edit) in changes {
        let full_path = vfs.file_path(file_id);
        let full_path = full_path.as_path().expect(INTERNAL_ERR);

        let mut file_text = read_to_string(&full_path)?;

        edit.apply(&mut file_text);
        write(&full_path, file_text)?;
    }

    // TODO: Modify Cargo.toml

    TERM_ERR.flush()?;
    TERM_OUT.flush()?;

    Ok(())
}

pub(crate) struct RunnerWrapper<'a> {
    runner: Runner,
    preloader: Preloader,
    upgrader: Upgrader,
    semantics: Semantics<'a>,
}

impl<'a> RunnerWrapper<'a> {
    fn new(runner: Runner, semantics: Semantics<'a>) -> Self {
        Self {
            runner,
            preloader: Preloader::default(),
            upgrader: Upgrader::default(),
            semantics,
        }
    }

    pub(crate) fn semantics(&self) -> &'a Semantics {
        &self.semantics
    }

    fn get_version(&self) -> Option<&Version> {
        self.runner.get_version()
    }

    fn init(&mut self, from: &SemverVersion) -> AnyResult<()> {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        if let Some(f) = &version.init {
            f(&mut self.upgrader, from)
        } else {
            Ok(())
        }
    }
}

impl<'a> Visitor for RunnerWrapper<'a> {
    fn visit_source_file(&mut self, _: &ast::SourceFile, _: &mut Options) {}

    // TODO: Remove the upgrader clone/copy hack once constrained mutable checks are implemented in rust
    fn visit_method_call_expr(&mut self, method_call_expr: &ast::MethodCallExpr, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_method_call_expr {
            hook(&mut upgrader, method_call_expr, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.methods,
            &version.hook_method_call_expr_on,
            method_call_expr,
            |n| get_name_from_name_ref(n.name_ref()),
            |s, n| s.resolve_method_call(n),
        );

        self.upgrader = upgrader;
    }

    fn visit_call_expr(&mut self, call_expr: &ast::CallExpr, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_call_expr {
            hook(&mut upgrader, call_expr, self.semantics());
        }

        self.upgrader = upgrader;
    }

    fn visit_ident_pat(&mut self, ident_pat: &ast::IdentPat, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_ident_pat {
            hook(&mut upgrader, ident_pat, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_ident_pat_on,
            ident_pat,
            |n| get_name_from_name(n.name()),
            |s, n| {
                if let Some(ModuleDef::EnumVariant(x)) = s.resolve_bind_pat_to_const(n) {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_path(&mut self, path: &ast::Path, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path {
            hook(&mut upgrader, path, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.structs,
            &version.hook_path_on,
            path,
            |n| get_name_from_path(Some(n.clone())),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::Adt(Adt::Struct(x)))) = s.resolve_path(n)
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_path_expr(&mut self, path_expr: &ast::PathExpr, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_expr {
            hook(&mut upgrader, path_expr, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.methods,
            &version.hook_path_expr_on,
            path_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::AssocItem(AssocItem::Function(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_path_expr_on,
            path_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::EnumVariant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_path_pat(&mut self, path_pat: &ast::PathPat, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_pat {
            hook(&mut upgrader, path_pat, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_path_pat_on,
            path_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::EnumVariant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_field_expr(&mut self, field_expr: &ast::FieldExpr, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_field_expr {
            hook(&mut upgrader, field_expr, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.members,
            &version.hook_field_expr_on,
            field_expr,
            |n| get_name_from_name_ref(n.name_ref()),
            |s, n| s.resolve_field(n),
        );

        self.upgrader = upgrader;
    }

    fn visit_record_pat(&mut self, record_pat: &ast::RecordPat, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat {
            hook(&mut upgrader, record_pat, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_record_pat_on,
            record_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::EnumVariant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_record_expr(&mut self, record_expr: &ast::RecordExpr, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr {
            hook(&mut upgrader, record_expr, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_record_expr_on,
            record_expr,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::EnumVariant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_record_expr_field(
        &mut self,
        record_expr_field: &ast::RecordExprField,
        _: &mut Options,
    ) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr_field {
            hook(&mut upgrader, record_expr_field, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.members,
            &version.hook_record_expr_field_on,
            record_expr_field,
            |n| get_name_from_name_ref(n.field_name()),
            |s, n| {
                if let Some((x, _)) = s.resolve_record_field(n) {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }

    fn visit_record_pat_field(&mut self, record_pat_field: &ast::RecordPatField, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat_field {
            hook(&mut upgrader, record_pat_field, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.members,
            &version.hook_record_pat_field_on,
            record_pat_field,
            |n| get_name(n.field_name()),
            |s, n| s.resolve_record_pat_field(n),
        );

        self.upgrader = upgrader;
    }

    fn visit_tuple_struct_pat(&mut self, tuple_struct_pat: &ast::TupleStructPat, _: &mut Options) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_tuple_struct_pat {
            hook(&mut upgrader, tuple_struct_pat, self.semantics());
        }

        self.run_hooks(
            &mut upgrader,
            &self.preloader.variants,
            &version.hook_tuple_struct_pat_on,
            tuple_struct_pat,
            |n| get_name_from_path(n.path()),
            |s, n| {
                if let Some(PathResolution::Def(ModuleDef::EnumVariant(x))) =
                    s.resolve_path(&n.path().expect(INTERNAL_ERR))
                {
                    Some(x)
                } else {
                    None
                }
            },
        );

        self.upgrader = upgrader;
    }
}

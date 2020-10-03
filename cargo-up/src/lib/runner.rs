use crate::{
    ra_ap_syntax::{
        ast::{self, NameOwner},
        AstNode,
    },
    semver::{SemVerError, Version as SemverVersion},
    utils::{normalize, Error, INTERNAL_ERR, TERM_ERR, TERM_OUT},
    Preloader, Semantics, Upgrader, Version,
};
use ra_ap_base_db::{FileId, SourceDatabaseExt};
use ra_ap_hir::{Adt, AssocItem, Crate, EnumVariant, ModuleDef, PathResolution};
use ra_ap_ide_db::symbol_index::SymbolsDatabase;
use ra_ap_rust_analyzer::cli::load_cargo;
use ra_ap_text_edit::TextEdit;
use rust_visitor::Visitor;
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
    let (host, vfs) = load_cargo(root, true, false).unwrap();
    let db = host.raw_database();

    let mut changes = Map::<FileId, TextEdit>::new();
    let semantics = Semantics::new(db);

    if let Some(min) = runner.minimum.clone() {
        if from < min {
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
        return Error::NoChanges(runner.version.to_string()).print_out();
    };

    let mut wrapper = RunnerWrapper::new(runner, semantics);

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

struct RunnerWrapper<'a> {
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

    fn semantics(&self) -> &'a Semantics {
        &self.semantics
    }

    fn get_version(&self) -> Option<&Version> {
        self.runner.get_version()
    }

    fn check_name_or_name_ref(
        name_or_name_ref: Option<ast::NameOrNameRef>,
        map: &Map<String, Map<String, String>>,
    ) -> Option<bool> {
        let name = match name_or_name_ref? {
            ast::NameOrNameRef::NameRef(name_ref) => name_ref.text().to_string(),
            ast::NameOrNameRef::Name(name) => name.text().to_string(),
        };

        Some(map.iter().any(|x| x.1.iter().any(|y| *y.0 == name)))
    }

    fn check_name_ref(
        name_ref: Option<ast::NameRef>,
        map: &Map<String, Map<String, String>>,
    ) -> Option<bool> {
        Self::check_name_or_name_ref(name_ref.map(|x| ast::NameOrNameRef::NameRef(x)), map)
    }

    fn check_path(path: Option<ast::Path>, map: &Map<String, Map<String, String>>) -> Option<bool> {
        Self::check_name_ref(path?.segment()?.name_ref(), map)
    }
}

impl<'a> Visitor for RunnerWrapper<'a> {
    fn visit_source_file(&mut self, _: &ast::SourceFile) {}

    fn visit_method_call_expr(&mut self, method_call_expr: &ast::MethodCallExpr) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_method_call_expr {
            hook(&mut upgrader, method_call_expr, self.semantics());
        }

        if let Some(true) =
            Self::check_name_ref(method_call_expr.name_ref(), &version.rename_methods)
        {
            if let Some(f) = self.semantics().resolve_method_call(method_call_expr) {
                if let Some((_, name)) = self.preloader.methods.iter().find(|x| *x.0 == f) {
                    if let Some(map) = version.rename_methods.get(name) {
                        let name_ref = method_call_expr.name_ref().expect(INTERNAL_ERR);
                        let method_name = name_ref.text().to_string();

                        if let Some(to) = map.get(&method_name) {
                            upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_call_expr(&mut self, call_expr: &ast::CallExpr) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_call_expr {
            hook(&mut upgrader, call_expr, self.semantics());
        }

        self.upgrader = upgrader;
    }

    fn visit_ident_pat(&mut self, ident_pat: &ast::IdentPat) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_ident_pat {
            hook(&mut upgrader, ident_pat, self.semantics());
        }

        if let Some(ModuleDef::EnumVariant(e)) =
            self.semantics().resolve_bind_pat_to_const(ident_pat)
        {
            if let Some((_, name)) = self.preloader.variants.iter().find(|x| *x.0 == e) {
                if let Some(map) = version.rename_variants.get(name) {
                    let name_ref = ident_pat.name().expect(INTERNAL_ERR);
                    let variant_name = name_ref.text().to_string();

                    if let Some(to) = map.get(&variant_name) {
                        upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_path(&mut self, path: &ast::Path) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path {
            hook(&mut upgrader, path, self.semantics());
        }

        if let Some(true) = Self::check_path(Some(path.clone()), &version.rename_structs) {
            if let Some(PathResolution::Def(ModuleDef::Adt(Adt::Struct(s)))) =
                self.semantics().resolve_path(path)
            {
                if let Some((_, name)) = self.preloader.structs.iter().find(|x| *x.0 == s) {
                    if let Some(map) = version.rename_structs.get(name) {
                        let name_ref = path
                            .segment()
                            .expect(INTERNAL_ERR)
                            .name_ref()
                            .expect(INTERNAL_ERR);
                        let struct_name = name_ref.text().to_string();

                        if let Some(to) = map.get(&struct_name) {
                            upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_path_expr(&mut self, path_expr: &ast::PathExpr) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_expr {
            hook(&mut upgrader, path_expr, self.semantics());
        }

        if let Some(true) = Self::check_path(path_expr.path(), &version.rename_methods) {
            let path = path_expr.path().expect(INTERNAL_ERR);

            if let Some(PathResolution::AssocItem(AssocItem::Function(f))) =
                self.semantics().resolve_path(&path)
            {
                if let Some((_, name)) = self.preloader.methods.iter().find(|x| *x.0 == f) {
                    if let Some(map) = version.rename_methods.get(name) {
                        let name_ref = path
                            .segment()
                            .expect(INTERNAL_ERR)
                            .name_ref()
                            .expect(INTERNAL_ERR);
                        let method_name = name_ref.text().to_string();

                        if let Some(to) = map.get(&method_name) {
                            upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                        }
                    }
                }
            }
        } else {
            rename_variants(
                path_expr.path(),
                &self.preloader.variants,
                &version.rename_variants,
                self.semantics(),
                &mut upgrader,
            );
        }

        self.upgrader = upgrader;
    }

    fn visit_path_pat(&mut self, path_pat: &ast::PathPat) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_path_pat {
            hook(&mut upgrader, path_pat, self.semantics());
        }

        rename_variants(
            path_pat.path(),
            &self.preloader.variants,
            &version.rename_variants,
            self.semantics(),
            &mut upgrader,
        );

        self.upgrader = upgrader;
    }

    fn visit_field_expr(&mut self, field_expr: &ast::FieldExpr) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_field_expr {
            hook(&mut upgrader, field_expr, self.semantics());
        }

        if let Some(true) = Self::check_name_ref(field_expr.name_ref(), &version.rename_members) {
            if let Some(f) = self.semantics().resolve_field(field_expr) {
                if let Some((_, name)) = self.preloader.members.iter().find(|x| *x.0 == f) {
                    if let Some(map) = version.rename_members.get(name) {
                        let name_ref = field_expr.name_ref().expect(INTERNAL_ERR);
                        let member_name = name_ref.text().to_string();

                        if let Some(to) = map.get(&member_name) {
                            upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_record_pat(&mut self, record_pat: &ast::RecordPat) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat {
            hook(&mut upgrader, record_pat, self.semantics());
        }

        rename_variants(
            record_pat.path(),
            &self.preloader.variants,
            &version.rename_variants,
            self.semantics(),
            &mut upgrader,
        );

        self.upgrader = upgrader;
    }

    fn visit_record_expr(&mut self, record_expr: &ast::RecordExpr) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr {
            hook(&mut upgrader, record_expr, self.semantics());
        }

        rename_variants(
            record_expr.path(),
            &self.preloader.variants,
            &version.rename_variants,
            self.semantics(),
            &mut upgrader,
        );

        self.upgrader = upgrader;
    }

    fn visit_record_expr_field(&mut self, record_expr_field: &ast::RecordExprField) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_expr_field {
            hook(&mut upgrader, record_expr_field, self.semantics());
        }

        if let Some(true) =
            Self::check_name_ref(record_expr_field.field_name(), &version.rename_members)
        {
            if let Some(f) = self.semantics().resolve_record_field(record_expr_field) {
                if let Some((_, name)) = self.preloader.members.iter().find(|x| *x.0 == f.0) {
                    if let Some(map) = version.rename_members.get(name) {
                        if let Some(name_ref) = record_expr_field.name_ref() {
                            let member_name = name_ref.text().to_string();

                            if let Some(to) = map.get(&member_name) {
                                upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                            }
                        } else if let Some(ast::Expr::PathExpr(path_expr)) =
                            record_expr_field.expr()
                        {
                            let member_name = path_expr
                                .path()
                                .expect(INTERNAL_ERR)
                                .segment()
                                .expect(INTERNAL_ERR)
                                .name_ref()
                                .expect(INTERNAL_ERR)
                                .text()
                                .to_string();

                            if let Some(to) = map.get(&member_name) {
                                upgrader.replace(
                                    path_expr.syntax().text_range(),
                                    format!("{}: {}", to, member_name),
                                );
                            }
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_record_pat_field(&mut self, record_pat_field: &ast::RecordPatField) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_record_pat_field {
            hook(&mut upgrader, record_pat_field, self.semantics());
        }

        if let Some(true) =
            Self::check_name_or_name_ref(record_pat_field.field_name(), &version.rename_members)
        {
            if let Some(f) = self.semantics().resolve_record_pat_field(record_pat_field) {
                if let Some((_, name)) = self.preloader.members.iter().find(|x| *x.0 == f) {
                    if let Some(map) = version.rename_members.get(name) {
                        match record_pat_field.field_name() {
                            Some(ast::NameOrNameRef::Name(name)) => {
                                let member_name = name.text().to_string();

                                if let Some(to) = map.get(&member_name) {
                                    upgrader.replace(
                                        record_pat_field.syntax().text_range(),
                                        format!("{}: {}", to, record_pat_field),
                                    );
                                }
                            }
                            Some(ast::NameOrNameRef::NameRef(name_ref)) => {
                                let member_name = name_ref.text().to_string();

                                if let Some(to) = map.get(&member_name) {
                                    upgrader
                                        .replace(name_ref.syntax().text_range(), to.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }

    fn visit_tuple_struct_pat(&mut self, tuple_struct_pat: &ast::TupleStructPat) {
        let mut upgrader = self.upgrader.clone();
        let version = self.get_version().expect(INTERNAL_ERR);

        for hook in &version.hook_tuple_struct_pat {
            hook(&mut upgrader, tuple_struct_pat, self.semantics());
        }

        rename_variants(
            tuple_struct_pat.path(),
            &self.preloader.variants,
            &version.rename_variants,
            self.semantics(),
            &mut upgrader,
        );

        self.upgrader = upgrader;
    }
}

// TODO: Maybe return Option<bool>
fn rename_variants(
    path: Option<ast::Path>,
    variants: &Map<EnumVariant, String>,
    map: &Map<String, Map<String, String>>,
    semantics: &Semantics,
    upgrader: &mut Upgrader,
) {
    if let Some(true) = RunnerWrapper::check_path(path.clone(), map) {
        let path = path.expect(INTERNAL_ERR);

        if let Some(PathResolution::Def(ModuleDef::EnumVariant(e))) = semantics.resolve_path(&path)
        {
            if let Some((_, name)) = variants.iter().find(|x| *x.0 == e) {
                if let Some(map) = map.get(name) {
                    let name_ref = path
                        .segment()
                        .expect(INTERNAL_ERR)
                        .name_ref()
                        .expect(INTERNAL_ERR);
                    let variant_name = name_ref.text().to_string();

                    if let Some(to) = map.get(&variant_name) {
                        upgrader.replace(name_ref.syntax().text_range(), to.to_string());
                    }
                }
            }
        }
    }
}

use crate::{
    ra_hir::{Adt, AsAssocItem, AssocItemContainer, Function, Module, Name, Semantics},
    ra_ide_db::{symbol_index::SymbolsDatabase, RootDatabase},
    ra_syntax::{ast, AstNode},
    semver::{SemVerError, Version as SemverVersion},
    Upgrader, Version, Visitor,
};
use ra_db::{SourceDatabase, SourceDatabaseExt};
use ra_text_edit::TextEdit;
use rust_analyzer::cli::load_cargo;
use std::{collections::BTreeMap as Map, path::Path};

pub struct Runner {
    pub(crate) minimum: Option<SemverVersion>,
    pub(crate) peers: Vec<String>,
    pub(crate) versions: Vec<Version>,
    upgrader: Upgrader,
    version: Option<SemverVersion>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            minimum: None,
            peers: vec![],
            versions: vec![],
            upgrader: Upgrader::default(),
            version: None,
        }
    }

    pub fn minimum(mut self, version: &str) -> Result<Self, SemVerError> {
        self.minimum = Some(SemverVersion::parse(version)?);
        Ok(self)
    }

    pub fn peers(mut self, peers: &[&str]) -> Self {
        self.peers = peers.to_vec().iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.versions.push(version);
        self
    }
}

impl Runner {
    #[doc(hidden)]
    pub fn run(&mut self, root: &Path, version: SemverVersion) {
        let (host, source_roots) = load_cargo(root, true, false).unwrap();
        let db = host.raw_database();

        let mut changes = Map::<String, TextEdit>::new();
        let semantics = Semantics::new(db);

        // TODO: Check for minimum version
        self.version = Some(version);

        // TODO: Allow other deps to be loaded too.
        // For example, if 2 crates are being combined into one.

        // Loop to find and eager load the dep we are upgrading
        for (source_root_id, project_root) in source_roots.iter() {
            if project_root.is_member() {
                continue;
            }

            let crate_ids = db.source_root_crates(*source_root_id);

            for crate_id in crate_ids.iter() {
                let crate_data = &db.crate_graph()[*crate_id];

                if let Some(name) = &crate_data.display_name {
                    // TODO: Store references from this dep so it's easy to compare
                    println!("{}", name);
                }
            }
        }

        // Actual loop to walk through the source code
        for source_root_id in db.local_roots().iter() {
            let source_root = db.source_root(*source_root_id);

            for file_id in source_root.walk() {
                let source_file = semantics.parse(file_id);

                self.visit(source_file.syntax(), &semantics);

                changes.insert(
                    db.file_relative_path(file_id).as_str().to_string(),
                    self.upgrader.finish(),
                );
            }
        }

        // Apply chnages
        // TODO:
        println!("{:#?}", changes);
    }

    fn get_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .find(|x| x.version == *self.version.as_ref().unwrap())
    }
}

impl Visitor for Runner {
    fn visit_source_file(&mut self, _: &ast::SourceFile, _: &Semantics<RootDatabase>) {}

    fn visit_method_call_expr(
        &mut self,
        method_call_expr: &ast::MethodCallExpr,
        semantics: &Semantics<RootDatabase>,
    ) {
        let mut upgrader = self.upgrader.clone();

        if let Some(version) = self.get_version() {
            for hook in &version.hook_method_call_expr {
                hook(&mut upgrader, method_call_expr, semantics);
            }

            if let Some(name_ref) = method_call_expr.name_ref() {
                let method = name_ref.text().to_string();

                // Filter out methods which don't have the same names we are looking for
                if !version
                    .rename_methods
                    .iter()
                    .any(|x| x.1.iter().any(|y| *y.0 == method))
                {
                    return;
                }

                // TODO: Compare with eager loaded methods

                let f = semantics.resolve_method_call(method_call_expr).unwrap();

                if let Some(name) = get_struct_name(&f, semantics.db) {
                    let mod_name = full_name(&f.module(semantics.db), semantics.db);

                    if let Some(map) = version
                        .rename_methods
                        .get(&format!("{}::{}", mod_name, name))
                    {
                        if let Some(to) = map.get(&method) {
                            upgrader.replace(
                                method_call_expr.name_ref().unwrap().syntax().text_range(),
                                to.to_string(),
                            );
                        }
                    }
                }
            }
        }

        self.upgrader = upgrader;
    }
}

fn get_struct_name(f: &Function, db: &RootDatabase) -> Option<Name> {
    let assoc_item = f.clone().as_assoc_item(db)?;

    if let AssocItemContainer::ImplDef(impl_def) = assoc_item.container(db) {
        if let Adt::Struct(s) = impl_def.target_ty(db).as_adt()? {
            return Some(s.name(db));
        }
    }

    None
}

fn full_name(m: &Module, db: &RootDatabase) -> String {
    let mut ret = vec![];
    let mut module = *m;

    loop {
        if let Some(name) = module.name(db) {
            ret.push(format!("{}", name));

            if let Some(p) = module.parent(db) {
                module = p;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if let Some(name) = m.krate().display_name(db) {
        ret.push(format!("{}", name));
    }

    ret.reverse();
    ret.join("::")
}

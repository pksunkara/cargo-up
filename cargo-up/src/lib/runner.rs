use crate::{
    ra_hir::Semantics, ra_ide_db::symbol_index::SymbolsDatabase, ra_syntax::AstNode,
    semver::Version, UpgradeVisitor, Upgrader, Visitor,
};
use ra_db::{SourceDatabase, SourceDatabaseExt};
use ra_text_edit::TextEdit;
use rust_analyzer::cli::load_cargo;
use std::panic::RefUnwindSafe;
use std::{collections::BTreeMap as Map, marker::PhantomData, path::Path};

pub struct Runner<T>(PhantomData<T>)
where
    T: UpgradeVisitor;

impl<T> Runner<T>
where
    T: UpgradeVisitor,
{
    pub fn run<F>(upgrader: F, root: &Path, version: Version)
    where
        F: Fn() -> Upgrader<T> + RefUnwindSafe,
    {
        let (host, source_roots) = load_cargo(root, true, false).unwrap();
        let analysis = host.analysis();

        analysis
            .with_db(|db| {
                let mut changes = Map::<String, TextEdit>::new();
                let mut upgrader = upgrader();
                let semantics = Semantics::new(db);

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

                        upgrader.visit(source_file.syntax(), &semantics);

                        changes.insert(
                            db.file_relative_path(file_id).as_str().to_string(),
                            upgrader.editor.finish(),
                        );
                    }
                }

                // Apply chnages
                // TODO:
                println!("{:#?}", changes);
            })
            .unwrap();
    }
}

use crate::{
    ra_hir::Semantics, ra_ide_db::symbol_index::SymbolsDatabase, ra_syntax::AstNode,
    ra_text_edit::TextEdit, Upgrader, Visitor,
};
use ra_db::{SourceDatabase, SourceDatabaseExt};
use rust_analyzer::cli::load_cargo;
use std::collections::BTreeMap as Map;
use std::{marker::PhantomData, path::Path};

#[derive(Default)]
pub struct Runner<T>(PhantomData<T>)
where
    T: Upgrader + Visitor + Default;

impl<T> Runner<T>
where
    T: Upgrader + Visitor + Default,
{
    pub fn run(&self, root: &Path) {
        let (host, source_roots) = load_cargo(root, true, false).unwrap();
        let analysis = host.analysis();

        analysis
            .with_db(|db| {
                let mut changes = Map::<String, TextEdit>::new();
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
                            // TODO:
                            println!("{}", name);
                        }
                    }
                }

                // Actual loop to walk through the source code
                for source_root_id in db.local_roots().iter() {
                    let source_root = db.source_root(*source_root_id);

                    for file_id in source_root.walk() {
                        let mut upgrader = T::default();
                        let source_file = semantics.parse(file_id);

                        upgrader.visit(source_file.syntax(), &semantics);

                        changes.insert(
                            db.file_relative_path(file_id).as_str().to_string(),
                            upgrader.finish(),
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

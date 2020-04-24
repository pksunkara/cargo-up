use ra_db::{SourceDatabase, SourceDatabaseExt};
use ra_hir::Semantics;
use ra_ide_db::symbol_index::SymbolsDatabase;
use ra_syntax::ast::AstNode;
use rust_analyzer::cli::load_cargo;
use std::path::Path;

fn main() {
    env_logger::init();

    let (host, roots) = load_cargo(Path::new("."), true, false).unwrap();
    let db = host.raw_database();
    let analysis = host.analysis();
    let semantics = Semantics::new(db);

    analysis
        .with_db(|db| {
            for &root in db.local_roots().iter() {
                let sr = db.source_root(root);
                println!("{:#?}", sr);

                for file_id in sr.walk() {
                    let tree = db.parse(file_id).tree();
                    let syntax = tree.syntax();

                    println!("{:#?}", syntax);
                }
            }

            for &root in db.library_roots().iter() {
                let sr = db.source_root(root);
                println!("{:#?}", sr);
            }
        })
        .unwrap();
}

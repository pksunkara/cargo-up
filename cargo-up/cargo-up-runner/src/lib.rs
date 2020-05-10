// use ra_db::SourceDatabaseExt;
// use ra_ide_db::symbol_index::SymbolsDatabase;
// use rust_analyzer::cli::load_cargo;
// use std::path::Path;

// pub fn run() {
//     let (host, _) = load_cargo(Path::new("."), true, false).unwrap();
//     let analysis = host.analysis();

//     analysis
//         .with_db(|db| {
//             let mut updater = Up::default();
//             let semantics = Semantics::new(db);

//             for &root in db.local_roots().iter() {
//                 let sr = db.source_root(root);

//                 for file_id in sr.walk() {
//                     let source_file = semantics.parse(file_id);

//                     updater.edit = TextEditBuilder::default();
//                     updater.visit(source_file.syntax(), &semantics);

//                     let mut source = source_file.to_string();
//                     updater.edit.finish().apply(&mut source);

//                     println!("{}", source);
//                 }
//             }
//         })
//         .unwrap();
// }

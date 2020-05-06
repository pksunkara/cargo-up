use ra_db::SourceDatabaseExt;
use ra_hir::{Adt, AsAssocItem, AssocItemContainer, Function, Module, Name, Semantics};
use ra_ide_db::{symbol_index::SymbolsDatabase, RootDatabase};
use ra_syntax::{
    ast::{self, AstNode},
    SyntaxKind, SyntaxNode,
};
use ra_text_edit::{Indel, TextEdit, TextEditBuilder};
use rust_analyzer::cli::load_cargo;
use std::path::Path;

pub trait Visitor<RootDatabase> {
    fn visit(&mut self, node: &SyntaxNode, semantics: &Semantics<RootDatabase>) {
        match node.kind() {
            SyntaxKind::SOURCE_FILE => {
                self.visit_source_file(ast::SourceFile::cast(node.clone()).unwrap(), &semantics)
            }
            SyntaxKind::METHOD_CALL_EXPR => self.visit_method_call_expr(
                ast::MethodCallExpr::cast(node.clone()).unwrap(),
                &semantics,
            ),
            _ => {}
        };

        for child in node.children() {
            self.visit(&child, &semantics);
        }
    }

    fn visit_source_file(&mut self, _: ast::SourceFile, _: &Semantics<RootDatabase>) {}
    fn visit_method_call_expr(&mut self, _: ast::MethodCallExpr, _: &Semantics<RootDatabase>) {}
}

#[derive(Default, Debug)]
pub struct Up {
    pub edit: TextEditBuilder,
}

impl Visitor<RootDatabase> for Up {
    fn visit_method_call_expr(
        &mut self,
        method_call_expr: ast::MethodCallExpr,
        semantics: &Semantics<RootDatabase>,
    ) {
        let f = semantics.resolve_method_call(&method_call_expr).unwrap();
        let fn_name = format!("{}", f.name(semantics.db));

        if let Some(name) = get_struct_name(&f, semantics.db) {
            let mod_name = full_name(&f.module(semantics.db), semantics.db);

            if format!("{}", name) == "Arg" && mod_name == "clap::args::arg" && fn_name == "help" {
                self.edit.replace(
                    method_call_expr.name_ref().unwrap().syntax().text_range(),
                    "about".to_string(),
                );
            }
        }
    }
}

fn get_struct_name(f: &Function, db: &RootDatabase) -> Option<Name> {
    let assoc_item = f.clone().as_assoc_item(db)?;

    if let AssocItemContainer::ImplDef(impl_def) = assoc_item.container(db) {
        if let Some(Adt::Struct(s)) = impl_def.target_ty(db).as_adt() {
            return Some(s.name(db));
        }
    }

    None
}

fn full_name(m: &Module, db: &RootDatabase) -> String {
    let mut ret = vec![];
    let mut module = m.clone();

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

fn main() {
    env_logger::init();

    let (host, _) = load_cargo(Path::new("."), true, false).unwrap();
    let analysis = host.analysis();

    analysis
        .with_db(|db| {
            let mut updater = Up::default();
            let semantics = Semantics::new(db);

            for &root in db.local_roots().iter() {
                let sr = db.source_root(root);

                for file_id in sr.walk() {
                    let source_file = semantics.parse(file_id);

                    updater.edit = TextEditBuilder::default();
                    updater.visit(source_file.syntax(), &semantics);

                    let mut source = source_file.to_string();
                    updater.edit.finish().apply(&mut source);

                    println!("{}", source);
                }
            }
        })
        .unwrap();
}

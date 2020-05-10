use cargo_up::{
    ra_hir::{Adt, AsAssocItem, AssocItemContainer, Function, Module, Name, Semantics},
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    upgrader, Upgrader, Visitor,
};

#[upgrader]
pub struct CargoUp;

impl Visitor for CargoUp {
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
                self.replace(
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

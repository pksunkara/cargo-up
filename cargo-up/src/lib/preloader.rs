use crate::{
    ra_ap_hir::{Adt, AssocItem, Crate, Field, Function, Module, ModuleDef, ScopeDef},
    ra_ap_ide_db::RootDatabase,
    utils::TERM_ERR,
};
use std::collections::HashMap as Map;

#[derive(Debug, Default)]
pub(crate) struct Preloader {
    pub(crate) methods: Map<Function, String>,
    pub(crate) members: Map<Field, String>,
    pub(crate) visited: Vec<String>,
}

impl Preloader {
    pub(crate) fn load(&mut self, name: &str, db: &RootDatabase, krate: &Crate) {
        if self.visited.iter().any(|x| x == name) {
            return;
        }

        eprint!("Pre loading {} ... ", name);
        TERM_ERR.flush().unwrap();

        let module = krate.root_module(db);
        self.load_module(db, &module, vec![name.to_string()]);

        self.visited.push(name.to_string());
        eprintln!("done");
    }

    fn load_module(&mut self, db: &RootDatabase, module: &Module, path: Vec<String>) {
        for item in module.scope(db, None) {
            match item.1 {
                // Load struct members
                ScopeDef::ModuleDef(ModuleDef::Adt(Adt::Struct(s))) => {
                    let name = format!("{}::{}", path.join("::"), s.name(db));

                    for field in s.fields(db) {
                        self.members.insert(field, name.clone());
                    }
                }
                // Load union memebrs
                ScopeDef::ModuleDef(ModuleDef::Adt(Adt::Union(u))) => {
                    let name = format!("{}::{}", path.join("::"), u.name(db));

                    for field in u.fields(db) {
                        self.members.insert(field, name.clone());
                    }
                }
                _ => {}
            }
        }

        for impl_def in module.impl_defs(db) {
            let target_ty = impl_def.target_ty(db);
            let target_trait = impl_def.target_trait(db);

            match target_ty.as_adt() {
                // Load struct instance methods
                Some(Adt::Struct(s)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), s.name(db));

                    for assoc_item in impl_def.items(db) {
                        if let AssocItem::Function(f) = assoc_item {
                            self.methods.insert(f, name.clone());
                        }
                    }
                }
                // Load enum instance methods
                Some(Adt::Enum(e)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), e.name(db));

                    for assoc_item in impl_def.items(db) {
                        if let AssocItem::Function(f) = assoc_item {
                            self.methods.insert(f, name.clone());
                        }
                    }
                }
                // Load union instance methods
                Some(Adt::Union(u)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), u.name(db));

                    for assoc_item in impl_def.items(db) {
                        if let AssocItem::Function(f) = assoc_item {
                            self.methods.insert(f, name.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        for child in module.children(db) {
            if let Some(name) = child.name(db) {
                let mut path = path.clone();
                path.push(format!("{}", name));

                self.load_module(db, &child, path);
            }
        }
    }
}

use log::trace;
use oclif::term::TERM_ERR;
use ra_ap_hir::{Adt, AssocItem, Crate, Field, Function, Module, ModuleDef, Struct, Variant};
use ra_ap_ide_db::RootDatabase;

use std::collections::HashMap as Map;

#[derive(Debug, Default)]
pub(crate) struct Preloader {
    pub(crate) methods: Map<Function, String>,
    pub(crate) structs: Map<Struct, String>,
    pub(crate) members: Map<Field, String>,
    pub(crate) variants: Map<Variant, String>,
    pub(crate) visited: Vec<String>,
}

impl Preloader {
    pub(crate) fn load(&mut self, name: &str, db: &RootDatabase, krate: &Crate) {
        if self.visited.iter().any(|x| x == name) {
            return;
        }

        eprintln!("Preloading {} ... ", name);
        TERM_ERR.flush().unwrap();

        let module = krate.root_module();
        self.load_module(db, &module, vec![name.to_string()]);

        self.visited.push(name.to_string());
        eprintln!("Preloading done");

        trace!("{:#?}", self);
    }

    fn load_module(&mut self, db: &RootDatabase, module: &Module, path: Vec<String>) {
        for def in module.declarations(db) {
            match def {
                // Load struct && members
                ModuleDef::Adt(Adt::Struct(s)) => {
                    self.structs.insert(s.clone(), path.join("::"));

                    let name = format!("{}::{}", path.join("::"), s.name(db).display(db));

                    for field in s.fields(db) {
                        self.members.insert(field, name.clone());
                    }
                }
                // Load union memebrs
                ModuleDef::Adt(Adt::Union(u)) => {
                    let name = format!("{}::{}", path.join("::"), u.name(db).display(db));

                    for field in u.fields(db) {
                        self.members.insert(field, name.clone());
                    }
                }
                // Load enum variants
                ModuleDef::Adt(Adt::Enum(e)) => {
                    let name = format!("{}::{}", path.join("::"), e.name(db).display(db));

                    for variant in e.variants(db) {
                        self.variants.insert(variant, name.clone());
                    }
                }
                _ => {}
            }
        }

        for impl_def in module.impl_defs(db) {
            let target_ty = impl_def.self_ty(db);
            let target_trait = impl_def.trait_(db);

            match target_ty.as_adt() {
                // Load struct instance methods
                Some(Adt::Struct(s)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), s.name(db).display(db));

                    for assoc_item in impl_def.items(db) {
                        if let AssocItem::Function(f) = assoc_item {
                            self.methods.insert(f, name.clone());
                        }
                    }
                }
                // Load enum instance methods
                Some(Adt::Enum(e)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), e.name(db).display(db));

                    for assoc_item in impl_def.items(db) {
                        if let AssocItem::Function(f) = assoc_item {
                            self.methods.insert(f, name.clone());
                        }
                    }
                }
                // Load union instance methods
                Some(Adt::Union(u)) if target_trait.is_none() => {
                    let name = format!("{}::{}", path.join("::"), u.name(db).display(db));

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
                path.push(format!("{}", name.display(db)));

                self.load_module(db, &child, path);
            }
        }
    }
}

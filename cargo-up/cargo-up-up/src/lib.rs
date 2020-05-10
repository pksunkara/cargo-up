use cargo_up::{
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    upgrader, Upgrader, Visitor,
};

#[upgrader]
pub struct CargoUp;

impl Visitor for CargoUp {}

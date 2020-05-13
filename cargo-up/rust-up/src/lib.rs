use cargo_up::{
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    upgrader, Upgrader, Visitor,
};

#[upgrader]
#[derive(Default)]
pub struct Rust;

impl Visitor for Rust {}

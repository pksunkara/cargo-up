use cargo_up::{
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    UpgradeVisitor, Upgrader,
};

#[derive(Default)]
pub struct CargoUp;

impl UpgradeVisitor for CargoUp {}

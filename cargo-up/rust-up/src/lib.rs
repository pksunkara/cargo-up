use cargo_up::{
    ra_hir::Semantics,
    ra_ide_db::RootDatabase,
    ra_syntax::ast::{self, AstNode},
    Runner, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new().minimum("1.40.0").unwrap()
}

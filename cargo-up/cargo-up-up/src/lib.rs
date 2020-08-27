use cargo_up::{
    ra_ap_hir::Semantics,
    ra_ap_ide_db::RootDatabase,
    ra_ap_syntax::ast::{self, AstNode},
    Runner, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new().minimum("0.1.1").unwrap()
}

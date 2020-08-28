use cargo_up::{
    ra_ap_syntax::ast::{self, AstNode},
    Runner, Semantics, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new().minimum("0.1.1").unwrap()
}

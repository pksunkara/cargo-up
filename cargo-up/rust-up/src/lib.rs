use cargo_up::{
    ra_ap_syntax::ast::{self, AstNode},
    Runner, Semantics, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new().minimum("1.40.0").unwrap()
}

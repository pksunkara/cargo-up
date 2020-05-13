use console::Term;
use lazy_static::lazy_static;

mod cargo;
mod error;

pub use cargo::cargo;
pub use error::Error;

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
}

pub type Result<T = ()> = std::result::Result<T, Error>;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

#[inline]
pub fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

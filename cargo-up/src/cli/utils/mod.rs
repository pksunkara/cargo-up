mod cargo;
mod error;
pub mod term;

pub use cargo::cargo;
pub use error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

#[inline]
pub fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

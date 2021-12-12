mod cargo;
pub mod crates;
mod error;

pub use cargo::cargo;
pub use error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/automa-app/cargo-up";

#[inline]
pub fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

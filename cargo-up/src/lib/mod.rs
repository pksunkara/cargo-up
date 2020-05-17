pub use ra_hir;
pub use ra_ide_db;
pub use ra_syntax;
pub use semver;

// pub use cargo_up_derive::{self, *};

const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

mod preloader;
mod runner;
mod upgrader;
mod version;
mod visitor;

pub use runner::Runner;
pub use upgrader::Upgrader;
pub use version::Version;

use preloader::Preloader;
use visitor::Visitor;

#[inline]
pub fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

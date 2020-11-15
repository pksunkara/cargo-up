#![cfg(feature = "lib")]

pub use anyhow;
pub use rust_visitor::ra_ap_syntax;
pub use semver;

pub type Semantics<'db> = ra_ap_hir::Semantics<'db, ra_ap_ide_db::RootDatabase>;

mod preloader;
mod runner;
mod upgrader;
mod utils;
mod version;

pub use runner::{run, Runner};
pub use upgrader::Upgrader;
pub use version::Version;

use preloader::Preloader;

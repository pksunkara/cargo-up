pub use ra_hir;
pub use ra_ide_db;
pub use ra_syntax;
pub use semver;

// pub use cargo_up_derive::{self, *};

mod runner;
mod upgrader;
mod visitor;

#[doc(hidden)]
pub use runner::Runner;
pub use upgrader::{Editor, UpgradeVisitor, Upgrader};

use visitor::Visitor;

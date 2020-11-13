use crate::utils::term::{RED_BOLD, TERM_ERR, YELLOW};

use console::Term;
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to find package {id} in your dependencies")]
    PackageNotFound { id: String },
    #[error("no upgrader {upgrader} found on crates.io for dependency {id}")]
    NoUpgrader { id: String, upgrader: String },
    #[error("no crate {dep} found on crates.io")]
    NoDependency { dep: String },
    #[error("malformed version info from crates.io")]
    BadRegistry,

    #[error("unable to find CARGO_HOME dir")]
    NoCargoHome,
    #[error("unable to run cargo command with args {args:?}, got {err}")]
    Cargo { err: io::Error, args: Vec<String> },

    #[error("unable to build the runner, please file an issue with the {upgrader}")]
    Building { upgrader: String },
    #[error("unable to execute the built upgrader command, got {err}")]
    Runner { err: io::Error },
    #[error("unable to upgrade your codebase, please file an issue with the {upgrader}")]
    Upgrading { upgrader: String },

    #[error("{0}")]
    Semver(#[from] semver::ReqParseError),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("cannot convert command output to string, {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

// From https://github.com/pksunkara/cargo-workspaces/blob/master/cargo-workspaces/src/utils/error.rs
impl Error {
    pub fn print_err(self) -> io::Result<()> {
        self.print(&TERM_ERR)
    }

    fn color(self) -> Self {
        match self {
            Self::PackageNotFound { id } => Self::PackageNotFound {
                id: YELLOW.apply_to(id).to_string(),
            },
            Self::NoUpgrader { id, upgrader } => Self::NoUpgrader {
                id: YELLOW.apply_to(id).to_string(),
                upgrader: YELLOW.apply_to(upgrader).to_string(),
            },
            Self::NoDependency { dep } => Self::NoDependency {
                dep: YELLOW.apply_to(dep).to_string(),
            },
            Self::Building { upgrader } => Self::Building {
                upgrader: YELLOW.apply_to(upgrader).to_string(),
            },
            Self::Upgrading { upgrader } => Self::Upgrading {
                upgrader: YELLOW.apply_to(upgrader).to_string(),
            },
            _ => self,
        }
    }

    pub fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error").to_string()))?;
        term.write_line(&self.color().to_string())?;
        term.flush()
    }
}

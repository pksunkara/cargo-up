use oclif::{term::ERR_YELLOW, CliError};
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to find crate {dep} in your dependencies")]
    PackageNotFound { dep: String },
    #[error("no upgrader {upgrader} found on crates.io for dependency {dep}")]
    NoUpgrader { dep: String, upgrader: String },
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
    #[error("unable to upgrade your codebase, please file an issue with {upgrader} if this is unexpected")]
    Upgrading { upgrader: String },

    #[error("{0}")]
    Semver(#[from] semver::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("cannot convert command output to string, {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl CliError for Error {
    fn color(self) -> Self {
        match self {
            Self::PackageNotFound { dep } => Self::PackageNotFound {
                dep: ERR_YELLOW.apply_to(dep).to_string(),
            },
            Self::NoUpgrader { dep, upgrader } => Self::NoUpgrader {
                dep: ERR_YELLOW.apply_to(dep).to_string(),
                upgrader: ERR_YELLOW.apply_to(upgrader).to_string(),
            },
            Self::NoDependency { dep } => Self::NoDependency {
                dep: ERR_YELLOW.apply_to(dep).to_string(),
            },
            Self::Building { upgrader } => Self::Building {
                upgrader: ERR_YELLOW.apply_to(upgrader).to_string(),
            },
            Self::Upgrading { upgrader } => Self::Upgrading {
                upgrader: ERR_YELLOW.apply_to(upgrader).to_string(),
            },
            _ => self,
        }
    }
}

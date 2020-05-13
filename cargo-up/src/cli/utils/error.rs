use crate::utils::TERM_ERR;
use console::{Style, Term};
use lazy_static::lazy_static;
use std::io;
use thiserror::Error;

lazy_static! {
    static ref YELLOW: Style = Style::new().for_stderr().yellow();
    static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to find package {id}")]
    PackageNotFound { id: String },
    #[error("no upgrader {upgrader} found on crates.io for dependency {id}")]
    NoUpgrader { id: String, upgrader: String },

    #[error("unable to find CARGO_HOME dir")]
    NoCargoHome,
    #[error("unable to run cargo command with args {args:?}, got {err}")]
    Cargo { err: io::Error, args: Vec<String> },

    #[error("unable to execute the built upgrader command, got {err}")]
    Runner { err: io::Error },

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
                id: format!("{}", YELLOW.apply_to(id)),
            },
            Self::NoUpgrader { id, upgrader } => Self::NoUpgrader {
                id: format!("{}", YELLOW.apply_to(id)),
                upgrader: format!("{}", YELLOW.apply_to(upgrader)),
            },
            _ => self,
        }
    }

    pub fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error")))?;

        let msg = format!("{}", self.color());

        term.write_line(&msg)?;
        term.flush()
    }
}

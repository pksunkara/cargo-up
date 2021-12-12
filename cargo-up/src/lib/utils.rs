use oclif::{term::ERR_YELLOW, CliError};
use thiserror::Error;

use std::io;

#[doc(hidden)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("minimum version of {0} that should be upgraded from is {1}")]
    NotMinimum(String, String),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl CliError for Error {
    fn color(self) -> Self {
        match self {
            Self::NotMinimum(dep, min) => Self::NotMinimum(
                ERR_YELLOW.apply_to(dep).to_string(),
                ERR_YELLOW.apply_to(min).to_string(),
            ),
            _ => self,
        }
    }
}

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/automa-app/cargo-up";

#[inline]
pub(crate) fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

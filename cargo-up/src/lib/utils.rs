use console::{Style, Term};
use lazy_static::lazy_static;
use std::io::Result;

lazy_static! {
    pub(crate) static ref TERM_ERR: Term = Term::stderr();
    pub(crate) static ref TERM_OUT: Term = Term::stdout();
    pub(crate) static ref YELLOW: Style = Style::new().for_stderr().yellow();
    pub(crate) static ref YELLOW_OUT: Style = Style::new().yellow();
    pub(crate) static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
}

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("There are no changes in the upgrader for target {0}")]
    NoChanges(String),
    #[error("minimum version of {0} that should be upgraded from is {1}")]
    NotMinimum(String, String),
}

impl Error {
    pub fn print_out(self) -> Result<()> {
        self.print(&TERM_OUT)
    }

    pub fn print_err(self) -> Result<()> {
        TERM_ERR.write_str(&format!("{}: ", RED_BOLD.apply_to("error")))?;
        self.print(&TERM_ERR)
    }

    fn color(self) -> Self {
        match self {
            Self::NoChanges(version) => Self::NoChanges(YELLOW_OUT.apply_to(version).to_string()),
            Self::NotMinimum(dep, min) => Self::NotMinimum(
                YELLOW.apply_to(dep).to_string(),
                YELLOW.apply_to(min).to_string(),
            ),
        }
    }

    fn print(self, term: &Term) -> Result<()> {
        term.write_line(&self.color().to_string())?;
        term.flush()
    }
}

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

#[inline]
pub(crate) fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

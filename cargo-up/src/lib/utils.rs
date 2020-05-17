use console::{Style, Term};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref TERM_ERR: Term = Term::stderr();
    pub(crate) static ref TERM_OUT: Term = Term::stdout();
    pub(crate) static ref YELLOW: Style = Style::new().for_stderr().yellow();
    pub(crate) static ref YELLOW_OUT: Style = Style::new().yellow();
    pub(crate) static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
}

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

#[inline]
pub(crate) fn normalize(name: &str) -> String {
    name.replace("-", "_")
}

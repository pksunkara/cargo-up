use console::{Style, Term};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
    pub static ref YELLOW: Style = Style::new().for_stderr().yellow();
    pub static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
}

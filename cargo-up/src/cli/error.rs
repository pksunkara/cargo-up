use cargo_up_utils::Error;
use console::{Style, Term};
use lazy_static::lazy_static;
use std::{fmt::Display, io, marker::Sized};

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
    static ref YELLOW: Style = Style::new().for_stderr().yellow();
    static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
}

pub trait ErrorPrint: Sized + Display {
    fn print_err(self) -> io::Result<()> {
        self.print(&TERM_ERR)
    }

    fn color(self) -> Self;

    fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error")))?;

        let msg = format!("{}", self.color());

        term.write_line(&msg)?;
        term.flush()
    }
}

impl ErrorPrint for Error {
    fn color(self) -> Self {
        match self {
            Self::PackageNotInWorkspace { id, ws } => Self::PackageNotInWorkspace {
                id: format!("{}", YELLOW.apply_to(id)),
                ws: format!("{}", YELLOW.apply_to(ws)),
            },
            Self::PackageNotFound { id } => Self::PackageNotFound {
                id: format!("{}", YELLOW.apply_to(id)),
            },
            _ => self,
        }
    }
}

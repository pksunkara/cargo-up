use console::Term;
use lazy_static::lazy_static;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

mod error;

pub use error::Error;

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
}

pub type Result<T = ()> = std::result::Result<T, Error>;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

pub fn cargo<'a>(root: &PathBuf, args: &[&'a str]) -> Result<(String, String)> {
    let mut args = args.to_vec();

    if TERM_ERR.features().colors_supported() {
        args.push("--color");
        args.push("always");
    }

    let mut output_stderr = vec![];
    let mut child = Command::new("cargo")
        .current_dir(root)
        .args(&args)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| Error::Cargo {
            err,
            args: args.iter().map(|x| x.to_string()).collect(),
        })?;

    {
        let stderr = child.stderr.as_mut().expect(INTERNAL_ERR);

        for line in BufReader::new(stderr).lines() {
            let line = line?;

            eprintln!("{}", line);
            output_stderr.push(line);
        }
    }

    let output = child.wait_with_output().map_err(|err| Error::Cargo {
        err,
        args: args.iter().map(|x| x.to_string()).collect(),
    })?;

    Ok((
        String::from_utf8(output.stdout)?.trim().to_owned(),
        output_stderr.join("\n").trim().to_owned(),
    ))
}

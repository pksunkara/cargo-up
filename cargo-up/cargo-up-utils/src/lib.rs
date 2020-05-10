mod error;

pub use error::Error;

pub type Result = std::result::Result<(), Error>;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/cargo-up";

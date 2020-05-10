use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{AppSettings, Clap};
use std::process::exit;

mod dep;
mod error;

use error::{ErrorPrint, TERM_ERR, TERM_OUT};

#[derive(Debug, Clap)]
enum Subcommand {
    Dep(dep::Dep),
}

#[derive(Debug, Clap)]
#[clap(version, global_setting(AppSettings::VersionlessSubcommands))]
struct Opt {
    /// Path to workspace Cargo.toml
    #[clap(long, value_name = "path")]
    manifest_path: Option<String>,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Clap)]
#[clap(
    name = "cargo-up",
    bin_name = "cargo",
    version,
    global_setting(AppSettings::ColoredHelp)
)]
enum Cargo {
    Up(Opt),
}

fn main() {
    env_logger::init();

    let Cargo::Up(opt) = Cargo::parse();

    let mut cmd = MetadataCommand::new();

    cmd.features(CargoOpt::AllFeatures);

    if let Some(path) = opt.manifest_path {
        cmd.manifest_path(path);
    }

    let metadata = cmd.exec().unwrap();

    let err = match opt.subcommand {
        Subcommand::Dep(x) => x.run(metadata),
    }
    .err();

    let code = if let Some(e) = err {
        e.print_err().unwrap();
        1
    } else {
        0
    };

    TERM_ERR.flush().unwrap();
    TERM_OUT.flush().unwrap();

    exit(code)
}

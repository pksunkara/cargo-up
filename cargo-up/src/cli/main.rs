use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{AppSettings, Clap};
use oclif::finish;

use std::process::exit;

mod dep;
mod utils;

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
    let Cargo::Up(opt) = Cargo::parse();

    let mut cmd = MetadataCommand::new();

    cmd.features(CargoOpt::AllFeatures);

    if let Some(path) = opt.manifest_path {
        cmd.manifest_path(path);
    }

    let metadata = match cmd.exec() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err.to_string());
            exit(1);
        }
    };

    let result = match opt.subcommand {
        Subcommand::Dep(x) => x.run(metadata),
    };

    finish(result)
}

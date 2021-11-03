use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;
use oclif::finish;

use std::process::exit;

mod dep;
mod utils;

#[derive(Debug, Parser)]
enum Subcommand {
    Dep(dep::Dep),
}

#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// Path to workspace Cargo.toml
    #[clap(long, value_name = "path")]
    manifest_path: Option<String>,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[clap(name = "cargo-up", bin_name = "cargo", version)]
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

use crate::utils::{cargo, Error, Result};
use cargo_metadata::Metadata;
use clap::Clap;
use std::{
    env::var_os,
    fs::{create_dir_all, write},
    path::PathBuf,
    process::Command,
};

/// Upgrade a specific dependency
#[derive(Debug, Clap)]
pub struct Dep {}

impl Dep {
    pub fn run(&self, metadata: Metadata) -> Result {
        let cargo_home = PathBuf::from(var_os("CARGO_HOME").ok_or(Error::NoCargoHome)?);
        let cache_dir = cargo_home.join("cargo-up-cache");

        create_dir_all(cache_dir.join("src"))?;

        write(
            cache_dir.join("Cargo.toml"),
            format!(
                r#"
                [package]
                name = "runner"
                version = "0.0.0"
                edition = "2018"
                publish = false

                [dependencies]
                cargo-up = {{ path = "/Users/pksunkara/Coding/pksunkara/cargo-up/cargo-up" }}
                clap_up = {{ path = "/Users/pksunkara/Coding/clap-rs/clap/clap_up" }}
                "#
            ),
        )?;

        write(
            cache_dir.join("src").join("main.rs"),
            format!(
                r#"
                use cargo_up::{{Runner, semver::Version}};
                use clap_up::Clap;
                use std::path::Path;

                fn main() {{
                    Runner::<Clap>::default().run(
                        Path::new("{}"),
                        Version::parse("3.0.0-beta.1").unwrap(),
                    );
                }}
                "#,
                &metadata.workspace_root.to_string_lossy()
            ),
        )?;

        let (_, err) = cargo(&cache_dir, &["build"])?;

        if !err.contains("Finished") {
            panic!("unable to build");
            // TODO: Error
        }

        let status = Command::new(cache_dir.join("target").join("debug").join("runner"))
            .current_dir(&cache_dir)
            .spawn()
            .map_err(|err| Error::Runner { err })?
            .wait()?;

        if !status.success() {
            panic!("exit status bad");
            // TODO: Error
        }

        Ok(())
    }
}

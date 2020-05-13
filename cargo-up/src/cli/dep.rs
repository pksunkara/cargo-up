use crate::utils::{cargo, normalize, Error, Result};
use cargo_metadata::Metadata;
use clap::Clap;
use crates_io_api::SyncClient;
use heck::CamelCase;
use semver::Version;
use std::{
    env::var_os,
    fs::{create_dir_all, write},
    path::PathBuf,
    process::Command,
};

/// Upgrade a specific dependency
#[derive(Debug, Clap)]
pub struct Dep {
    /// Dependency name
    dep: String,

    // TODO: Allow specifying path?
    /// Specify version of upgrader
    #[clap(short, long)]
    version: Option<Version>,
}

impl Dep {
    pub fn run(&self, metadata: Metadata) -> Result {
        let dep = normalize(&self.dep);
        let dep_camel = dep.to_camel_case();

        // Find the dep in metadata first
        let pkg = metadata
            .packages
            .iter()
            .find(|x| normalize(&x.name) == dep)
            .ok_or(Error::PackageNotFound {
                id: self.dep.clone(),
            })?;

        // Find the upgrader in crates.io
        let upgrader = format!("{}_up", &dep);
        let client = SyncClient::new();

        // let krate = client.get_crate(&upgrader).map_err(|_| Error::NoUpgrader {
        //     id: dep.clone(),
        //     upgrader: upgrader.clone(),
        // })?;

        // Write the upgrader
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
                "#,
                // &krate.crate_data.name,
                // self.version
                //     .as_ref()
                //     .map_or_else(|| krate.crate_data.max_version.clone(), |x| x.to_string()),
            ),
        )?;

        write(
            cache_dir.join("src").join("main.rs"),
            format!(
                r#"
                use cargo_up::{{Runner, semver::Version}};
                use {}::{};
                use std::path::Path;

                fn main() {{
                    Runner::<{}>::default().run(
                        Path::new("{}"),
                        Version::parse("{}").unwrap(),
                    );
                }}
                "#,
                &upgrader,
                &dep_camel,
                &dep_camel,
                &metadata.workspace_root.to_string_lossy(),
                pkg.version.to_string(),
            ),
        )?;

        // Execute the upgrader
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

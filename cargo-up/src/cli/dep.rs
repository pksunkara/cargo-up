use crate::utils::{
    cargo,
    crates::{Upgrader, Versions},
    normalize, Error, Result, INTERNAL_ERR,
};

use cargo_metadata::{Metadata, Package};
use clap::{crate_version, Parser};
use oclif::term::{OUT_YELLOW, TERM_OUT};
use semver::Version;

use std::{
    env::{current_dir, var_os},
    fs::{create_dir_all, remove_file, write},
    path::PathBuf,
    process::Command,
};

/// Upgrade a specific dependency
#[derive(Debug, Parser)]
pub struct Dep {
    /// Dependency name
    dep: String,

    /// Specify version of upgrader
    #[clap(long)]
    upgrader_version: Option<Version>,

    /// Specify version of the dependency to upgrade to
    #[clap(long)]
    dep_version: Option<Version>,

    /// Specify version of cargo-up library
    #[clap(long)]
    lib_version: Option<Version>,

    /// Suppress cargo build output
    #[clap(long, hide = true)]
    suppress_cargo_output: bool,

    /// Specify path for upgrader
    #[cfg(debug_assertions)]
    #[clap(long, hide = true, requires_all = &["upgrader-name", "dep-version", "lib-path"])]
    upgrader_path: Option<String>,

    /// Specify name for upgrader if upgrader path is given
    #[cfg(debug_assertions)]
    #[clap(long, hide = true, requires_all = &["upgrader-path", "dep-version", "lib-path"], conflicts_with_all = &["upgrader-version", "lib-version"])]
    upgrader_name: Option<String>,

    /// Specify path for cargo-up library
    #[cfg(debug_assertions)]
    #[clap(long, hide = true, requires_all = &["upgrader-name", "upgrader-path", "dep-version"])]
    lib_path: Option<String>,
}

fn get_path(path: &Option<String>) -> Result<String> {
    let path = current_dir()?.join(path.as_ref().expect(INTERNAL_ERR));

    Ok(format!(
        r#"{{ path = {:?} }}"#,
        path.canonicalize().unwrap().to_string_lossy(),
    ))
}

impl Dep {
    pub fn run(&self, metadata: Metadata) -> Result {
        let dep = normalize(&self.dep);

        // Find the dep in metadata first
        let pkg = metadata
            .packages
            .iter()
            .find(|x| normalize(&x.name) == *dep)
            .ok_or(Error::PackageNotFound {
                dep: self.dep.clone(),
            })?;

        if let Some(upgrader_name) = &self.upgrader_name {
            // Use the given options on CLI for local testing
            let dep_version = self.dep_version.as_ref().expect(INTERNAL_ERR).to_string();

            self.upgrade(
                &metadata,
                &dep,
                pkg,
                upgrader_name,
                &get_path(&self.upgrader_path)?,
                &dep_version,
                &get_path(&self.lib_path)?,
            )
        } else {
            // Find the upgrader in crates.io
            let upgrader = format!("{}_up", &dep);

            let upgrader_krate =
                ureq::get(&format!("https://crates.io/api/v1/crates/{}", upgrader))
                    .call()
                    .into_json_deserialize::<Upgrader>()
                    .map_err(|_| Error::NoUpgrader {
                        dep: dep.clone(),
                        upgrader,
                    })?;

            let lib_version = format!(
                r#""={}""#,
                self.lib_version
                    .clone()
                    .unwrap_or_else(|| crate_version!().parse().expect(INTERNAL_ERR))
            );
            let upgrader_version = format!(
                r#""={}""#,
                self.upgrader_version.as_ref().map_or_else(
                    || upgrader_krate.krate.max_version.clone(),
                    |x| x.to_string(),
                )
            );

            // We get the versions sorted already by semver in descending order
            // https://github.com/rust-lang/crates.io/blob/c128a6765648d46a0e2246a669c994bfd494fef4/src/krate.rs#L281
            let versions = ureq::get(&format!("https://crates.io/api/v1/crates/{}/versions", dep))
                .call()
                .into_json_deserialize::<Versions>()
                .map_err(|_| Error::NoDependency { dep: dep.clone() })?
                .versions
                .into_iter()
                .map(|x| Version::parse(&x.num).map_err(|_| Error::BadRegistry))
                .rev()
                .collect::<Result<Vec<Version>>>()?
                .into_iter()
                .filter(|x| *x > pkg.version)
                .collect::<Vec<_>>();

            for dep_version in versions {
                TERM_OUT.write_line(&format!(
                    "Trying to upgrade {} dependency to {} version ...",
                    OUT_YELLOW.apply_to(&self.dep),
                    OUT_YELLOW.apply_to(&dep_version),
                ))?;
                TERM_OUT.flush()?;

                self.upgrade(
                    &metadata,
                    &dep,
                    pkg,
                    &upgrader_krate.krate.name,
                    &upgrader_version,
                    &dep_version.to_string(),
                    &lib_version,
                )?;
            }

            Ok(())
        }
    }

    fn upgrade(
        &self,
        metadata: &Metadata,
        dep: &String,
        pkg: &Package,
        upgrader_name: &str,
        upgrader_version: &str,
        dep_version: &str,
        lib_version: &str,
    ) -> Result {
        // Write the upgrade runner
        let cargo_home = PathBuf::from(var_os("CARGO_HOME").ok_or(Error::NoCargoHome)?);
        let cache_dir = cargo_home.join("cargo-up-cache");

        let lock_file = cache_dir.join("Cargo.lock");

        if lock_file.exists() {
            remove_file(lock_file)?;
        }

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
                env_logger = "0.7"
                oclif = "0.3"
                cargo-up = {}
                {} = {}
                "#,
                lib_version, upgrader_name, upgrader_version
            ),
        )?;

        write(
            cache_dir.join("src").join("main.rs"),
            format!(
                r#"
                use oclif::finish;
                use cargo_up::{{semver::Version, run, Runner}};
                use std::path::Path;

                // To type check the returned runner
                fn runner() -> Runner {{
                    {}::runner()
                }}

                fn main() {{
                    env_logger::builder()
                        .format_timestamp(None)
                        .init();

                    let result = run(
                        Path::new({:?}),
                        "{}",
                        runner(),
                        Version::parse("{}").unwrap(),
                        Version::parse("{}").unwrap(),
                    );

                    finish(result);
                }}
                "#,
                upgrader_name,
                metadata
                    .workspace_root
                    .clone()
                    .into_os_string()
                    .to_string_lossy(),
                dep,
                pkg.version,
                dep_version,
            ),
        )?;

        // Compile the upgrader
        let (_, err) = cargo(&cache_dir, &["build"], !self.suppress_cargo_output)?;

        if !err.contains("Finished") {
            return Err(Error::Building {
                upgrader: upgrader_name.into(),
            });
        }

        // Execute the upgrader
        let status = Command::new(cache_dir.join("target").join("debug").join("runner"))
            .current_dir(&cache_dir)
            .spawn()
            .map_err(|err| Error::Runner { err })?
            .wait()?;

        if !status.success() {
            return Err(Error::Upgrading {
                upgrader: upgrader_name.into(),
            });
        }

        Ok(())
    }
}

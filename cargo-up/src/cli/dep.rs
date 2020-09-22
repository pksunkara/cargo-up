use crate::utils::{cargo, normalize, Error, Result, INTERNAL_ERR};
use cargo_metadata::Metadata;
use clap::{crate_version, Clap};
use crates_io_api::SyncClient;
use semver::Version;
use std::{
    env::{current_dir, var_os},
    fs::{create_dir_all, remove_file, write},
    path::PathBuf,
    process::Command,
};

/// Upgrade a specific dependency
#[derive(Debug, Clap)]
pub struct Dep {
    /// Dependency name
    dep: String,

    /// Specify version of upgrader
    #[clap(short, long)]
    version: Option<Version>,

    /// Specify version to upgrade to if upgrader path is given
    #[clap(long, hidden = true, requires_all = &["name", "path", "lib-path"])]
    to_version: Option<Version>,

    /// Specify path for upgrader
    #[clap(long, hidden = true, requires_all = &["name", "to-version", "lib-path"], conflicts_with_all = &["version"])]
    path: Option<String>,

    /// Specify name for upgrader if upgrader path is given
    #[clap(long, hidden = true, requires_all = &["path", "to-version", "lib-path"])]
    name: Option<String>,

    /// Specify path for cargo-up library
    #[clap(long, hidden = true, requires_all = &["path", "name", "to-version"])]
    lib_path: Option<String>,

    /// Suppress cargo build output
    #[clap(long, hidden = true)]
    suppress_cargo_output: bool,
}

fn get_path(path: &Option<String>) -> Result<String> {
    let path = current_dir()?.join(path.as_ref().expect(INTERNAL_ERR));

    Ok(format!(
        r#"{{ path = "{}" }}"#,
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
            .find(|x| normalize(&x.name) == dep)
            .ok_or(Error::PackageNotFound {
                id: self.dep.clone(),
            })?;

        let (upgrader, upgrader_version, to_version, lib_version) = if let Some(name) = &self.name {
            (
                name.to_string(),
                get_path(&self.path)?,
                self.to_version.as_ref().expect(INTERNAL_ERR).to_string(),
                get_path(&self.lib_path)?,
            )
        } else {
            // Find the upgrader in crates.io
            let upgrader = format!("{}_up", &dep);
            let client = SyncClient::new();

            let krate = client.get_crate(&upgrader).map_err(|_| Error::NoUpgrader {
                id: dep.clone(),
                upgrader,
            })?;

            (
                krate.crate_data.name.clone(),
                self.version
                    .as_ref()
                    .map_or_else(|| krate.crate_data.max_version.clone(), |x| x.to_string()),
                // TODO: Get the next version from crates.io
                String::from("3.0.0-rc.0"),
                format!(r#""={}""#, crate_version!()),
            )
        };

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
                log = "0.4"
                env_logger = "0.7"
                cargo-up = {}
                {} = {}
                "#,
                lib_version, upgrader, upgrader_version
            ),
        )?;

        write(
            cache_dir.join("src").join("main.rs"),
            format!(
                r#"
                use cargo_up::{{semver::Version, Runner}};
                use std::path::Path;

                // To type check the returned runner
                fn runner() -> Runner {{
                    {}::runner()
                }}

                fn main() {{
                    env_logger::builder()
                        .format_timestamp(None)
                        .init();

                    runner().run(
                        Path::new("{}"),
                        "{}",
                        Version::parse("{}").unwrap(),
                        Version::parse("{}").unwrap(),
                    ).unwrap();
                }}
                "#,
                &upgrader,
                &metadata.workspace_root.to_string_lossy(),
                &dep,
                pkg.version.to_string(),
                &to_version,
            ),
        )?;

        // Execute the upgrader
        let (_, err) = cargo(&cache_dir, &["build"], !self.suppress_cargo_output)?;

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

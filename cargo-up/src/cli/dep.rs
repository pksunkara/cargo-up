use cargo_metadata::Metadata;
use cargo_up_utils::Result;
use clap::Clap;
use std::fs::{create_dir_all, remove_dir_all};

/// Upgrade a specific dependency
#[derive(Debug, Clap)]
pub struct Dep {}

impl Dep {
    pub fn run(&self, metadata: Metadata) -> Result {
        let tmp_dir = &metadata.workspace_root.join("cargo-up-tmp");

        // find clap version

        create_dir_all(&tmp_dir)?;

        remove_dir_all(&tmp_dir)?;

        Ok(())
    }
}

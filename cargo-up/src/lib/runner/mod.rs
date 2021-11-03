use crate::{
    ra_ap_syntax::AstNode,
    semver::{Error as SemVerError, Version as SemverVersion},
    utils::{normalize, Error, INTERNAL_ERR},
    Semantics, Version,
};

use log::{debug, info, trace};
use oclif::term::{OUT_YELLOW, TERM_OUT};
use ra_ap_base_db::{FileId, SourceDatabase, SourceDatabaseExt};
use ra_ap_hir::Crate;
use ra_ap_ide_db::symbol_index::SymbolsDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use ra_ap_rust_analyzer::cli::load_cargo::{load_workspace, LoadCargoConfig};
use ra_ap_text_edit::TextEdit;
use rust_visitor::Visitor;

use std::{
    collections::HashMap as Map,
    fs::{read_to_string, write},
    path::Path,
};

mod context;
mod helpers;
mod visitor_impl;

pub(crate) use context::Context;

pub struct Runner {
    pub(crate) minimum: Option<SemverVersion>,
    pub(crate) versions: Vec<Version>,
    version: SemverVersion,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            minimum: None,
            versions: vec![],
            version: SemverVersion::parse("0.0.0").expect(INTERNAL_ERR),
        }
    }

    pub fn minimum(mut self, version: &str) -> Result<Self, SemVerError> {
        self.minimum = Some(SemverVersion::parse(version)?);
        Ok(self)
    }

    pub fn version(mut self, version: Version) -> Self {
        self.versions.push(version);
        self
    }
}

impl Runner {
    fn get_version(&self) -> Option<&Version> {
        self.versions.iter().find(|x| x.version == self.version)
    }
}

#[doc(hidden)]
pub fn run(
    root: &Path,
    dep: &str,
    mut runner: Runner,
    from: SemverVersion,
    to: SemverVersion,
) -> Result<(), Error> {
    info!("Workspace root: {}", root.display());

    if let Some(min) = &runner.minimum {
        if from < *min {
            return Err(Error::NotMinimum(dep.into(), min.to_string()));
        }
    }

    runner.version = to;
    let version = runner.get_version();

    let peers = if let Some(version) = version {
        let mut peers = vec![dep.to_string()];
        peers.extend(version.peers.clone());
        peers
    } else {
        return Ok(TERM_OUT.write_line(&format!(
            "Upgrader for crate {} has not described any changes for {} version",
            OUT_YELLOW.apply_to(dep),
            OUT_YELLOW.apply_to(runner.version),
        ))?);
    };

    // Loading project
    let manifest = ProjectManifest::discover_single(&AbsPathBuf::assert(root.into())).unwrap();

    let no_progress = &|_| {};

    let mut cargo_config = CargoConfig::default();
    cargo_config.no_sysroot = true;

    let mut workspace = ProjectWorkspace::load(manifest, &cargo_config, no_progress)?;
    let bs = workspace.run_build_scripts(&cargo_config, no_progress)?;
    workspace.set_build_scripts(bs);

    let load_cargo_config = LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro: true,
        prefill_caches: false,
    };
    let (host, vfs, _) = load_workspace(workspace, &load_cargo_config).unwrap();

    // Preparing running wrapper
    let db = host.raw_database();
    let semantics = Semantics::new(db);

    let mut changes = Map::<FileId, TextEdit>::new();
    let mut context = Context::new(runner, semantics);

    // Run init hook
    context.init(&from)?;

    trace!("Crate graph: {:#?}", db.crate_graph());

    // Loop to find and eager load the dep we are upgrading
    for krate in Crate::all(db) {
        if let Some(name) = krate.display_name(db) {
            debug!("Checking if we need to preload: {}", name);

            if let Some(peer) = peers
                .iter()
                .find(|x| **x == normalize(&format!("{}", name)))
            {
                context.preloader.load(peer, db, &krate);
            }
        }
    }

    // Actual loop to walk through the source code
    for source_root_id in db.local_roots().iter() {
        let source_root = db.source_root(*source_root_id);
        let krates = db.source_root_crates(*source_root_id);

        // Get all crates for this source root and skip if no root files of those crates
        // are in the root path we are upgrading.
        if !krates
            .iter()
            .filter_map(|crate_id| {
                let krate: Crate = (*crate_id).into();
                source_root.path_for_file(&krate.root_file(db))
            })
            .filter_map(|path| path.as_path())
            .any(|path| {
                debug!("Checking if path in workspace: {}", path.display());
                path.as_ref().starts_with(root)
            })
        {
            continue;
        }

        for file_id in source_root.iter() {
            let file = vfs.file_path(file_id);
            info!("Walking: {}", file.as_path().expect(INTERNAL_ERR).display());

            let source_file = context.semantics.parse(file_id);
            trace!("Syntax: {:#?}", source_file.syntax());

            context.walk(source_file.syntax());

            let edit = context.upgrader.finish();
            debug!("Changes to be made: {:#?}", edit);

            changes.insert(file_id, edit);
        }
    }

    // Apply changes
    for (file_id, edit) in changes {
        let full_path = vfs.file_path(file_id);
        let full_path = full_path.as_path().expect(INTERNAL_ERR);

        let mut file_text = read_to_string(&full_path)?;

        edit.apply(&mut file_text);
        write(&full_path, file_text)?;
    }

    // TODO: Modify Cargo.toml

    Ok(())
}

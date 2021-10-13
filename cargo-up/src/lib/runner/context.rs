use crate::{
    preloader::Preloader, semver::Version as SemverVersion, utils::INTERNAL_ERR, Runner, Semantics,
    Upgrader,
};

use anyhow::Result as AnyResult;

pub(crate) struct Context<'a> {
    pub(crate) runner: Runner,
    pub(crate) preloader: Preloader,
    pub(crate) upgrader: Upgrader,
    pub(crate) semantics: Semantics<'a>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(runner: Runner, semantics: Semantics<'a>) -> Self {
        Self {
            runner,
            preloader: Preloader::default(),
            upgrader: Upgrader::default(),
            semantics,
        }
    }

    pub(crate) fn init(&mut self, from: &SemverVersion) -> AnyResult<()> {
        let version = self.runner.get_version().expect(INTERNAL_ERR);

        if let Some(f) = &version.init {
            f(&mut self.upgrader, from)
        } else {
            Ok(())
        }
    }
}

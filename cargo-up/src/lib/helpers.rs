use crate::{
    ra_ap_syntax::ast::{Name, NameOrNameRef, NameRef, Path},
    runner::RunnerWrapper,
    version::Hooks,
    Semantics, Upgrader,
};

use std::collections::HashMap as Map;

pub(crate) fn get_name(name_or_name_ref: Option<NameOrNameRef>) -> Option<String> {
    Some(match name_or_name_ref? {
        NameOrNameRef::Name(name) => name.text().to_string(),
        NameOrNameRef::NameRef(name_ref) => name_ref.text().to_string(),
    })
}

pub(crate) fn get_name_from_name_ref(name_ref: Option<NameRef>) -> Option<String> {
    Some(name_ref?.text().to_string())
}

pub(crate) fn get_name_from_name(name: Option<Name>) -> Option<String> {
    Some(name?.text().to_string())
}

pub(crate) fn get_name_from_path(path: Option<Path>) -> Option<String> {
    get_name_from_name_ref(path?.segment()?.name_ref())
}

impl<'a> RunnerWrapper<'a> {
    pub(crate) fn run_hooks<'b, I, N, NG, PG>(
        &self,
        upgrader: &mut Upgrader,
        item_paths: &Map<I, String>,
        path_map: &'b Hooks<N>,
        node: &N,
        name_getter: NG,
        path_getter: PG,
    ) -> Option<()>
    where
        I: Eq,
        NG: Fn(&N) -> Option<String>,
        PG: Fn(&Semantics, &N) -> Option<I>,
    {
        let name = name_getter(node)?;

        if !path_map.iter().any(|x| x.1.iter().any(|y| *y.0 == name)) {
            return None;
        }

        let semantics = self.semantics();
        let item = path_getter(semantics, node)?;
        let path = item_paths.iter().find(|x| *x.0 == item)?.1;
        let map = path_map.get(path)?;
        let hooks = map.get(&name)?;

        for hook in hooks {
            hook(upgrader, node, semantics);
        }

        Some(())
    }
}

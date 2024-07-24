use crate::{package::Package, source::Source, workspace::Workspace};

pub fn sources_to_workspaces(sources: &Vec<Source>) -> Vec<Workspace> {
    sources
        .iter()
        .map(|s| Workspace::init_from_dir(s.path.clone()))
        .collect()
}

pub fn packages_jsons(workspaces: Vec<Workspace>) -> Vec<String> {
    workspaces
        .iter()
        .flat_map(|w| w.walk_package_jsons())
        .flatten()
        .map(|p| p.to_str().unwrap().to_string())
        .collect()
}

pub fn packages_from_source(source: Source) -> Vec<Package> {
    let workspace = Workspace::init_from_dir(source.path.clone());
    workspace
        .walk_package_jsons()
        .unwrap()
        .map(|p| p.to_str().unwrap().to_string())
        .map(|p| {
            let ws = Workspace::init_from_dir(&p);
            Package::new(p.clone(), p, source.id.clone())
        })
        .collect()
}

use crate::{package::Package, source::Source, workspace::Workspace};

pub fn packages_jsons(workspaces: Vec<Workspace>) -> Vec<String> {
    workspaces
        .iter()
        .flat_map(|w| w.walk_package_jsons())
        .map(|p| p.to_str().unwrap().to_string())
        .collect()
}

pub fn packages_from_source(source: &Source) -> Vec<Package> {
    let workspace = Workspace::init_from_dir(source.path.clone());
    workspace
        .walk_package_jsons()
        .filter(|p| p.parent().is_some())
        .map(|p| {
            let ws_dir = p.parent().unwrap();
            let ws = Workspace::init_from_dir(ws_dir);
            let pkg = ws.package_json();
            Package::new(
                pkg.name.clone(),
                ws_dir.to_str().unwrap().to_string(),
                source.id.clone(),
            )
        })
        .collect()
}

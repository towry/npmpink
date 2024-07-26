use package_json_schema::Private;

use crate::{package::Package, source::Source, workspace::Workspace};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn packages_jsons_of_workspaces(workspaces: Vec<Workspace>) -> Vec<String> {
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
        .filter_map(|p| {
            let ws_dir = p.parent().unwrap();
            let ws = Workspace::init_from_dir(ws_dir);
            // it will fail if parse with invalid package.json, like duplicate field etc.
            ws.package_json()
                .ok()
                .filter(|pkg| pkg.name.is_some())
                .filter(|pkg| {
                    pkg.private
                        .as_ref()
                        .is_some_and(package_private_is_not_falsy)
                })
                .map(|pkg| {
                    Package::new(
                        pkg.name.clone().unwrap(),
                        ws_dir.to_str().unwrap().to_string(),
                        source.id.clone(),
                    )
                })
        })
        .collect()
}

pub fn difference_packages<'a>(left: &'a [Package], right: &'a [Package]) -> Vec<Package> {
    let lhs = HashSet::<&Package>::from_iter(left);
    let rhs = HashSet::from_iter(right);

    lhs.difference(&rhs)
        .cloned()
        .cloned()
        .collect::<Vec<Package>>()
}

fn package_private_is_not_falsy(v: &Private) -> bool {
    match v {
        Private::False => true,
        Private::Bool(b) => !*b,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacakge_private_falsy() {
        let package_is_private = Private::True;
        let package_private_is_bool = Private::Bool(true);
        assert!(!package_private_is_not_falsy(&package_is_private));
        assert!(!package_private_is_not_falsy(&package_private_is_bool));
    }
}

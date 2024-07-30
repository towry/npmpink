#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;
use std::iter::Iterator;

use crate::package::Package;

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct LockfileContent {
    pub version: String,
    pub packages: BTreeMap<String, Package>,
}

impl LockfileContent {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn init_from_lockfile_string(content: String) -> Result<Self> {
        serde_json::from_str(&content).map_err(anyhow::Error::from)
    }
    pub fn is_empty() -> bool {
        true
    }
    pub fn add_package(&mut self, pkg_name: String, pkg: Package) -> &Self {
        self.packages.insert(pkg_name, pkg);
        self
    }
    pub fn remove_package(&mut self, pkg_name: String) -> &Self {
        self.packages.remove(&pkg_name);
        self
    }
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(anyhow::Error::from)
    }
    // Since we are allocate new iterator from packages, we need to use collect.
    // and the return type have 'static bound means the receiver can hold it for infinite.
    pub fn packages_iter(&self) -> impl Iterator<Item = Package> + 'static {
        self.packages
            .values()
            .cloned()
            // use collect to allocate the iterator
            .collect::<Vec<Package>>()
            .into_iter()
    }
}

impl Default for LockfileContent {
    fn default() -> Self {
        LockfileContent {
            version: "0.0.1".to_owned(),
            packages: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::package::Package;

    use super::LockfileContent;
    use std::collections::BTreeMap;

    #[test]
    fn test_lockfile_content() {
        let _ = LockfileContent {
            version: "1.0.0".into(),
            packages: BTreeMap::new(),
        };
    }

    #[test]
    fn test_serialize_lockfile_content() {
        use serde_test::{assert_tokens, Token};

        let mut packages = BTreeMap::<String, Package>::new();
        packages.insert("foo_package".into(), Package::test_new());

        let lockfile = LockfileContent {
            version: "1.0.0".into(),
            packages,
        };

        assert_tokens(
            &lockfile,
            &[
                Token::Struct {
                    name: "LockfileContent",
                    len: 2,
                },
                Token::String("version"),
                Token::String("1.0.0"),
                Token::String("packages"),
                // map
                Token::Map { len: Some(1) },
                Token::String("foo_package"),
                Token::Struct {
                    name: "Package",
                    len: 3,
                },
                Token::String("name"),
                Token::String("foo"),
                //
                Token::String("dir"),
                Token::String("foo/bar"),
                //
                Token::String("source_id"),
                Token::String("1"),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }
}

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::package::Package;

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct LockfileContent {
    version: String,
    pub packages: BTreeMap<String, Package>,
}

impl LockfileContent {
    pub fn is_empty() -> bool {
        true
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
                    len: 4,
                },
                Token::String("name"),
                Token::String("foo"),
                //
                Token::String("version"),
                Token::String("bar"),
                //
                Token::String("location"),
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
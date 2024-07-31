use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Package {
    pub name: String,
    pub dir: String,
    pub source_id: String,
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.name.clone() + &self.source_id).hash(state);
    }
}

impl Package {
    pub fn new(name: String, dir: String, source_id: String) -> Self {
        Package {
            name,
            dir,
            source_id,
        }
    }
}

// TODO: better format this package for different pickers.
impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.dir, self.source_id)
    }
}

#[cfg(test)]
impl Package {
    pub(super) fn test_new() -> Self {
        Package {
            name: "foo".into(),
            dir: "foo/bar".into(),
            source_id: "1".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Package;

    #[test]
    fn test_package() {
        let _ = Package::default();
    }

    #[test]
    fn test_package_serialize() {
        use serde_test::{assert_tokens, Token};

        let pkg = Package {
            name: "foo".into(),
            dir: "bar".into(),
            source_id: "foo".into(),
        };

        assert_tokens(
            &pkg,
            &[
                Token::Struct {
                    name: "Package",
                    len: 3,
                },
                //
                Token::String("name"),
                Token::String("foo"),
                //
                Token::String("dir"),
                Token::String("bar"),
                //
                Token::String("source_id"),
                Token::String("foo"),
                //
                Token::StructEnd,
            ],
        );
    }
}

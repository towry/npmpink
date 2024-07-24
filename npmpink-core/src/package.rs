use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, Eq, Hash)]
pub struct Package {
    pub name: String,
    pub dir: String,
    pub source_id: String,
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
                    len: 4,
                },
                //
                Token::String("name"),
                Token::String("foo"),
                //
                Token::String("location"),
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

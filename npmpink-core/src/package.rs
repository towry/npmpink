use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub location: String,
    pub source_id: String,
}

impl Package {
    pub fn new(name: String, location: String, source_id: String) -> Self {
        Package {
            name,
            location,
            source_id,
        }
    }
}

#[cfg(test)]
impl Package {
    pub(super) fn test_new() -> Self {
        Package {
            name: "foo".into(),
            location: "foo/bar".into(),
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
            location: "bar".into(),
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

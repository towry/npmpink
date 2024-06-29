use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug)]
pub(crate) struct Package {
    pub name: String,
    pub version: String,
    pub location: String,
    pub source_id: String,
}

#[cfg(test)]
impl Package {
    pub(super) fn test_new() -> Self {
        Package {
            name: "foo".into(),
            version: "bar".into(),
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
            version: "1.0.0".into(),
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
                Token::String("version"),
                Token::String("1.0.0"),
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

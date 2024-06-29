pub(super) struct Header {
    labels: Vec<String>,
}

impl Default for Header {
    fn default() -> Header {
        Header { labels: Vec::new() }
    }
}

impl Header {
    pub(super) fn new() -> Self {
        Header::default()
    }

    pub(super) fn add_label(mut self, label: String) -> Self {
        self.labels.push(label);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_label() {
        let header = Header::new().add_label("hello".to_string());
        assert_eq!(header.labels, vec!["hello".to_string()]);
    }
}

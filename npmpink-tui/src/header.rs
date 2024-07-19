#[derive(Default)]
pub(super) struct Header {
    pub(super) labels: Vec<String>,
}


impl Header {
    pub(super) fn new() -> Self {
        Header::default()
    }

    pub(super) fn add_label(mut self, label: String) -> Self {
        self.labels.push(label);
        self
    }

    pub(super) fn set_labels(mut self, labels: &[String]) -> Self {
        self.labels = labels.to_vec();
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

    #[test]
    fn test_set_labels() {
        let labels: &[String] = &["label 1".to_string(), "label2".to_string()];
        let header = Header::new().set_labels(labels);
        assert_eq!(header.labels[0], "label 1".to_string(),);
        assert_eq!(header.labels[1], "label2".to_string());
    }
}

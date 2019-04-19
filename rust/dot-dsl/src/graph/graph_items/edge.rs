use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Edge {
    pub start: String,
    pub end: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(start: &str, end: &str) -> Self {
        Self {
            start: start.to_string(),
            end: end.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs
            .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        self
    }
}

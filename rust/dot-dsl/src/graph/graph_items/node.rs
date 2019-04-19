use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs
            .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(String::as_str)
    }
}

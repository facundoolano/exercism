pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    // FIXME move mods into separate files
    pub mod graph_items {
        pub mod node {
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

                // TODO duplicated, extract to trait
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    // FIXME try to change to &str instead of strings?
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    if let Some(v) = self.attrs.get(key) {
                        Some(&v)
                    } else {
                        None
                    }
                }
            }
        }

        pub mod edge {
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

                // TODO duplicated, extract to trait
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    // FIXME try to change to &str instead of strings?
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }
            }
        }
    }

    #[derive(Default, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            // FIXME move instead of clone?
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            // FIXME move instead of clone?
            self.edges.extend_from_slice(edges);
            self
        }

        // TODO duplicated, extract to trait
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            // FIXME try to change to &str instead of strings?
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn get_node(self, node_name: &str) -> Option<Node> {
            self.nodes.into_iter().find(|node| node.name == node_name)
        }
    }
}

pub mod graph {
    pub mod graph_items;
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

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
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn get_node(self, node_name: &str) -> Option<Node> {
            self.nodes.into_iter().find(|node| node.name == node_name)
        }
    }
}

pub mod graph {
    pub mod graph_items {
        pub mod node {
            #[derive(Default)]
            pub struct Node;
        }
        pub mod edge {
            #[derive(Default)]
            pub struct Edge;
        }
    }

    #[derive(Default)]
    pub struct Graph;

    impl Graph {
        pub fn new() -> Self {
            unimplemented!("Construct a new Graph struct.");
        }
    }
}

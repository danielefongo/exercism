pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    fn attrs_from(attrs: &[(&str, &str)]) -> HashMap<String, String> {
        attrs
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs_from(attrs),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            self.nodes
                .iter()
                .find(|&node| node.name == name.to_owned())
                .map(|node| node.clone())
        }
    }

    pub mod graph_items {
        pub mod edge {
            use crate::graph::attrs_from;
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_owned(),
                        to: to.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs_from(attrs),
                        ..self
                    }
                }
            }
        }

        pub mod node {
            use crate::graph::attrs_from;
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        name: value.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs_from(attrs),
                        ..self
                    }
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs
                        .iter()
                        .find(|(attr_name, _)| attr_name.as_str() == name)
                        .map(|(_, value)| value.as_str())
                }
            }
        }
    }
}

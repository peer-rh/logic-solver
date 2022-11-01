use std::collections::HashMap;

use super::{Graph, Idx, Node, Operation};

pub struct Session {
    nodes: HashMap<Idx, Node>,
    pub values: HashMap<Idx, bool>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            nodes: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Operation) -> Idx {
        let next_idx = Idx(self.nodes.len());
        self.nodes.insert(next_idx, node.get_node());
        next_idx
    }

    pub fn eval_graph(&mut self, graph: &Graph, feed_dict: Option<HashMap<Idx, bool>>) -> bool {
        if let Some(feed_dict) = feed_dict {
            self.values.extend(feed_dict.iter());
        }
        let mut new_values = self.values.clone();
        let out = graph.evaluate(&mut new_values, self);
        self.values = new_values;
        out
    }

    pub fn reset(&mut self) {
        self.values = HashMap::new();
    }

    pub fn get_value(&self, idx: &Idx) -> bool {
        self.values[idx]
    }

    pub fn get_node(&self, idx: &Idx) -> &Node {
        &self.nodes[idx]
    }
}

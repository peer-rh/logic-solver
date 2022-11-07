use std::collections::HashMap;

use crate::{Graph, Idx, Operation};

pub struct NodeMatcher {
    nodes: HashMap<Idx, Operation>,
    new_nodes: HashMap<Idx, Operation>,
    match_node: Idx,
}

impl NodeMatcher {
    pub fn new(match_nodes: HashMap<Idx, Operation>, new_nodes: HashMap<Idx, Operation>) -> Self {
        Self {
            match_node: match_nodes.len() - 1,
            new_nodes,
            nodes: match_nodes,
        }
    }

    pub fn match_with_node(
        &self,
        idx: Idx,
        nodes: &HashMap<Idx, Operation>,
        out_nodes: &Vec<Idx>,
    ) -> Option<Graph> {
        let mut input_idxs: HashMap<Idx, Idx> = HashMap::new();
        if self.match_with_node_recursive(self.match_node, idx, &mut input_idxs, nodes) {
            let mut new_nodes = nodes.clone();
            (0..self.new_nodes.len()).for_each(|i| {
                let op = &self.new_nodes[&i];
                match op {
                    Operation::And(a, b) => {
                        new_nodes.insert(
                            nodes.len() + i,
                            Operation::And(
                                if input_idxs.contains_key(a) {
                                    input_idxs[a]
                                } else {
                                    nodes.len() + a
                                },
                                if input_idxs.contains_key(b) {
                                    input_idxs[b]
                                } else {
                                    nodes.len() + b
                                },
                            ),
                        );
                    }
                    Operation::Or(a, b) => {
                        new_nodes.insert(
                            nodes.len() + i,
                            Operation::Or(
                                if input_idxs.contains_key(a) {
                                    input_idxs[a]
                                } else {
                                    nodes.len() + a
                                },
                                if input_idxs.contains_key(b) {
                                    input_idxs[b]
                                } else {
                                    nodes.len() + b
                                },
                            ),
                        );
                    }
                    Operation::Neg(a) => {
                        new_nodes.insert(
                            nodes.len() + i,
                            Operation::Neg(if input_idxs.contains_key(a) {
                                input_idxs[a]
                            } else {
                                nodes.len() + a
                            }),
                        );
                    }

                    Operation::CTrue => {
                        new_nodes.insert(nodes.len() + 1, Operation::CTrue);
                    }
                    Operation::CFalse => {
                        new_nodes.insert(nodes.len() + 1, Operation::CFalse);
                    }
                    _ => {}
                }
            });

            let new_idx = if input_idxs.contains_key(&(self.new_nodes.len() - 1)) {
                input_idxs[&(self.new_nodes.len() - 1)]
            } else {
                nodes.len() + self.new_nodes.len() - 1
            };

            for (_, node) in new_nodes.iter_mut() {
                node.change_input_nodes(idx, new_idx)
            }

            println!("{:?}", new_nodes);
            let new_out_nodes = out_nodes
                .iter()
                .map(|on| if on == &idx { new_idx } else { *on })
                .collect();

            return Some(Graph::generate(new_out_nodes, &new_nodes));
        }
        None
    }

    fn match_with_node_recursive(
        &self,
        this_idx: Idx,
        other_idx: Idx,
        input_idxs: &mut HashMap<Idx, Idx>,
        nodes: &HashMap<Idx, Operation>,
    ) -> bool {
        match self.nodes[&this_idx] {
            Operation::Input => {
                println!("Matched Input");
                if input_idxs.contains_key(&this_idx) && input_idxs[&this_idx] != other_idx {
                    return false;
                }
                input_idxs.insert(this_idx, other_idx);
                true
            }
            Operation::And(a, b) => {
                if let Operation::And(c, d) = nodes[&other_idx] {
                    if self.match_with_node_recursive(a, c, input_idxs, nodes)
                        && self.match_with_node_recursive(b, d, input_idxs, nodes)
                    {
                        return true;
                    } else if self.match_with_node_recursive(a, d, input_idxs, nodes)
                        && self.match_with_node_recursive(b, c, input_idxs, nodes)
                    {
                        return true;
                    } else {
                        return false;
                    }
                }
                false
            }
            Operation::Or(a, b) => {
                if let Operation::Or(c, d) = nodes[&other_idx] {
                    if self.match_with_node_recursive(a, c, input_idxs, nodes)
                        && self.match_with_node_recursive(b, d, input_idxs, nodes)
                    {
                        return true;
                    } else if self.match_with_node_recursive(a, d, input_idxs, nodes)
                        && self.match_with_node_recursive(b, c, input_idxs, nodes)
                    {
                        return true;
                    } else {
                        return false;
                    }
                }
                false
            }
            Operation::Neg(a) => {
                if let Operation::Neg(b) = nodes[&other_idx] {
                    return self.match_with_node_recursive(a, b, input_idxs, nodes);
                }
                false
            }
            Operation::CFalse => nodes[&other_idx] == Operation::CFalse,
            Operation::CTrue => nodes[&other_idx] == Operation::CTrue,
        }
    }
}

use std::collections::HashMap;

use crate::{Graph, Idx, Operation};

pub struct NodeMatcher {
    nodes_l: HashMap<Idx, Operation>,
    nodes_r: HashMap<Idx, Operation>,
    reversible: bool,
}

impl NodeMatcher {
    pub fn new(
        nodes_l: HashMap<Idx, Operation>,
        nodes_r: HashMap<Idx, Operation>,
        reversible: bool,
    ) -> Self {
        Self {
            nodes_l,
            nodes_r,
            reversible,
        }
    }

    pub fn match_with_node(
        &self,
        idx: Idx,
        nodes: &HashMap<Idx, Operation>,
        out_nodes: &Vec<Idx>,
    ) -> Vec<Graph> {
        let mut out: Vec<Graph> = Vec::new();

        // Check for nodes_l
        let mut input_idxs: HashMap<Idx, Idx> = HashMap::new();
        if self.match_with_node_recursive(
            self.nodes_l.len() - 1,
            idx,
            &mut input_idxs,
            nodes,
            &self.nodes_l,
        ) {
            out.push(self.generate_variant(nodes, &self.nodes_r, &input_idxs, idx, out_nodes));
        }

        // Check for nodes_r
        let mut input_idxs: HashMap<Idx, Idx> = HashMap::new();
        if self.reversible
            && self.match_with_node_recursive(
                self.nodes_r.len() - 1,
                idx,
                &mut input_idxs,
                nodes,
                &self.nodes_r,
            )
        {
            out.push(self.generate_variant(nodes, &self.nodes_l, &input_idxs, idx, out_nodes));
        }

        out
    }

    fn match_with_node_recursive(
        &self,
        this_idx: Idx,
        other_idx: Idx,
        input_idxs: &mut HashMap<Idx, Idx>,
        nodes: &HashMap<Idx, Operation>,
        match_nodes: &HashMap<Idx, Operation>,
    ) -> bool {
        match match_nodes[&this_idx] {
            Operation::Input => {
                if input_idxs.contains_key(&this_idx) && input_idxs[&this_idx] != other_idx {
                    return false;
                }
                input_idxs.insert(this_idx, other_idx);
                true
            }
            Operation::And(a, b) => {
                if let Operation::And(c, d) = nodes[&other_idx] {
                    if self.match_with_node_recursive(a, c, input_idxs, nodes, match_nodes)
                        && self.match_with_node_recursive(b, d, input_idxs, nodes, match_nodes)
                    {
                        return true;
                    } else if self.match_with_node_recursive(a, d, input_idxs, nodes, match_nodes)
                        && self.match_with_node_recursive(b, c, input_idxs, nodes, match_nodes)
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
                    if self.match_with_node_recursive(a, c, input_idxs, nodes, match_nodes)
                        && self.match_with_node_recursive(b, d, input_idxs, nodes, match_nodes)
                    {
                        return true;
                    } else if self.match_with_node_recursive(a, d, input_idxs, nodes, match_nodes)
                        && self.match_with_node_recursive(b, c, input_idxs, nodes, match_nodes)
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
                    return self.match_with_node_recursive(a, b, input_idxs, nodes, match_nodes);
                }
                false
            }
            Operation::CFalse => nodes[&other_idx] == Operation::CFalse,
            Operation::CTrue => nodes[&other_idx] == Operation::CTrue,
        }
    }

    fn generate_variant(
        &self,
        nodes: &HashMap<Idx, Operation>,
        gen_nodes: &HashMap<Idx, Operation>,
        input_idxs: &HashMap<Idx, Idx>,
        orig_idx: Idx,
        out_nodes: &Vec<Idx>,
    ) -> Graph {
        let mut new_nodes = nodes.clone();
        (0..gen_nodes.len()).for_each(|i| {
            let op = &gen_nodes[&i];
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

        let new_idx = if input_idxs.contains_key(&(gen_nodes.len() - 1)) {
            input_idxs[&(gen_nodes.len() - 1)]
        } else {
            nodes.len() + gen_nodes.len() - 1
        };

        for (_, node) in new_nodes.iter_mut() {
            node.change_input_nodes(orig_idx, new_idx)
        }

        let new_out_nodes = out_nodes
            .iter()
            .map(|on| if on == &orig_idx { new_idx } else { *on })
            .collect();

        return Graph::generate(new_out_nodes, &new_nodes);
    }
}

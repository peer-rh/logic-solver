use std::collections::HashMap;

use crate::Idx;
use crate::Operation;

#[derive(Debug, PartialEq, Eq)]

pub struct Graph {
    in_nodes: Vec<Idx>,
    nodes: HashMap<Idx, Operation>,
    keys_sorted: Vec<Idx>,
    out_nodes: Vec<Idx>,
}

fn _ggh(
    idx: Idx,
    nodes: &mut HashMap<Idx, Operation>,
    in_nodes: &mut Vec<Idx>,
    gc_hashmap: &HashMap<Idx, Operation>,
    idx_conv: &mut HashMap<Idx, Idx>,
) {
    // generate graph helper
    if !idx_conv.contains_key(&idx) {
        let operation = gc_hashmap[&idx].clone();
        if let Some(children) = operation.get_input_nodes() {
            if children.len() == 1 {
                _ggh(children[0], nodes, in_nodes, gc_hashmap, idx_conv);
            } else if children.len() == 2 {
                _ggh(children[0], nodes, in_nodes, gc_hashmap, idx_conv);
                _ggh(children[1], nodes, in_nodes, gc_hashmap, idx_conv);
            }
        } else {
            // Add to in_nodes
            in_nodes.push(nodes.len());
        }
        idx_conv.insert(idx, nodes.len());
        nodes.insert(nodes.len(), operation);
    }
}

impl Graph {
    pub fn generate(out_nodes: Vec<Idx>, node_hashmap: &HashMap<Idx, Operation>) -> Self {
        let mut nodes: HashMap<Idx, Operation> = HashMap::new();
        let mut idx_conv: HashMap<Idx, Idx> = HashMap::new();
        let mut in_nodes: Vec<Idx> = Vec::new();

        for out_node in out_nodes.iter() {
            if !idx_conv.contains_key(&out_node) {
                _ggh(
                    *out_node,
                    &mut nodes,
                    &mut in_nodes,
                    node_hashmap,
                    &mut idx_conv,
                );
            }
        }

        for i in 0..nodes.len() {
            nodes.get_mut(&i).unwrap().change_input_nodes_hs(&idx_conv);
        }

        // TODO: Obsolete
        let mut keys_sorted = Vec::from_iter(nodes.keys().cloned());
        keys_sorted.sort();

        Graph {
            in_nodes,
            nodes,
            keys_sorted,
            out_nodes: out_nodes.iter().map(|on| idx_conv[on]).collect(),
        }
    }

    pub fn evaluate(&self, feed_dict: &HashMap<Idx, bool>) -> Vec<bool> {
        let mut current_values = feed_dict.clone();
        for i in self.keys_sorted.iter() {
            let node = &self.nodes[i];
            if !current_values.contains_key(i) {
                let value = node.forward(&current_values);
                current_values.insert(*i, value);
            }
        }
        self.out_nodes.iter().map(|on| current_values[on]).collect()
    }

    pub fn is_solvable(&self, wanted_solution: Vec<bool>) -> bool {
        // Checks if there is a way where all values equal one
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) == wanted_solution {
                return true;
            }
        }
        false
    }

    pub fn solve_all_solutions(&self, wanted_solution: Vec<bool>) -> Vec<HashMap<Idx, bool>> {
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        let mut all_solution: Vec<HashMap<Idx, bool>> = Vec::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) == wanted_solution {
                all_solution.push(feed_dict.clone());
            }
        }
        all_solution
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

mod variants;

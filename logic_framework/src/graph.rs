use std::collections::{HashMap, HashSet};

use crate::{Node, Operation};

use super::{Idx, Session};

// static mut total_calls: usize = 0;
// static mut total_not_contained: usize = 0;

pub struct Graph {
    nodes: Vec<Idx>,
    out_node: Idx,
    in_nodes: Vec<Idx>,
}

// NOTE: This call takes long and isn't necessary as Index is automaticlly sorted
//
fn _dfs(idx: &Idx, nodes: &mut HashSet<Idx>, in_nodes: &mut HashSet<Idx>, session: &Session) {
    // Options to Optimize this step
    // - Make the Values Tensors
    // - Enable Shared Inputs (Cuts out on duplicate checks)
    // - Leave as is, as end goal doesn't use Tensors
    // unsafe {
    //     total_calls += 1;
    //     println!(
    //         "dfs: {}/{}, nodes_len: {}",
    //         total_not_contained,
    //         total_calls,
    //         nodes.len()
    //     );
    // }
    //
    let children = session.get_node(idx).get_input_nodes();
    if !nodes.contains(idx) {
        // unsafe {
        //     total_not_contained += 1;
        // }
        if let Some(children) = children {
            for child in children {
                _dfs(&child, nodes, in_nodes, session);
            }
        } else {
            in_nodes.insert(*idx);
        }
        nodes.insert(*idx);
    }
}

impl Graph {
    pub fn construct(out_node: Idx, session: &Session) -> Self {
        let mut nodes: HashSet<Idx> = HashSet::new();
        let mut in_nodes: HashSet<Idx> = HashSet::new();

        _dfs(&out_node, &mut nodes, &mut in_nodes, session);

        let mut nodes: Vec<Idx> = nodes.iter().cloned().collect();
        let mut in_nodes: Vec<Idx> = in_nodes.iter().cloned().collect();
        nodes.sort();
        in_nodes.sort();

        Graph {
            nodes,
            out_node,
            in_nodes,
        }
    }

    pub fn evaluate(&self, current_values: &mut HashMap<Idx, bool>, session: &Session) -> bool {
        for node in self.nodes.iter() {
            if current_values.contains_key(node) {
                continue;
            }
            let value = session.get_node(node).evaluate(current_values);
            current_values.insert(*node, value);
        }

        current_values[&self.out_node]
    }

    pub fn get_n_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_output(&self) -> Idx {
        self.out_node.clone()
    }

    pub fn get_inputs(&self) -> Vec<Idx> {
        self.in_nodes.clone()
    }

    pub fn generate_variants(&self, session: &mut Session) -> Vec<Self> {
        for i in 0..self.nodes.len() {
            let this_node = session.get_node(&self.nodes[i]);
            if find_idempotence(this_node) {
                print!("Found Idempotence")
            }
        }
        vec![]
    }
}

// find different Congruences
fn find_idempotence(node: &Node) -> bool {
    match node.get_operation() {
        Operation::LAnd(a, b) | Operation::LOr(a, b) => a == b,
        _ => false,
    }
}

fn find_commutativity(node: &Node) -> bool {
    match node.get_operation() {
        Operation::LAnd(a, b) | Operation::LOr(a, b) => a == b,
        _ => false,
    }
}

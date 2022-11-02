use std::collections::HashMap;

use crate::Idx;
use crate::Operation;

#[derive(Debug, PartialEq, Eq)]

pub struct Graph {
    in_nodes: Vec<Idx>,
    nodes: HashMap<Idx, Operation>,
    keys_sorted: Vec<Idx>,
    out_node: Idx,
}

// TODO: Make IDX Sorted
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
            in_nodes.push(idx);
        }
        idx_conv.insert(idx, nodes.len());
        nodes.insert(nodes.len(), operation);
    }
}

impl Graph {
    pub fn generate(out_node: Idx, node_hashmap: &HashMap<Idx, Operation>) -> Self {
        let mut nodes: HashMap<Idx, Operation> = HashMap::new();
        let mut idx_conv: HashMap<Idx, Idx> = HashMap::new();
        let mut in_nodes: Vec<Idx> = Vec::new();
        _ggh(
            out_node,
            &mut nodes,
            &mut in_nodes,
            node_hashmap,
            &mut idx_conv,
        );

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
            out_node: idx_conv[&out_node],
        }
    }

    pub fn evaluate(&self, feed_dict: &HashMap<Idx, bool>) -> bool {
        let mut current_values = feed_dict.clone();
        for i in self.keys_sorted.iter() {
            let node = &self.nodes[i];
            if !current_values.contains_key(i) {
                let value = node.forward(&current_values);
                current_values.insert(*i, value);
            }
        }

        current_values[&self.out_node]
    }

    pub fn is_solvable(&self) -> bool {
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) {
                return true;
            }
        }
        false
    }

    pub fn solve_all_solutions(&self) -> Vec<HashMap<Idx, bool>> {
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        let mut all_solution: Vec<HashMap<Idx, bool>> = Vec::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) {
                all_solution.push(feed_dict.clone());
            }
        }
        all_solution
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn generate_variants(&self, levels: usize) -> Vec<Self> {
        if levels == 0 {
            return vec![];
        }
        let mut all_solutions: Vec<Graph> = Vec::new();
        let mut further_solutions: Vec<Graph> = Vec::new();

        for i in self.keys_sorted.iter() {
            // Problem if one idx inside of the changes gets referenced by another variable

            if let Some(new_graph) = self.gen_absorbition(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_idempotence(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_commutativity(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_associativity(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_first_distributive_law_shrink(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_second_distributive_law_shrink(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_first_distributive_law_expand(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_second_distributive_law_expand(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_double_negation(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_de_morgan_rule_expand(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
            if let Some(new_graph) = self.gen_de_morgan_rule_shrink(i, &self.nodes) {
                all_solutions.push(new_graph);
            }
        }

        all_solutions.iter().for_each(|x| {
            let mut this_solutions = x.generate_variants(levels - 1);
            further_solutions.append(&mut this_solutions)
        });
        all_solutions.append(&mut further_solutions);

        all_solutions
    }

    fn gen_absorbition(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::Or(c, _) = nodes[&b] {
                    if c == a {
                        // Change all references to idx to a
                        let mut new_nodes = nodes.clone();
                        for (_, node) in new_nodes.iter_mut() {
                            node.change_input_nodes(*idx, a)
                        }
                        let new_out_node = if self.out_node == *idx {
                            a
                        } else {
                            self.out_node
                        };

                        return Some(Graph::generate(new_out_node, &new_nodes));
                    }
                }
                None
            }
            Operation::Or(a, b) => {
                if let Operation::And(c, _) = nodes[&b] {
                    if c == a {
                        // Change all references to idx to a
                        let mut new_nodes = nodes.clone();
                        for (_, node) in new_nodes.iter_mut() {
                            node.change_input_nodes(*idx, a)
                        }
                        let new_out_node = if self.out_node == *idx {
                            a
                        } else {
                            self.out_node
                        };

                        return Some(Graph::generate(new_out_node, &new_nodes));
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn gen_idempotence(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if a == b {
                    let mut new_nodes = nodes.clone();
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, a)
                    }
                    let new_out_node = if self.out_node == *idx {
                        a
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }
                None
            }
            Operation::Or(a, b) => {
                if a == b {
                    let mut new_nodes = nodes.clone();
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, a)
                    }
                    let new_out_node = if self.out_node == *idx {
                        a
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }
                None
            }
            _ => None,
        }
    }

    fn gen_commutativity(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                let mut new_nodes = nodes.clone();
                new_nodes.insert(*idx, Operation::And(b, a));

                return Some(Graph::generate(self.out_node, &new_nodes));
            }
            Operation::Or(a, b) => {
                let mut new_nodes = nodes.clone();
                new_nodes.insert(*idx, Operation::Or(b, a));

                return Some(Graph::generate(self.out_node, &new_nodes));
            }
            _ => None,
        }
    }

    fn gen_associativity(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::And(c, d) = nodes[&b] {
                    // a and (b and c) -> (a and b) and c
                    let mut new_nodes = nodes.clone();
                    let new_idx = new_nodes.len() - 1;
                    new_nodes.insert(new_idx + 1, Operation::And(a, c));
                    new_nodes.insert(new_idx + 2, Operation::And(new_idx + 1, d));
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 2)
                    }

                    let new_out_node = if self.out_node == *idx {
                        new_idx + 2
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));

                    // Change all references to idx to a
                }
                None
            }
            Operation::Or(a, b) => {
                if let Operation::Or(c, d) = nodes[&b] {
                    // a and (b and c) -> (a and b) and c
                    let mut new_nodes = nodes.clone();
                    let new_idx = new_nodes.len() - 1;
                    new_nodes.insert(new_idx + 1, Operation::Or(a, c));
                    new_nodes.insert(new_idx + 2, Operation::Or(new_idx + 1, d));
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 2)
                    }

                    let new_out_node = if self.out_node == *idx {
                        new_idx + 2
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));

                    // Change all references to idx to a
                }
                None
            }
            _ => None,
        }
    }

    fn gen_first_distributive_law_shrink(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::Or(a, b) => {
                if let Operation::And(c, d) = nodes[&a] {
                    if let Operation::And(e, f) = nodes[&b] {
                        if c == e {
                            // Change all references to idx to a
                            let mut new_nodes = nodes.clone();
                            let new_idx = *new_nodes.keys().max().unwrap();
                            new_nodes.insert(new_idx + 1, Operation::Or(d, f));
                            new_nodes.insert(new_idx + 2, Operation::And(c, new_idx + 1));

                            for (_, node) in new_nodes.iter_mut() {
                                node.change_input_nodes(*idx, new_idx + 2)
                            }

                            let new_out_node = if self.out_node == *idx {
                                new_idx + 2
                            } else {
                                self.out_node
                            };

                            return Some(Graph::generate(new_out_node, &new_nodes));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn gen_second_distributive_law_shrink(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::Or(c, d) = nodes[&a] {
                    if let Operation::Or(e, f) = nodes[&b] {
                        if c == e {
                            // Change all references to idx to a
                            let mut new_nodes = nodes.clone();
                            let new_idx = *new_nodes.keys().max().unwrap();
                            new_nodes.insert(new_idx + 1, Operation::And(d, f));
                            new_nodes.insert(new_idx + 2, Operation::Or(c, new_idx + 1));
                            for (_, node) in new_nodes.iter_mut() {
                                node.change_input_nodes(*idx, new_idx + 2)
                            }
                            let new_out_node = if self.out_node == *idx {
                                new_idx + 2
                            } else {
                                self.out_node
                            };

                            return Some(Graph::generate(new_out_node, &new_nodes));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn gen_first_distributive_law_expand(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::Or(c, d) = nodes[&b] {
                    // Change all references to idx to a
                    let mut new_nodes = nodes.clone();
                    let new_idx = *new_nodes.keys().max().unwrap();
                    new_nodes.insert(new_idx + 1, Operation::And(a, c));
                    new_nodes.insert(new_idx + 2, Operation::And(a, d));
                    new_nodes.insert(new_idx + 3, Operation::Or(new_idx + 1, new_idx + 2));

                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 3)
                    }

                    let new_out_node = if self.out_node == *idx {
                        new_idx + 3
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }

                None
            }
            _ => None,
        }
    }

    fn gen_second_distributive_law_expand(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::Or(a, b) => {
                if let Operation::And(c, d) = nodes[&b] {
                    // Change all references to idx to a
                    let mut new_nodes = nodes.clone();
                    let new_idx = *new_nodes.keys().max().unwrap();
                    new_nodes.insert(new_idx + 1, Operation::Or(a, c));
                    new_nodes.insert(new_idx + 2, Operation::Or(a, d));
                    new_nodes.insert(new_idx + 3, Operation::And(new_idx + 1, new_idx + 2));

                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 3)
                    }

                    let new_out_node = if self.out_node == *idx {
                        new_idx + 3
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }

                None
            }
            _ => None,
        }
    }

    fn gen_double_negation(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::Neg(a) => {
                if let Operation::Neg(b) = nodes[&a] {
                    let mut new_nodes = nodes.clone();
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, b)
                    }
                    let new_out_node = if self.out_node == *idx {
                        a
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }

                None
            }

            _ => None,
        }
    }

    fn gen_de_morgan_rule_expand(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::Neg(a) => {
                if let Operation::And(b, c) = nodes[&a] {
                    // Change all references to idx to a
                    let mut new_nodes = nodes.clone();
                    let new_idx = *self.keys_sorted.last().unwrap();
                    new_nodes.insert(new_idx + 1, Operation::Neg(b));
                    new_nodes.insert(new_idx + 2, Operation::Neg(c));
                    new_nodes.insert(new_idx + 3, Operation::Or(new_idx + 1, new_idx + 2));
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 3)
                    }
                    let new_out_node = if self.out_node == *idx {
                        new_idx + 3
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                } else if let Operation::Or(b, c) = nodes[&a] {
                    // Change all references to idx to a
                    let mut new_nodes = nodes.clone();
                    let new_idx = *self.keys_sorted.last().unwrap();
                    new_nodes.insert(new_idx + 1, Operation::Neg(b));
                    new_nodes.insert(new_idx + 2, Operation::Neg(c));
                    new_nodes.insert(new_idx + 3, Operation::And(new_idx + 1, new_idx + 2));
                    for (_, node) in new_nodes.iter_mut() {
                        node.change_input_nodes(*idx, new_idx + 3)
                    }
                    let new_out_node = if self.out_node == *idx {
                        new_idx + 3
                    } else {
                        self.out_node
                    };

                    return Some(Graph::generate(new_out_node, &new_nodes));
                }

                None
            }
            _ => None,
        }
    }

    fn gen_de_morgan_rule_shrink(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::Neg(c) = nodes[&a] {
                    if let Operation::Neg(d) = nodes[&b] {
                        // Change all references to idx to a
                        let mut new_nodes = nodes.clone();
                        let new_idx = *self.keys_sorted.last().unwrap();
                        new_nodes.insert(new_idx + 1, Operation::Or(c, d));
                        new_nodes.insert(new_idx + 2, Operation::Neg(new_idx + 1));
                        for (_, node) in new_nodes.iter_mut() {
                            node.change_input_nodes(*idx, new_idx + 2)
                        }
                        let new_out_node = if self.out_node == *idx {
                            new_idx + 2
                        } else {
                            self.out_node
                        };

                        return Some(Graph::generate(new_out_node, &new_nodes));
                    }
                }

                None
            }
            Operation::Or(a, b) => {
                if let Operation::Neg(c) = nodes[&a] {
                    if let Operation::Neg(d) = nodes[&b] {
                        // Change all references to idx to a
                        let mut new_nodes = nodes.clone();
                        let new_idx = *self.keys_sorted.last().unwrap();
                        new_nodes.insert(new_idx + 1, Operation::And(c, d));
                        new_nodes.insert(new_idx + 2, Operation::Neg(new_idx + 1));
                        for (_, node) in new_nodes.iter_mut() {
                            node.change_input_nodes(*idx, new_idx + 2)
                        }
                        let new_out_node = if self.out_node == *idx {
                            new_idx + 2
                        } else {
                            self.out_node
                        };

                        return Some(Graph::generate(new_out_node, &new_nodes));
                    }
                }

                None
            }
            _ => None,
        }
    }
}

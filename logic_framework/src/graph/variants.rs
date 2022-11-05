use crate::{Graph, Idx, Operation};
use std::collections::HashMap;

macro_rules! add_to_solutions_constructor {
    // TODO: Better method - Very Hacky
    ($idx:expr, $all_solutions:ident) => {
        macro_rules! add_to_solutions {
            ($class:ident.$func:ident) => {
                if let Some(new_graph) = $class.$func($idx, &$class.nodes) {
                    if !$all_solutions.contains(&new_graph) {
                        $all_solutions.push(new_graph);
                    }
                }
            };
        }
    };
}

impl Graph {
    // TODO Find duplicate code
    pub fn generate_variants(&self, levels: usize) -> Vec<Self> {
        let mut all_solutions = Vec::new();
        self.generate_variants_helper(&mut all_solutions);

        for _ in 1..levels {
            let len = all_solutions.len();
            let mut new_solutions = Vec::new();
            for g in 0..len {
                all_solutions[g].generate_variants_helper(&mut new_solutions);
            }
            all_solutions.append(&mut new_solutions);
            println!("{}", all_solutions.len());

            // filter variants
            let min = all_solutions
                .iter()
                .fold(all_solutions[0].len(), |run_val, g| {
                    if g.len() < run_val {
                        g.len()
                    } else {
                        run_val
                    }
                });
            all_solutions = all_solutions
                .into_iter()
                .filter(|g| g.len() <= min)
                .collect();
        }
        all_solutions
    }

    pub fn generate_variants_helper(&self, all_solutions: &mut Vec<Graph>) {
        for i in self.keys_sorted.iter() {
            // Problem if one idx inside of the changes gets referenced by another variable
            add_to_solutions_constructor!(i, all_solutions);

            add_to_solutions!(self.gen_absorbition);
            add_to_solutions!(self.gen_idempotence);
            add_to_solutions!(self.gen_commutativity);
            add_to_solutions!(self.gen_associativity);
            add_to_solutions!(self.gen_first_distributive_law_expand);
            add_to_solutions!(self.gen_second_distributive_law_expand);
            add_to_solutions!(self.gen_first_distributive_law_shrink);
            add_to_solutions!(self.gen_second_distributive_law_shrink);
            add_to_solutions!(self.gen_double_negation);
            add_to_solutions!(self.gen_de_morgan_rule_expand);
            add_to_solutions!(self.gen_de_morgan_rule_shrink);
        }
    }
    fn gen_helper(
        &self,
        old_idx: &Idx,
        new_idx: Idx,
        new_nodes: &mut HashMap<Idx, Operation>,
    ) -> Graph {
        for (_, node) in new_nodes.iter_mut() {
            node.change_input_nodes(*old_idx, new_idx)
        }
        let new_out_nodes = self
            .out_nodes
            .iter()
            .map(|on| if on == old_idx { new_idx } else { *on })
            .collect();

        return Graph::generate(new_out_nodes, &new_nodes);
    }

    fn gen_absorbition(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::Or(c, _) = nodes[&b] {
                    if c == a {
                        return Some(self.gen_helper(idx, a, &mut nodes.clone()));
                    }
                }
                None
            }
            Operation::Or(a, b) => {
                if let Operation::And(c, _) = nodes[&b] {
                    if c == a {
                        return Some(self.gen_helper(idx, a, &mut nodes.clone()));
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
                    return Some(self.gen_helper(idx, a, &mut nodes.clone()));
                }
                None
            }
            Operation::Or(a, b) => {
                if a == b {
                    return Some(self.gen_helper(idx, a, &mut nodes.clone()));
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
                return Some(self.gen_helper(idx, *idx, &mut new_nodes));
            }
            Operation::Or(a, b) => {
                let mut new_nodes = nodes.clone();
                new_nodes.insert(*idx, Operation::Or(b, a));
                return Some(self.gen_helper(idx, *idx, &mut new_nodes));
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

                    return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
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
                    return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
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
                            return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn gen_constants(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        match nodes[idx] {
            Operation::And(a, b) => {
                if let Operation::CTrue = nodes[&b] {
                    return Some(self.gen_helper(idx, a, &mut nodes.clone()));
                } else if let Operation::CFalse = nodes[&b] {
                    return Some(self.gen_helper(idx, b, &mut nodes.clone()));
                } else if let Operation::Neg(c) = nodes[&b] {
                    if a == c {
                        let mut new_nodes = nodes.clone();
                        new_nodes.insert(new_nodes.len(), Operation::CFalse);

                        return Some(self.gen_helper(idx, new_nodes.len() - 1, &mut new_nodes));
                    }
                }
                None
            }
            Operation::Or(a, b) => {
                if let Operation::CTrue = nodes[&b] {
                    return Some(self.gen_helper(idx, b, &mut nodes.clone()));
                } else if let Operation::CFalse = nodes[&b] {
                    return Some(self.gen_helper(idx, a, &mut nodes.clone()));
                } else if let Operation::Neg(c) = nodes[&b] {
                    if a == c {
                        let mut new_nodes = nodes.clone();
                        new_nodes.insert(new_nodes.len(), Operation::CTrue);

                        return Some(self.gen_helper(idx, new_nodes.len() - 1, &mut new_nodes));
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
                            return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
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

                    return Some(self.gen_helper(idx, new_idx + 3, &mut new_nodes));
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

                    return Some(self.gen_helper(idx, new_idx + 3, &mut new_nodes));
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
                    return Some(self.gen_helper(idx, b, &mut new_nodes));
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
                    return Some(self.gen_helper(idx, new_idx + 3, &mut new_nodes));
                } else if let Operation::Or(b, c) = nodes[&a] {
                    // Change all references to idx to a
                    let mut new_nodes = nodes.clone();
                    let new_idx = *self.keys_sorted.last().unwrap();
                    new_nodes.insert(new_idx + 1, Operation::Neg(b));
                    new_nodes.insert(new_idx + 2, Operation::Neg(c));
                    new_nodes.insert(new_idx + 3, Operation::And(new_idx + 1, new_idx + 2));
                    return Some(self.gen_helper(idx, new_idx + 3, &mut new_nodes));
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
                        return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
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
                        return Some(self.gen_helper(idx, new_idx + 2, &mut new_nodes));
                    }
                }

                None
            }
            _ => None,
        }
    }
}

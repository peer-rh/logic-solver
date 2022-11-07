use crate::{graph::match_nodes::NodeMatcher, Graph, Idx, Operation};
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
            add_to_solutions!(self.gen_constants);
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
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::And(0, 1)),
                (3, Operation::Or(0, 2)),
            ]),
            HashMap::from([(0, Operation::Input)]),
        );

        if let Some(out) = match_n.match_with_node(*idx, nodes, &self.out_nodes) {
            return Some(out);
        } else {
            let match_n = NodeMatcher::new(
                HashMap::from([
                    (0, Operation::Input),
                    (1, Operation::Input),
                    (2, Operation::Or(0, 1)),
                    (3, Operation::And(0, 2)),
                ]),
                HashMap::from([(0, Operation::Input)]),
            );
            return match_n.match_with_node(*idx, nodes, &self.out_nodes);
        }
    }

    fn gen_idempotence(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([(0, Operation::Input), (1, Operation::And(0, 0))]),
            HashMap::from([(0, Operation::Input)]),
        );

        if let Some(out) = match_n.match_with_node(*idx, nodes, &self.out_nodes) {
            return Some(out);
        } else {
            let match_n = NodeMatcher::new(
                HashMap::from([(0, Operation::Input), (1, Operation::Or(0, 0))]),
                HashMap::from([(0, Operation::Input)]),
            );
            return match_n.match_with_node(*idx, nodes, &self.out_nodes);
        }
    }

    fn gen_commutativity(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::And(0, 1)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::And(1, 0)),
            ]),
        );

        if let Some(out) = match_n.match_with_node(*idx, nodes, &self.out_nodes) {
            return Some(out);
        } else {
            let match_n = NodeMatcher::new(
                HashMap::from([
                    (0, Operation::Input),
                    (1, Operation::Input),
                    (2, Operation::Or(0, 1)),
                ]),
                HashMap::from([
                    (0, Operation::Input),
                    (1, Operation::Input),
                    (2, Operation::Or(1, 0)),
                ]),
            );
            return match_n.match_with_node(*idx, nodes, &self.out_nodes);
        }
    }

    fn gen_associativity(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(1, 2)),
                (4, Operation::And(0, 3)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(0, 1)),
                (4, Operation::And(3, 2)),
            ]),
        );

        if let Some(out) = match_n.match_with_node(*idx, nodes, &self.out_nodes) {
            return Some(out);
        } else {
            let match_n = NodeMatcher::new(
                HashMap::from([
                    (0, Operation::Input),
                    (1, Operation::Input),
                    (2, Operation::Input),
                    (3, Operation::Or(1, 2)),
                    (4, Operation::Or(0, 3)),
                ]),
                HashMap::from([
                    (0, Operation::Input),
                    (1, Operation::Input),
                    (2, Operation::Input),
                    (3, Operation::Or(0, 1)),
                    (4, Operation::Or(3, 2)),
                ]),
            );
            return match_n.match_with_node(*idx, nodes, &self.out_nodes);
        }
    }

    fn gen_first_distributive_law_shrink(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(0, 1)),
                (4, Operation::And(0, 2)),
                (5, Operation::Or(3, 4)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(1, 2)),
                (4, Operation::And(0, 3)),
            ]),
        );

        match_n.match_with_node(*idx, nodes, &self.out_nodes)
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
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(0, 1)),
                (4, Operation::Or(0, 2)),
                (5, Operation::And(3, 4)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(1, 2)),
                (4, Operation::Or(0, 3)),
            ]),
        );

        match_n.match_with_node(*idx, nodes, &self.out_nodes)
    }

    fn gen_first_distributive_law_expand(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(1, 2)),
                (4, Operation::Or(0, 3)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(0, 1)),
                (4, Operation::Or(0, 2)),
                (5, Operation::And(3, 4)),
            ]),
        );

        match_n.match_with_node(*idx, nodes, &self.out_nodes)
    }

    fn gen_second_distributive_law_expand(
        &self,
        idx: &Idx,
        nodes: &HashMap<Idx, Operation>,
    ) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(1, 2)),
                (4, Operation::And(0, 3)),
            ]),
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(0, 1)),
                (4, Operation::And(0, 2)),
                (5, Operation::Or(3, 4)),
            ]),
        );

        match_n.match_with_node(*idx, nodes, &self.out_nodes)
    }

    fn gen_double_negation(&self, idx: &Idx, nodes: &HashMap<Idx, Operation>) -> Option<Graph> {
        let match_n = NodeMatcher::new(
            HashMap::from([
                (0, Operation::Input),
                (1, Operation::Neg(0)),
                (2, Operation::Neg(1)),
            ]),
            HashMap::from([(0, Operation::Input)]),
        );

        match_n.match_with_node(*idx, nodes, &self.out_nodes)
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

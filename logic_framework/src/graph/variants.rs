use crate::{graph::match_nodes::NodeMatcher, Graph, Idx, Operation};
use std::collections::HashMap;

pub struct VariantGenerator {
    different_rules: Vec<NodeMatcher>,
}

impl VariantGenerator {
    pub fn new() -> Self {
        Self {
            different_rules: vec![
                // Absorbtion And
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::And(0, 1)),
                        (3, Operation::Or(0, 2)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                // Absorbtion Or
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Or(0, 1)),
                        (3, Operation::And(0, 2)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                // Associativity And
                NodeMatcher::new(
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
                    false,
                ),
                // Associativity Or
                NodeMatcher::new(
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
                    false,
                ),
                // Commutative And
                NodeMatcher::new(
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
                    false,
                ),
                // Commutative Or
                NodeMatcher::new(
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
                    false,
                ),
                // Constant True
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CTrue),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CTrue),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CTrue)]),
                    false,
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CTrue)]),
                    false,
                ),
                // Constant False
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CFalse),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CFalse)]),
                    false,
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CFalse),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CFalse)]),
                    false,
                ),
                // De morgan And
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::And(0, 1)),
                        (3, Operation::Neg(2)),
                    ]),
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Neg(0)),
                        (3, Operation::Neg(1)),
                        (4, Operation::Or(2, 3)),
                    ]),
                    true,
                ),
                // De morgan Or
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Or(0, 1)),
                        (3, Operation::Neg(2)),
                    ]),
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Neg(0)),
                        (3, Operation::Neg(1)),
                        (4, Operation::And(2, 3)),
                    ]),
                    true,
                ),
                // Double Negation
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::Neg(1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                // Idempotence And
                NodeMatcher::new(
                    HashMap::from([(0, Operation::Input), (1, Operation::And(0, 0))]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                // Idempotence Or
                NodeMatcher::new(
                    HashMap::from([(0, Operation::Input), (1, Operation::Or(0, 0))]),
                    HashMap::from([(0, Operation::Input)]),
                    false,
                ),
                // 1 dist law
                NodeMatcher::new(
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
                    true,
                ),
                // 2 dist law
                NodeMatcher::new(
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
                    true,
                ),
            ],
        }
    }

    pub fn apply(&self, idx: Idx, graph: &Graph) -> Vec<Graph> {
        let mut all_solutions: Vec<Graph> = Vec::new();
        for rule in &self.different_rules {
            all_solutions.append(&mut rule.match_with_node(idx, &graph.nodes, &graph.out_nodes));
        }

        all_solutions = all_solutions.iter().map(|g| find_duplicates(g)).collect();

        all_solutions
    }
}
fn find_duplicates(graph: &Graph) -> Graph {
    let mut new_nodes = graph.nodes.clone();
    let mut new_out_nodes = graph.out_nodes.clone();
    let mut found = true;
    while found {
        // Check if duplicate exists
        if let Some(matches) = _find_d_i(&new_nodes) {
            for (i, j) in matches {
                new_nodes.remove(&j);

                for (_, node) in new_nodes.iter_mut() {
                    node.change_input_nodes(j, i)
                }

                new_out_nodes = new_out_nodes
                    .iter()
                    .map(|on| if on == &j { i } else { *on })
                    .collect();
            }
        } else {
            found = false;
        }
    }
    Graph::generate(new_out_nodes, &new_nodes)
}
fn _find_d_i(nodes: &HashMap<Idx, Operation>) -> Option<Vec<(usize, usize)>> {
    for i in 0..(nodes.len() - 1) {
        if nodes.contains_key(&i) && nodes[&i] != Operation::Input {
            let mut matches = Vec::new();
            nodes.iter().for_each(|(j, x)| {
                if i != *j && *x == nodes[&i] {
                    matches.push((i, *j))
                }
            });
            if matches.len() > 0 {
                return Some(matches);
            }
        }
    }
    None
}

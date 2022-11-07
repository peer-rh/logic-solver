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

        // TODO: Detect Duplicates

        all_solutions
    }
}

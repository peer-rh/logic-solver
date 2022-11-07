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
                ),
                // Constant True
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CTrue),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CTrue),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CTrue)]),
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CTrue)]),
                ),
                // Constant False
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CFalse),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CFalse)]),
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::CFalse),
                        (2, Operation::Or(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                ),
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::And(0, 1)),
                    ]),
                    HashMap::from([(0, Operation::CFalse)]),
                ),
                // De morgan Expand And
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
                ),
                // De morgan Expand Or
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
                ),
                // De morgan Contract And
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Neg(0)),
                        (3, Operation::Neg(1)),
                        (4, Operation::And(2, 3)),
                    ]),
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Or(0, 1)),
                        (3, Operation::Neg(2)),
                    ]),
                ),
                // De morgan Contract Or
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::Neg(0)),
                        (3, Operation::Neg(1)),
                        (4, Operation::Or(2, 3)),
                    ]),
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Input),
                        (2, Operation::And(0, 1)),
                        (3, Operation::Neg(2)),
                    ]),
                ),
                // Double Negation
                NodeMatcher::new(
                    HashMap::from([
                        (0, Operation::Input),
                        (1, Operation::Neg(0)),
                        (2, Operation::Neg(1)),
                    ]),
                    HashMap::from([(0, Operation::Input)]),
                ),
                // Idempotence And
                NodeMatcher::new(
                    HashMap::from([(0, Operation::Input), (1, Operation::And(0, 0))]),
                    HashMap::from([(0, Operation::Input)]),
                ),
                // Idempotence Or
                NodeMatcher::new(
                    HashMap::from([(0, Operation::Input), (1, Operation::Or(0, 0))]),
                    HashMap::from([(0, Operation::Input)]),
                ),
                // 1 dist law Contract
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
                ),
                // 1 dist law Expand
                NodeMatcher::new(
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
                ),
                // 2 dist law Contract
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
                ),
                // 2 dist law Expand
                NodeMatcher::new(
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
                ),
            ],
        }
    }

    pub fn apply(&self, idx: Idx, graph: &Graph) -> Vec<Graph> {
        let mut all_solutions: Vec<Graph> = Vec::new();
        for rule in &self.different_rules {
            if let Some(g) = rule.match_with_node(idx, &graph.nodes, &graph.out_nodes) {
                all_solutions.push(g);
            }
        }

        all_solutions
    }
}

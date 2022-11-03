use std::collections::HashMap;

use crate::Idx;
use crate::Operation;

#[derive(Debug)]
pub struct GraphConstructor {
    nodes: Vec<Operation>,
}

impl GraphConstructor {
    pub fn new() -> Self {
        GraphConstructor { nodes: vec![] }
    }

    pub fn input(&mut self) -> Idx {
        self.nodes.push(Operation::Input);
        self.nodes.len() - 1
    }

    pub fn l_and(&mut self, a: Idx, b: Idx) -> Idx {
        self.nodes.push(Operation::And(a, b));
        self.nodes.len() - 1
    }

    pub fn get_hashmap(&self) -> HashMap<Idx, Operation> {
        HashMap::from_iter(self.nodes.iter().cloned().enumerate())
    }

    pub fn l_neg(&mut self, a: Idx) -> Idx {
        self.nodes.push(Operation::Neg(a));
        self.nodes.len() - 1
    }

    pub fn l_or(&mut self, a: Idx, b: Idx) -> Idx {
        self.nodes.push(Operation::Or(a, b));
        self.nodes.len() - 1
    }
}

#[derive(Debug, PartialEq, Eq)]

pub struct Graph {
    in_nodes: Vec<Idx>,
    nodes: HashMap<Idx, Operation>,
    keys_sorted: Vec<Idx>,
    out_node: Idx,
}

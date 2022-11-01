use std::collections::HashMap;

use super::{Idx, Operation};

pub struct Node {
    // idx: Idx,
    operation: Operation,
    // requires_grad: bool
}

impl Node {
    pub fn new(operation: Operation) -> Self {
        Node { operation }
    }

    pub fn evaluate(&self, current_values: &mut HashMap<Idx, bool>) -> bool {
        let out = self.operation.forward(current_values);
        out
    }

    pub fn get_input_nodes(&self) -> Option<Vec<Idx>> {
        self.operation.get_input_nodes()
    }

    pub fn get_operation(&self) -> Operation {
        self.operation
    }
}

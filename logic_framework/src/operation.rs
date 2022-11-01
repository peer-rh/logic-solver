// TODO: Think about removing Node Struct

use std::collections::HashMap;

use super::{Idx, Node, Session};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Input,
    LAnd(Idx, Idx),
    LOr(Idx, Idx),
    LNot(Idx),
    CTrue,
    CFalse,
}

impl Operation {
    pub fn get_node(&self) -> Node {
        Node::new(*self)
    }

    pub fn forward(&self, current_values: &HashMap<Idx, bool>) -> bool {
        match self {
            Self::LAnd(a, b) => current_values[a] && current_values[b],
            Self::LOr(a, b) => current_values[a] || current_values[b],
            Self::LNot(a) => !current_values[a],
            Self::Input => false, // Placeholder will not be called if initialized
            Self::CTrue => true,
            Self::CFalse => false,
        }
    }

    pub fn get_input_nodes(&self) -> Option<Vec<Idx>> {
        match self {
            Self::LAnd(a, b) => Some(vec![*a, *b]),
            Self::LOr(a, b) => Some(vec![*a, *b]),
            Self::LNot(a) => Some(vec![*a]),
            _ => None,
        }
    }
}

impl Session {
    pub fn l_and(&mut self, a: Idx, b: Idx) -> Idx {
        self.add_node(Operation::LAnd(a, b))
    }

    pub fn input(&mut self) -> Idx {
        self.add_node(Operation::Input)
    }

    pub fn l_or(&mut self, a: Idx, b: Idx) -> Idx {
        self.add_node(Operation::LOr(a, b))
    }

    pub fn l_not(&mut self, a: Idx) -> Idx {
        self.add_node(Operation::LNot(a))
    }

    pub fn l_implies(&mut self, a: Idx, b: Idx) -> Idx {
        let c = self.l_not(a);
        self.l_or(c, b)
    }

    pub fn l_iff(&mut self, a: Idx, b: Idx) -> Idx {
        let c = self.l_implies(a, b);
        let d = self.l_implies(b, a);
        self.l_and(c, d)
    }

    pub fn c_true(&mut self) -> Idx {
        self.add_node(Operation::CTrue)
    }

    pub fn c_false(&mut self) -> Idx {
        self.add_node(Operation::CFalse)
    }
}

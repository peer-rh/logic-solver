use std::collections::HashMap;

use crate::Idx;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub enum Operation {
    Input,
    And(Idx, Idx),
    Or(Idx, Idx),
    Neg(Idx),
    CTrue,
    CFalse,
}

impl Operation {
    pub fn forward(&self, current_values: &HashMap<Idx, bool>) -> bool {
        match self {
            Self::And(a, b) => current_values[a] && current_values[b],
            Self::Or(a, b) => current_values[a] || current_values[b],
            Self::Neg(a) => !current_values[a],
            Self::Input => false, // Placeholder will not be called if initialized
            Self::CTrue => true,
            Self::CFalse => false,
        }
    }

    pub fn get_input_nodes(&self) -> Option<Vec<Idx>> {
        match self {
            Self::And(a, b) => Some(vec![*a, *b]),
            Self::Or(a, b) => Some(vec![*a, *b]),
            Self::Neg(a) => Some(vec![*a]),
            _ => None,
        }
    }
    pub fn change_input_nodes(&mut self, orig: Idx, new: Idx) {
        match self {
            Self::And(a, b) | Self::Or(a, b) => {
                if *a == orig {
                    *a = new
                }
                if *b == orig {
                    *b = new
                }
            }
            Self::Neg(a) => {
                if *a == orig {
                    *a = new
                }
            }
            _ => (),
        }
    }

    pub fn change_input_nodes_hs(&mut self, conv: &HashMap<Idx, Idx>) {
        match self {
            Self::And(a, b) | Self::Or(a, b) => {
                if conv.contains_key(a) {
                    *a = conv[a]
                }
                if conv.contains_key(b) {
                    *b = conv[b]
                }
            }
            Self::Neg(a) => {
                if conv.contains_key(a) {
                    *a = conv[a]
                }
            }
            _ => (),
        }
    }
}

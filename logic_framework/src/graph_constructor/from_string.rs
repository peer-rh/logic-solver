use std::collections::HashMap;

use crate::{GraphConstructor, Idx, Operation};
use lazy_static::lazy_static;
use regex::Regex;

pub struct StringConverter {
    var_to_idx: HashMap<String, Idx>,
    gc: GraphConstructor,
}

impl StringConverter {
    pub fn new() -> Self {
        StringConverter {
            var_to_idx: HashMap::new(),
            gc: GraphConstructor::new(),
        }
    }

    pub fn get_hashmap(&self) -> HashMap<Idx, Operation> {
        self.gc.get_hashmap()
    }

    pub fn convert(&mut self, formula: &str) -> Result<Idx, String> {
        // ∧ ∨ ¬
        // && || !
        // AND OR !

        let formula_stripped = formula.replace(" ", "");

        self._convert_helper(&formula_stripped)
    }

    fn _convert_helper(&mut self, formula: &str) -> Result<Idx, String> {
        if formula.contains("=") {
            let parts: Vec<&str> = formula.split("=").collect();
            assert_eq!(parts.len(), 2);
            let rhs = self._convert_helper(parts[1])?;
            self.var_to_idx.insert(parts[0].to_string(), rhs);
            return Ok(rhs);
        }
        let starts_with_neg = formula.starts_with("!");
        let mut formula = formula;
        if starts_with_neg {
            formula = &formula[1..];
        }

        match formula.chars().next() {
            Some('(') => {
                let e_i = find_ending_paranthesis(formula)?;
                if e_i == formula.len() - 1 {
                    let mut out = self._convert_helper(&formula[1..formula.len() - 1]);
                    if starts_with_neg {
                        out = Ok(self.gc.l_neg(out?));
                    }
                    return out;
                } else {
                    println!("{} {}", formula, e_i);
                    let lhs = &formula[1..e_i];
                    let op = &formula[e_i + 1..e_i + 3];
                    let rhs = &formula[e_i + 3..];
                    let mut lhs = self._convert_helper(lhs)?;
                    if starts_with_neg {
                        lhs = self.gc.l_neg(lhs);
                    }
                    let rhs = self._convert_helper(rhs)?;
                    if op == "&&" {
                        return Ok(self.gc.l_and(lhs, rhs));
                    } else if op == "||" {
                        return Ok(self.gc.l_or(lhs, rhs));
                    } else {
                        return Err(format!("op: {} doesn't match any known Operator", op));
                    }
                }
            }
            Some('A'..='Z') | Some('a'..='z') => {
                let e_i = find_ending_parameter(formula)?;
                if e_i == formula.len() {
                    if !self.var_to_idx.contains_key(formula) {
                        self.var_to_idx.insert(formula.to_string(), self.gc.input());
                    }
                    let mut out = self.var_to_idx[formula];
                    if starts_with_neg {
                        out = self.gc.l_neg(out);
                    }
                    return Ok(out);
                } else {
                    let mut lhs = self._convert_helper(&formula[0..e_i])?;
                    let op = &formula[e_i..e_i + 2];
                    let rhs = self._convert_helper(&formula[e_i + 2..])?;
                    if starts_with_neg {
                        lhs = self.gc.l_neg(lhs);
                    }
                    if op == "&&" {
                        return Ok(self.gc.l_and(lhs, rhs));
                    } else if op == "||" {
                        return Ok(self.gc.l_or(lhs, rhs));
                    } else {
                        return Err(format!("op: {} doesn't match any known Operator", op));
                    }
                }
            }
            _ => Err("Error in match formula".to_string()),
        }
    }
}

fn find_ending_paranthesis(formula: &str) -> Result<usize, String> {
    let mut p_i = 1;
    let mut i = 0;
    let mut chars = formula.chars();
    chars.next();
    while p_i > 0 && i < formula.len() - 1 {
        let this_char = chars.next().unwrap();
        i += 1;
        if this_char == '(' {
            p_i += 1
        } else if this_char == ')' {
            p_i -= 1
        }
    }
    if p_i == 0 {
        return Ok(i);
    }
    Err("Non Matching Parantheses".to_string())
}

fn find_ending_parameter(formula: &str) -> Result<usize, String> {
    let mut i = 0;
    let mut chars = formula.chars();
    let mut next_char = chars.next();
    while next_char.is_some() && next_char.unwrap().is_alphabetic() {
        i += 1;
        next_char = chars.next();
    }
    return Ok(i);
}

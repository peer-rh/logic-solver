use std::collections::HashMap;

fn main() {
    let mut gc = GraphConstructor::new();
    let a = gc.input();
    let b = gc.input();

    let c = gc.l_and(a, b);
    let out = gc.l_and(a, c);

    let graph = Graph::generate(out, &gc);
    println!("{}", graph.is_solvable());
}

type Idx = usize;

struct GraphConstructor {
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

    pub fn get_operation(&self, idx: Idx) -> &Operation {
        &self.nodes[idx]
    }
}

struct Graph {
    in_nodes: Vec<Idx>,
    nodes: HashMap<Idx, Operation>,
    out_node: Idx,
}

fn _ggh(
    idx: Idx,
    nodes: &mut HashMap<Idx, Operation>,
    in_nodes: &mut Vec<Idx>,
    gc: &GraphConstructor,
) {
    // generate graph helper
    if !nodes.contains_key(&idx) {
        let mut a: Option<usize> = None;
        let mut b: Option<usize> = None;
        if let Some(children) = operation.get_input_nodes() {
            if children.len() == 1 {
                a = Some(_ggh(gc.get_operation(children[0]), nodes, in_nodes, gc));
            } else if children.len() == 2 {
                a = Some(_ggh(gc.get_operation(children[0]), nodes, in_nodes, gc));
                b = Some(_ggh(gc.get_operation(children[1]), nodes, in_nodes, gc));
            }
        } else {
            // Add to in_nodes
            in_nodes.push(nodes.len());
        }

        nodes.insert(idx, gc.get_operation(idx).clone());
    }
}

impl Graph {
    fn generate(out_node: Idx, gc: &GraphConstructor) -> Self {
        let mut nodes: Vec<Operation> = Vec::new();
        let mut in_nodes: Vec<Idx> = Vec::new();

        let out_node = _ggh(gc.get_operation(out_node), &mut nodes, &mut in_nodes, gc);

        Graph {
            in_nodes,
            nodes,
            out_node,
        }
    }

    fn evaluate(&self, feed_dict: &HashMap<Idx, bool>) -> bool {
        let mut current_values = feed_dict.clone();
        for (i, node) in self.nodes.iter().enumerate() {
            let value = node.forward(&current_values);
            current_values.insert(i, value);
        }

        current_values[&self.out_node]
    }

    fn is_solvable(&self) -> bool {
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) {
                return true;
            }
        }
        false
    }

    fn solve_all_solutions(&self) -> Vec<HashMap<Idx, bool>> {
        let mut feed_dict: HashMap<Idx, bool> = HashMap::new();
        let mut all_solution: Vec<HashMap<Idx, bool>> = Vec::new();
        for i in 0..(2u32.pow(self.in_nodes.len() as u32)) {
            let mut case = i;
            self.in_nodes.iter().for_each(|x| {
                feed_dict.insert(*x, case % 2 == 1);
                case /= 2
            });
            if self.evaluate(&feed_dict) {
                all_solution.push(feed_dict.clone());
            }
        }
        all_solution
    }

    fn generate_variants(levels: usize) -> Vec<Self> {
        todo!()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Operation {
    Input,
    And(Idx, Idx),
    Or(Idx, Idx),
    Neg(Idx),
}

impl Operation {
    pub fn forward(&self, current_values: &HashMap<Idx, bool>) -> bool {
        match self {
            Self::And(a, b) => current_values[a] && current_values[b],
            Self::Or(a, b) => current_values[a] || current_values[b],
            Self::Neg(a) => !current_values[a],
            Self::Input => false, // Placeholder will not be called if initialized
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
}

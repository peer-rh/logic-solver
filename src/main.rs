use std::collections::HashMap;

use logic_framework::{Graph, Idx, Session};

fn main() {
    let mut sess = Session::new();
    let a = sess.input();
    let b = sess.input();

    let out = sess.l_or(a, a);
    let graph = Graph::construct(out, &sess);
    graph.generate_variants(&mut sess);
    // let mut feed_dict = HashMap::new();
    // feed_dict.insert(a, false);
    // feed_dict.insert(b, true);

    // let solutions = solve_graph(&graph, &mut sess);
    // println!("Solutions: {:?}", solutions);
}

fn solve_graph(graph: &Graph, sess: &mut Session) -> Vec<HashMap<Idx, bool>> {
    let inputs = graph.get_inputs();
    let out = graph.get_output();
    let mut solutions: Vec<HashMap<Idx, bool>> = Vec::new();
    let mut feed_dict = HashMap::new();
    let total_cases: usize = 2usize.pow(inputs.len() as u32);
    (0..total_cases).for_each(|i| {
        let mut case = i;
        inputs.iter().for_each(|input| {
            feed_dict.insert(*input, case % 2 == 1);
            case /= 2;
        });
        sess.eval_graph(&graph, Some(feed_dict.clone()));
        if sess.get_value(&out) {
            solutions.push(feed_dict.clone());
        }
        sess.reset();
    });
    solutions
}

fn is_tautology(graph: &Graph, sess: &mut Session) -> bool {
    let inputs = graph.get_inputs();
    solve_graph(graph, sess).len() == 2usize.pow(inputs.len() as u32)
}

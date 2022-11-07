#[cfg(test)]
mod test {
    use logic_framework::{gc_macros, Graph, GraphConstructor, Operation};
    use std::collections::HashMap;

    fn test_graph(graph: &Graph, final_graph: &Graph) {
        let variants = graph.generate_variants(1);
        assert!(variants.contains(final_graph));
    }

    #[test]
    fn test_absorbtion_or() {
        let mut gc = GraphConstructor::new();
        gc_macros!(gc);

        let a = l_input!();
        let out = l_or!(a, l_and!(a, l_input!()));

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(vec![0], &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_absorbtion_and() {
        let mut gc = GraphConstructor::new();
        gc_macros!(gc);

        let a = l_input!();
        let out = l_and!(a, l_or!(a, l_input!()));

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(vec![0], &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_idempotence_or() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();

        let out = gc.l_or(a, a);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(vec![0], &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_idempotence_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();

        let out = gc.l_and(a, a);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(vec![0], &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_commutativity_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let out = gc.l_and(a, b);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![2],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::And(1, 0)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_commutativity_or() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let out = gc.l_or(a, b);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![2],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Or(1, 0)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_associativity_or() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_or(b, c);
        let out = gc.l_or(a, d);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![4],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(0, 1)),
                (4, Operation::Or(3, 2)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_associativity_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_and(b, c);
        let out = gc.l_and(a, d);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![4],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(0, 1)),
                (4, Operation::And(3, 2)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_first_diff_law_shrink() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_and(a, b);
        let e = gc.l_and(a, c);
        let out = gc.l_or(d, e);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![4],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(1, 2)),
                (4, Operation::And(0, 3)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }
    #[test]
    fn test_second_diff_law_shrink() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_or(a, b);
        let e = gc.l_or(a, c);
        let out = gc.l_and(d, e);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![4],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(1, 2)),
                (4, Operation::Or(0, 3)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_first_diff_law_expand() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_or(b, c);
        let out = gc.l_and(a, d);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![5],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::And(0, 1)),
                (4, Operation::And(0, 2)),
                (5, Operation::Or(3, 4)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_second_diff_law_expand() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();
        let c = gc.input();

        let d = gc.l_and(b, c);
        let out = gc.l_or(a, d);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![5],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(0, 1)),
                (4, Operation::Or(0, 2)),
                (5, Operation::And(3, 4)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_double_negation() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();

        let out = gc.l_neg(a);
        let out = gc.l_neg(out);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(vec![0], &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    // TODO: Test De Morgans Expand Or
    // TODO: Test De Morgans Expand And
    // TODO: Test De Morgans Shrink Or
    // TODO: Test De Morgans Shrink And

    #[test]
    fn test_duplicate() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let c = gc.l_and(a, b);
        let d = gc.l_and(a, b);
        let out = gc.l_and(c, d);

        let mut graph = Graph::generate(vec![out], &gc.get_hashmap());
        let graph_test = Graph::generate(
            vec![3],
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::And(0, 1)),
                (3, Operation::And(2, 2)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }
}

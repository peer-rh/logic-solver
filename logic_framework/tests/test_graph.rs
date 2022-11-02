#[cfg(test)]
mod test {
    use logic_framework::{Graph, GraphConstructor, Operation};
    use std::collections::HashMap;

    fn test_graph(graph: &Graph, final_graph: &Graph) {
        let variants = graph.generate_variants(1);
        assert!(variants.contains(final_graph));
    }

    #[test]
    fn test_absorbtion_or() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let c = gc.l_and(a, b);
        let out = gc.l_or(a, c);

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(0, &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_absorbtion_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let c = gc.l_or(a, b);
        let out = gc.l_and(a, c);

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(0, &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_idempotence_or() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();

        let out = gc.l_or(a, a);

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(0, &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_idempotence_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();

        let out = gc.l_and(a, a);

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(0, &HashMap::from([(0, Operation::Input)]));

        test_graph(&graph, &graph_test);
    }

    #[test]
    fn test_commutativity_and() {
        let mut gc = GraphConstructor::new();

        let a = gc.input();
        let b = gc.input();

        let out = gc.l_and(a, b);

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(
            2,
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

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(
            2,
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

        let mut graph = Graph::generate(out, &gc.get_hashmap());
        let graph_test = Graph::generate(
            4,
            &HashMap::from([
                (0, Operation::Input),
                (1, Operation::Input),
                (2, Operation::Input),
                (3, Operation::Or(0, 2)),
                (4, Operation::Or(1, 3)),
            ]),
        );

        test_graph(&graph, &graph_test);
    }
}

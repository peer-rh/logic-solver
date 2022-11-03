use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = l_input!();
    let out = l_and!(a, l_or!(a, l_neg!(l_input!())));

    let graph = Graph::generate(out, &gc.get_hashmap());
    let variants = graph.generate_variants(1);
    let smallest_variant =
        variants.iter().fold(
            &graph,
            |item, this| {
                if this.len() > item.len() {
                    item
                } else {
                    this
                }
            },
        );
    println!("{:?}", smallest_variant);
}

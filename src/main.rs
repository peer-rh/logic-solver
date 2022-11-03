use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = l_input!();
    let b = l_input!();
    let c = l_input!();

    let out = l_and!(l_and!(l_or!(a, b), a), l_or!(l_and!(b, a), c));

    let graph = Graph::generate(out, &gc.get_hashmap());
    let variants = graph.generate_variants(7);
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

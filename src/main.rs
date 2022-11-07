use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = l_input!();
    let b = l_input!();
    let c = l_input!();
    let out = l_or!(l_and!(a, b), l_and!(a, c));

    let graph = Graph::generate(vec![out], &gc.get_hashmap());
    let variants = graph.generate_variants(12);
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
    println!(
        "Starting length {}; Ending length {}",
        graph.len(),
        smallest_variant.len()
    );
    // println!("{:?}", smallest_variant);
}

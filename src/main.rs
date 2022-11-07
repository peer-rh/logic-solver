use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = gc.input();
    let b = gc.input();

    let c = gc.l_and(a, b);
    let d = gc.l_and(a, b);
    let out = gc.l_and(c, d);

    let graph = Graph::generate(vec![out], &gc.get_hashmap());
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
    println!(
        "Starting length {}; Ending length {}",
        graph.len(),
        smallest_variant.len()
    );
    // println!("{:?}", smallest_variant);
}

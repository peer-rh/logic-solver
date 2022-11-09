use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = gc.input();
    let b = gc.input();
    let c = gc.input();

    let e = l_and!(l_or!(l_neg!(a), l_neg!(b)), l_neg!(a));
    let d = l_or!(l_and!(l_neg!(b), l_neg!(a)), c);
    let f = l_and!(e, d);
    let g = l_and!(l_or!(l_neg!(a), l_neg!(b)), l_neg!(a));
    let h = l_or!(l_and!(l_neg!(b), l_neg!(a)), c);
    let j = l_and!(g, h);
    let out = l_and!(f, j);

    let graph = Graph::generate(vec![out], &gc.get_hashmap());
    let variants = graph.generate_variants(8);
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
    println!("{:?}", smallest_variant);
}

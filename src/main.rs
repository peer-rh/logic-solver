use logic_framework::{Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();

    let a = gc.input();
    let b = gc.input();
    let c = gc.input();

    let a_neg = gc.l_neg(a);
    let b_neg = gc.l_neg(b);

    let d = gc.l_or(a_neg, b_neg);
    let e = gc.l_and(a_neg, d);

    let f = gc.l_and(a_neg, b_neg);
    let g = gc.l_or(f, c);

    let out = gc.l_and(e, g);

    let graph = Graph::generate(out, &gc.get_hashmap());
    let variants = graph.generate_variants(4);
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

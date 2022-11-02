use logic_framework::{Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();

    let a = gc.input();

    let out = gc.l_neg(a);
    let out = gc.l_neg(out);

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
    println!("{:?}", variants);
}

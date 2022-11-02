use logic_framework::{Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();

    let a = gc.input();
    let b = gc.input();
    let c = gc.input();

    let d = gc.l_and(b, c);
    let out = gc.l_and(a, d);

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

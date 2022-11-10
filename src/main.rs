use logic_framework::{GenerationStrategy, Graph, OnlyKeepBest, StringConverter};

fn main() {
    let mut sc = StringConverter::new();

    let out = sc.convert("!(AAAA || (B && C))").unwrap();

    let graph = Graph::generate(vec![out], &sc.get_hashmap());
    println!("{:?}", graph);
    let variant_generator = OnlyKeepBest::new(2, 1);
    let variants = variant_generator.generate(&graph);
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

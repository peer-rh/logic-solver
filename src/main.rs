use logic_framework::{gc_macros, Graph, GraphConstructor};

fn main() {
    let mut gc = GraphConstructor::new();
    gc_macros!(gc);

    let a = l_input!();
    let b = l_input!();
    let c = l_input!();
    let d = l_input!();

    let out_1 = l_or!(l_and!(a, l_neg!(b)), l_and!(l_neg!(a), b));
    let carry_over = l_and!(a, b);
    let out_2 = l_or!(
        l_or!(
            l_or!(
                l_and!(l_and!(c, l_neg!(d)), l_neg!(l_and!(a, b))),
                l_and!(l_and!(l_neg!(c), d), l_neg!(l_and!(a, b)))
            ),
            l_and!(l_and!(l_neg!(c), l_neg!(d)), l_and!(a, b))
        ),
        l_and!(l_and!(c, d), l_and!(a, b))
    );
    let out_3 = l_or!(
        l_or!(
            l_or!(
                l_and!(l_and!(c, l_neg!(d)), l_and!(a, b)),
                l_and!(l_and!(l_neg!(c), d), l_and!(a, b))
            ),
            l_and!(l_and!(c, d), l_neg!(l_and!(a, b)))
        ),
        l_and!(l_and!(c, d), l_and!(a, b))
    );

    let graph = Graph::generate(vec![out_3], &gc.get_hashmap());
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

// function to compute fibonacci

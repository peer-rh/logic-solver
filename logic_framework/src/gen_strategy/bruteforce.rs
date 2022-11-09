use crate::{variants::VariantGenerator, Graph};

use super::GenerationStrategy;

pub struct BruteforceStrategy {
    levels: usize,
    variant_generator: VariantGenerator,
}

impl BruteforceStrategy {
    pub fn new(levels: usize) -> Self {
        BruteforceStrategy {
            levels,
            variant_generator: VariantGenerator::new(),
        }
    }

    fn gen_helper(&self, all_solutions: &mut Vec<Graph>, g: &Graph) {
        for i in 0..g.len() {
            // Problem if one idx inside of the changes gets referenced by another variable
            all_solutions.append(&mut self.variant_generator.apply(i, &g));
        }
    }
}

impl GenerationStrategy for BruteforceStrategy {
    fn generate(&self, graph: &Graph) -> Vec<Graph> {
        let mut all_solutions = Vec::new();
        self.gen_helper(&mut all_solutions, &graph);

        for _ in 1..self.levels {
            let len = all_solutions.len();
            let mut new_solutions = Vec::new();
            for g in 0..len {
                self.gen_helper(&mut new_solutions, &all_solutions[g]);
            }
            all_solutions.append(&mut new_solutions);
        }
        all_solutions
    }
}

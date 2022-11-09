use crate::{variants::VariantGenerator, Graph};

use super::GenerationStrategy;

pub struct OnlyKeepBest {
    levels: usize,
    keep_best_val: usize,
    variant_generator: VariantGenerator,
}

impl OnlyKeepBest {
    pub fn new(levels: usize, keep_best_val: usize) -> Self {
        OnlyKeepBest {
            levels,
            keep_best_val,
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

impl GenerationStrategy for OnlyKeepBest {
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

            let min = all_solutions
                .iter()
                .fold(all_solutions[0].len(), |run_val, g| {
                    if g.len() < run_val {
                        g.len()
                    } else {
                        run_val
                    }
                });
            // TODO: better path finding algorithm
            all_solutions = all_solutions
                .into_iter()
                .filter(|g| g.len() <= min + 1)
                .collect();

            let min = all_solutions
                .iter()
                .fold(all_solutions[0].len(), |run_val, g| {
                    if g.len() < run_val {
                        g.len()
                    } else {
                        run_val
                    }
                });
            all_solutions = all_solutions
                .into_iter()
                .filter(|g| g.len() <= min + self.keep_best_val)
                .collect();
        }
        all_solutions
    }
}

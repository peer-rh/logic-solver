mod bruteforce;
pub use bruteforce::BruteforceStrategy;

mod only_keep_best;
pub use only_keep_best::OnlyKeepBest;

use crate::Graph;

pub trait GenerationStrategy {
    fn generate(&self, graph: &Graph) -> Vec<Graph>;
}

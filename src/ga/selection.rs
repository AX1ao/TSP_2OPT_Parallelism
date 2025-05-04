// selection.rs
// 🎯 Implements tournament selection for choosing parents in GA.

use crate::ga::individual::Individual;
use rand::thread_rng;
use rand::Rng;

pub fn tournament_selection<'a>(population: &'a [Individual], k: usize) -> &'a Individual {
    let mut rng = thread_rng();
    let selected = (0..k)
        .map(|_| {
            let idx = rng.gen_range(0..population.len());
            &population[idx]
        })
        .collect::<Vec<_>>();

    selected
        .into_iter()
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap()
}

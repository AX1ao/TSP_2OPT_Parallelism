// crossover.rs
// 🔄 Contains crossover logic used in the Genetic Algorithm.
// Currently implements Order Crossover (OX) to combine two parent tours.

use rand::Rng;
use std::collections::HashSet;

pub fn order_crossover(parent1: &[usize], parent2: &[usize]) -> Vec<usize> {
    let len = parent1.len();
    let mut rng = rand::thread_rng();

    let mut start = rng.gen_range(0..len);
    let mut end = rng.gen_range(0..len);

    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    let mut child = vec![None; len];
    let mut used: HashSet<usize> = HashSet::new();

    // Copy the segment from parent1
    for i in start..=end {
        child[i] = Some(parent1[i]);
        used.insert(parent1[i]);
    }

    // Fill the rest from parent2, in order
    let mut p2_index = 0;
    for i in 0..len {
        if child[i].is_some() {
            continue;
        }
        while used.contains(&parent2[p2_index]) {
            p2_index += 1;
        }
        child[i] = Some(parent2[p2_index]);
        used.insert(parent2[p2_index]);
    }

    // Unwrap the final result
    child.into_iter().map(|x| x.unwrap()).collect()
}
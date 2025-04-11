/*
while improved {
    1. Generate (i, j) candidates
    2. In parallel: compute delta for all (i, j)
    3. Sort by delta descending
    4. Pick top-k *non-overlapping* swaps
    5. Apply them all
}
*/
use rayon::prelude::*;
use std::cmp::Ordering;
use crate::tsp::{City, compute_total_distance, euclidean_distance};

#[allow(dead_code)]
pub fn par_topk(tour: &[usize], cities: &[City], k: usize) -> (Vec<usize>, f64) {
    let mut improved = true;
    let mut new_tour = tour.to_vec();
    let mut loop_count = 0;

    while improved {
        loop_count += 1;
        if loop_count > 1000 {
            println!("⚠️ Breaking out to avoid infinite loop!");
            break;
        }

        improved = false;
        let n = new_tour.len();

        // Step 1: Generate all (i, j) pairs
        let candidates: Vec<(f64, usize, usize)> = (1..n - 1)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect::<Vec<_>>()
            .par_iter()
            .filter_map(|&(i, j)| {
                let a = new_tour[i - 1];
                let b = new_tour[i];
                let c = new_tour[j];
                let d = new_tour[(j + 1) % n];

                let current_dist = euclidean_distance(&cities[a], &cities[b])
                    + euclidean_distance(&cities[c], &cities[d]);
                let new_dist = euclidean_distance(&cities[a], &cities[c])
                    + euclidean_distance(&cities[b], &cities[d]);

                let delta = current_dist - new_dist;
                if delta > 1e-6 {
                    Some((delta, i, j))
                } else {
                    None
                }
            })
            .collect();

        if candidates.is_empty() {
            break;
        }

        // Step 2: Sort by best delta
        let mut sorted = candidates;
        sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

        // Step 3: Pick top-k non-overlapping swaps
        let mut applied_flags = vec![false; n];
        let mut to_apply = vec![];
        let mut count = 0;

        for &(_, i, j) in &sorted {
            if count >= k {
                break;
            }

            if applied_flags[i..=j].iter().any(|&x| x) {
                continue;
            }

            to_apply.push((i, j));
            for idx in i..=j {
                applied_flags[idx] = true;
            }

            count += 1;
        }

        // Step 4: Apply all selected swaps
        for (i, j) in to_apply {
            new_tour[i..=j].reverse();
            improved = true;
        }
    }

    let final_cost = compute_total_distance(&new_tour, cities);
    (new_tour, final_cost)
}

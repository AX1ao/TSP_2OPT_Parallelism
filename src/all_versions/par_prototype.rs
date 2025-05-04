use crate::all_versions::utils::*;
use rayon::prelude::*;
use std::cmp::Ordering;

/// Basic 2-opt Version 1️⃣
/// Parallel 2-opt prototype: Applies the best swap per iteration.
/// Uses Rayon to evaluate all candidate (i, j) pairs concurrently.
#[allow(dead_code)]
pub fn par_prototype(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
    let mut improved = true;
    let mut new_tour = tour.to_vec();
    let mut loop_count = 0;
    const DELTA_THRESHOLD: f64 = 1e-6;

    while improved {
        loop_count += 1;
        if loop_count > 1000 {
            println!("⚠️ Breaking out to avoid infinite loop!");
            break;
        }

        improved = false;
        let n = new_tour.len();
        let candidates: Vec<(usize, usize)> = (1..n - 1)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();

        if let Some((i_best, j_best)) = candidates
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

                if delta > DELTA_THRESHOLD {
                    Some((delta, i, j))
                } else {
                    None
                }
            })
            .max_by(|(da, _, _), (db, _, _)| da.partial_cmp(db).unwrap_or(Ordering::Equal))
            .map(|(_, i, j)| (i, j))
        {
            new_tour[i_best..=j_best].reverse();
            improved = true;
        }
    }

    let final_cost = compute_total_distance(&new_tour, cities);
    (new_tour, final_cost)
}

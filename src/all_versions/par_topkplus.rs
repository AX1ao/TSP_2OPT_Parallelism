use crate::all_versions::utils::*;
use rayon::prelude::*;
use std::cmp::Ordering;

/// Basic 2-opt Version 3️⃣
/// Top-K++ Smart Parallel 2-Opt with Early Cutoff and Deferred Application
/// 1. Filters candidate swaps by `delta_thresh`
/// 2. Selects top-k non-overlapping swaps
/// 3. Only commits changes if the full tour improves
#[allow(dead_code)]
pub fn par_topkplus(tour: &[usize], cities: &[City], k: usize, delta_thresh: f64) -> (Vec<usize>, f64) {
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

        // Step 1: Evaluate (i, j) pairs in parallel and filter by delta_thresh
        let mut candidates: Vec<(f64, usize, usize)> = (1..n - 1)
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
                if delta > delta_thresh {
                    Some((delta, i, j))
                } else {
                    None
                }
            })
            .collect();

        if candidates.is_empty() {
            break;
        }

        // Step 2: Sort by delta (descending)
        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

        // Step 3: Select top-k non-overlapping swaps
        let mut applied_flags = vec![false; n];
        let mut to_apply = vec![];
        let mut count = 0;

        for &(_, i, j) in &candidates {
            if count >= k {
                break;
            }
            if applied_flags[i..=j].iter().any(|&used| used) {
                continue;
            }
            to_apply.push((i, j));
            for idx in i..=j {
                applied_flags[idx] = true;
            }
            count += 1;
        }

        // Step 4: Simulate applying swaps and only commit if overall tour improves
        let mut simulated_tour = new_tour.clone();
        for (i, j) in &to_apply {
            simulated_tour[*i..=*j].reverse();
        }

        let new_cost = compute_total_distance(&simulated_tour, cities);
        let old_cost = compute_total_distance(&new_tour, cities);

        if new_cost < old_cost {
            new_tour = simulated_tour;
            improved = true;
        }
    }

    let final_cost = compute_total_distance(&new_tour, cities);
    (new_tour, final_cost)
}

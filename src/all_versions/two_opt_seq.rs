use crate::all_versions::utils::*;

/// 0️⃣ Standard sequential 2-opt algorithm for TSP
/// Iteratively reverses segments to reduce total tour distance.
pub fn two_opt_seq(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
    let mut improved = true;
    let mut new_tour = tour.to_vec();

    while improved {
        improved = false;

        for i in 1..new_tour.len() - 1 {
            for j in i + 1..new_tour.len() {
                let a = new_tour[i - 1];
                let b = new_tour[i];
                let c = new_tour[j];
                let d = new_tour[(j + 1) % new_tour.len()];

                let current_dist = euclidean_distance(&cities[a], &cities[b])
                    + euclidean_distance(&cities[c], &cities[d]);
                let new_dist = euclidean_distance(&cities[a], &cities[c])
                    + euclidean_distance(&cities[b], &cities[d]);

                if new_dist < current_dist {
                    new_tour[i..=j].reverse();
                    improved = true;
                }
            }
        }
    }

    let final_cost = compute_total_distance(&new_tour, cities);
    (new_tour, final_cost)
}

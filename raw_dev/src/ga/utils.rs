// utils.rs
// 🧰 Utility functions used by the GA (e.g., 2-opt local refinement).
// (Assumed to exist from context, though contents were not shown.)

use crate::City;

pub fn two_opt(tour: &[usize], cities: &[City]) -> Vec<usize> {
    let mut improved = true;
    let mut new_tour = tour.to_vec();
    while improved {
        improved = false;
        for i in 1..new_tour.len() - 2 {
            for j in i + 1..new_tour.len() - 1 {
                let a = new_tour[i - 1];
                let b = new_tour[i];
                let c = new_tour[j];
                let d = new_tour[(j + 1) % new_tour.len()];
                let before = euclidean_distance(&cities[a], &cities[b])
                           + euclidean_distance(&cities[c], &cities[d]);
                let after = euclidean_distance(&cities[a], &cities[c])
                          + euclidean_distance(&cities[b], &cities[d]);
                if after < before {
                    new_tour[i..=j].reverse();
                    improved = true;
                }
            }
        }
    }
    new_tour
}

fn euclidean_distance(a: &City, b: &City) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

mod ga;
mod two_opt;

use crate::ga::ga::run_ga_parallel;
use crate::two_opt::par_topkplus::par_topkplus;

use rand::{Rng, SeedableRng};
use std::time::Instant;
use crate::two_opt::tsp::{City, compute_total_distance};

pub fn generate_seeded_cities(n: usize, seed: u64) -> Vec<City> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| City {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
        })
        .collect()
}
fn main() {
    let n = 1000;
    let seed = 121;
    let cities = generate_seeded_cities(n, seed);
    let tour: Vec<usize> = (0..n).collect();

    // --- Top-K++ Benchmark ---
    let start = Instant::now();
    let (_, final_cost) = par_topkplus(&tour, &cities[..], 10, 1e-5); // k=10, delta=1e-5
    let duration = start.elapsed();
    println!("=== Top-K++ 2-Opt (k=10, delta=1e-5) ===");
    println!("Final cost: {:.2}", final_cost);
    println!("Time: {:.2?}", duration);

    // --- Parallel GA Benchmark ---
    let start = Instant::now();
    let best = run_ga_parallel(&cities, 300, 1000, 0.10, 2, 300, 100, 10);
    let duration = start.elapsed();
    println!("\n=== Parallel GA ===");
    println!("Final cost: {:.2}", best.distance());
    println!("Time: {:.2?}", duration);
}

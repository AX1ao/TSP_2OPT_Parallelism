mod ga;
use crate::ga::ga::{run_ga, run_ga_config, run_ga_baseline, run_ga_parallel};
use rand::{Rng, SeedableRng};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

pub fn generate_cities(n: usize, seed: u64) -> Vec<City> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| City {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
        })
        .collect()
}

pub fn compute_total_distance(tour: &[usize], cities: &[City]) -> f64 {
    let mut dist = 0.0;
    for i in 0..tour.len() {
        let from = &cities[tour[i]];
        let to = &cities[tour[(i + 1) % tour.len()]];
        dist += ((from.x - to.x).powi(2) + (from.y - to.y).powi(2)).sqrt();
    }
    dist
}

// for general comparison
/*
fn main() {
    let sizes = [50, 100, 200];
    let seeds = [42, 123];

    for &n in &sizes {
        for &seed in &seeds {
            println!("================ n = {}, seed = {} ================", n, seed);
            let cities = generate_seeded_cities(n, seed);

            // --- Version 1: Baseline GA ---
            let start = Instant::now();
            let best = run_ga_baseline(&cities, 300, 1000, 0.05);
            let dur = start.elapsed();
            println!("🧪 Baseline GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            // --- Version 2: GA with config ---
            let start = Instant::now();
            let best = run_ga_config(&cities, 300, 1000, 0.10, 2, 300, 100, 10);
            let dur = start.elapsed();
            println!("🧪 Configurable GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            // --- Version 3: Final Delayed 2-Opt ---
            let start = Instant::now();
            let best = run_ga(&cities, 300, 1000, 0.10); // final delayed version
            let dur = start.elapsed();
            println!("🧪 Delayed 2-Opt GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            // --- Version 4: Parallel GA ---
            let start = Instant::now();
            let best = run_ga_parallel(&cities, 300, 1000, 0.10, 2, 300, 100, 10);
            let dur = start.elapsed();
            println!("🧪 Parallel GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            println!();
        }
    }
}
*/

// for multiple problem sizes
fn main() {
    let sizes = [50, 100, 200, 500, 1000];
    let seeds = [42, 123];

    for &n in &sizes {
        for &seed in &seeds {
            println!("================ n = {}, seed = {} ================", n, seed);
            let cities = generate_cities(n, seed);

            // --- Version 1: Delayed 2-Opt GA ---
            let start = Instant::now();
            let best = run_ga(&cities, 300, 1000, 0.10); // final delayed version
            let dur = start.elapsed();
            println!("🧪 Delayed 2-Opt GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            // --- Version 2: Parallel GA ---
            let start = Instant::now();
            let best = run_ga_parallel(&cities, 300, 1000, 0.10, 2, 300, 100, 10);
            let dur = start.elapsed();
            println!("🧪 Parallel GA: dist = {:.2}, time = {:.2?}", best.distance(), dur);

            println!();
        }
    }
}
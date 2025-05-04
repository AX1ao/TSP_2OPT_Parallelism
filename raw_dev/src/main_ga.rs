mod ga;

use crate::ga::run_ga_parallel;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

pub fn generate_random_cities(n: usize) -> Vec<City> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| City {
        x: rng.gen_range(0.0..100.0),
        y: rng.gen_range(0.0..100.0),
    }).collect()
}

pub fn compute_total_distance(tour: &[usize], cities: &[City]) -> f64 {
    let mut dist = 0.0;
    for i in 0..tour.len() {
        let from = &cities[tour[i]];
        let to = &cities[tour[(i + 1) % tour.len()]];
        dist += euclidean_distance(from, to);
    }
    dist
}

fn euclidean_distance(a: &City, b: &City) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let cities = generate_random_cities(100); // adjust size as needed

    // ✅ Version 3: delayed 2-opt with final config
    let best = run_ga_parallel(
        &cities,     // cities
        300,         // population size
        1000,        // generations
        0.10,        // base mutation rate
        2,           // elitism_k
        300,         // refine_start (delay 2-opt until gen 300)
        100,         // refine_every (apply 2-opt every 100 gens)
        10           // top_n individuals to refine
    );

    println!("Best distance: {:.4}", best.distance());
    println!("Tour: {:?}", best.tour);
}

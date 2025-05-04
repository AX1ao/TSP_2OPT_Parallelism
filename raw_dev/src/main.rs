mod ga;
use rand::Rng;
use ga::run_ga;

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

fn sample_cities() -> Vec<City> {
    vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 1.0, y: 1.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 0.5, y: 0.5 },
    ]
}

fn main() {
    //let cities = sample_cities();
    let cities = generate_random_cities(100); // or 200, etc.

    /*
    Experiment with:
    More generations (e.g., 1000)
    Larger population sizes (e.g., 300)
    Slightly higher mutation rate (e.g., 0.1)
    Use elitism with top-2 or top-5 instead of just the best
    */
    let best = run_ga(
        &cities,         // cities
        300,             // population size
        1000,             // generations
        0.05             // mutation rate
    );

    println!("Best distance: {}", best.distance());
    println!("Tour: {:?}", best.tour);
}

use rand::prelude::*;

pub struct City {
    pub x: f64,
    pub y:f64,
}

pub fn generate_cities(n: usize) -> Vec<City> {
    let mut rng = thread_rng();
    (0..n)
        .map(|_| City {
            x: rng.gen_range(0.0..1000.0),
            y: rng.gen_range(0.0..1000.0)
        })
        .collect()
}

pub fn compute_total_distance(tour: &[usize], cities: &[City]) -> f64{
    let mut total = 0.0;
    for i in 0..tour.len(){
        let from = &cities[tour[i]];
        let to = &cities[tour[(i + 1) % tour.len()]];
        total += euclidean_distance(from, to);
    }
    total
}

pub fn euclidean_distance(a: &City, b: &City) -> f64{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn shuffle_tour(tour: &mut Vec<usize>) {
    let mut rng = thread_rng();
    tour.shuffle(&mut rng);
}

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
mod tsp;
mod two_opt_seq;
mod two_opt_par;

use tsp::*;
use two_opt_seq::*;
use two_opt_par::*;

fn main() {
    let num_cities = 50;
    let cities = generate_cities(num_cities);

    // Random tour
    let mut tour: Vec<usize> = (0..num_cities).collect();
    shuffle_tour(&mut tour);

    println!("Initial tour cost: {:.2}", compute_total_distance(&tour, &cities));

    let start = std::time::Instant::now();
    let (optimized_tour, final_cost) = two_opt_seq(&tour, &cities);
    let duration = start.elapsed();

    println!("Final tour cost: {:0.2}", final_cost);
    println("Time taken: {:.2}", duration);
}

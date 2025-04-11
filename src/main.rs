mod tsp;
mod par;
mod par_prototype;
mod par_topk;
mod par_topkplus;

use tsp::*;
#[allow(unused_imports)]
use par::*;
#[allow(unused_imports)]
use par_prototype::*;
#[allow(unused_imports)]
use par_topk::*;
#[allow(unused_imports)]
use par_topkplus::*;


use std::env;

fn main() {
    // ======== CORRECTNESS TEST (n = 5) =============
    /*
    let cities = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.5, y: 0.5 },
    ];

    let mut tour = vec![0, 1, 2, 3, 4]; // ordered tour
    shuffle_tour(&mut tour); // or comment out for fixed test

    println!("🧪 [Correctness Test]");
    println!("Initial tour: {:?}", tour);
    println!("Initial cost: {:.2}", compute_total_distance(&tour, &cities));

    let (opt_tour, cost) = two_opt_seq(&tour, &cities);

    println!("Optimized tour: {:?}", opt_tour);
    println!("Final cost: {:.2}\n", cost);

    // ======== CONSISTENCY TEST (n = 50) ============
    let n = 50;
    let cities = generate_cities(n);
    let mut tour = (0..n).collect::<Vec<_>>();
    shuffle_tour(&mut tour);

    let original_cost = compute_total_distance(&tour, &cities);
    let (opt_tour, final_cost) = two_opt_seq(&tour, &cities);

    println!("🧪 [Consistency Test]");
    println!("Original cost: {:.2}", original_cost);
    println!("Final cost: {:.2}", final_cost);

    assert_eq!(opt_tour.len(), n);               // All cities visited
    assert!(final_cost <= original_cost);        // Should not get worse
    */

    // ======== CLI VERSION (optional later) ============
    let args: Vec<String> = env::args().collect();
    let num_cities = args.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50);

    println!("Generating {} cities...", num_cities);
    let cities = generate_cities(num_cities);

    let mut tour: Vec<usize> = (0..num_cities).collect();
    shuffle_tour(&mut tour);

    println!("Initial tour cost: {:.2}", compute_total_distance(&tour, &cities));

    // SEQ VERSION
    let start = std::time::Instant::now();
    let (_, final_cost) = two_opt_seq(&tour, &cities);
    // if need to check tour detail, then use the line below:
    // let (optimized_tour, final_cost) = two_opt_seq(&tour, &cities);
    let duration = start.elapsed();
    println!("Sequential Version:");
    println!("Final tour cost: {:.2}", final_cost);
    println!("Time taken: {:.2?}", duration);

    // Prototype Version
    /* 
    let start = std::time::Instant::now();
    let (_, final_cost) = par_prototype(&tour, &cities);
    let duration = start.elapsed();
    println!("Parallel Version:");
    println!("Final tour cost: {:.2}", final_cost);
    println!("Time taken: {:.2?}", duration);
    */

    // Top K Batches Version
    /*
    let start = std::time::Instant::now();
    let k = 5; // and 2, 3, 10, etc.
    let (_, final_cost) = par_topk(&tour, &cities, k);
    let duration = start.elapsed();
    println!("Parallel Version k = 5:");
    println!("Final tour cost: {:.2}", final_cost);
    println!("Time taken: {:.2?}", duration);
    */

    // Top K Plus VERSION
    let k_values = [2, 3, 5, 10];
    let delta_thresh_values = [1e-6, 1e-5, 1e-4];

    for &k in &k_values {
        for &delta_thresh in &delta_thresh_values {
            let start = std::time::Instant::now();
            let (_, final_cost) = par_topkplus::par_topkplus(&tour, &cities, k, delta_thresh);
            let duration = start.elapsed();

            println!("Parallel TopK++ | k = {}, delta_thresh = {:.0e}", k, delta_thresh);
            println!("Final tour cost: {:.2}", final_cost);
            println!("Time taken: {:.2?}\n", duration);
        }
    }


}
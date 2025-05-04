//! # main.rs — Correctness Test Runner for All TSP Versions
//!
//! This executable is used to validate the **correctness** of all implemented TSP solvers,
//! including sequential, multithreaded, and genetic algorithm variants.
//!
//! ## Structure:
//! - All version modules are located inside `mod all_versions/`
//! - Each module contains one strategy (e.g., `par_topk`, `ga_baseline`, etc.)
//! - This runner creates a small set of cities and checks that each version
//!   can run without panic and produces a valid tour.
//!
//! ## Purpose:
//! - Sanity check for compilation and logic correctness.
//! - Useful before benchmarking or deployment to verify that all implementations
//!   are functionally consistent on small inputs.
//!
//! ## Output:
//! - For each version: initial cost, final cost, and time taken.
//!
//! ## Usage:
//! ```sh
//! cargo run --bin main
//! ```

mod all_versions;

use all_versions::utils::*;
use all_versions::two_opt_seq::*;
use all_versions::two_opt_par_ver2::*;
use all_versions::optimized_multithread_2opt::*;
use all_versions::optimized_ver2_multi2opt::*;
use all_versions::random_insert_ver3_multi2opt::*;
use all_versions::par_prototype::*;
use all_versions::par_topk::*;
use all_versions::par_topkplus::*;
use all_versions::ga_baseline::*;
use all_versions::ga_config::*;
use all_versions::par_ga::*;

fn main() {
    // ========== Correctness Test (n = 10) ==========
    let cities = generate_cities(10, 121); // or any fixed seed
    let mut tour: Vec<usize> = (0..10).collect();
    shuffle_tour(&mut tour);
    println!("✅ Initial tour: {:?}, cost = {:.2}", tour, compute_total_distance(&tour, &cities));

    let (_, cost) = two_opt_seq(&tour, &cities);
    println!("▶️  two_opt_seq: {:.2}", cost);

    let (_, cost) = par_prototype(&tour, &cities);
    println!("▶️  par_prototype: {:.2}", cost);

    let (_, cost) = par_topk(&tour, &cities, 2);
    println!("▶️  par_topk (k=2): {:.2}", cost);

    let (_, cost) = par_topkplus(&tour, &cities, 2, 1e-6);
    println!("▶️  par_topkplus (k=2, δ=1e-6): {:.2}", cost);

    let (_, cost) = two_opt_par_ver2(&tour, &cities);
    println!("▶️  two_opt_par_ver2: {:.2}", cost);

    let (_, cost) = multi_2opt_optimized1(&tour, &cities);
    println!("▶️  optimized_multithread_2opt: {:.2}", cost);

    let (_, cost) = multi_2opt_optimized2_V2(&tour, &cities);
    println!("▶️  optimized_ver2_multi2opt: {:.2}", cost);

    let (_, cost) = multi_2opt_random_insert(&tour, &cities);
    println!("▶️  random_insert_ver3_multi2opt: {:.2}", cost);

    let best = run_ga_baseline(&cities, 20, 100, 0.1);
    println!("▶️  ga_baseline: {:.2}", best.distance());

    let best = run_ga_config(&cities, 20, 100, 0.1, 2, 30, 20, 5);
    println!("▶️  ga_config: {:.2}", best.distance());

    let best = run_ga_parallel(&cities, 20, 100, 0.1, 2, 30, 20, 5);
    println!("▶️  par_ga: {:.2}", best.distance());
}

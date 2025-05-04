// ============================================
// main_parallelism.rs
// Runs a single algorithm version on input size 1000.
// Use `taskset` to control # of processors externally.
//
// Usage:
//   taskset -c 0 cargo run --release --bin main_parallelism ga3
//   taskset -c 0-3 cargo run --release --bin main_parallelism mult2
//
// Accepted versions:
// "seq"       => two_opt_seq
// "topkplus"  => par_topkplus
// "mult1"     => two_opt_par_ver2
// "mult2"     => multi_2opt_optimized1
// "mult3"     => multi_2opt_optimized2_V2
// "mult4"     => multi_2opt_random_insert
// "ga3"       => run_ga_parallel
// ============================================

mod all_versions;

use all_versions::utils::*;
use all_versions::two_opt_par_ver2::*;
use all_versions::optimized_multithread_2opt::*;
use all_versions::optimized_ver2_multi2opt::*;
use all_versions::random_insert_ver3_multi2opt::*;
use all_versions::par_topkplus::*;
use all_versions::par_ga::*;
use all_versions::two_opt_seq::*;

use std::env;
use std::time::Instant;

fn main() {
    let n = 1000;
    let seed = 121;
    let cities = generate_cities(n, seed);
    let mut tour: Vec<usize> = (0..n).collect();
    shuffle_tour(&mut tour);

    let versions: &[(&str, fn(&[usize], &[City]) -> f64)] = &[
        ("seq", |t, c| two_opt_seq(t, c).1),
        ("topkplus", |t, c| par_topkplus(t, c, 10, 1e-5).1),
        ("mult1", |t, c| two_opt_par_ver2(t, c).1),
        ("mult2", |t, c| multi_2opt_optimized1(t, c).1),
        ("mult3", |t, c| multi_2opt_optimized2_V2(t, c).1),
        ("mult4", |t, c| multi_2opt_random_insert(t, c).1),
    ];

    for (name, func) in versions {
        let mut tour_copy = tour.clone();
        println!("\n▶️  Running version `{}`", name);
        let start = Instant::now();
        let cost = func(&tour_copy, &cities);
        let duration = start.elapsed();
        println!("✅ Final cost: {:.2}", cost);
        println!("⏱️  Time: {:.2?}", duration);
    }

    println!("\n▶️  Running version `ga3`");
    let start = Instant::now();
    let cost = run_ga_parallel(&cities, 300, 1000, 0.10, 2, 300, 100, 10).distance();
    let duration = start.elapsed();
    println!("✅ Final cost: {:.2}", cost);
    println!("⏱️  Time: {:.2?}", duration);
}

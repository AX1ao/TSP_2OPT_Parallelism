// ============================================
// main_scalability.rs
// Runs scalability tests across input sizes for a specified algorithm version.
// 
// Versions include: 
// "seq" => two_opt_seq, 
// "prototype" => par_prototype,
// "topk" => par_topk,
// "topkplus" => par_topkplus, 
// "mult1" => two_opt_par_ver2,
// "mult2" => multi_2opt_optimized1, 
// "mult3" => multi_2opt_optimized2_V2,
// "mult4" => multi_2opt_random_insert, 
// "ga1" => run_ga_baseline,
// "ga2" => run_ga_config, 
// "ga3" => run_ga_parallel
//
// Usage (example):
//   cargo run --bin main_scalability topk
//
// Tests with sizes: [50, 100, 200, 500, 1000]
// ============================================

mod all_versions;

use all_versions::utils::*;
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
use all_versions::two_opt_seq::*;

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = args.get(1).map(String::as_str).unwrap_or("seq");

    let sizes = [50, 100, 200, 500, 1000];
    let seed = 121;

    println!("Running scalability test for version: `{}`", version);

    for &n in &sizes {
        println!("\nProblem size: {n}");
        let cities = generate_cities(n, seed);
        let mut tour: Vec<usize> = (0..n).collect();
        shuffle_tour(&mut tour);

        let start = Instant::now();
        let cost = match version {
            "seq" => two_opt_seq(&tour, &cities).1,
            "prototype" => par_prototype(&tour, &cities).1,
            "topk" => par_topk(&tour, &cities, 2).1,
            "topkplus" => par_topkplus(&tour, &cities, 2, 1e-6).1,
            "mult1" => two_opt_par_ver2(&tour, &cities).1,
            "mult2" => multi_2opt_optimized1(&tour, &cities).1,
            "mult3" => multi_2opt_optimized2_V2(&tour, &cities).1,
            "mult4" => multi_2opt_random_insert(&tour, &cities).1,
            "ga1" => run_ga_baseline(&cities, 100, 300, 0.1).distance(),
            "ga2" => run_ga_config(&cities, 100, 300, 0.1, 5, 100, 100, 10).distance(),
            "ga3" => run_ga_parallel(&cities, 100, 300, 0.1, 5, 100, 100, 10).distance(),
            _ => {
                eprintln!("❌ Unknown version: `{}`", version);
                return;
            }
        };
        let duration = start.elapsed();

        println!("✅ Final cost: {:.2}", cost);
        println!("⏱️ Time: {:.2?}", duration);
    }
}
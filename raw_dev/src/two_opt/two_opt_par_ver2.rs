use rayon::prelude::*;
use std::cmp::Ordering;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::tsp::{City, compute_total_distance, euclidean_distance};
// high level idea of how to optimize the original parallel version of 2-opt

// 1. firstly, Generate all possible (i, j) 2-opt candidates parallely before spawning
// multithreads
// Spawn a fixed number of threads, may be change later,
// --  for Each thread:

//        1. Generates a shuffled tour
//        2. Applies all (i, j) swaps to its tour
//        3. Picks the one with the shortest distance
//        4. Compare results from all threads
//        5. Update the global best tour

// After that, we can do some extra computation to get better result

// TODO: if there is a way to parallize the calculation of the total distance among the current shuffle
fn get_tot_dis(tour: &[usize], cities: &[City]) -> f64 {
    // using the windown to generate an iterator to split the array into 2 subarry
    let dist_sum: f64 = tour
        .windows(2) 
        .map(|w| euclidean_distance(&cities[w[0]], &cities[w[1]]))
        .sum();

    let return_dist = euclidean_distance(&cities[tour[tour.len() - 1]], &cities[tour[0]]);
    dist_sum + return_dist
}

fn swap2edges(tour: &mut [usize], i: usize, j: usize) {
    tour[i..=j].reverse();
}
pub fn two_opt_par_ver2(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {

    // to make the random generator for each pair of (i, j) that i < j and also within the 
    // range of the cities

    // basically, with the help of rayron, we can generate the pairs by fixing the j value,
    // so that each thread get their own value of i, then generate its related (i, j) pairs
    
    // the threads won't overlap with each other 
    
    let n = tour.len();
    let possibilities: Vec<(usize, usize)> = (1..n - 1)
            .into_par_iter()
            .flat_map_iter(|i| (i + 1..n).map(move |j| (i, j)))
            .collect(); 
    
    let repeat_times = 32;
    let sample_size = n * 2;
    // the number of threads can be changed later, the original number of threads is 16
    let better_res = (0..32)
    .into_par_iter()
    .map(|_| {
        let mut rng = thread_rng(); // thread specific random number generator to get a random seeds
        // with the help of this generator, we may get different shuffle value of the array
        let mut tour: Vec<usize> = (0..n).collect();
        // 2 ways of shuffering the original vector
        // fixed the first element, which means the start pos of the travel is fixed
        // tour[1..].shuffle(&mut rng); 
        // another way is we can choose any city as the start point, in the case of the traverller
        // will return to this city at the end of the trip
        tour.shuffle(&mut rng); 
        
        let mut route = tour.clone(); // original shuffle for current thread
        let mut dis = get_tot_dis(&route, &cities);
        for _ in 0..repeat_times {
            let sampled = possibilities.choose_multiple(&mut rng, sample_size)
                            .cloned()
                            .collect::<Vec<_>>();
            for &(i, j) in &sampled {
                let mut new_route = route.clone();
                swap2edges(&mut new_route, i, j);
                let new_dis = get_tot_dis(&new_route, &cities);
                if dis - new_dis > 1.0 {
                    dis = new_dis;
                    route = new_route;
                }
            }
        }
        (route, dis)
    })
    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .unwrap();
    
    better_res
}

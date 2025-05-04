use rayon::prelude::*;
use std::cmp::Ordering;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::tsp::{City, compute_total_distance, euclidean_distance};

// what about construct some route that may already have some local optimization

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

fn get_initial_route(remain: Vec<usize>, cities: &[City]) -> Vec<usize> {
    // a randomized start_position to start a greedy algotrithm to get the initial route
    let n = cities.len();

    // start with first several points
    // the initial set can be adjust to smaller set, but the idea is that make sure
    // there is less possibility that some of the threads will get the same initial set
    // then they will do some redundent job
    let mut initial_route: Vec<usize> = remain[..n / 100].to_vec();
    
    for &city in &remain[n / 100..] {
        // find the best position to insert the city
        let mut best_pos = 0;
        let mut lowest_cost = f64::INFINITY;

        for i in 0..(initial_route.len() + 1) {
            // consider add the city to the start point
            let new_cost = if i == 0 {
                euclidean_distance(&cities[city], &cities[initial_route[0]])
            } else if i == initial_route.len() { // add the city to the tail
                euclidean_distance(&cities[initial_route[initial_route.len() - 1]], &cities[city])
            } else {
                // add the city between 2 original cities
                let prev = initial_route[i - 1];
                let next = initial_route[i];
                euclidean_distance(&cities[prev], &cities[city])
                    + euclidean_distance(&cities[city], &cities[next])
                    - euclidean_distance(&cities[prev], &cities[next])
            };

            if new_cost < lowest_cost {
                lowest_cost = new_cost;
                best_pos = i;
            }
        }

        initial_route.insert(best_pos, city);
    }

    initial_route
}
pub fn multi_2opt_random_insert(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
    let n = tour.len();
    let possibilities: Vec<(usize, usize)> = (1..n - 1)
            .into_par_iter()
            .flat_map_iter(|i| (i + 1..n).map(move |j| (i, j)))
            .collect(); 
    
    let repeat_times = 10;
    let sample_size = possibilities.len() / 2;
    // the number of threads can be changed later, the original number of threads is 16
    let better_res = (0..32)
    .into_par_iter()
    .map(|_| {
        
        // we may construct a better initial route using randon insertion
        // and to ensure that all the threads are optimized themselves without
        // doing redundent works, we may also do some randomize work on the start set
        let mut rng = thread_rng();
        let mut tmp = (0..n).collect::<Vec<_>>();
        tmp.shuffle(&mut rng);
        let mut initial_route = if rng.gen_bool(0.5) {
            tmp
        } else {
            get_initial_route(tmp, &cities)
        };
        
        let mut route = initial_route.clone(); // original shuffle for current thread
        let mut dis = get_tot_dis(&route, &cities);
        while true { // continue using the repeated sampled based method
            
            let sampled = possibilities.choose_multiple(&mut rng, sample_size)
                            .cloned()
                            .collect::<Vec<_>>();

            let mut can_modify = vec![];

            for &(i, j) in &sampled {
                if i == 0 || j + 1 >= n { continue; }
               
                // in the previous version, if we compute the total_dis everytime we swap 2 edges is too costly. 
                // And since swap 2 edges only change the 2 distance among the total cost of the trip. 
                // we can try to compare the original dis of the 2 edge and after swap the 2 edges. 
                // but there is an issue, if we modify the original route immediately if the new route cost 
                // less than original version, then the next operation is not on the original version
                let non_swap = euclidean_distance(&cities[route[i - 1]], &cities[route[i]])
                        + euclidean_distance(&cities[route[j]], &cities[route[(j + 1) % n]]);
                let after_swap = euclidean_distance(&cities[route[i - 1]], &cities[route[j]])
                        + euclidean_distance(&cities[route[i]], &cities[route[(j + 1) % n]]);

                if non_swap - after_swap > 10.0 {
                    can_modify.push((i, j, non_swap - after_swap));
                }
            }
            // sort based on the thrid element
            can_modify.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

            let mut used = vec![false; n]; // define another vector to store the status of wether each postions has
            // been chosen
            let mut selected = vec![];

            for &(i, j, diff) in &can_modify {
                if used[i - 1] || used[i] || used[j] || used[(j + 1) % n] {
                    continue;
                }
                selected.push((i, j));
                used[i - 1] = true;
                used[i] = true;
                used[j] = true; 
                used[(j + 1) % n] = true; // mark as used for these pairs of cities
            }

            for &(i, j) in &selected {
                swap2edges(&mut route, i, j);
            }
            // println!("the number of edges inside selected {:?}", selected.len());
            let new_dis = get_tot_dis(&route, &cities);
            if dis - new_dis < 1.0 {break;}
            dis = new_dis;
        }
        (route, dis)
    })
    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .unwrap();
    
    better_res
}

// the number of cities = 1000
// the start set equals to n / 2
// Generating 1000 cities...
// Initial tour cost: 527425.31
// Sequential Version:
// Final tour cost: 25740.54
// Time taken: 122.90ms
// Optimized version 3 of multithread 2opt:
// Total cost: 258461.26
// Total duration: 797.49ms

// the start set equals to n / 10
// Generating 1000 cities...
// Initial tour cost: 515184.07
// Sequential Version:
// Final tour cost: 26574.82
// Time taken: 119.46ms
// Optimized version 3 of multithread 2opt:
// Total cost: 53942.52
// Total duration: 790.28ms

// the start set equals to n / 100
// Generating 1000 cities...
// Initial tour cost: 516172.81
// Sequential Version:
// Final tour cost: 25980.85
// Time taken: 119.74ms
// Optimized version 3 of multithread 2opt:
// Total cost: 26184.17
// Total duration: 664.60ms

// the start set equals to 3
// Generating 1000 cities...
// Initial tour cost: 522710.59
// Sequential Version:
// Final tour cost: 26179.58
// Time taken: 104.60ms
// Optimized version 3 of multithread 2opt:
// Total cost: 25898.29
// Total duration: 900.75ms


// the number of cities = 10000
// Generating 10000 cities...
// the start set equals to n / 100
// Initial tour cost: 5203408.76
// Sequential Version:
// Final tour cost: 81963.37
// Time taken: 13.94s
// Optimized version 3 of multithread 2opt:
// Total cost: 93695.99
// Total duration: 84.23s

// the start set equals to 3
// Initial tour cost: 5192890.98
// Sequential Version:
// Final tour cost: 81507.93
// Time taken: 13.92s
// Optimized version 3 of multithread 2opt:
// Total cost: 81343.65
// Total duration: 57.34s
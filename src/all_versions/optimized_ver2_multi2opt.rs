/// Multithreaded Version 3️⃣
use rayon::prelude::*;
use std::cmp::Ordering;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::all_versions::utils::*;

// after doing some experiments on the optimize version1 of multithread 2opt
// when the number of cities grows over 1000, the result is pretty bad
// although the runtime improves a lot compared to the first version of multithread
// 2opt, the output result is even worse than the initial version
// the idea of operating multi-edges swap will not cause this sharply downgrade
// on the result, in fact, it should result in a lower cost compared to the initial version
// (although I did some experiments on the initial code to change the variable to get better results
// and did not apply it on the revised version)

// Thus, I think the key bottleneck of the version 1 is that we use fully shuffled initial route for all the threads
// the 2opt is a local optimized algorithm, so compared to the sequential version, the 
// multithread version can neither get performance gain nor better result

// what about construct some route that may already have some local optimization

fn swap2edges(tour: &mut [usize], i: usize, j: usize) {
    tour[i..=j].reverse();
}

fn get_initial_route(start_point: usize, cities: &[City]) -> Vec<usize> {
    // a randomized start_position to start a greedy algotrithm to get the initial route
    let n = cities.len();
    let mut initial_route = vec![];
    let mut vis = vec![false; n]; // to mark whether a city is visited
    initial_route.push(start_point);
    vis[start_point] = true;
    let mut prev = start_point;

    for _ in 1..n {
        let nxt = (0..n).filter(|&i| !vis[i]).min_by(|&a, &b| {
            euclidean_distance(&cities[prev], &cities[a])
                    .partial_cmp(&euclidean_distance(&cities[prev], &cities[b]))
                    .unwrap()
        }).unwrap(); // the idea is that get all the distance from unvisited cities to the previous added city
        // and choose the nearest city to push back to the vector 
        initial_route.push(nxt);
        vis[nxt] = true; // this city
        prev = nxt; // like the linked list, move the pointer to the next position as the previous city
    }
    initial_route
}
#[allow(dead_code)]
pub fn multi_2opt_optimized2_V2(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
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
        
        // we may construct a better initial route
        // and to ensure that all the threads are optimized themselves without
        // doing redundent works, we may also do some randomize work on the start position
        let mut rng = thread_rng();
        let mut initial_route = if rng.gen_bool(0.5) {
            let mut tmp = (0..n).collect::<Vec<_>>();
            tmp.shuffle(&mut rng);
            tmp
        } else {
            get_initial_route(rng.gen_range(0..n), &cities)
        };
        
        let mut route = initial_route.clone(); // original shuffle for current thread
        let mut dis = compute_total_distance(&route, &cities);
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

            can_modify.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

            let mut used = vec![false; n];
            let mut selected = vec![];

            for &(i, j, _) in &can_modify {
                if used[i - 1] || used[i] || used[j] || used[(j + 1) % n] {
                    continue;
                }
                selected.push((i, j));
                used[i - 1] = true;
                used[i] = true;
                used[j] = true; 
                used[(j + 1) % n] = true;
            }

            for &(i, j) in &selected {
                swap2edges(&mut route, i, j);
            }

            let new_dis = compute_total_distance(&route, &cities);
            if dis - new_dis < 1.0 { break; }
            dis = new_dis;
        }

        (route, dis)
    })
    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .unwrap();
    
    better_res
}

// res
//  half shuffle + half local optimized initial routes

// Generating 100 cities...
// Initial tour cost: 51275.63
// Sequential Version:
// Final tour cost: 8422.33
// Time taken: 872.18µs
// Optimized version 2 of multithread 2opt:
// Total cost: 8118.07
// Total duration: 16.70ms

// Generating 200 cities...
// Initial tour cost: 108291.95
// Sequential Version:
// Final tour cost: 11259.63
// Time taken: 3.45ms
// Optimized version 2 of multithread 2opt:
// Total cost: 11399.23
// Total duration: 28.55ms

// Generating 1000 cities...
// Initial tour cost: 514295.53
// Sequential Version:
// Final tour cost: 26271.03
// Time taken: 99.56ms
// Optimized version 2 of multithread 2opt:
// Total cost: 28491.26
// Total duration: 451.21ms


// Generating 10000 cities...
// Initial tour cost: 5169210.34
// Sequential Version:
// Final tour cost: 81551.06
// Time taken: 13.12s
// Optimized version 2 of multithread 2opt:
// Total cost: 87902.14
// Total duration: 41.35s


// Generating 20000 cities...
// Initial tour cost: 10401205.85
// Sequential Version:
// Final tour cost: 115590.88
// Time taken: 56.38s
// Optimized version 2 of multithread 2opt:
// Total cost: 124416.38
// Total duration: 183.75s
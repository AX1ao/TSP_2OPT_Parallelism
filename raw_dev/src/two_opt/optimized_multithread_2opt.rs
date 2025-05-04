use rayon::prelude::*;
use std::cmp::Ordering;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::tsp::{City, compute_total_distance, euclidean_distance};

// this version to optimize the original code is try to avoid calculating the total distance
// after the swap op of each pair of edges
// the changes on the total cost after an operation to swap a pair of edges is the
// differences between the original pair of edge and current pair

// TODO: we can apply a lazy swap way that calculated all the possible pairs of swap that can
// make the result better, then choose them if the points do not overlap
// which means not exist in the previous selected range

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
pub fn multi_2opt_optimized1(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
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
        let mut rng = thread_rng();
        let mut tour: Vec<usize> = (0..n).collect();

        tour.shuffle(&mut rng); // keep these part the same
        
        let mut route = tour.clone(); // original shuffle for current thread
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
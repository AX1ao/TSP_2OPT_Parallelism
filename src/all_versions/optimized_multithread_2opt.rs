/// Multithreaded Version 2️⃣ with Lazy Swap Evaluation
use rayon::prelude::*;
use std::cmp::Ordering;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::all_versions::utils::*;

// this version to optimize the original code is try to avoid calculating the total distance
// after the swap op of each pair of edges
// the changes on the total cost after an operation to swap a pair of edges is the
// differences between the original pair of edge and current pair

// TODO: we can apply a lazy swap way that calculated all the possible pairs of swap that can
// make the result better, then choose them if the points do not overlap
// which means not exist in the previous selected range

// TODO: if there is a way to parallize the calculation of the total distance among the current shuffle

fn swap2edges(tour: &mut [usize], i: usize, j: usize) {
    tour[i..=j].reverse();
}
#[allow(dead_code)]
pub fn multi_2opt_optimized1(tour: &[usize], cities: &[City]) -> (Vec<usize>, f64) {
    let n = tour.len();
    let possibilities: Vec<(usize, usize)> = (1..n - 1)
        .into_par_iter()
        .flat_map_iter(|i| (i + 1..n).map(move |j| (i, j)))
        .collect(); 
    
    let repeat_times = 10;
    let sample_size = possibilities.len() / 2;

    let better_res = (0..32)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut tour: Vec<usize> = (0..n).collect();

            tour.shuffle(&mut rng); // keep these part the same

            let mut route = tour.clone(); // original shuffle for current thread
            let mut dis = compute_total_distance(&route, &cities); // ⬅️ updated here

            while true {
                let sampled = possibilities.choose_multiple(&mut rng, sample_size)
                    .cloned()
                    .collect::<Vec<_>>();

                let mut can_modify = vec![];

                for &(i, j) in &sampled {
                    if i == 0 || j + 1 >= n { continue; }

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

                let new_dis = compute_total_distance(&route, &cities); // ⬅️ updated here
                if dis - new_dis < 1.0 { break; }
                dis = new_dis;
            }

            (route, dis)
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    
    better_res
}

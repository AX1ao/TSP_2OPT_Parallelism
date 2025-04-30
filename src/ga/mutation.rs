// mutation.rs
// 🔁 Defines mutation operators for GA.
// Currently includes basic swap mutation.

use rand::Rng;

pub fn swap_mutation(tour: &mut Vec<usize>) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..tour.len());
    let j = rng.gen_range(0..tour.len());
    tour.swap(i, j);
}

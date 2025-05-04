// 🔄 utils.rs
// Shared utilities for TSP and GA: city generation, distance computation,
// tour shuffling, Individual struct, crossover/mutation, selection, etc.

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::HashSet;

/// Representation of a city in 2D space
#[derive(Clone, Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

/// Generate `n` random cities using a fixed seed for reproducibility
pub fn generate_cities(n: usize, seed: u64) -> Vec<City> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| City {
            x: rng.gen_range(0.0..1000.0),
            y: rng.gen_range(0.0..1000.0),
        })
        .collect()
}

/// Euclidean distance between two cities
pub fn euclidean_distance(a: &City, b: &City) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Compute total distance of a tour
pub fn compute_total_distance(tour: &[usize], cities: &[City]) -> f64 {
    tour.windows(2)
        .map(|w| euclidean_distance(&cities[w[0]], &cities[w[1]]))
        .sum::<f64>()
        + euclidean_distance(&cities[*tour.last().unwrap()], &cities[tour[0]])
}

/// Randomly shuffle a tour in-place
pub fn shuffle_tour(tour: &mut Vec<usize>) {
    let mut rng = thread_rng();
    tour.shuffle(&mut rng);
}

pub fn two_opt(tour: &[usize], cities: &[City]) -> Vec<usize> {
    let mut improved = true;
    let mut new_tour = tour.to_vec();
    while improved {
        improved = false;
        for i in 1..new_tour.len() - 2 {
            for j in i + 1..new_tour.len() - 1 {
                let a = new_tour[i - 1];
                let b = new_tour[i];
                let c = new_tour[j];
                let d = new_tour[(j + 1) % new_tour.len()];
                let before = euclidean_distance(&cities[a], &cities[b])
                           + euclidean_distance(&cities[c], &cities[d]);
                let after = euclidean_distance(&cities[a], &cities[c])
                          + euclidean_distance(&cities[b], &cities[d]);
                if after < before {
                    new_tour[i..=j].reverse();
                    improved = true;
                }
            }
        }
    }
    new_tour
}

/// Representation of a TSP solution with fitness (inverse of total distance)
#[derive(Clone)]
pub struct Individual {
    pub tour: Vec<usize>,
    pub fitness: f64,
}

impl Individual {
    pub fn new(tour: Vec<usize>, cities: &[City]) -> Self {
        let distance = compute_total_distance(&tour, cities);
        let fitness = 1.0 / distance;
        Self { tour, fitness }
    }

    pub fn distance(&self) -> f64 {
        1.0 / self.fitness
    }
}

/// Simple swap mutation for a tour
pub fn swap_mutation(tour: &mut Vec<usize>) {
    let mut rng = thread_rng();
    let i = rng.gen_range(0..tour.len());
    let j = rng.gen_range(0..tour.len());
    tour.swap(i, j);
}

/// Order Crossover (OX) operator for two parent tours
pub fn order_crossover(parent1: &[usize], parent2: &[usize]) -> Vec<usize> {
    let len = parent1.len();
    let mut rng = thread_rng();
    let (mut start, mut end) = (rng.gen_range(0..len), rng.gen_range(0..len));
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    let mut child = vec![None; len];
    let mut used: HashSet<usize> = HashSet::new();

    // Copy segment from parent1
    for i in start..=end {
        child[i] = Some(parent1[i]);
        used.insert(parent1[i]);
    }

    // Fill remaining from parent2
    let mut p2_index = 0;
    for i in 0..len {
        if child[i].is_some() {
            continue;
        }
        while used.contains(&parent2[p2_index]) {
            p2_index += 1;
        }
        child[i] = Some(parent2[p2_index]);
        used.insert(parent2[p2_index]);
    }

    child.into_iter().map(|x| x.unwrap()).collect()
}

/// Tournament selection (select best of `k` random individuals)
pub fn tournament_selection<'a>(population: &'a [Individual], k: usize) -> &'a Individual {
    let mut rng = thread_rng();
    (0..k)
        .map(|_| &population[rng.gen_range(0..population.len())])
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap()
}

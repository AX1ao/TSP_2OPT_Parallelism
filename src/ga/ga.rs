use crate::{City, compute_total_distance};
use crate::ga::individual::Individual;
use crate::ga::selection::tournament_selection;
use crate::ga::crossover::order_crossover;
use crate::ga::mutation::swap_mutation;
use crate::ga::utils::two_opt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn run_ga(cities: &[City], population_size: usize, generations: usize, mutation_rate: f64) -> Individual {
    let city_count = cities.len();
    let mut rng = thread_rng();

    // 1. Initialize population
    let mut population: Vec<Individual> = (0..population_size)
        .map(|_| {
            let mut tour: Vec<usize> = (0..city_count).collect();
            tour.shuffle(&mut rng);
            Individual::new(tour, cities)
        })
        .collect();

    // 2. Evolve over generations
    for gen in 0..generations {
        let mut new_population = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            // a) Select parents
            let parent1 = tournament_selection(&population, 5);
            let parent2 = tournament_selection(&population, 5);

            // b) Crossover
            let mut child_tour = order_crossover(&parent1.tour, &parent2.tour);

            // c) Mutation
            if rng.gen::<f64>() < mutation_rate {
                swap_mutation(&mut child_tour);
            }

            // d) No more per-child 2-opt (for speed)
            let child = Individual::new(child_tour, cities);
            new_population.push(child);
        }

        // 3. Elitism: keep best from previous generation
        let best_prev = population
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
            .clone();

        new_population[0] = best_prev;

        population = new_population;

        // Print progress
        if gen % 50 == 0 {
            let best = population
                .iter()
                .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
                .unwrap();
            println!("Generation {}: Best distance = {}", gen, best.distance());
        }
    }

    // 4. Return best individual (after final 2-opt refinement)
    let best = population
        .into_iter()
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap();

    let refined_tour = two_opt(&best.tour, cities);
    Individual::new(refined_tour, cities)
}

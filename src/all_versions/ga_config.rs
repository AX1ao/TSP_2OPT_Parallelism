/// GA Version 2️⃣ Configurable with Logging + Delayed 2-Opt
use std::fs::File;
use std::io::{BufWriter, Write};
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;
use crate::all_versions::utils::*;

#[allow(dead_code)]
pub fn run_ga_config(
    cities: &[City],
    population_size: usize,
    generations: usize,
    base_mutation_rate: f64,
    elitism_k: usize,
    refine_start: usize,
    refine_every: usize,
    top_n: usize,
) -> Individual {
    let city_count = cities.len();
    let mut rng = thread_rng();

    let early_stop_patience = 100;
    let mut best_fitness_so_far = f64::MIN;
    let mut generations_without_improvement = 0;

    let mut population: Vec<Individual> = (0..population_size)
        .map(|_| {
            let mut tour: Vec<usize> = (0..city_count).collect();
            tour.shuffle(&mut rng);
            Individual::new(tour, cities)
        })
        .collect();

    for gen in 0..generations {
        let mutation_rate = base_mutation_rate * (1.0 - gen as f64 / generations as f64);
        let mut new_population = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            let parent1 = tournament_selection(&population, 5);
            let parent2 = tournament_selection(&population, 5);
            let mut child_tour = order_crossover(&parent1.tour, &parent2.tour);

            if rng.gen::<f64>() < mutation_rate {
                swap_mutation(&mut child_tour);
            }

            let child = Individual::new(child_tour, cities);
            new_population.push(child);
        }

        // Elitism
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        for (i, elite) in population.iter().take(elitism_k).cloned().enumerate() {
            new_population[i] = elite;
        }

        population = new_population;

        // Apply 2-opt refinement after delay
        if gen >= refine_start && gen % refine_every == 0 {
            for i in 0..top_n.min(population.len()) {
                let refined = two_opt(&population[i].tour, cities);
                population[i] = Individual::new(refined, cities);
            }
        }

        // Status print every 50 generations or final
        let best = population
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap();

        if gen % 50 == 0 || gen == generations - 1 {
            println!(
                "Generation {}: Best distance = {:.4} | Mutation rate = {:.4}",
                gen,
                best.distance(),
                mutation_rate
            );
        }

        // Early stopping
        if best.fitness > best_fitness_so_far {
            best_fitness_so_far = best.fitness;
            generations_without_improvement = 0;
        } else {
            generations_without_improvement += 1;
        }

        if generations_without_improvement >= early_stop_patience {
            println!(
                "Early stopping at generation {} (no improvement in {} generations)",
                gen, early_stop_patience
            );
            break;
        }
    }

    let best = population
        .into_iter()
        .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap();

    let refined_tour = two_opt(&best.tour, cities);
    Individual::new(refined_tour, cities)
}
/// GA Version 3️⃣ Fully Parallel GA with Delayed 2-Opt
use rayon::prelude::*;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use crate::all_versions::utils::*;

#[allow(dead_code)]
pub fn run_ga_parallel(
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

        // Parallel child generation
        let new_population: Vec<Individual> = (0..population_size)
            .into_par_iter()
            .map(|_| {
                let parent1 = tournament_selection(&population, 5);
                let parent2 = tournament_selection(&population, 5);
                let mut child_tour = order_crossover(&parent1.tour, &parent2.tour);

                if rand::random::<f64>() < mutation_rate {
                    swap_mutation(&mut child_tour);
                }

                Individual::new(child_tour, cities)
            })
            .collect();

        // Elitism (sequential)
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let mut new_population = new_population;
        for (i, elite) in population.iter().take(elitism_k).cloned().enumerate() {
            new_population[i] = elite;
        }

        population = new_population;

        // Parallel 2-opt refinement on top N individuals
        if gen >= refine_start && gen % refine_every == 0 {
            population
                .par_iter_mut()
                .take(top_n)
                .for_each(|ind| {
                    let refined = two_opt(&ind.tour, cities);
                    *ind = Individual::new(refined, cities);
                });
        }

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

    let refined = two_opt(&best.tour, cities);
    Individual::new(refined, cities)
}

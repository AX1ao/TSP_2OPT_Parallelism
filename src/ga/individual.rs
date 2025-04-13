use crate::{City, compute_total_distance};

#[derive(Clone)]
pub struct Individual {
    pub tour: Vec<usize>,
    pub fitness: f64,
}

impl Individual {
    pub fn new(tour: Vec<usize>, cities: &[City]) -> Self {
        let distance = compute_total_distance(&tour, cities);
        let fitness = 1.0 / distance;
        Individual { tour, fitness }
    }

    pub fn distance(&self) -> f64 {
        1.0 / self.fitness
    }
}

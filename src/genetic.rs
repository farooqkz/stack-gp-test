use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;

use crate::individual::Individual;

pub struct Genetic {
    pub population: Vec<Individual>,
    pub props: GeneticProperties,
}

pub struct GeneticProperties {
    pub range_up: usize,
    pub range_down: usize,
    pub population_size: usize,
    pub removal_mutation_rate: f32,
    pub addition_mutation_rate: f32,
    pub cross_over_rate: f32,
}

impl Genetic {
    pub fn new(props: GeneticProperties) -> Self {
        let range_down = props.range_down;
        let range_up = props.range_up;
        let pop = props.population_size;
        Genetic {
            population: (0..pop)
                .into_par_iter()
                .map(|_| Individual::new(range_up, range_down))
                .collect(),
            props,
        }
    }

    pub fn sort_population_by_fitness(&mut self, dataset: &Vec<Vec<i32>>) {
        self.recompute_fitness_values(dataset);
        self.population.par_sort_unstable_by(|ind_a, ind_b| {
            ind_a
                .fitness()
                .partial_cmp(&ind_b.fitness())
                .unwrap_or(Ordering::Equal)
        });
    }
    
    fn recompute_fitness_values(&mut self, dataset: &Vec<Vec<i32>>) {
        self.population.par_iter_mut()
            .for_each(|ind| ind.compute_fitness(dataset));
    }

    pub fn run(&mut self, generations: usize, dataset: &Vec<Vec<i32>>) -> Vec<Vec<f64>> {
        let pop = self.props.population_size as f32;
        let mut rng = rand::thread_rng();
        let mut avg_fitness_values: Vec<f64> = vec![];
        let mut best_fitness_values: Vec<f64> = vec![];
        let mut worst_fitness_values: Vec<f64> = vec![];
        let cross_over_pop = (self.props.cross_over_rate * pop) as usize;
        let addition_mutation_pop = (self.props.addition_mutation_rate * pop) as usize;
        let removal_mutation_pop = (self.props.removal_mutation_rate * pop) as usize;
        let mut total_fitness: f32;
        for g in 0..generations {
            // The population which the operators must be done on them
            self.recompute_fitness_values(dataset);
            total_fitness = self.total_fitness();
            let mut crossover_offsprings: Vec<Individual> = self
                .population
                .par_iter()
                .take(cross_over_pop / 2)
                .map(|mother| {
                    let mut i;
                    let mut rng = rand::thread_rng();
                    let father: &Individual = loop {
                        i = rng.gen_range(0..self.population.len());
                        let probability = (self.population[i].fitness() / total_fitness) as f64;
                        if rng.gen_bool((self.population[i].fitness() / total_fitness) as f64) {
                            break &self.population[i];
                        }
                    };
                    let offsprings = mother.crossover(father);
                    return [offsprings.0, offsprings.1];
                })
                .flatten()
                .collect();
            self.population
                .par_iter_mut()
                .skip(addition_mutation_pop + rng.gen_range(0..(self.props.population_size / 4)))
                .take(removal_mutation_pop)
                .for_each(|ind| ind.mutate_remove());
            self.population
                .par_iter_mut()
                .take(addition_mutation_pop)
                .for_each(|ind| ind.mutate_add());
            self.population.append(&mut crossover_offsprings);
            self.recompute_fitness_values(dataset);
            total_fitness = self.total_fitness();
            let mut i;
            while self.population.len() > self.props.population_size {
                i = rng.gen_range(0..self.population.len());
                let probability = 1.0 - (self.population[i].fitness() / total_fitness);
                if rng.gen_bool(probability as f64) {
                    self.population.swap_remove(i);
                }
            }
            total_fitness = self.population.par_iter().map(|ind| ind.fitness()).sum();
            self.sort_population_by_fitness(dataset);
            worst_fitness_values.push(self.population[0].fitness() as f64);
            best_fitness_values.push(self.population.last().expect("Empty population!").fitness() as f64);
            avg_fitness_values.push((total_fitness / pop) as f64);
            println!("Gen: {} Pop: {}", g, self.population.len());
        }
        vec![best_fitness_values, avg_fitness_values, worst_fitness_values]
    }
    
    fn total_fitness(&self) -> f32 {
        self.population.par_iter().map(|ind| ind.fitness()).sum()
    }

    pub fn sort_population_by_complexity(&mut self) {
        self.population
            .par_sort_unstable_by(|ind_a, ind_b| ind_a.stack.len().cmp(&ind_b.stack.len()));
    }
}

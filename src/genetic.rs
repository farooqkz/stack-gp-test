use std::cmp::Ordering;
use tinyrand::{Rand, StdRand};
use rayon::prelude::*;

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
    pub reproduction_rate: f32,
}

impl Genetic {
    pub fn new(props: GeneticProperties, rng: &mut StdRand) -> Self {
        let range_down = props.range_down;
        let range_up = props.range_up;
        let pop = props.population_size;
        Genetic {
            population: (0..pop)
                .map(|_| Individual::new(range_up, range_down, rng))
                .collect(),
            props,
        }
    }

    pub fn sort_population_by_fitness(&mut self, dataset: &Vec<Vec<i32>>) {
        self.population.par_sort_unstable_by(|ind_a, ind_b| {
            ind_a
                .fitness(dataset)
                .partial_cmp(&ind_b.fitness(dataset))
                .unwrap_or(Ordering::Equal)
        });
    }

    pub fn run(&mut self, generations: usize, dataset: &Vec<Vec<i32>>, rng: &mut StdRand) {
        let pop = self.props.population_size as f32;
        for g in 0..generations {
            // The population which the operators must be done on them
            let cross_over_pop = (self.props.cross_over_rate * pop) as usize;
            let reproduction_pop = (self.props.reproduction_rate * pop) as usize;
            let addition_mutation_pop = (self.props.addition_mutation_rate * pop) as usize;
            let removal_mutation_pop = (self.props.removal_mutation_rate * pop) as usize;
            for _ in 0..(cross_over_pop / 2) {
                let mother = &self.population[rng.next_lim_usize(self.props.population_size)];
                let father = &self.population[rng.next_lim_usize(self.props.population_size)];
                let (offspring0, offspring1) = mother.crossover(father, rng);
                self.population.push(offspring0);
                self.population.push(offspring1);
            }
            for _ in 0..addition_mutation_pop {
                let i = rng.next_lim_usize(self.props.population_size);
                self.population[i].mutate_add(rng);
            }
            for _ in 0..removal_mutation_pop {
                let i = rng.next_lim_usize(self.props.population_size);
                self.population[i].mutate_remove();
            }
            self.sort_population_by_fitness(dataset);
            let mut reproduction_pop: Vec<Individual> = self.population.par_iter().take(reproduction_pop).map(|ind| ind.reproduce()).collect();
            self.population.append(&mut reproduction_pop);
            println!("{}", g);
        }
    }

    pub fn sort_population_by_complexity(&mut self) {
        self.population
            .par_sort_unstable_by(|ind_a, ind_b| ind_a.stack.len().cmp(&ind_b.stack.len()));
    }
}

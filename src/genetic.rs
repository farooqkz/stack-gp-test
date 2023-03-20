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
    pub reproduction_rate: f32,
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
        self.population.par_sort_unstable_by(|ind_a, ind_b| {
            ind_a
                .fitness(dataset)
                .partial_cmp(&ind_b.fitness(dataset))
                .unwrap_or(Ordering::Equal)
        });
    }

    pub fn run(&mut self, generations: usize, dataset: &Vec<Vec<i32>>) {
        let pop = self.props.population_size as f32;
        let mut rng = rand::thread_rng();
        for g in 0..generations {
            // The population which the operators must be done on them
            let cross_over_pop = (self.props.cross_over_rate * pop) as usize;
            let reproduction_pop = (self.props.reproduction_rate * pop) as usize;
            let addition_mutation_pop = (self.props.addition_mutation_rate * pop) as usize;
            let removal_mutation_pop = (self.props.removal_mutation_rate * pop) as usize;
            let mut cross_over_offsprings: Vec<Individual> = self
                .population
                .par_iter()
                .zip(self.population.par_iter().rev())
                .take(cross_over_pop / 2)
                .map(|(mother, father)| {
                    let offsprings = mother.crossover(&father);
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
            self.population.append(&mut cross_over_offsprings);
            self.sort_population_by_fitness(dataset);
            let mut reproduction_pop: Vec<Individual> = self
                .population
                .par_iter()
                .take(reproduction_pop)
                .map(|ind| ind.reproduce())
                .collect();
            self.population.append(&mut reproduction_pop);
            self.sort_population_by_fitness(dataset);
            self.population.drain(self.props.population_size..);
            println!("Gen: {} Pop: {}", g, self.population.len());
            println!("{:?}", self.population[0]);
        }
    }

    pub fn sort_population_by_complexity(&mut self) {
        self.population
            .par_sort_unstable_by(|ind_a, ind_b| ind_a.stack.len().cmp(&ind_b.stack.len()));
    }
}

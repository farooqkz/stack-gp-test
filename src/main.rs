pub mod genetic;
pub mod individual;
pub mod instruction;

use crate::individual::*;
use crate::instruction::Instruction;
use crate::genetic::*;

fn main() {
    let mut dataset: Vec<Vec<i32>> = (0..100).map(|i| vec![i, 2*i*i]).collect();
    let props = GeneticProperties {
        population_size: 1000,
        cross_over_rate: 0.8,
        addition_mutation_rate: 0.0075,
        removal_mutation_rate: 0.005,
        range_up: 4,
        range_down: 2,
    };
    {
        let mut stack = vec![
            Instruction::Integer(2),
            Instruction::Integer(3),
            Instruction::Sum,
            Instruction::Integer(2),
            Instruction::Neg,
            Instruction::Multiply,
        ];
        println!("Testing evaluate stack function...");
        assert!(evaluate_stack(&stack, vec![]) == -10);
        stack.push(Instruction::Integer(5));
        stack.push(Instruction::Multiply);
        assert!(evaluate_stack(&stack, vec![]) == -50);
        stack.push(Instruction::Integer(-1));
        stack.push(Instruction::Sum);
        assert!(evaluate_stack(&stack, vec![]) == -51);
        let stack = vec![
            Instruction::Duplicate,
            Instruction::Sum,
            Instruction::Multiply,
        ];
        assert!(evaluate_stack(&stack, vec![2, -2]) == -8);
        println!("Testing done it's fine :)");
    }
    {
        println!("Testing select random instruction fn...");
        println!("{:?}", select_random_instruction());
        println!("{:?}", select_random_instruction());
        println!("{:?}", select_random_instruction());
        println!("{:?}", select_random_instruction());
        println!("{:?}", select_random_instruction());
        println!("{:?}", select_random_instruction());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(0) // 0 means rayon will decide number of threads
        .build_global()
        .unwrap();
    let mut g = Genetic::new(props);
    let graphpoints = g.run(100, &dataset);
    g.sort_population_by_fitness(&dataset);
    let fitness: Vec<f32> = g.population.iter().map(|ind| ind.fitness()).collect();
    println!("{fitness:?}");
    g.sort_population_by_fitness(&dataset);
    let most_fit = &g.population.last().unwrap().stack;
    println!("Most fit: {most_fit:?}");
    println!("Let's test some dataset");
    for datapoint in dataset.iter().skip(20).take(8) {
        let i = datapoint.first().unwrap();
        let actual = datapoint.last().unwrap();
        let ind = &g.population.last().unwrap();
        println!("For {}: A {} P {}", i, actual, ind.eval(vec![*i]));
    }
    g.sort_population_by_complexity();
    let least_complex = &g.population[0].stack;
    println!("Least complex: {least_complex:?}");
    let graph = rasciigraph::plot_many(graphpoints, rasciigraph::Config::default().with_height(40));
    println!("{graph}");
}

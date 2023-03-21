pub mod genetic;
pub mod individual;
pub mod instruction;

use crate::individual::evaluate_stack;
use crate::instruction::Instruction;
use clap::{arg, command, Command};

fn cli() -> Command {
    command!().args([
        arg!(--rangeup <VALUE> "Maximum size of initial program"),
        arg!(--rangedown <VALUE> "Minimum size of initial program"),
        arg!(--pop <VALUE> "Size of population"),
        arg!(--gen <VALUE> "Number of generations"),
        arg!(--reproduction <VALUE> "Reproduction rate"),
        arg!(--crossover <VALUE> "Crossover rate"),
        arg!(--additionmutation <VALUE> "Addition mutation rate"),
        arg!(--removalmutation <VALUE> "Removal mutation rate"),
    ])
}

fn main() {
    let mut dataset: Vec<Vec<i32>> = vec![];
    for i in 0..100 {
        let i = i as i32;
        dataset.push([i, i * i + i * i].to_vec());
    }
    let matches = cli().get_matches();
    let props = genetic::GeneticProperties {
        range_up: *matches.get_one::<usize>("rangeup").unwrap_or(&4),
        range_down: *matches.get_one::<usize>("rangeup").unwrap_or(&1),
        population_size: *matches.get_one::<usize>("pop").unwrap_or(&2000),
        removal_mutation_rate: *matches.get_one::<f32>("removalmutation").unwrap_or(&0.01),
        addition_mutation_rate: *matches.get_one::<f32>("additionmutation").unwrap_or(&0.005),
        reproduction_rate: *matches.get_one::<f32>("reproduction").unwrap_or(&0.05),
        cross_over_rate: *matches.get_one::<f32>("crossover").unwrap_or(&0.9),
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
        let mut args: Vec<i32> = vec![];
        println!("Testing evaluate stack function...");
        assert!(evaluate_stack(&stack, &args) == -10);
        stack.push(Instruction::Integer(5));
        stack.push(Instruction::Multiply);
        assert!(evaluate_stack(&stack, &args) == -50);
        stack.push(Instruction::Integer(-1));
        stack.push(Instruction::Sum);
        assert!(evaluate_stack(&stack, &args) == -51);
        let stack = vec![
            Instruction::Duplicate,
            Instruction::Sum,
            Instruction::Multiply,
        ];
        args.push(2);
        args.push(-2);
        assert!(evaluate_stack(&stack, &args) == -8);
        println!("Testing done it's fine :)");
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(0)
        .build_global()
        .unwrap();
    let mut g = genetic::Genetic::new(props);
    g.run(400, &dataset);
    g.sort_population_by_fitness(&dataset);
    println!("Most fit: {:?}", g.population[0].stack);
    g.sort_population_by_complexity();
    println!("Least complex: {:?}", g.population[0].stack);
}

pub mod genetic;
pub mod individual;
pub mod instruction;

use crate::individual::*;
use crate::instruction::Instruction;
use crate::genetic::*;
use clap::{arg, command, Command};

fn cli() -> Command {
    command!().args([
        arg!(--rangeup <VALUE> "Maximum size of initial program"),
        arg!(--rangedown <VALUE> "Minimum size of initial program"),
        arg!(--pop <VALUE> "Size of population"),
        arg!(--gen <VALUE> "Number of generations"),
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
    let err_message_usize = "Invalid base 10 positive integer";
    let err_message_f32 = "Invalid positive float";
    /*
    let props = genetic::GeneticProperties {
        range_up: usize::from_str_radix(*matches.get_one::<String>("rangeup"), 10)
            .expect(err_message_usize),
        range_down: usize::from_str_radix(*matches.get_one::<String>("rangeup").unwrap_or(&"1"), 10)
            .expect(err_message_usize),
        population_size: usize::from_str_radix(
            *matches.get_one::<String>("pop").unwrap_or(&"3000"),
            10,
        )
        .expect(err_message_usize),
        removal_mutation_rate: matches
            .get_one::<String>("removalmutation")
            .unwrap_or(&"0.01")
            .parse::<f32>()
            .expect(err_message_f32),
        addition_mutation_rate: matches
            .get_one::<String>("additionmutation")
            .unwrap_or(&"0.005")
            .parse::<f32>()
            .expect(err_message_f32),
        cross_over_rate: matches
            .get_one::<String>("crossover")
            .unwrap_or("0.9")
            .parse::<f32>()
            .expect(err_message_f32),
    };
    */
    let props = GeneticProperties {
        population_size: 1000,
        cross_over_rate: 0.8,
        addition_mutation_rate: 0.0075,
        removal_mutation_rate: 0.005,
        range_up: 4,
        range_down: 2,
    };
    if props.removal_mutation_rate < 0.0 {
        panic!("{}", err_message_f32);
    }
    if props.addition_mutation_rate < 0.0 {
        panic!("{}", err_message_f32);
    }
    if props.cross_over_rate < 0.0 {
        panic!("{}", err_message_f32);
    }
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
        .num_threads(0)
        .build_global()
        .unwrap();
    let mut g = Genetic::new(props);
    let graphpoints = g.run(100, &dataset);
    g.sort_population_by_fitness(&dataset);
    let fitness: Vec<f32> = g.population.iter().map(|ind| ind.fitness()).collect();
    println!("{:?}", fitness);
    g.sort_population_by_fitness(&dataset);
    println!("Most fit: {:?}", g.population.last().unwrap().stack);
    println!("Let's test some dataset");
    for datapoint in dataset.iter().skip(20).take(8) {
        let i = datapoint.first().unwrap();
        let actual = datapoint.last().unwrap();
        let ind = &g.population.last().unwrap();
        println!("For {}: A {} P {}", i, actual, ind.eval(vec![*i]));
    }
    g.sort_population_by_complexity();
    println!("Least complex: {:?}", g.population[0].stack);
    let graph = rasciigraph::plot_many(graphpoints, rasciigraph::Config::default().with_height(40));
    println!("{}", graph);
}

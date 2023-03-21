use rand::prelude::*;

use std::collections::VecDeque;

use crate::instruction::Instruction;


#[derive(Debug, Clone)]
struct Fitness {
    pub ft: f32,
    stack: Vec<Instruction>,
}

impl Fitness {
    fn new(stack: &Vec<Instruction>) -> Self {
        Fitness {
            stack: stack.clone(),
            ft: -1.0,
        }
    }

    fn update(&mut self, dataset: &Vec<Vec<i32>>, stack: &Vec<Instruction>) {
        let mut need_update = self.ft < 0.0;
        if !need_update {
            for (this, that) in self.stack.iter().zip(stack.iter()) {
                if this != that {
                    need_update = true;
                    break;
                }
            }
        }
        if need_update {
            self.stack.clone_from(stack);
            let mut results: Vec<u32> = vec![];
            for datapoint in dataset.iter() {
                if let Some(actual) = datapoint.last() {
                    let predicted = evaluate_stack(&self.stack, &datapoint);
                    results.push(predicted.abs_diff(*actual));
                }
            }
            self.ft = results.len() as f32 / results.iter().sum::<u32>() as f32 ;
        }
    }
}

#[derive(Debug)]
pub struct Individual {
    pub stack: Vec<Instruction>,
    fitness: Fitness,
}

fn select_random_instruction() -> crate::instruction::Instruction {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => Instruction::Neg,
        1 => Instruction::Sum,
        2 => Instruction::Duplicate,
        3..=u16::MAX => Instruction::Multiply,
    }
}

impl Individual {
    pub fn new(range_up: usize, range_down: usize) -> Self {
        let mut stack: Vec<Instruction> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(range_down..range_up) {
            stack.push(select_random_instruction());
        }
        Individual { fitness: Fitness::new(&stack), stack }
    }
    
    pub fn new_from_stack(stack: &Vec<Instruction>) -> Self {
        Individual {
            stack: stack.clone(),
            fitness: Fitness::new(stack),
        }
    }
    pub fn reproduce(&self) -> Self {
        Individual {
            stack: self.stack.clone(),
            fitness: self.fitness.clone(),
        }
    }

    pub fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut rng = rand::thread_rng();
        let (new0_right, new0_left) = self.stack.split_at(rng.gen_range(0..self.stack.len()));
        let (new1_right, new1_left) = other.stack.split_at(rng.gen_range(0..other.stack.len()));
        let new1 = Individual::new_from_stack(&[new0_left, new1_right].concat());
        let new0 = Individual::new_from_stack(&[new1_left, new0_right].concat());
        (new0, new1)
    }

    pub fn mutate_add(&mut self) {
        self.stack.push(select_random_instruction());
    }

    pub fn mutate_remove(&mut self) {
        if self.stack.len() > 3 {
            self.stack.pop();
        }
    }

    pub fn eval(&self, args: &Vec<i32>) -> i32 {
        return evaluate_stack(&self.stack, args);
    }
    
    pub fn compute_fitness(&mut self, dataset: &Vec<Vec<i32>>) {
        self.fitness.update(dataset, &self.stack);
    }

    pub fn fitness(&self) -> f32 {
        self.fitness.ft
    }
}

pub fn evaluate_stack(stack: &Vec<Instruction>, args: &Vec<i32>) -> i32 {
    let stack: Vec<Instruction> = {
        let mut new_stack: Vec<Instruction> =
            args.iter().map(|arg| Instruction::Integer(*arg)).collect();
        let mut old_stack: Vec<Instruction> = stack.clone();
        new_stack.append(&mut old_stack);
        new_stack
    };
    let mut stack: VecDeque<Instruction> = VecDeque::from(stack);
    let mut operands: VecDeque<i32> = VecDeque::new();
    while stack.len() > 0 {
        if let Some(item) = stack.pop_front() {
            match item {
                Instruction::Integer(x) => {
                    operands.push_front(x);
                }
                Instruction::Multiply => {
                    if operands.len() >= 2 {
                        let item = operands.iter().take(2).product();
                        operands.drain(..=1);
                        operands.push_front(item);
                    }
                }
                Instruction::Sum => {
                    if operands.len() >= 2 {
                        let item = operands.iter().take(2).sum();
                        operands.drain(..=1);
                        operands.push_front(item);
                    }
                }
                Instruction::Neg => {
                    if let Some(x) = operands.pop_front() {
                        operands.push_front(-x);
                    }
                }
                Instruction::Duplicate => {
                    if let Some(x) = operands.pop_front() {
                        operands.push_front(x);
                        operands.push_front(x);
                    }
                }
            }
        } else {
            break;
        }
    }
    if let Some(x) = operands.pop_back() {
        return x;
    } else {
        println!("{:?}", stack);
        panic!();
    }
}

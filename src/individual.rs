use tinyrand::{Probability, Rand, RandRange, StdRand};
use std::collections::VecDeque;

use crate::instruction::Instruction;


pub struct Individual {
    pub stack: Vec<Instruction>
}

fn select_random_instruction(rng: &mut StdRand) -> crate::instruction::Instruction {
    match rng.next_lim_u16(5) {
        0 => { Instruction::Duplicate },
        1 => { Instruction::Swap },
        2 => { Instruction::Neg },
        3 => { Instruction::Sum },
        4..=u16::MAX => { Instruction::Multiply },
    }
}

impl Individual {
    pub fn new(range_up: usize, range_down: usize, rng: &mut StdRand) -> Self {
        let mut stack: Vec<Instruction> = vec![];
        for _ in 0..rng.next_range(range_down..range_up) {
            stack.push(select_random_instruction(rng));
        }
        Individual {
            stack,
        }
    }

    pub fn reproduce(&self) -> Self {
        Individual {
            stack: self.stack.clone()
        }
    }

    pub fn crossover(&self, other: &Self, rng: &mut StdRand) -> (Self, Self) {
        let point = rng.next_lim_u32(self.stack.len() as u32) as usize;
        let mut new0_left = self.stack.clone();
        let new0_right = new0_left.split_off(point);
        let point = rng.next_lim_u32(other.stack.len() as u32) as usize;
        let mut new1_left = other.stack.clone();
        let new1_right = new1_left.split_off(point);
        let new1 = Individual {
            stack: [new0_left, new1_right].concat(),
        };
        let new0 = Individual {
            stack: [new1_left, new0_right].concat(),
        };
        (new0, new1)
    }

    pub fn mutate_add(&mut self, rng: &mut StdRand) {
        self.stack.push(select_random_instruction(rng));
    }

    pub fn mutate_remove(&mut self) {
        if self.stack.len() > 0 {
            self.stack.pop();
        }
    }

    pub fn eval(&self, args: Vec<i32>) -> i32 {
        return evaluate_stack(&self.stack, args);
    }

    pub fn fitness(&self, dataset: &Vec<Vec<i32>>) -> f32 {
        let mut results: Vec<u32> = vec![];
        for datapoint in dataset.iter() {
            let mut datapoint = datapoint.clone();
            let actual = datapoint.pop().unwrap();
            let predicted = self.eval(datapoint.to_vec());
            results.push(predicted.abs_diff(actual));
        }
        results.iter().sum::<u32>() as f32 / results.len() as f32 
    }
}

fn evaluate_stack(stack: &Vec<Instruction>, args: Vec<i32>) -> i32 {
    let stack: Vec<Instruction> = {
        let mut new_stack: Vec<Instruction> = args.iter().map(|arg| Instruction::Integer(*arg)).collect();
        let mut old_stack: Vec<Instruction> = stack.to_vec().clone();
        new_stack.append(&mut old_stack);
        new_stack
    };
    let mut new_stack: Vec<Instruction> = vec![];
    loop {
        for (index, item) in stack.iter().enumerate() {
            match item {
                Instruction::Duplicate => {
                    if index > 0 {
                        new_stack.push(stack[index-1]);
                        new_stack.push(stack[index-1]);
                    }
                },
                Instruction::Swap => {
                    if index > 1 {
                        new_stack.push(stack[index-2]);
                        new_stack.push(stack[index-1]);
                    }
                },
                _ => { new_stack.push(*item); }
            }
        }
    }
    let mut new_stack = loop {
        let mut stack = new_stack;
        let mut new_stack: Vec<Instruction> = vec![]; 
        let mut instruction_seen = false;
        if !instruction_seen {
            break new_stack;
        }
    };

    if let Some(Instruction::Integer(x)) = new_stack.pop() {
        return x;
    } else {
        return 0;
    }
}

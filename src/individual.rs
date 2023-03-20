use rand::prelude::*;

use std::collections::VecDeque;

use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Individual {
    pub stack: Vec<Instruction>,
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
        Individual { stack }
    }

    pub fn reproduce(&self) -> Self {
        Individual {
            stack: self.stack.clone(),
        }
    }

    pub fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut rng = rand::thread_rng();
        let point = rng.gen_range(0..self.stack.len());
        let mut new0_left = self.stack.clone();
        let new0_right = new0_left.split_off(point);
        let point = rng.gen_range(0..other.stack.len());
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

    pub fn mutate_add(&mut self) {
        self.stack.push(select_random_instruction());
    }

    pub fn mutate_remove(&mut self) {
        if self.stack.len() > 3 {
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

pub fn evaluate_stack(stack: &Vec<Instruction>, args: Vec<i32>) -> i32 {
    let stack: Vec<Instruction> = {
        let mut new_stack: Vec<Instruction> =
            args.iter().map(|arg| Instruction::Integer(*arg)).collect();
        let mut old_stack: Vec<Instruction> = stack.to_vec().clone();
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

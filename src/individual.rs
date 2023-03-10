use std::collections::VecDeque;
use tinyrand::{Probability, Rand, RandRange, StdRand};

use crate::instruction::Instruction;

pub struct Individual {
    pub stack: Vec<Instruction>,
}

fn select_random_instruction(rng: &mut StdRand) -> crate::instruction::Instruction {
    match rng.next_lim_u16(2) {
        0 => Instruction::Neg,
        1 => Instruction::Sum,
        2..=u16::MAX => Instruction::Multiply,
    }
}

impl Individual {
    pub fn new(range_up: usize, range_down: usize, rng: &mut StdRand) -> Self {
        let mut stack: Vec<Instruction> = vec![];
        for _ in 0..rng.next_range(range_down..range_up) {
            stack.push(select_random_instruction(rng));
        }
        Individual { stack }
    }

    pub fn reproduce(&self) -> Self {
        Individual {
            stack: self.stack.clone(),
        }
    }

    pub fn crossover(&self, other: &Self, rng: &mut StdRand) -> (Self, Self) {
        let point = rng.next_lim_usize(self.stack.len());
        let mut new0_left = self.stack.clone();
        let new0_right = new0_left.split_off(point);
        let point = rng.next_lim_usize(other.stack.len());
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
    let mut stack: Vec<Instruction> = {
        let mut new_stack: Vec<Instruction> =
            args.iter().map(|arg| Instruction::Integer(*arg)).collect();
        let mut old_stack: Vec<Instruction> = stack.to_vec().clone();
        new_stack.append(&mut old_stack);
        new_stack
    };
    let mut new_stack: Vec<Instruction> = vec![];
    let mut instruction_seen = true;
    while instruction_seen {
        instruction_seen = false;
        for (_index, operator) in stack.iter().enumerate() {
            if let Instruction::Integer(_x) = operator {
            } else {
                instruction_seen = true;
            }
            match operator {
                Instruction::Multiply => {
                    if new_stack.len() >= 2 {
                        match (new_stack[0], new_stack[1]) {
                            (Instruction::Integer(x), Instruction::Integer(y)) => {
                                new_stack.pop();
                                new_stack.pop();
                                new_stack.push(Instruction::Integer(x * y));
                            }
                            _ => {
                                new_stack.push(*operator);
                            }
                        }
                    }
                }
                Instruction::Sum => {
                    if new_stack.len() >= 2 {
                        match (new_stack[1], new_stack[0]) {
                            (Instruction::Integer(x), Instruction::Integer(y)) => {
                                new_stack.pop();
                                new_stack.pop();
                                new_stack.push(Instruction::Integer(x + y));
                            }
                            _ => {
                                new_stack.push(*operator);
                            }
                        }
                    }
                }
                Instruction::Neg => {
                    if new_stack.len() >= 1 {
                        if let Some(Instruction::Integer(x)) = new_stack.pop() {
                            new_stack.push(Instruction::Integer(-x));
                        } else {
                        }
                    }
                }
                _ => {
                    new_stack.push(*operator);
                }
            }
        }
        stack.clear();
        stack.append(&mut new_stack);
    }

    if let Some(Instruction::Integer(x)) = stack.pop() {
        return x;
    } else {
        return 0;
    }
}

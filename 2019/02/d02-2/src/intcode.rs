use std::include_str;

#[derive(Debug, PartialEq)]
struct Code(Vec<usize>);

impl Code {
    pub fn add(&mut self, a: &usize, b: &usize, index: &usize) {
        let a: usize = self.0[*a];
        let b: usize = self.0[*b];
        let sum: usize = a + b;
        self.0[*index] = sum;
    }

    pub fn multiply(&mut self, a: &usize, b: &usize, index: &usize) {
        let a: usize = self.0[*a];
        let b: usize = self.0[*b];
        let product: usize = a * b;

        self.0[*index] = product;
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) -> Option<usize> {
        match instruction {
            Instruction::Multiply(a, b, i) => self.multiply(a, b, i),
            Instruction::Add(a, b, i) => self.add(a, b, i),
            Instruction::Finish => return Some(self.0[0]),
        };
        None
    }

    pub fn get_chunk(&self, i: usize) -> &[usize] {
        let lists: Vec<&[usize]> = self.0.chunks(4).collect();
        lists[i]
    }

    pub fn amount_of_instructions(&self) -> usize {
        let lists: Vec<&[usize]> = self.0.chunks(4).collect();
        lists.len()
    }
}

enum Instruction {
    Multiply(usize, usize, usize),
    Add(usize, usize, usize),
    Finish,
}

pub fn intcode(noun: usize, verb: usize) -> usize {
    let puzzle_input = include_str!("../../input.txt");
    let numbers: Vec<usize> = puzzle_input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut code = numbers_to_code(numbers, noun, verb);

    for i in 0..code.amount_of_instructions() {
        let instruction = list_to_instruction(code.get_chunk(i));
        let result = code.apply_instruction(&instruction);
        if let Some(n) = result {
            return n;
        }
    }
    panic!("No opcode \"99\" found");
}

fn list_to_instruction(list: &[usize]) -> Instruction {
    if list.len() == 1 {
        return Instruction::Finish;
    }

    let (action, a, b, i) = (list[0], list[1], list[2], list[3]);
    match action {
        1 => Instruction::Add(a, b, i),
        2 => Instruction::Multiply(a, b, i),
        99 => Instruction::Finish,
        _ => panic!("unknown action: {}", action),
    }
}

fn numbers_to_code(ns: Vec<usize>, noun: usize, verb: usize) -> Code {
    let mut numbers = ns;
    numbers[1] = noun;
    numbers[2] = verb;
    Code(numbers)
}

#[cfg(test)]
mod Tests {
    use super::*;

    #[test]
    fn test_intcode() {
        assert_eq!(intcode(12, 2), 3765464);
    }
}

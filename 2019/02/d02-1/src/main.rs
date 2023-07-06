use std::include_str;

#[derive(Debug, PartialEq)]
pub struct Code(Vec<usize>);

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

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Multiply(a, b, i) => self.multiply(a, b, i),
            Instruction::Add(a, b, i) => self.add(a, b, i),
            Instruction::Finish => {
                println!("-- program finished ---\n\nindex 0 is: {}", self.0[0]);
                panic!("bla");
            }
        };
    }

    pub fn run_instructions(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.apply_instruction(&instruction);
        }
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

pub enum Instruction {
    Multiply(usize, usize, usize),
    Add(usize, usize, usize),
    Finish,
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

// fn code_to_instructions(code: &Code) -> Vec<Instruction> {
//     let lists: Vec<&[i32]> = code.0.chunks(4).collect();
//     dbg!(&lists);
//     let mut result: Vec<Instruction> = Vec::new();

//     for list in lists {
//         result.push(list_to_instruction(list));
//     }

//     result
// }

fn numbers_to_code(ns: Vec<usize>) -> Code {
    let mut numbers = ns;
    numbers[1] = 12;
    numbers[2] = 2;
    Code(numbers)
}

fn main() {
    let puzzle_input = include_str!("../../input.txt");
    let numbers: Vec<usize> = puzzle_input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut code = numbers_to_code(numbers);

    for i in 0..code.amount_of_instructions() {
        let instruction = list_to_instruction(code.get_chunk(i));
        code.apply_instruction(&instruction);
        dbg!(&code.0);
    }
    // dbg!(code.0);
    // code.run_instructions(instructions);
}

#[cfg(test)]
mod Tests {
    use crate::Code;

    #[test]
    fn test_add() {
        let mut code = Code(vec![1, 2, 3]);
        code.add(&2, &2, &0);
        assert_eq!(code.0, vec![5, 2, 3]);
    }

    #[test]
    fn test_multiply() {
        let mut code = Code(vec![1, 2, 3]);
        code.multiply(&3, &2, &0);
        assert_eq!(code.0, vec![7, 2, 3]);
    }
}

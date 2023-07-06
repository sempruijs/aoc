use std::include_str;
use std::process;

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
                process::exit(0);
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

pub fn list_to_instruction(list: &[usize]) -> Instruction {
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
//     let mut result: Vec<Instruction> = Vec::new();

//     for list in lists {
//         result.push(list_to_instruction(list));
//     }

//     result
// }

pub fn numbers_to_code(ns: Vec<usize>) -> Code {
    let mut numbers = ns;
    numbers[1] = 12;
    numbers[2] = 2;
    Code(numbers)
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
